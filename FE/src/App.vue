<!--
 * @Author: QHGG
 * @Date: 2021-12-04 19:54:30
 * @LastEditTime: 2021-12-17 21:44:55
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/App.vue
-->
<template>
  <div id="app">
    <div class="login">
      <Login/>
    </div>
    <div class="wrapper">
      <div class="sider">
        <Index />
      </div>
      <div class="container">
        <router-view></router-view>
      </div>
    </div>
  </div>
</template>

<script>
import Index from './components/Index.vue'
import Login from './components/Login.vue'
export default {
  name: 'App',
  components: {
    Index,Login
  }
}
</script>

<style scoped>
.login {
  position: absolute;
  left: 0;
  top : 0;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  /* margin: 0 auto; */
  position: absolute;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
  /* padding-top: 50px; */
  background-color: rgb(219, 219, 219);
}
.sider {
  /* position: absolute; */
  float: left;
  overflow: hidden;
  /* left: 0; */
  height: 100%;
  width: 200px;
}
.container {
  overflow: hidden;
  overflow-y: scroll;
  box-sizing: border-box;
  height: 100%;
  padding: 20px;
  /* float: left; */
  /* position: absolute;
  left: 20%;
  height: 100%;
  width: 80%; */
}
.wrapper {
  width: 80%;
  margin: 0 auto;
  height: 100%;
  position: relative;
  background-color: #fff;
  overflow: hidden;

}
</style>
