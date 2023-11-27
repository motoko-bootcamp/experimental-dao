# Bounty System
## What & Why
- Bounties are a mechanism for rewarding people who contribute to a DAO.
- Bounties contain staked tokens.
- When a bounty is collected the tokens that are staked in it are transferred to the collector.
- Bounties are tied to proposals and consecutively, their completion criterion is derived from said proposal.
    - The exact condition depends on the type of the proposal.
- Bounties can have expiration dates.
- There is a one to many relation between proposals and bounties.

## GitHub Issues
Whenever an issue is created on GitHub, a webhook is triggered and a new proposal is generated in the DAO. Members who find the issue useful and want to incentivize developers to get it done can create bounties tied to the completion of the bounty. Once a member submits a successful pull request to the issue. The issue changes its status to resolved and the submitter collects all the bounties tied to the issue.

## Reward vs. Time
Using the bounty properties as described above, a reward vs. time chart can be created. A member could use this chart to plan their work. Since a proposal can be linked to many bounties, and each bounty can have a different expiration date it is important for members to understand exactly how much they would receive if they submit their work at any point in the future.
