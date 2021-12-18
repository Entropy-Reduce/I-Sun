/*
 * @Author: QHGG
 * @Date: 2021-12-17 21:19:52
 * @LastEditTime: 2021-12-17 21:27:39
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/config/dev.env.js
 */
'use strict'
const merge = require('webpack-merge')
const prodEnv = require('./prod.env')

module.exports = merge(prodEnv, {
  NODE_ENV: '"development"',
})
