yarn dev:deploy:contract 
yarn run v1.22.18
$ 

near dev-deploy
Starting deployment. 
Account id: dev-1652385931022-92387187648267, 
node: https://rpc.testnet.near.org, 
helper: https://helper.testnet.near.org,

Transaction Id 939gdyepf2ZNRCKUBQBeJoV36UyPuCyvp4wQ4L6Tdxru
https://explorer.testnet.near.org/transactions/939gdyepf2ZNRCKUBQBeJoV36UyPuCyvp4wQ4L6Tdxru


nuevo dev 
dev-1652457081622-60835668121812




# neardev/dev-account.env
# export CONTRACT_NAME=dev-1652385931022-92387187648267
# export CONTRACT_NAME=dev-1652457081622-60835668121812
# export CONTRACT_NAME=dev-1652460782376-79683009532007
export CONTRACT_NAME=dev-1652472334005-76834385663034
echo $CONTRACT_NAME

export ID=mariogarcia_ar.testnet
#export ID=dev01_tco.testnet
echo $ID

echo $CONTRACT_NAME
echo $ID

near call $CONTRACT_NAME set_greeting '{"message": "Hola mario garcia"}' --accountId $ID
near view $CONTRACT_NAME get_greeting "{\"account_id\": \"$ID\"}"
near view $CONTRACT_NAME get_greeting_payable "{\"account_id\": \"$ID\"}"









near call $CONTRACT_NAME set_ipfs '{"cid": "fasdfasdfasd garcia"}' --accountId $ID
near view $CONTRACT_NAME get_ipfs "{\"account_id\": \"$ID\"}"

near view $CONTRACT_NAME get_greeting "{\"account_id\": \"$ID\"}"
near view $CONTRACT_NAME get_ipfs "{\"account_id\": \"$ID\"}"

export ID=dev01_tco.testnet
near view $CONTRACT_NAME get_greeting "{\"account_id\": \"$ID\"}"
near view $CONTRACT_NAME get_ipfs "{\"account_id\": \"$ID\"}"

 
export ID=mariogarcia_ar.testnet
near view $CONTRACT_NAME get_greeting "{\"account_id\": \"$ID\"}"
near view $CONTRACT_NAME get_ipfs "{\"account_id\": \"$ID\"}"

export CONTRACT_NAME=dev-1652385931022-92387187648267
echo $CONTRACT_NAME

export ID=mariogarcia_ar.testnet
echo $ID







Starting deployment. Account id: dev-1652385931022-92387187648267, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: ./out/main.wasm
Transaction Id 28jXDDNCjqbnZn769RUnd9fVdVHsPCg1A7aJK8SAimtK
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/28jXDDNCjqbnZn769RUnd9fVdVHsPCg1A7aJK8SAimtK
Done deploying to dev-1652385931022-92387187648267









graph init --product hosted-service mariogarcia-ar/tco-exa 

Subgraph mariogarcia-ar/tco-exa created in subgraph

Next steps:

  1. Run `graph auth` to authenticate with your deploy key.

  2. Type `cd subgraph` to enter the subgraph.

  3. Run `yarn deploy` to deploy the subgraph.


yarn codegen


graph auth --product hosted-service 2138ee84d4cb4a67981fcf63905744d4

Deploy key set for https://api.thegraph.com/deploy/


graph deploy --product hosted-service mariogarcia-ar/tco-exa



Subgraph endpoints:
Queries (HTTP):     https://api.thegraph.com/subgraphs/name/mariogarcia-ar/tco-exa
Subscriptions (WS): wss://api.thegraph.com/subgraphs/name/mariogarcia-ar/tco-exa

ID
QmYwsSB1oKaVmhR48Fwt95dbc5fCXYiqANzK69wjGEJ5CD
Queries (HTTP)
https://api.thegraph.com/subgraphs/name/mariogarcia-ar/tco-exa


https://github.com/graphprotocol/example-subgraph/blob/near-receipts-example/subgraph.yaml






 near dev-deploy
Starting deployment. Account id: dev-1652472334005-76834385663034, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: ./out/main.wasm
Transaction Id Hvxz25zAm85H1bKzFYyPMKSGDsotb6cpNFJA7G63J84s
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/Hvxz25zAm85H1bKzFYyPMKSGDsotb6cpNFJA7G63J84s
Done deploying to dev-1652472334005-76834385663034
Done in 4.17s.
