
# Openbrush-Governance


The idea is to create a governance module as an extension of the open-brush library.

It is made up of different modules and the first version will allow you to create simple DAOs managed by a group of members (with an admin) where it will be possible to create and vote on-chain proposals:

- governor: the core contract that contains all the logic and primitives. It is abstract and requires choosing one of each of the modules below, or custom ones.

Votes modules determine the source of voting power, and sometimes quorum number.

- governor_voting_group: extracts voting weight from a group of members and controls that the proposer and the voter are part of that dao.

Counting modules determine valid voting options.    

- governor_counting_simple: simple voting mechanism with 3 voting options: Against, For and Abstain.



