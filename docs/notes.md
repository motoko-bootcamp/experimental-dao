# PR Restriction System
In this system members are restricted on which PRs they are allowed to vote on. A PR is accepted only after all allowed members have voted.

## PR Level System
This system describes when and how a member can vote on PRs.
- PRs are split into categories based on their complexity and impact.
    - The level is set by the member who submitted the PR.
    - Admins can override the level.
- Levels are specified by bit flatgs.

|level|value|
|---|---|
| minor change | `0b00001` |
| small feature | `0b00010` |
| large feature | `0b00100` |
| breaking change | `0b01000` |
| core change | `0b10000` |

### Interaction w/ Experience: Level Mask
- Each member has a _level mask_ that is determined by their experience.
- The _level mask_ specifies which PRs a member is allowed to vote on.

| experience | level mask | value |
| --- | --- | --- |
| 0 | newbie | `0b00001` |
| 100 | novice | `0b00011` |
| 200 | junior | `0b01111` |
| 1000 | senior | `0b11111` |

For example, a person with `150` experience will have a level mask of _novice_, and will be allowed to vote on minor changes and small features.
```
novice
    = 0b00011
    = 0b00010 | 0b00001
    = minor change | small feature
```

## Personalization
Members might not want to vote on every PR, even if they have the right to. For example a senior would prefer to vote only on breaking changes, or novice might want to go on vacation and not vote on anything all. Members can choose on which PRs they want to vote on using their _personal mask_.

The combination of the two masks describes the intersection of what the member wants to vote on and what they are allowed to vote on.

For example consider the following member:
```
Name         : Alice
Exp          : 1000
level mask   : 0b11111 (senior)
personal mask: 0b11000
---
voting on    : 0b11000
               = 0b01000 | 0b10000
               = breaking change | core change
```
In this example despite having a level mask of senior, which allows voting on every possible PR. Alice will also vote on breaking and core changes.

Lets look at another example:
```
Name         : Bob
Exp          : 150
level mask   : 0b00011 (novice)
personal mask: 0b11111
---
voting on    : 0b00011
               = 0b00001 | 0b10010
               = minor change | small feature
```
In this example, Bob has set its personal mask to `0b11111`. That can basically can be though of as _"I want to vote on everything that I am allowed to"_.
