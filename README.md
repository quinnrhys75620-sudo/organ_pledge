# organ_pledge

## Project Title
organ_pledge

## Project Description
organ_pledge is a Soroban smart contract that lets any user record a voluntary
organ donation pledge on the Stellar blockchain, witnessed by a second party,
and revoke that pledge at any time with a reason. Today, organ donor registries
are scattered across hospitals, paper cards, and driver-license databases that
do not talk to each other, and donors have no portable, tamper-proof way to
prove their wishes. By anchoring pledges on-chain, organ_pledge gives pledgers
self-sovereign control over their decision and gives hospitals a single,
auditable registry to consult in critical moments.

## Project Vision
Our long-term vision is a globally accessible, donor-controlled organ
pledge registry that any hospital, clinic, or transplant coordinator can
verify in seconds, regardless of the donor's country of residence. We want
organ donation intent to be as portable and trustworthy as a digital identity,
removing friction for grieving families and accelerating the matching of
donors to recipients. organ_pledge is the first building block: a minimal,
honest on-chain registry that respects the donor's autonomy and is auditable
by anyone.

## Key Features
- **Self-sovereign pledges**: A pledger signs their own pledge on-chain via
  `require_auth`, so only they can create or revoke a record under their
  address.
- **Witnessed consent**: Every pledge must include a witness address, and the
  witness must also sign the transaction, mirroring the two-party consent
  pattern used in real donation paperwork.
- **Multi-organ support**: The `make_pledge` function accepts a list of
  organ symbols (e.g. `Kidney`, `Liver`, `Cornea`, `Heart`), so one pledge can
  cover multiple organs.
- **Revocable at any time**: The pledger can call `revoke_pledge` with a
  reason symbol (`medical`, `personal`, `family`, ...) to deactivate the
  record. Revocation does not delete history; it flips an `active` flag and
  stores the reason.
- **Public, read-only registry**: Hospitals and coordinators can call
  `is_active` and `get_pledge` to query any pledger's status without needing
  any special authorization.
- **No funds at risk**: The contract never moves XLM or any token. It is
  purely a data registry, so a deployment cannot lose user funds even if
  the contract is exploited.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** healthcare dApp — see `contracts/organ_pledge/src/lib.rs` for the full organ_pledge business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CBZCFFWQMIZZ7UNUB6PR7PS4IKRZ4S7BYWU3RCIJHA2S7ATK4SC2BG2N`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/979b7440a4e2ca1d89234100511089885fcf07e0aec70c9620b22c80e90ef8dc`

## Future Scope
- **Off-chain identity binding**: Pair each wallet address with verified
  real-world identity (e.g. via Freighter + a KYC oracle) so hospitals can
  trust that an on-chain pledge maps to a real person.
- **Hospital-side registry views**: Add authorized-hospital roles and a
  bulk `list_pledges` view, so transplant coordinators can pull the full
  active-pledge registry for a region.
- **Time-locked and beneficiary pledges**: Support specifying a beneficiary
  (e.g. a family member with a matching condition) and time-locked pledges
  that only become effective after a confirm-by date.
- **Cross-chain mirroring**: Mirror the registry state to other chains
  (e.g. via a simple bridge) so global hospitals can verify pledges without
  running a Stellar node.
- **Privacy layer**: Encrypt the organ list and witness identity at rest,
  with a hospital-side decryption key, so the public ledger does not leak
  sensitive medical data.
- **Frontend dApp**: A small web frontend (Freighter + Stellar SDK) that
  walks a pledger through pledging, viewing, and revoking, and a hospital
  dashboard for registry lookups.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `organ_pledge` (healthcare)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
