<!--
 * @Author: QHGG
 * @Date: 2021-12-04 20:07:53
 * @LastEditTime: 2021-12-18 14:54:50
 * @LastEditors: QHGG
 * @Description: ;
 * @FilePath: /ii/src/components/NFT1.vue
-->
<template>
    <div>
      <div class="clearfix">
          <div class="content" v-for="item in contens" :key="item.canisterId" @click="click(item)">
              <el-card shadow="hover" style="height: 100%" >
              <!-- <a href=" " rel="noreferrer" target="_blank"><div style="height: 0px; overflow: hidden; padding-top: 100%; position: relative;"><img alt="bfpkx-fikor-uwiaa-aaaaa-b4apt-yaqca-aacdh-q" :src="https://njgly-uaaaa-aaaah-qb6pa-cai.raw.ic0.app/?cc=0&amp;type=thumbnail&amp;tokenid=bfpkx-fikor-uwiaa-aaaaa-b4apt-yaqca-aacdh-q" style="position: absolute; top: 0%; left: 0%; width: 100%; height: 100%; margin: 0px auto; object-fit: contain; display: block;"><span class="MuiSkeleton-root MuiSkeleton-rect MuiSkeleton-pulse" style="width: 70%; height: 70%; position: absolute; top: 15%; left: 15%; margin: 0px auto; display: none;"></span></div></a > -->
              <!-- <div>{{item.symbol}}</div> -->
              <div style="height: 100px; overflow: hidden">
                <img :src="item.symbol" width="100" alt="">
            </div>
              <!-- <embed :src="item.symbol" width="100" height="100" type="image/svg+xml" pluginspage="http://www.adobe.com/svg/viewer/install/" /> -->
              <div class="title">{{item.name}}</div>
              <!-- <div class="align">
                <div v-for="k,v in item" :key="v">{{v}}:{{k}}</div>
              </div> -->
              </el-card>
          </div>
      </div>
      <el-dialog
        :title="diaItem.name"
        :visible.sync="dialogVisible"
        width="60%"
        >
        <div class="aligns">
            <div v-for="k,v in diaItem" :key="v">
                <div v-if="v!='symbol'">
                  <span class="sp sp1">{{v}}</span>:
                <span class="sp">{{k}}</span>
                </div>
            </div>
            </div>
        <span slot="footer" class="dialog-footer">
          <el-button @click="dialogVisible = false">取 消</el-button>
          <el-button type="primary" @click="dialogVisible = false">确 定</el-button>
        </span>
      </el-dialog>
    </div>
</template>

<style scoped>
    .index{
        position: relative;
        height: 100%;
        width: 100%;
    }
    .clearfix:after{ /* 正常浏览器 清除浮动 */
        content: "";
        display: block;
        height: 0;
        clear: both;
        visibility: hidden;
    }
    .clearfix{ 
        *zoom: 1; /* zoom 1 就是ie6 清除浮动的方法 * 是ie7以下的版本所识别 */
    }
    .content{
        /* position: absolute; */
        width: 200px;
        height: 150px;
        margin: 10px;
        float: left;
        position:relative
    }
    .align {
      text-align: left;
      width: 100%;
      white-space: nowrap;
      overflow: scroll;
    }
    .title {
      /* margin-top:-50px */
      padding-top: 10px;
    }
    .aligns {
      text-align: left;
    }
    .sp{
        display: inline-block;
        padding: 5px 0;
    }
    .sp1{
        width: 150px;
    }
</style>

<script>

export default {
  data() {
    return {
      dialogVisible: false,
      contens: [],
      diaItem: {}
    };
  },
  methods:{
      click(n){
        //   this.$router.push('/info/' + n)
        this.diaItem = n
        this.dialogVisible=true
      }
  },
  created() {
    this.fetchData('nftg', {}).then((res)=>{
            this.contens = res
            // for (k,v in this.contens){
            //   list.append(k,v)
            // }
              
        })
  },
  name: 'nft1',
}
</script>
