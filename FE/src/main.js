/*
 * @Author: QHGG
 * @Date: 2021-12-04 19:54:30
 * @LastEditTime: 2021-12-18 05:53:28
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/main.js
 */
import Vue from 'vue';
import ElementUI from 'element-ui';
import 'element-ui/lib/theme-chalk/index.css';
import App from './App.vue';
import axios from 'axios';
import fetchData from './host/http';
import './assets/reset.css'
import VCharts from 'v-charts'

import Router from 'vue-router'
// import Lists from '@/components/Lists.vue'
import NFT1 from '@/components/NFT1.vue'
import Home from '@/components/Home.vue'
import NFT2 from '@/components/NFT2.vue'
// import Login from '@/components/Login.vue'
import Info from '@/components/Info.vue'
import Defi1 from '@/components/Defi1.vue'
import Defi2 from '@/components/Defi2.vue'
import Defi3 from '@/components/Defi3.vue'
import timeEXP from '@/utils/time.js'
const includPush = Router.prototype.push
Router.prototype.push = function push(location) {
    return includPush.call(this, location).catch(err => err)
}
Vue.use(Router)
Vue.prototype.fetchData = fetchData;
Vue.prototype.timeEXP = timeEXP;

Vue.prototype.$fetchData = axios;
Vue.use(VCharts)
// const Home = { template: '<div>This is Home</div>' }
// const Foo = { template: '<div>This is Foo</div>' }
// const Bar = { template: '<div>This is Bar {{ $route.params.id }}</div>' }

const router = new Router({
  mode: 'history',
  base: __dirname,
  routes: [
    { path: '/', name: 'home', component: Home },
    { path: '/nft1', name: 'nft1', component: NFT1 },
    { path: '/nft2', name: 'nft2', component: NFT2 },
    { path: '/nftinfo/:dapp_id/:index', name: 'info', component: Info },
    { path: '/defi1', name: 'defi1', component: Defi1 },
    { path: '/defi2', name: 'defi2', component: Defi2 },
    { path: '/defi3/:sym/:key/:id/:index', name: 'defi3', component: Defi3 },
  ]
})
Vue.use(ElementUI);

new Vue({
  router,
  el: '#app',
  render: h => h(App)
});