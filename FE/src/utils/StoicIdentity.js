import { StoicIdentity } from "ic-stoic-identity";
import { HttpAgent } from '@dfinity/agent';

let agent = null;

async function getHttpAgent() {
    return new Promise(async (resolve, reject) => {
        if (agent) {
            resolve(agent)
            return 
        }

        let identity = await StoicIdentity.load()
        if (identity === false) {
            identity = await StoicIdentity.connect();
        }
         
        agent = new HttpAgent({ identity }) 
        resolve(agent) 
    })
}


export { getHttpAgent };