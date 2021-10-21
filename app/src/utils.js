import * as nearAPI from "near-api-js";

async function viewMethodOnContract(nearConfig, method) {
    const provider = new nearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
    
}