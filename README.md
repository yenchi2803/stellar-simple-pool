# Soroban Simple Poll Contract

Welcome! This repository contains a basic "Simple Poll" smart contract built using Rust for the [Soroban](https://developers.stellar.org/docs/build/smart-contracts) platform on the Stellar network.

The purpose of this project is to provide a clear, beginner-friendly example of how to build, deploy, and interact with a stateful smart contract on Soroban. It intentionally avoids complex logic to focus on the fundamentals of storage, functions, and deployment.

## What is Simple Poll?

This contract allows anyone to cast a vote for one of two options: "Option A" or "Option B". It keeps a running tally of the votes for each option.

## Features

The contract exposes the following functions that can be invoked:

* **`vote_a()`**: Increments the vote count for "Option A" by one.
* **`vote_b()`**: Increments the vote count for "Option B" by one.
* **`get_votes()`**: A read-only function that returns the
  current vote counts as a pair `(votes_a, votes_b)`.
* **`reset()`**: A function to reset both vote counts back to zero.

## Technology Stack

* [**Rust**](https://www.rust-lang.org/): The programming language used to write the smart contract.
* [**Soroban SDK**](https://crates.io/crates/soroban-sdk): The official Soroban SDK for Rust.
* [**Stellar Testnet**](https://www.google.com/search?q=https://developers.stellar.org/docs/fundamentals-and-concepts/testnet-and-pubnet): The public test network where this contract is designed to be deployed.
