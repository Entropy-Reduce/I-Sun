/*
 * @Author: QHGG
 * @Date: 2021-12-17 19:14:47
 * @LastEditTime: 2021-12-18 01:57:45
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/host/apiConfig.js
 */
const APIS = {
    nftg: ['/api/info/NFT/general_info', 'GET'],
    mydata: ['/api/info/NFT/user_info', 'GET'],
    info: ['/api/info/NFT/nft_info', 'GET'],
    dfitokenbalances: ['/api/dfinance/token/balances', 'GET'],
    dfidswapbalances: ['/api/dfinance/dswap/balances', 'GET'],
    dfitrans: ['/api/dfinance/token/transactions', 'GET'],
    dfidswap: ['/api/dfinance/dswap/transactions', 'GET'],
    dfig: ['/api/dfinance/general_info', 'GET']
    
};
const getConfig = apiName => APIS[apiName]
export default getConfig;
