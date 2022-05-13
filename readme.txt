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


# neardev/dev-account.env
export CONTRACT_NAME=dev-1652385931022-92387187648267
echo $CONTRACT_NAME

export ID=mariogarcia_ar.testnet
echo $ID


near call $CONTRACT_NAME set_greeting '{"message": "Hola mario"}' --accountId $ID
near view $CONTRACT_NAME get_greeting "{\"account_id\": \"$ID\"}"




 
export CONTRACT_NAME=dev-1652385931022-92387187648267
echo $CONTRACT_NAME

export ID=mariogarcia_ar.testnet
echo $ID







Starting deployment. Account id: dev-1652385931022-92387187648267, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: ./out/main.wasm
Transaction Id 28jXDDNCjqbnZn769RUnd9fVdVHsPCg1A7aJK8SAimtK
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/28jXDDNCjqbnZn769RUnd9fVdVHsPCg1A7aJK8SAimtK
Done deploying to dev-1652385931022-92387187648267
