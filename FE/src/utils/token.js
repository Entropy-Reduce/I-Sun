import { getHttpAgent, tokenLogout } from "./identity";


export default class TokenInfo {
    static Instance = new TokenInfo();
    static TEXAL_TOKEN = "8ca93b50b3d080e0a18d1999c21596cf30b6823d0feb840107192aca972d7fe4"

    constructor() {
        this.isLogin = false
    }

    async login() {
        if (!this.isLogin) {
            this.agent = await getHttpAgent()
            // this.token = Actor.createActor(texas_token_idl, { agent: this.agent, canisterId: texas_token_id });
            // this.tokenEvent = Actor.createActor(texas_event_idl, { agent: this.agent, canisterId: texas_event_id });
            // this.token.approve(TokenInfo.TEXAL_TOKEN, BigInt(10000000000000000000))
            // this.isLogin = true
        }
      }

    async setTokenInfo() {
        const [name, symbol, decimals, totalMint, createTime, feePercent] = 
        await Promise.all([
            this.token.name(),
            this.token.symbol(),
            this.token.decimals(),
            this.token.totalMint(),
            this.token.createTime(),
            this.token.getFeePercent()
        ])

        this.name = name
        this.symbol = symbol
        this.decimals = 10 ** Number(decimals)
        this.totalMint = totalMint
        this.createTime = createTime
        this.feePercent = feePercent
    }


    // async approve(address, amount) {
    //     try {
    //         return await this.token.approve(address, multipBigInt(amount, this.decimals))
    //     } catch {
    //         ErrorHandle()
    //     }
    // }

    async logout() {
        this.isLogin = false
        tokenLogout()
    }
}

window.token = TokenInfo.Instance;