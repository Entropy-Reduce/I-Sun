/*
 * @Author: QHGG
 * @Date: 2021-12-17 18:09:11
 * @LastEditTime: 2021-12-17 22:33:21
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/vue.config.js
 */
const path = require('path');

module.exports = {
    transpileDependencies: ['@dfinity'],
    // publicPath: '/test'
    devServer: {
        disableHostCheck: true,
        https: false, // 是否使用`https`协议。
        open: true, // 是否运行完成自动弹出浏览器界面。
        hotOnly: false, // 是否开启热更新。
        proxy: {
          "/api/": {
            target: "http://localhost:8000", // 你的API服务器地址
            ws: true, // 代理websockets
            changeOrigin: true,
            pathRewrite: { "^/api": "" }, // 这里会重写请求的接口路径 比如 '/api/bbb/ccc' 重写为 '/bbb/ccc'
          },
        },
      },
}
