#
# Accounts public key query
#

POST {{url}}
```graphql
{
  accounts(
    query: {
      publicKey: "B62qj8KB2fk59NkV4VuoTkVXHjw8VJzC3ybKrWo7zuDC9xTiWXPygEe"
    }
  ) {
    publicKey
    balance
    username
    nonce
    delegate
    timeLocked
    is_genesis_account
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 1

# only datum
jsonpath "$.data.accounts[0].publicKey" == "B62qj8KB2fk59NkV4VuoTkVXHjw8VJzC3ybKrWo7zuDC9xTiWXPygEe"
jsonpath "$.data.accounts[0].balance" == 720020000000
jsonpath "$.data.accounts[0].nonce" == 0
jsonpath "$.data.accounts[0].delegate" == "B62qj8KB2fk59NkV4VuoTkVXHjw8VJzC3ybKrWo7zuDC9xTiWXPygEe"
jsonpath "$.data.accounts[0].timeLocked" == false
jsonpath "$.data.accounts[0].is_genesis_account" == true

duration < 100

#
# Accounts delegate query
#

POST {{url}}
```graphql
{
  accounts(
    query: {
      delegate: "B62qj8KB2fk59NkV4VuoTkVXHjw8VJzC3ybKrWo7zuDC9xTiWXPygEe"
    }
  ) {
    publicKey
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 7

# only datum
jsonpath "$.data.accounts[0].publicKey" == "B62qkHHBnLpwcg1u4jgajoUUoSXHxMFEfZ9ji3JRbzftY6gTmuYZAxH"
jsonpath "$.data.accounts[1].publicKey" == "B62qp1MrxFnKMUEgAcWq2AJscVJTVzpSZHVJjgf6GGS2fcJTDU5LhVS"
jsonpath "$.data.accounts[2].publicKey" == "B62qnQCf7ccbb8f39jDzKT8kRNq2939EpPaFGpaQ4agUuy5ozytU79a"
jsonpath "$.data.accounts[3].publicKey" == "B62qoTaYnDYrtmver4jnkgJbFU6iohjtALTU4ebf4yCKrxFFHd9Dimh"
jsonpath "$.data.accounts[4].publicKey" == "B62qoj4NpnbGSLV62tu2UbTRFV7Ec8kYcQW6H16BfNYhHstBMiahmbi"
jsonpath "$.data.accounts[5].publicKey" == "B62qneDu6K65UhAuf7YCXADJWtgJh1y9xk4nEtGKggLaTr2M3Fbj3iR"
jsonpath "$.data.accounts[6].publicKey" == "B62qj8KB2fk59NkV4VuoTkVXHjw8VJzC3ybKrWo7zuDC9xTiWXPygEe"

duration < 500


#
# Account public key timing query
#

POST {{url}}
```graphql
{
  accounts(
    query: {
      publicKey: "B62qoTaYnDYrtmver4jnkgJbFU6iohjtALTU4ebf4yCKrxFFHd9Dimh"
    }
  ) {
    publicKey
    balance
    username
    nonce
    delegate
    timeLocked
    timing {
      initial_minimum_balance
      cliff_time
      cliff_amount
      vesting_period
      vesting_increment
    }
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 1

# account info
jsonpath "$.data.accounts[0].publicKey" == "B62qoTaYnDYrtmver4jnkgJbFU6iohjtALTU4ebf4yCKrxFFHd9Dimh"
jsonpath "$.data.accounts[0].balance" == 502777775000000
jsonpath "$.data.accounts[0].username" == "Mina Foundation"
jsonpath "$.data.accounts[0].nonce" == 0
jsonpath "$.data.accounts[0].delegate" == "B62qj8KB2fk59NkV4VuoTkVXHjw8VJzC3ybKrWo7zuDC9xTiWXPygEe"

# timing info
jsonpath "$.data.accounts[0].timeLocked" == true
jsonpath "$.data.accounts[0].timing.initial_minimum_balance" == 1000000000
jsonpath "$.data.accounts[0].timing.cliff_time" == 691200
jsonpath "$.data.accounts[0].timing.cliff_amount" == 1000000000
jsonpath "$.data.accounts[0].timing.vesting_period" == 1
jsonpath "$.data.accounts[0].timing.vesting_increment" == 0

duration < 750

#
# Accounts public key epoch & total blocks produced query
#

POST {{url}}
```graphql
{
  accounts(
    query: {
      publicKey: "B62qqE5R5pJDUjPrKZMtTkPKUPL27kwNZ1sHkZaowxSLosZvLudt3kW"
    }
  ) {
    publicKey
    pk_epoch_num_blocks
    pk_total_num_blocks
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 1

# block production info
jsonpath "$.data.accounts[0].publicKey" == "B62qqE5R5pJDUjPrKZMtTkPKUPL27kwNZ1sHkZaowxSLosZvLudt3kW"
jsonpath "$.data.accounts[0].pk_epoch_num_blocks" == 5
jsonpath "$.data.accounts[0].pk_total_num_blocks" == 5

duration < 750

#
# Accounts public key epoch & total snarks query
#

POST {{url}}
```graphql
{
  accounts(
    query: {
      publicKey: "B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h"
    }
  ) {
    publicKey
    pk_epoch_num_snarks
    pk_total_num_snarks
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 1

# block production info
jsonpath "$.data.accounts[0].publicKey" == "B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h"
jsonpath "$.data.accounts[0].pk_epoch_num_snarks" == 64
jsonpath "$.data.accounts[0].pk_total_num_snarks" == 64

duration < 750

#
# Accounts public key epoch & total user commands query
#

POST {{url}}
```graphql
{
  accounts(
    query: {
      publicKey: "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
    }
  ) {
    publicKey
    pk_epoch_num_user_commands
    pk_total_num_user_commands
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 1

# block production info
jsonpath "$.data.accounts[0].publicKey" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.accounts[0].pk_epoch_num_user_commands" == 294
jsonpath "$.data.accounts[0].pk_total_num_user_commands" == 294

duration < 750

#
# Accounts public key epoch & total internal commands query
#

POST {{url}}
```graphql
{
  account0: accounts(
    query: {
      publicKey: "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
    }
  ) {
    publicKey
    pk_epoch_num_internal_commands
    pk_total_num_internal_commands
  }

  account1: accounts(
    query: {
      publicKey: "B62qrdhG66vK71Jbdz6Xs7cnDxQ8f6jZUFvefkp3pje4EejYUTvotGP"
    }
  ) {
    publicKey
    pk_epoch_num_internal_commands
    pk_total_num_internal_commands
  }
}
```
HTTP 200
[Asserts]

# data counts
jsonpath "$.data.account0" count == 1
jsonpath "$.data.account1" count == 1

# first account
jsonpath "$.data.account0[0].publicKey" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.account0[0].pk_epoch_num_internal_commands" == 0
jsonpath "$.data.account0[0].pk_total_num_internal_commands" == 0

# last account
jsonpath "$.data.account1[0].publicKey" == "B62qrdhG66vK71Jbdz6Xs7cnDxQ8f6jZUFvefkp3pje4EejYUTvotGP"
jsonpath "$.data.account1[0].pk_epoch_num_internal_commands" == 43
jsonpath "$.data.account1[0].pk_total_num_internal_commands" == 43

duration < 500

#
# Account default (balance descending) query
#


POST {{url}}
```graphql
{
  accounts {
    publicKey
    balance
    username
    nonce
    delegate
    timeLocked
    timing {
      initial_minimum_balance
      cliff_time
      cliff_amount
      vesting_period
      vesting_increment
    }
  }
}
```
HTTP 200
[Asserts]

jsonpath "$.data.accounts" count == 100

# first datum
jsonpath "$.data.accounts[0].publicKey" == "B62qpbZkvpHZ1a5nsTbANuRtrdw4YraTyA4nvJDm6HpP1YMC9QStxX3"
jsonpath "$.data.accounts[0].balance" == 75000000000000000
jsonpath "$.data.accounts[0].nonce" == 0
jsonpath "$.data.accounts[0].delegate" == "B62qqhURJQo3CvWC3WFo9LhUhtcaJWLBcJsaA3DXaU2GH5KgXujZiwB"
jsonpath "$.data.accounts[0].timeLocked" == false
jsonpath "$.data.accounts[0].timing" == null

# last datum
jsonpath "$.data.accounts[99].publicKey" == "B62qmKJxgh6h4i56hTXmkDL2Xpesm94NNm2Ev4ySSA4rCE2Z3JbfZhc"
jsonpath "$.data.accounts[99].balance" == 1693980637751650
jsonpath "$.data.accounts[99].nonce" == 0
jsonpath "$.data.accounts[99].delegate" == "B62qnvzUAvwnAiK3eMVQooshDA5AmEF9jKRrUTt5cwbCvVFiF47vdqp"
jsonpath "$.data.accounts[99].timeLocked" == true
jsonpath "$.data.accounts[99].timing.initial_minimum_balance" == 1000000000
jsonpath "$.data.accounts[99].timing.cliff_time" == 345600
jsonpath "$.data.accounts[99].timing.cliff_amount" == 1000000000
jsonpath "$.data.accounts[99].timing.vesting_period" == 1
jsonpath "$.data.accounts[99].timing.vesting_increment" == 0

duration < 750

#
# Account balance descending query
#


POST {{url}}
```graphql
{
  accounts(sortBy: BALANCE_DESC) {
    publicKey
    balance
    username
    nonce
    delegate
    timeLocked
    timing {
      initial_minimum_balance
      cliff_time
      cliff_amount
      vesting_period
      vesting_increment
    }
  }
}
```
HTTP 200
[Asserts]

jsonpath "$.data.accounts" count == 100

# first datum
jsonpath "$.data.accounts[0].publicKey" == "B62qpbZkvpHZ1a5nsTbANuRtrdw4YraTyA4nvJDm6HpP1YMC9QStxX3"
jsonpath "$.data.accounts[0].balance" == 75000000000000000
jsonpath "$.data.accounts[0].nonce" == 0
jsonpath "$.data.accounts[0].delegate" == "B62qqhURJQo3CvWC3WFo9LhUhtcaJWLBcJsaA3DXaU2GH5KgXujZiwB"
jsonpath "$.data.accounts[0].timeLocked" == false

# last datum
jsonpath "$.data.accounts[99].publicKey" == "B62qmKJxgh6h4i56hTXmkDL2Xpesm94NNm2Ev4ySSA4rCE2Z3JbfZhc"
jsonpath "$.data.accounts[99].balance" == 1693980637751650
jsonpath "$.data.accounts[99].nonce" == 0
jsonpath "$.data.accounts[99].delegate" == "B62qnvzUAvwnAiK3eMVQooshDA5AmEF9jKRrUTt5cwbCvVFiF47vdqp"
jsonpath "$.data.accounts[99].timeLocked" == true
jsonpath "$.data.accounts[99].timing.initial_minimum_balance" == 1000000000
jsonpath "$.data.accounts[99].timing.cliff_time" == 345600
jsonpath "$.data.accounts[99].timing.cliff_amount" == 1000000000
jsonpath "$.data.accounts[99].timing.vesting_period" == 1
jsonpath "$.data.accounts[99].timing.vesting_increment" == 0

duration < 750

#
# Accounts data integrity
#


POST {{url}}
```graphql
{
  accounts(
    limit: 2000
    query: { token: "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf" }
  ) {
    publicKey
  }
}
```
HTTP 200
[Asserts]

jsonpath "$.data.accounts" count == 1702

duration < 1000

#
# Genesis account query
#

POST {{url}}
```graphql
{
  accounts(
    query: {
      token: "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"
      publicKey: "B62qrBYNNHZSLNZwaY4FZVNkesEPkFbZfq3YUTa4ZyqRkz1aN86BUFN"
    }
  ) {
    genesis_account
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 1

# only datum
jsonpath "$.data.accounts[0].genesis_account" == 222221235183807

duration < 100

#
# Account balance_lte query
#


POST {{url}}
```graphql
{
  accounts(limit: 1, query: { balance_lte: 75000000000000000 }) {
    publicKey
    balance
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.accounts" count == 1

# only datum
jsonpath "$.data.accounts[0].publicKey" == "B62qpbZkvpHZ1a5nsTbANuRtrdw4YraTyA4nvJDm6HpP1YMC9QStxX3"
jsonpath "$.data.accounts[0].balance" == 75000000000000000

duration < 500
