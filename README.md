# EssenSwap

EssenSwap is a limit orderbook-based exchange built on the Essential blockchain.

In EssenSwap, the bids and asks are submitted as constraints, as required by Essential's declarative virtual machine. Every time there is a positive spread, an external solver bot is created that matches bids and asks and submits a solution to propose a new state of the system, viz., the updated balances of the bidders and askers.

Branch mater contains the front-end, backend, and contract logic.
Branch solver contains the solver.
