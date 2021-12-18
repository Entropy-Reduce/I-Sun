<!--
 * @Author: QHGG
 * @Date: 2021-12-04 21:52:40
 * @LastEditTime: 2021-12-18 16:43:05
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/components/defi1.vue
-->
<template>
    <div>
        <div class="title">dfinance</div>
<el-divider></el-divider>
        <!-- <div>
            <el-card shadow="hover" style="height: 100%" >
            <img src="#" alt="">
            nftgs
            </el-card>
        </div> -->
        <!-- <el-divider><i class="el-icon-mobile-phone"></i></el-divider> -->
        <div class="clearfix">
            <div class="content" v-for="item in contens.tokens" :key="item.last_update_time" @click="click(item)">
            <el-card shadow="hover" style="height: 100%" >
            <div style="height: 120px">
                <img :src="item.logo" width="100" alt="">
            </div>
            <div  style="padding: 10px 0">
            {{item.symbol}}

            </div>
                
                <!-- <div v-for="k,v in item" :key="v">{{v}}:{{k}}</div> -->
            </el-card>
        </div>
        </div>
        <el-divider></el-divider>
        <div class="clearfix">
            <div class="contents" v-for="item in contens.pairs" :key="item.timestamp" @click="pairclick(item)">
        
            <el-card shadow="hover"  >
            <!-- <img :src="item.logo" width="150" alt=""> -->
            <!-- {{item.symbol}} -->
                
                <pre style="font-size: 14px">{{item.lp_token.split(':').join('  :  ')}}</pre>
            </el-card>
        </div>
        </div>
        <!-- <div class="clearfix">
            <div class="content" v-for="item in contens" :key="item">
            <el-card shadow="hover" style="height: 100%" >
            <img src="#" alt="">
            nftgs
            </el-card>
        </div> -->
        <!-- </div> -->
        <el-dialog
        :title="diaItem.symbol"
        :visible.sync="dialogVisible"
        width="60%"
        >
        <div  class="align">
            <div><span class="sp sp1">canister_id</span>: <span class="sp">{{diaItem.canister_id}}</span></div>
                <div><span class="sp sp1">fee</span>: <span class="sp">{{diaItem.fee/Math.pow(10, diaItem.decimals)}}</span></div>
                <div><span class="sp sp1">index</span>: <span class="sp">{{diaItem.index}}</span></div>
                <div><span class="sp sp1">name</span>: <span class="sp">{{diaItem.name}}</span></div>
                <div><span class="sp sp1">owner</span>: <span class="sp">{{diaItem.owner}}</span></div>
                <div><span class="sp sp1">supply</span>: <span class="sp">{{diaItem.supply}}</span></div>
                <div><span class="sp sp1">symbol</span>: <span class="sp">{{diaItem.symbol}}</span></div>
                <div><span class="sp sp1">create time</span>: <span class="sp">{{timeEXP(diaItem.timestamp/1e6)}}</span></div>
        </div>
        
        <span slot="footer" class="dialog-footer">
          <el-button @click="dialogVisible = false">取 消</el-button>
          <el-button type="primary" @click="dialogVisible = false">确 定</el-button>
        </span>
      </el-dialog>
      <el-dialog
        :title="pairdiaItem.lp_token"
        :visible.sync="pairdialogVisible"
        width="60%"
        >   
        
            <div class="align">
                <div v-for="k,v in pairdiaItem" :key="v">
                    <div v-if="v!='last_update_time'">
                        <span class="sp sp1">{{v}}</span>:
                    <span class="sp">{{k}}</span>
                    </div>
                    <div v-else>
                        <span class="sp sp1">{{v}}</span>:
                    <span class="sp">{{timeEXP(k/1e6)}}</span>
                    </div>
                    

                </div>
                
            </div>
        
        <span slot="footer" class="dialog-footer">
          <el-button @click="pairdialogVisible = false">取 消</el-button>
          <el-button type="primary" @click="pairdialogVisible = false">确 定</el-button>
        </span>
      </el-dialog>
    </div>
</template>

<script>
export default {
    name: 'defi1',
    methods: {
        click(n){
        //   this.$router.push('/info/' + n)
        this.diaItem = n
        this.dialogVisible=true
      },
      pairclick(n){
        //   this.$router.push('/info/' + n)
        this.pairdiaItem = n
        this.pairdialogVisible=true
      }
    },
    data() {
        return {
            'infoid': '',
            contens: [],
            dialogVisible: false,
            pairdialogVisible: false,
            diaItem: {},
            pairdiaItem: {}
        }
    },
    created(){
        this.fetchData('dfig', {}).then((res)=>{
            this.contens = res
            // this.contens.data = this.contens.tokens.
        })
    },
    beforeMount() {
        // console.log(this.$route.params.id)
        this.infoid = this.$route.params.id
    }
}
</script>
<style scoped>
.title {
    font-size: 16px;
    font-weight: 800;
}
    .index{
        position: relative;
        /* height: 100%;
        width: 100%; */
        /* overflow: hidden; */
    }
    .align {
        text-align: left;
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
    .sp{
        display: inline-block;
        padding: 5px 0;
    }
    .sp1{
        width: 150px;
    }
    .content{
        /* position: absolute; */
        width: 200px;
        height: 200px;
        margin: 10px;
        float: left;
        /* overflow: hidden; */
    }
    .contents{
        /* position: absolute; */
        width: 200px;
        height: 80px;
        margin: 10px;
        float: left;
        /* overflow: hidden; */
    }
    .card{
        height: 100%;
        background-color: rgb(194, 194, 194);
    }
</style>