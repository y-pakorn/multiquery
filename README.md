# multiquery

On-chain query batcher for CosmWasm. Similar to [SCB 10X](https://github.com/scb-10x)'s [multicall](https://github.com/scb-10x/multicall) contract, but supports any serializable query request, not limited to WASM queries.

## Example

On `bombay-12` network, use the following query:

```json
[
  {
    "bank": {
      "all_balances": {
        "address": "terra19dtgj9j5j7kyf3pmejqv8vzfpxtejaypgzkz5u"
      }
    }
  },
  {
    "wasm": {
      "smart": {
        "contract_addr": "terra12hgwnpupflfpuual532wgrxu2gjp0tcagzgx4n",
        "msg": "eyJ0b2tlbl9pbmZvIjp7fX0="
      }
    }
  }
]
```

where `eyJ0b2tlbl9pbmZvIjp7fX0=` is the base64 encoding of the query message `{"token_info":{}}`.

This returns the response:

```json
[
  {
    "success": true,
    "data": "eyJhbW91bnQiOlt7ImRlbm9tIjoidWx1bmEiLCJhbW91bnQiOiI2MjI2NjMwOTI5MzIifSx7ImRlbm9tIjoidXVzZCIsImFtb3VudCI6IjE2OTUxODkxMTQzODYzMiJ9XX0="
  },
  {
    "success": true,
    "data": "eyJuYW1lIjoiTUFSUyIsInN5bWJvbCI6Ik1BUlMiLCJkZWNpbWFscyI6NiwidG90YWxfc3VwcGx5IjoiMTAwMDAwMDAwMDAwMDAwMCJ9"
  }
]
```

The base64-encoded results can be decoded as:

```json
[
  {
    "success": true,
    "data": {
      "amount": [
        {
          "denom": "uluna",
          "amount": "622663092932"
        },
        {
          "denom": "uusd",
          "amount": "169518961438632"
        }
      ]
    }
  },
  {
    "success": true,
    "data": {
      "name": "MARS",
      "symbol": "MARS",
      "decimals": 6,
      "total_supply": "1000000000000000"
    }
  }
]
```

See [`./scripts/3_query_example.ts`](./scripts/3_query_example.ts) for details.

## Deployment

| Network    | Address                                                                                                                            |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Columbus-5 | [`terra1swrywzkphty20e2uzpy582xu836luw0e5yp25m`](https://terrasco.pe/mainnet/address/terra1swrywzkphty20e2uzpy582xu836luw0e5yp25m) |
| Bombay-12  | [`terra1t5twwglq9vlmf0pz8yadmd6gr6es2gfc4fkjww`](https://terrasco.pe/testnet/address/terra1t5twwglq9vlmf0pz8yadmd6gr6es2gfc4fkjww) |

## License

Contents of this repository are open source under [GNU General Public License v3](https://github.com/scb-10x/multicall) or later.
