# Current limitations

Many shortcuts, hacks and workarounds have been made to be able to deliver a minimum working version before the submission deadline:

- **Chainlink External Initiator**: As I didn't manage to get the external initiator to work, I had to reproduce its behaviour in a very quick and dirty NodeJS script in order to trigger jobs in Chainlink Nodes.
My initial plan was to use Offchain workers, but it seems that they are [not working in parachains](https://substrate.stackexchange.com/questions/2597/offchain-workers-in-parachain) _(well I didn't succeed)_.
- **Feed API**: Only the _getLatestRoundData_ method has been implemented. [Others methods](https://docs.chain.link/docs/price-feeds-api-reference/) need to be added, as well as [Feed Registry](https://docs.chain.link/docs/feed-registry/).
- **Job and Feed association**: the Chainlink jobs ID to trigger need to be defined when the feed is created on chain. For the hackathon, fixed ID have been used.
- **Initial job triggering** is done manually after the feed creation.
- **Tokenomics** have been completely forgotten for the hackathon scope.
- And probably a lot more items that I don't remember when writing this list ;)