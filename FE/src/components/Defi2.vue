<!--
 * @Author: QHGG
 * @Date: 2021-12-04 21:52:40
 * @LastEditTime: 2021-12-18 06:51:37
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/components/defi2.vue
-->
<template>
    <div>
        <div style="text-align: center; font-size: 18px; margin-bottom: 30px">tokens</div>
        <el-table
        :data="tableData"
        stripe
        @row-click=rowClick
        style="width: 100%">
        <el-table-column
        prop="symbol"
        label="symbol"
        >
        </el-table-column>
        <el-table-column
        prop="balance"
        label="balance"
        >
        </el-table-column>
    </el-table>
    
    <el-divider></el-divider>


    <div style="text-align: center; font-size: 18px; margin-top:30px">swap token</div>
        <el-table
        :data="tableData1"
        stripe
        @row-click=rowClick1
        style="width: 100%">
        <el-table-column
        prop="symbol"
        label="symbol"
        >
        </el-table-column>
        <el-table-column
        prop="balance"
        label="balance"
        >
        </el-table-column>
    </el-table>
    </div>
</template>

<script>
export default {
    name: 'defi2',
    methods: {
        rowClick(row){
            this.$router.push('/defi3/'+row.symbol+'/'+row.key+'/'+this.principal+'/0')
        },
        rowClick1(row){
            this.$router.push('/defi3/'+row.symbol+'/'+row.key+'/'+this.principal+'/1')
        }
    },
    data() {
        return {
            'infoid': 'defi2',
            tableData: [],
            tableData1: [],
            keys: [],
            principal: ''
        }
    },
    created(){
        this.principal = '4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae'
        this.fetchData('dfitokenbalances', {
            principal: this.principal
        }).then((res)=>{
            for (var v in res){
                this.tableData.push({
                    'symbol': res[v][0],
                    'balance': res[v][1],
                    'key': v
                })
            }
        })

        this.fetchData('dfidswapbalances', {
            principal: '4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae'
        }).then((res)=>{
            for (var v in res){
                this.tableData1.push({
                    'symbol': res[v][0],
                    'balance': res[v][1],
                    'key': v
                })
            }
        })

        // 
    },
    beforeMount() {
        // console.log(this.$route.params.id)
        this.infoid = this.$route.params.id
    }
}
</script>