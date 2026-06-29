#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol, Vec};

/// The `OrganPledge` contract stores voluntary organ donation pledges
/// on the Stellar / Soroban network. A pledger records the list of
/// organs they wish to donate together with a witness address. The
/// pledge can later be revoked by the pledger with a free-text reason.
/// Hospitals and other authorized parties can read the registry to
/// check the status of a pledge for a given pledger.
#[contract]
pub struct OrganPledge;

/// On-chain representation of a single pledge. Stored inside the
/// contract's instance storage under the `pledges` key.
#[contracttype]
#[derive(Clone)]
pub struct PledgeData {
    /// The set of organs the pledger has pledged to donate
    /// (e.g. `Kidney`, `Liver`, `Heart`, `Cornea`).
    pub organs: Vec<Symbol>,
    /// The address of the witness who attests the pledge.
    pub witness: Address,
    /// Whether the pledge is currently active. Becomes `false`
    /// after the pledger calls `revoke_pledge`.
    pub active: bool,
    /// The reason provided when the pledge was revoked.
    /// Defaults to the symbol `none` for an active pledge.
    pub reason: Symbol,
}

#[contractimpl]
impl OrganPledge {
    /// Record a new organ donation pledge.
    ///
    /// Both the `pledger` and the `witness` must authorize the
    /// transaction. The `organs` vector must contain at least one
    /// organ symbol. If a pledge already exists for this pledger
    /// and is still active, the call is rejected.
    pub fn make_pledge(
        env: Env,
        pledger: Address,
        organs: Vec<Symbol>,
        witness: Address,
    ) {
        // Both parties must sign the transaction.
        pledger.require_auth();
        witness.require_auth();

        // A pledge without organs does not make sense.
        if organs.is_empty() {
            panic!("Pledge must include at least one organ");
        }

        // Load the existing registry, creating it on first use.
        let mut pledges: Map<Address, PledgeData> = env
            .storage()
            .instance()
            .get(&"pledges")
            .unwrap_or_else(|| Map::new(&env));

        // Reject duplicate active pledges from the same pledger.
        if let Some(existing) = pledges.get(pledger.clone()) {
            if existing.active {
                panic!("Pledger already has an active pledge");
            }
        }

        let data = PledgeData {
            organs: organs.clone(),
            witness: witness.clone(),
            active: true,
            reason: Symbol::new(&env, "none"),
        };

        pledges.set(pledger.clone(), data);
        env.storage().instance().set(&"pledges", &pledges);
    }

    /// Revoke an existing pledge. Only the original pledger may
    /// revoke, and they must provide a `reason` symbol (e.g.
    /// `medical`, `personal`, `family`). An already-revoked
    /// pledge cannot be revoked again.
    pub fn revoke_pledge(env: Env, pledger: Address, reason: Symbol) {
        pledger.require_auth();

        let mut pledges: Map<Address, PledgeData> = env
            .storage()
            .instance()
            .get(&"pledges")
            .unwrap_or_else(|| Map::new(&env));

        let mut data = pledges
            .get(pledger.clone())
            .unwrap_or_else(|| panic!("No pledge found for this pledger"));

        if !data.active {
            panic!("Pledge is already revoked");
        }

        data.active = false;
        data.reason = reason.clone();

        pledges.set(pledger.clone(), data);
        env.storage().instance().set(&"pledges", &pledges);
    }

    /// Returns `true` if the pledger currently has an active pledge,
    /// and `false` otherwise (including when no pledge exists).
    pub fn is_active(env: Env, pledger: Address) -> bool {
        let pledges: Map<Address, PledgeData> = env
            .storage()
            .instance()
            .get(&"pledges")
            .unwrap_or_else(|| Map::new(&env));

        match pledges.get(pledger) {
            Some(data) => data.active,
            None => false,
        }
    }

    /// Look up a pledger's pledge. Returns a tuple of
    /// `(organs, witness, active)` so hospitals and other
    /// authorized parties can read the registry.
    pub fn get_pledge(env: Env, pledger: Address) -> (Vec<Symbol>, Address, bool) {
        let pledges: Map<Address, PledgeData> = env
            .storage()
            .instance()
            .get(&"pledges")
            .unwrap_or_else(|| Map::new(&env));

        let data = pledges
            .get(pledger)
            .unwrap_or_else(|| panic!("No pledge found for this pledger"));

        (data.organs, data.witness, data.active)
    }
}
