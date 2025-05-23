#
# Transactions canonical block height ascending query
#

POST {{url}}
```graphql
{
  transactions(limit: 50, sortBy: BLOCKHEIGHT_ASC, query: { canonical: true }) {
    blockHeight
    canonical
    amount
    fee
    kind
    nonce
    memo
    hash
    sender
    receiver_account {
      publicKey
    }
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 50

# first datum
jsonpath "$.data.transactions[0].blockHeight" == 3
jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[0].memo" == ""
jsonpath "$.data.transactions[0].hash" == "CkpYeG32dVJUjs6iq3oroXWitXar1eBtV3GVFyH5agw7HPp9bG4yQ"
jsonpath "$.data.transactions[0].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[0].receiver_account.publicKey" == "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"

# last datum
jsonpath "$.data.transactions[49].blockHeight" == 37
jsonpath "$.data.transactions[49].canonical" == true
jsonpath "$.data.transactions[49].memo" == ""
jsonpath "$.data.transactions[49].hash" == "CkpYsbai6qHEoF53e8pFHdiWqhT34sPhSgbUVdxLmfzAZ8oMpKDuK"
jsonpath "$.data.transactions[49].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[49].receiver_account.publicKey" == "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"

duration < 100

#
# Transactions canonical block height descending query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 50
    sortBy: BLOCKHEIGHT_DESC
    query: { canonical: true }
  ) {
    blockHeight
    canonical
    memo
    hash
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 50

# first datum
jsonpath "$.data.transactions[0].blockHeight" == 120
jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[0].memo" == ""
jsonpath "$.data.transactions[0].hash" == "CkpZreaWRNr1eANhVYLmi8vzRrwkoEwdNyk2FyUa7M4ZQVnA752wL"

# last datum
jsonpath "$.data.transactions[49].blockHeight" == 89
jsonpath "$.data.transactions[49].canonical" == true
jsonpath "$.data.transactions[49].memo" == ""
jsonpath "$.data.transactions[49].hash" == "CkpaJb8GcyrbGYjGeC7aUtwDq3m6yxXSBFhuUVy4Jm8FVvaimbioM"

duration < 100

#
# Transactions canonical specific block height query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 50
    sortBy: BLOCKHEIGHT_DESC
    query: { blockHeight: 11 }
  ) {
    blockHeight
    canonical
    amount
    kind
    hash
    fee
    sender
    receiver_account {
      publicKey
    }
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 2

# first datum
jsonpath "$.data.transactions[0].blockHeight" == 11
jsonpath "$.data.transactions[0].hash" == "CkpZwTdKUDFM8Nsq6noJ4pP6esqP7FeJaHiisGrXSqHYvidfvukDA"
jsonpath "$.data.transactions[0].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[0].receiver_account.publicKey" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[0].amount" == 1000
jsonpath "$.data.transactions[0].fee" == 10000000

# last datum
jsonpath "$.data.transactions[1].blockHeight" == 11
jsonpath "$.data.transactions[1].hash" == "CkpZE6rHwLM8iJwBU1ssm1tdMzx9rdyDEhpGU8aCAionzF1k82d5N"
jsonpath "$.data.transactions[1].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[1].receiver_account.publicKey" == "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"
jsonpath "$.data.transactions[1].amount" == 1000000000
jsonpath "$.data.transactions[1].fee" == 10000000

duration < 100

#
# Transactions from (sender) query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 50
    sortBy: BLOCKHEIGHT_DESC
    query: {
      from: "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
      canonical: true
    }
  ) {
    blockHeight
    canonical
    sender
    receiver_account {
      publicKey
    }
    hash
    amount
    fee
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 50

# first datum
jsonpath "$.data.transactions[0].blockHeight" == 120
jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[0].hash" == "CkpZreaWRNr1eANhVYLmi8vzRrwkoEwdNyk2FyUa7M4ZQVnA752wL"
jsonpath "$.data.transactions[0].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[0].receiver_account.publicKey" == "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"
jsonpath "$.data.transactions[0].amount" == 1000
jsonpath "$.data.transactions[0].fee" == 10000000

# last datum
jsonpath "$.data.transactions[49].blockHeight" == 89
jsonpath "$.data.transactions[49].canonical" == true
jsonpath "$.data.transactions[49].hash" == "CkpaJb8GcyrbGYjGeC7aUtwDq3m6yxXSBFhuUVy4Jm8FVvaimbioM"
jsonpath "$.data.transactions[49].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[49].receiver_account.publicKey" == "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"
jsonpath "$.data.transactions[49].amount" == 1000
jsonpath "$.data.transactions[49].fee" == 10000000

duration < 100

#
# Transactions to (receiver) query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 50
    sortBy: BLOCKHEIGHT_DESC
    query: {
      to: "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"
      canonical: true
    }
  ) {
    blockHeight
    canonical
    sender
    receiver_account {
      publicKey
    }
    hash
    amount
    fee
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 50

# first datum
jsonpath "$.data.transactions[0].blockHeight" == 120
jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[0].hash" == "CkpZreaWRNr1eANhVYLmi8vzRrwkoEwdNyk2FyUa7M4ZQVnA752wL"
jsonpath "$.data.transactions[0].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[0].receiver_account.publicKey" == "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"
jsonpath "$.data.transactions[0].amount" == 1000
jsonpath "$.data.transactions[0].fee" == 10000000

# last datum
jsonpath "$.data.transactions[49].blockHeight" == 89
jsonpath "$.data.transactions[49].canonical" == true
jsonpath "$.data.transactions[49].hash" == "CkpaJb8GcyrbGYjGeC7aUtwDq3m6yxXSBFhuUVy4Jm8FVvaimbioM"
jsonpath "$.data.transactions[49].sender" == "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
jsonpath "$.data.transactions[49].receiver_account.publicKey" == "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM"
jsonpath "$.data.transactions[49].amount" == 1000
jsonpath "$.data.transactions[49].fee" == 10000000

duration < 100

#
# Transactions blockHeight_gte/lte filters are inclusive
#

POST {{url}}
```graphql
{
  transactions(
    limit: 1000
    sortBy: BLOCKHEIGHT_DESC
    query: { blockHeight_lte: 100, blockHeight_gte: 99, canonical: true }
  ) {
    blockHeight
    canonical
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 4

# data correctness
jsonpath "$.data.transactions[0].blockHeight" == 100
jsonpath "$.data.transactions[1].blockHeight" == 99
jsonpath "$.data.transactions[2].blockHeight" == 99
jsonpath "$.data.transactions[3].blockHeight" == 99

jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[1].canonical" == true
jsonpath "$.data.transactions[2].canonical" == true
jsonpath "$.data.transactions[3].canonical" == true

duration < 100

#
# Transactions blockHeight_gt/lt filters are exclusive
#

POST {{url}}
```graphql
{
  transactions(
    limit: 1000
    sortBy: BLOCKHEIGHT_DESC
    query: { blockHeight_lt: 101, blockHeight_gt: 98, canonical: true }
  ) {
    blockHeight
    canonical
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 4

# data correctness
jsonpath "$.data.transactions[0].blockHeight" == 100
jsonpath "$.data.transactions[1].blockHeight" == 99
jsonpath "$.data.transactions[2].blockHeight" == 99
jsonpath "$.data.transactions[3].blockHeight" == 99

jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[1].canonical" == true
jsonpath "$.data.transactions[2].canonical" == true
jsonpath "$.data.transactions[3].canonical" == true

duration < 100

#
# Transactions specific block state hash query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 1
    sortBy: BLOCKHEIGHT_DESC
    query: {
      canonical: true
      block: {
        stateHash: "3NLNyQC4XgQX2Q9H7fC2UxFZKY4xwwUZop8jVR24SWYNNE93FsnS"
      }
    }
  ) {
    hash
    blockHeight
    canonical
    block {
      stateHash
    }
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 1

# only datum
jsonpath "$.data.transactions[0].blockHeight" == 120
jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[0].hash" == "CkpZreaWRNr1eANhVYLmi8vzRrwkoEwdNyk2FyUa7M4ZQVnA752wL"
jsonpath "$.data.transactions[0].block.stateHash" == "3NLNyQC4XgQX2Q9H7fC2UxFZKY4xwwUZop8jVR24SWYNNE93FsnS"

duration < 100

#
# Transactions by hash (block order) query
#

POST {{url}}
```graphql
{
  transactions(
    query: { hash: "CkpZrxGwdxH2CL8wGTF3B9BgJcc5xyfiF8iyzNvTYMALccvmNcNSN" }
  ) {
    blockHeight
    canonical
    hash
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 4

# data correctness
jsonpath "$.data.transactions[0].blockHeight" == 6
jsonpath "$.data.transactions[0].canonical" == true
jsonpath "$.data.transactions[0].hash" == "CkpZrxGwdxH2CL8wGTF3B9BgJcc5xyfiF8iyzNvTYMALccvmNcNSN"

jsonpath "$.data.transactions[1].blockHeight" == 6
jsonpath "$.data.transactions[1].canonical" == false
jsonpath "$.data.transactions[1].hash" == "CkpZrxGwdxH2CL8wGTF3B9BgJcc5xyfiF8iyzNvTYMALccvmNcNSN"

jsonpath "$.data.transactions[2].blockHeight" == 6
jsonpath "$.data.transactions[2].canonical" == false
jsonpath "$.data.transactions[2].hash" == "CkpZrxGwdxH2CL8wGTF3B9BgJcc5xyfiF8iyzNvTYMALccvmNcNSN"

jsonpath "$.data.transactions[3].blockHeight" == 6
jsonpath "$.data.transactions[3].canonical" == false
jsonpath "$.data.transactions[3].hash" == "CkpZrxGwdxH2CL8wGTF3B9BgJcc5xyfiF8iyzNvTYMALccvmNcNSN"

duration < 100

#
# Transactions from (block/nonce order) query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 6
    query: {
      canonical: true
      from: "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy"
    }
  ) {
    nonce
    blockHeight
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 6

# data correctness
jsonpath "$.data.transactions[0].blockHeight" == 120
jsonpath "$.data.transactions[0].nonce" == 176

jsonpath "$.data.transactions[1].blockHeight" == 119
jsonpath "$.data.transactions[1].nonce" == 175

jsonpath "$.data.transactions[2].blockHeight" == 119
jsonpath "$.data.transactions[2].nonce" == 174

jsonpath "$.data.transactions[3].blockHeight" == 118
jsonpath "$.data.transactions[3].nonce" == 173

jsonpath "$.data.transactions[4].blockHeight" == 117
jsonpath "$.data.transactions[4].nonce" == 172

jsonpath "$.data.transactions[5].blockHeight" == 116
jsonpath "$.data.transactions[5].nonce" == 171

duration < 100

#
# Transactions applied & failed
#

POST {{url}}
```graphql
{
  transactions(
    limit: 20
    sortBy: BLOCKHEIGHT_ASC
    query: { from: "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy" }
  ) {
    nonce
    blockHeight
    failureReason
    isApplied
  }

  applied: transactions(
    limit: 20
    sortBy: BLOCKHEIGHT_ASC
    query: { isApplied: true }
  ) {
    nonce
    blockHeight
    isApplied
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 20
jsonpath "$.data.applied" count == 20

# first data
jsonpath "$.data.transactions[0].blockHeight" == 3
jsonpath "$.data.transactions[0].nonce" == 0
jsonpath "$.data.transactions[0].failureReason" == "Amount_insufficient_to_create_account"
jsonpath "$.data.transactions[0].isApplied" == false

jsonpath "$.data.applied[0].blockHeight" == 11
jsonpath "$.data.applied[0].nonce" == 12
jsonpath "$.data.applied[0].isApplied" == true

# last data
jsonpath "$.data.transactions[19].blockHeight" == 12
jsonpath "$.data.transactions[19].nonce" == 14
jsonpath "$.data.transactions[19].failureReason" == null
jsonpath "$.data.transactions[19].isApplied" == true

jsonpath "$.data.applied[19].blockHeight" == 17
jsonpath "$.data.applied[19].nonce" == 24
jsonpath "$.data.applied[19].isApplied" == true

duration < 100

#
# Transactions failure reason filter (results sorted by txn hash)
#

POST {{url}}
```graphql
{
  transactions(
    limit: 5
    sortBy: BLOCKHEIGHT_ASC
    query: { failureReason: "Amount_insufficient_to_create_account" }
  ) {
    nonce
    blockHeight
    failureReason
    hash
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.transactions" count == 5

# data correctness
jsonpath "$.data.transactions[0].blockHeight" == 3
jsonpath "$.data.transactions[0].nonce" == 2
jsonpath "$.data.transactions[0].failureReason" == "Amount_insufficient_to_create_account"
jsonpath "$.data.transactions[0].hash" == "CkpYeG32dVJUjs6iq3oroXWitXar1eBtV3GVFyH5agw7HPp9bG4yQ"

jsonpath "$.data.transactions[1].blockHeight" == 3
jsonpath "$.data.transactions[1].nonce" == 3
jsonpath "$.data.transactions[1].failureReason" == "Amount_insufficient_to_create_account"
jsonpath "$.data.transactions[1].hash" == "CkpZ1u12zrTuEttp7QktfEy7wosHrPV6r3DJkq4sA9f1yKgEqmj5k"

jsonpath "$.data.transactions[2].blockHeight" == 3
jsonpath "$.data.transactions[2].nonce" == 1
jsonpath "$.data.transactions[2].failureReason" == "Amount_insufficient_to_create_account"
jsonpath "$.data.transactions[2].hash" == "CkpZB4WE3wDRJ4CqCXqS4dqF8hoRQDVK8banePKUgTR6kvhTfyjRp"

jsonpath "$.data.transactions[3].blockHeight" == 3
jsonpath "$.data.transactions[3].nonce" == 0
jsonpath "$.data.transactions[3].failureReason" == "Amount_insufficient_to_create_account"
jsonpath "$.data.transactions[3].hash" == "CkpZirFuoLVVab6x2ry4j8Ld5gMmQdak7VHW6f5C7VJYE34WAEWqa"

jsonpath "$.data.transactions[4].blockHeight" == 4
jsonpath "$.data.transactions[4].nonce" == 5
jsonpath "$.data.transactions[4].failureReason" == "Amount_insufficient_to_create_account"
jsonpath "$.data.transactions[4].hash" == "CkpZK28AEmzhB8AjsfT6Dd1cKdR5WF2gQw4xqDu4f93ozDX2jekTq"

duration < 100

#
# Transactions date time bounded (next/prev) query
#

POST {{url}}
```graphql
{
  prev: transactions(
    limit: 1
    sortBy: DATETIME_DESC
    query: {
      canonical: true
      kind: "PAYMENT"
      dateTimeLte: "2021-03-17T00:24:00.000Z"
    }
  ) {
    nonce
    blockHeight
    globalSlot
    block {
      dateTime
    }
  }

  next: transactions(
    limit: 1
    sortBy: DATETIME_ASC
    query: {
      canonical: true
      kind: "PAYMENT"
      dateTimeGte: "2021-03-17T00:30:00.000Z"
    }
  ) {
    nonce
    blockHeight
    globalSlot
    block {
      dateTime
    }
  }
}
```
HTTP 200
[Asserts]

# total data count
jsonpath "$.data.prev" count == 1
jsonpath "$.data.next" count == 1

# data correctness
jsonpath "$.data.prev[0].blockHeight" == 8
jsonpath "$.data.prev[0].globalSlot" == 7
jsonpath "$.data.prev[0].nonce" == 11
jsonpath "$.data.prev[0].block.dateTime" == "2021-03-17T00:21:00.000Z"

jsonpath "$.data.next[0].blockHeight" == 11
jsonpath "$.data.next[0].globalSlot" == 11
jsonpath "$.data.next[0].nonce" == 12
jsonpath "$.data.next[0].block.dateTime" == "2021-03-17T00:33:00.000Z"

duration < 100

#
# Transactions bound by height sorted by global slot descending query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 25
    sortBy: GLOBALSLOT_DESC
    query: { blockHeight_lte: 120, blockHeight_gt: 118 }
  ) {
    blockHeight
    globalSlot
  }
}
```
HTTP 200
[Asserts]

# data count
jsonpath "$.data.transactions" count == 4

# data correctness
jsonpath "$.data.transactions[0].blockHeight" == 120
jsonpath "$.data.transactions[0].globalSlot" == 173

jsonpath "$.data.transactions[1].blockHeight" == 120
jsonpath "$.data.transactions[1].globalSlot" == 173

jsonpath "$.data.transactions[2].blockHeight" == 119
jsonpath "$.data.transactions[2].globalSlot" == 172

jsonpath "$.data.transactions[3].blockHeight" == 119
jsonpath "$.data.transactions[3].globalSlot" == 172

duration < 100

#
# Transactions bound by height sorted by global slot ascending query
#

POST {{url}}
```graphql
{
  transactions(
    limit: 25
    sortBy: GLOBALSLOT_ASC
    query: { blockHeight_lte: 120, blockHeight_gt: 118 }
  ) {
    blockHeight
    globalSlot
  }
}
```
HTTP 200
[Asserts]

# data count
jsonpath "$.data.transactions" count == 4

# data correctness
jsonpath "$.data.transactions[0].blockHeight" == 119
jsonpath "$.data.transactions[0].globalSlot" == 172

jsonpath "$.data.transactions[1].blockHeight" == 119
jsonpath "$.data.transactions[1].globalSlot" == 172

jsonpath "$.data.transactions[2].blockHeight" == 120
jsonpath "$.data.transactions[2].globalSlot" == 173

jsonpath "$.data.transactions[3].blockHeight" == 120
jsonpath "$.data.transactions[3].globalSlot" == 173

duration < 100
