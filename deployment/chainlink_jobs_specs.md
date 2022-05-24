## Job specs to define in Chainlink nodes

```
type = "webhook"
schemaVersion = 1
name = ""
externalJobID = "9ce988a3-9b74-4525-91fd-000000000000"
externalInitiators = [
  { name = "sublink-initiator", spec = "{}" }
]
observationSource = """
    
    parse_feed  [type="jsonparse" path="data,feed_id" data="$(jobRun.requestBody)"]
    parse_round  [type="jsonparse" path="data,round_id" data="$(jobRun.requestBody)"]


    get_price [type="http" method=GET url="https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USD"]
    parse_value [type="jsonparse" path="USD" data="$(get_price)"]
    multiply [type="multiply" input="$(parse_value)" times=1000]

     substrate [type="bridge" name="sublink-adapter" requestData="{\\"id\\":\\"001\\",\\"data\\":{\\"feed_id\\": $(parse_feed), \\"round_id\\": $(parse_round), \\"value\\": $(multiply) ,\\"request_type\\": \\"fluxmonitor\\"}}"]

    parse_feed->parse_round -> get_price-> parse_value-> substrate
"""
```

```
type = "webhook"
schemaVersion = 1
name = ""
externalJobID = "9ce988a3-9b74-4525-91fd-000000000001"
externalInitiators = [
  { name = "sublink-initiator", spec = "{}" }
]
observationSource = """
    
    parse_feed  [type="jsonparse" path="data,feed_id" data="$(jobRun.requestBody)"]
    parse_round  [type="jsonparse" path="data,round_id" data="$(jobRun.requestBody)"]


    get_price [type="http" method=GET url="https://api.coingecko.com/api/v3/simple/price?ids=polkadot&vs_currencies=usd"]
    parse_value [type="jsonparse" path="polkadot,usd" data="$(get_price)"]
    multiply [type="multiply" input="$(parse_value)" times=1000]

     substrate [type="bridge" name="sublink-adapter2" requestData="{\\"id\\":\\"001\\",\\"data\\":{\\"feed_id\\": $(parse_feed), \\"round_id\\": $(parse_round), \\"value\\": $(multiply) ,\\"request_type\\": \\"fluxmonitor\\"}}"]

    parse_feed->parse_round-> get_price-> parse_value-> substrate
"""
```

```
type = "webhook"
schemaVersion = 1
name = ""
externalJobID = "9ce988a3-9b74-4525-91fd-000000000002"
externalInitiators = [
  { name = "sublink-initiator", spec = "{}" }
]
observationSource = """
    
    parse_feed  [type="jsonparse" path="data,feed_id" data="$(jobRun.requestBody)"]
    parse_round  [type="jsonparse" path="data,round_id" data="$(jobRun.requestBody)"]


    get_price [type="http" method=GET url="https://api.coincap.io/v2/assets/polkadot"]
    parse_value [type="jsonparse" path="data,priceUsd" data="$(get_price)"]
    multiply [type="multiply" input="$(parse_value)" times=1000]

     substrate [type="bridge" name="sublink-adapter3" requestData="{\\"id\\":\\"001\\",\\"data\\":{\\"feed_id\\": $(parse_feed), \\"round_id\\": $(parse_round), \\"value\\": $(multiply) ,\\"request_type\\": \\"fluxmonitor\\"}}"]

    parse_feed->parse_round->get_price-> parse_value-> substrate
"""
```
