/*
 * @Author: QHGG
 * @Date: 2021-12-17 19:14:47
 * @LastEditTime: 2021-12-17 23:20:30
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/host/http.js
 */
import axios from 'axios';
import Vue from 'vue';
import getConfig from './apiConfig';

const BUS = new Vue({
    data() {
        return { xhrs: [] };
    }
});


axios.interceptors.response.use((response) => {
    BUS.xhrs.pop();
    // if (BUS.xhrs.length === 0) {
    //     document.getElementById('loading').style.display = 'none';
    // }

    return response;
}, (error) => {
    Promise.reject(error)
})

const fetchData = (apiName, params = {}, header = {}) => {
    const APICONFIG = getConfig(apiName);
    if (!APICONFIG) {
        BUS.$message.error('api接口未定义');
        return Promise.reject();
    }
    const url = APICONFIG[0];
    const method = APICONFIG[1] || 'GET';
    const ajaxConfig = {
        url,
        method,
    };
    if (method === 'GET') {
        ajaxConfig.params = params;
        // console.log(url)
    } else {
        ajaxConfig.data = params;
    }
    // set header
    const rodom = '' + parseInt(Math.random() * 10) + parseInt(Math.random() * 10) + parseInt(Math.random() * 10) + parseInt(Math.random() * 10);
    if (header) {
        ajaxConfig.headers = Object.assign({  random: `${new Date().getTime()}${rodom}`, }, header);
    }
    // deal data
    // deal  request data
    return axios(ajaxConfig).then((res = {}) => {
        return  res.data
    });
};

export default fetchData;
