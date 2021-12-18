<!--
 * @Author: QHGG
 * @Date: 2021-12-04 20:07:53
 * @LastEditTime: 2021-12-18 07:00:54
 * @LastEditors: QHGG
 * @Description: ;
 * @FilePath: /ii/src/components/NFT2.vue
-->
<template>
    
    <div class="index">
        <div class="content" v-for="item in contens" :key="item.k" @click="click(item)">
            <el-card shadow="hover" style="height: 100%" >
            <div style="height: 100px">
                <img :src="item[2]" width="100" alt="">
            </div>
            <div style="padding-top: 10px;">
              {{item[0]}}     {{item[1]}}
            </div>
            </el-card>
        </div>
    </div>
</template>

<style scoped>
    .index{
        position: relative;
        /* height: 100%; */
        width: 100%;
    }
    .content{
        /* position: absolute; */
        width: 200px;
        height: 150px;
        margin: 10px;
        float: left;
    }
</style>

<script>
export default {
  data() {
    return {
      contens: []
    };
  },
  created(){
    this.fetchData('mydata', {
      'id': 'fmyb6-ppedy-qqqfn-2p3bm-fd7km-bdcq4-ybkko-uzqpa-qccer-il756-iae',
    }).then((res)=>{
            // console.log(res)

            this.contens = res
            res.forEach(item => {
                item.k = Math.random()
            });
            // for (k,v in this.contens){
            //   list.append(k,v)
            // }
              
        })
  },
  methods:{
      click(n){
        this.$router.push('/nftinfo/' + n[1]+'/'+n[3])
        // console.log(n)
      }
  },
  name: 'nft2',
}
</script>