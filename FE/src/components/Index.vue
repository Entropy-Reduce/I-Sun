<template>
<el-menu
  :default-active="activeIndex"
  class="el-menu-demo"
  mode="vertical"
  @select="handleSelect"
  >
  <el-submenu index="1">
        <template slot="title">
          <!-- <i class="el-icon-location"></i> -->
          <span>NFT</span>
        </template>
        <el-menu-item-group>
          <!-- <template slot="title">分组一</template> -->
          <el-menu-item index="1-1">general</el-menu-item>
          <el-menu-item index="1-2">my data</el-menu-item>
        </el-menu-item-group>
      </el-submenu>
       <el-submenu index="2">
        <template slot="title">
          <!-- <i class="el-icon-location"></i> -->
          <span>defi</span>
        </template>
        <el-menu-item-group>
          <!-- <template slot="title">分组一</template> -->
          <el-menu-item index="2-1">general</el-menu-item>
          <el-menu-item index="2-2">my data</el-menu-item>
        </el-menu-item-group>
      </el-submenu>
      <el-menu-item index="3" disabled>
        <!-- <i class="el-icon-document"></i> -->
        <span slot="title">DAPP</span>
      </el-menu-item>







  <!-- <router-link to="/login">
  <el-menu-item index="0">
    login in
  </el-menu-item>
  </router-link>
    <router-link to="/dapps">
  
      <el-menu-item index="1">dapps</el-menu-item>
    </router-link>
  <router-link to="/mydapps">
  <el-menu-item index="2">
    my dapps
    
  </el-menu-item>
  </router-link> -->
  <!-- <el-menu-item index="3" disabled>消息中心</el-menu-item> -->
  <!-- <el-menu-item index="4"><a href="https://www.ele.me" target="_blank">订单管理</a></el-menu-item> -->
</el-menu>
</template>
<style>
  .el-menu-demo{
    height: 100%;
  }
</style>
<script>
  export default {
    data() {
      return {
        activeIndex: '1',
      };
    },
    methods: {
      handleSelect(key) {
        // console.log(key, keyPath);
        this.activeIndex = key
        switch (key) {
          case '1-1': 
            this.$router.push('/nft1')
            break
          case '1-2': 
            this.$router.push('/nft2')
            break
          case '2-1': 
            this.$router.push('/defi1')
            break
          case '2-2': 
            this.$router.push('/defi2')
            break
          default: this.$router.push('/')
        }
      }
    }
  }
</script>