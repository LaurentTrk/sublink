![](./images/sublink_text.png)

A [Substrate](https://substrate.io/) Parachain connected to [Chainlink](https://chain.link/) nodes, to bring accurate price feeds to [ink!](https://paritytech.github.io/ink-docs/) contracts and others parachains.

Personal project for the [Chainlink Spring 22 Hackathon](https://chain.link/hackathon).

> **Disclaimer**: This project is a hackathon project, and should be treated as is. It is obviously not ready to be used in any parachain, and is more like a proof of concept.
## Inspiration

During my first [Chainlink Hackathon](https://devpost.com/software/ki-dot-a-substrate-based-blockchain-to-help-micro-funding) in 2020, I had to setup, debug and update the existing [ChainLink Polkadot bridge](https://github.com/smartcontractkit/chainlink-polkadot).

It was not so easy, and it seems that things did not change very much, and it's still very difficult and tedious to connect Chainlink nodes to a Substrate chain.

Maybe that's why no Polkadot/Kusama parachain is using Chainlink as an Oracle in 2022... _(well I did not find any)_

Maybe it's time to change that, and use the power of native cross chain messaging and parachains specialization to bring accurate price feeds to Substrate in a simpler way.

That's what the SubLink project is about :)

## What it does

SubLink is a [substrate parachain](https://wiki.polkadot.network/docs/learn-parachains), connected to some Chainlink nodes, and configured to retrieve and store asset price (aka [Price feeds](https://docs.chain.link/docs/using-chainlink-reference-contracts/)) from these nodes. The bridge configuration between Chainlink nodes and the parachain is done only for the SubLink parachain. Price values are retrieved from different external sources, and aggregated/consolidated on chain.

As a parachain, SubLink is able to send these price feeds to others parachains connected to the same relay chain. This is done through the exchange of messages following the [XCM format](https://wiki.polkadot.network/docs/learn-crosschain), and the others parachains don't need to get connected to any Chainlink nodes.

SubLink is also able to provide the price feeds inside [ink! smartcontracts](https://ink.substrate.io/), using a dedicated [ink! chain extension](https://ink.substrate.io/macros-attributes/chain-extension/). This way, getting a token price value is as easy as doing it [with Solidity](https://docs.chain.link/docs/get-the-latest-price/#solidity).

![](./images/sublink_parachain.png)

## How I built it

### Play with the Chainlink Price feeds pallet

Since 2020, the Chainlink Polkadot bridge has been enhanced to include a [pallet dedicated to price feeds](https://github.com/smartcontractkit/chainlink-polkadot/tree/master/pallet-chainlink-feed).

I dedicated some time to understand, update and experience this new pallet, and to get my first price feed updated on a substrate node.

### Convert my local chain to a parachain

As I needed cross chains messaging, I had to [convert my local chain to a parachain](https://docs.substrate.io/how-to-guides/v3/parachains/convert/), and setup a [relay chain](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/) to connect to.

### Get price feeds in ink! contract

My ultimate goal was to get price feed in ink! smartcontract, so I decided to first check I will be able to do it on my new parachain before introducing XCM.

I looked at some examples of ink! chain extension on GitHub to learn how this thing work, and build a minimalist chain extension to bridge the Chainlink Price feed pallet to ink! contract.

### Implement cross chains messaging for price feeds

With the use of [this tutorial](https://medium.com/oak-blockchain/tutorial-polkadot-cross-chain-message-passing-xcmp-demo-with-ping-pallet-f53397158ab4), I crafted a new SubLink XCM pallet to deal with message exchange to carry price request and values.

I added a second parachain (Defi Example Parachain) to be able to test this new pallet, and exchange price feeds between this parachain and the SubLink parachain.

### Get price feeds in ink! contract but through XCM

The final step was to use the previous ink! chain extension in the Defi Example Parachain: instead of using the Chainlink pallet, the extension relies on a SubLink Parachain Oracle that exposes the same interface of the Chainlink Pallet, but uses the SubLink XCM pallet to get the price feeds from the SubLink parachain.

### Putting all things together online

Well, this project needs a lot of elements to get the simpliest use case ready :

- At least one [chainlink node](https://chainlink.ltk.codes/) with 3 jobs to get prices from 3 different sources
- The [relay chain](https://sublink.ltk.codes/?rpc=wss%3A%2F%2Frelaychain.ltk.codes#/explorer) with 2 validators
- The [SubLink parachain](https://sublink.ltk.codes/?rpc=wss%3A%2F%2Fsublinkchain.ltk.codes#/explorer) with 2 collators
- The [Defi Example parachain](https://sublink.ltk.codes/?rpc=wss%3A%2F%2Fdefichain.ltk.codes#/explorer) with one collator
- A custom Polkadot JS apps that displays only the 3 chains
- A custom [Contracts UI](https://contracts.ltk.codes/) application to connect to the Defi Example parachain
- We need 3 differents Substrate Adapters to update prices on the SubLink chain from these 3 jobs
- An external initiator to trigger Chainlink jobs
- An Nginx reverse proxy to rule them all
- A [Gitpod](https://gitpod.io/) environment to demonstrate the ink! contract build online
  
All these elements have been deployed in a cloud [Kubernetes](./k8s/) cluster.

### Technical overview

![](./images/SubLink%20Components.png)

For technical reasons, the source code has been splitted in several repositories:
- [SubLink Parachain](https://github.com/LaurentTrk/sublink)
- [Chainlink Pricefeed Pallet Updated](https://github.com/LaurentTrk/chainlink-polkadot/tree/sublink)
- [SubLink Pallets](https://github.com/LaurentTrk/sublink-pallets)
- [Defi Example Parachain](https://github.com/LaurentTrk/sublink-defichain)
- [ink! Sample Contract](https://github.com/LaurentTrk/sublink-defi-contract)
- [Chainlink Substrate Adapter Updated](https://github.com/LaurentTrk/substrate-adapter/tree/sublink)
- [Contracts UI](https://github.com/LaurentTrk/contracts-ui/tree/sublink)
- [Polkadot JS Apps](https://github.com/LaurentTrk/polkadot-js-apps/tree/sublink)

## Challenges I ran into

The main challenge I faced was the time constraint, as I started the hackathon a little bit too late, with only a couple of spare time days.

So I had to do it quickly, and had to make numerous [shortcuts, hacks and workarounds](./limitations.md) :(

Technical challenges were related to the use of new Substrate features like XCM and ink! chain extensions.

## What I learned

During this hackathon, I learned a lot about Parachains and XCM. It was a good way to deep dive into these stuffs.


## What's next for SubLink

There is still a huge work to do to have a minimal viable parachain, and some concerns need to be adressed (_non limitated list_):

- Tokenomics is part of the game: how incentivize chainlink operators to connect to the SubLink parachain ? Is the Oracle service provided by SubLink a free service for others parachains ?
- Adapters and External initiator deployment need to be simplified and improved
- To be honest, I don't know how this will/could scale with hundreds of nodes reporting hundreds of feeds dispatched on hundreds of parachains...
- Feed update : need to find a way to update price values only when needed, both at the report side (from Chainlink nodes, as it's done on Ethereum) and for the XCM dispatching

This project could become a common good parachain for every others parachain on Kusama/Polkadot, bringing asset prices to everyone very easily.
We could imagine having democratic features to select nodes or new feeds to be available, or even integrate others Oracle.
A sort of meta Substrate Oracle, but sounds like I should not say that in a Chainlink hackathon right ? ;)

Well, I cant' predict the future, but I can say I definitely enjoyed these few days spent in this Chainlink/Polkadot world :pray:

Thank you for reading me.
