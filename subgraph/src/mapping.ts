import { near, BigInt, log, json , JSONValueKind } from "@graphprotocol/graph-ts";
import { Saludo } from "../generated/schema";

export function handleReceipt(receipt: near.ReceiptWithOutcome): void {
  const actions = receipt.receipt.actions;
  for (let i = 0; i < actions.length; i++) {
    handleAction(
      actions[i], 
      receipt.receipt, 
      receipt.block.header,
      receipt.outcome,
      receipt.receipt.signerPublicKey);
  }
}

function handleAction(
  action: near.ActionValue,
  receipt: near.ActionReceipt,
  blockHeader: near.BlockHeader,
  outcome: near.ExecutionOutcome,
  publicKey: near.PublicKey,
): void {
  if (action.kind != near.ActionKind.FUNCTION_CALL) {
    log.info("Early return: {}", ["Not a function call"]);
    return;
  }

  const functionCall = action.toFunctionCall();
  if (functionCall.methodName == "set_greeting") {
    log.info("estoy en set_greeting  A_A_A", []);
    // https://nomicon.io/RuntimeSpec/FunctionCall
    let jsonData = outcome.logs[0];
    let parsedJSON = json.fromString(jsonData);

    log.info("jsonData: {}", [jsonData]);

    if(parsedJSON.kind == JSONValueKind.OBJECT){
      log.info("procesando", []);
      let entry = parsedJSON.toObject();
      let message : string;
      let signer_account_id : string;
      
      for(let i = 0; i < entry.entries.length; i++){
        log.info("i:{}", [i.toString()]);
        let _entry = entry.entries[i]; 
        let key = _entry.key;
        switch (true) {
          case key == 'message':
            message = _entry.value.toString();
            break;
          case key == 'signer_account_id':
            signer_account_id = _entry.value.toString();
            // greeting.id = _entry.value.toString();
            break;
        }
      }

      log.info("message:{}, signer_account_id:{}", [message, signer_account_id]);
      let greeting = new Saludo(signer_account_id);
      greeting.message = message;
      greeting.signer_account_id = signer_account_id;
      greeting.save();
      log.info("se guardoooooooooooooooooooo", []);


    }else{
      log.info('Error no es un json',[]);
    }
 
  } else {
    log.info("Not processed - FunctionCall is: {}", [functionCall.methodName]);
  }
}