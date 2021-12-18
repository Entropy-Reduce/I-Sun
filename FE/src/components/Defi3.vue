<!--
 * @Author: QHGG
 * @Date: 2021-12-04 21:52:40
 * @LastEditTime: 2021-12-18 05:29:47
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/components/Defi3.vue
-->
<template>
    <div>
        <div style="text-align: center; font-size: 18px; margin-bottom: 30px">{{sym}}</div>

        <el-table
        v-if="index=='0'"
        :data="tableData"
        stripe
        style="width: 100%">
        <el-table-column
        prop="from"
        label="from">
        </el-table-column>
        <el-table-column
        prop="amount"
        label="amount">
        </el-table-column>
        <el-table-column
        prop="op"
        label="operation">
        </el-table-column>
        <el-table-column
        prop="to"
        label="to">
        </el-table-column>
        <el-table-column
        prop="timestamp"
        label="time">
        <template slot-scope="scope">
            {{timeEXP(scope.row.timestamp/1e6)}}
      </template>
        </el-table-column>
        <el-table-column
        prop="successful"
        label="status">
        <template slot-scope="scope">
            {{scope.row.successful ? 'success' : 'failed'}}
      </template>
        </el-table-column>
    </el-table>
    <el-table
        v-else
        :data="tableData"
        stripe
        style="width: 100%">
        <el-table-column
        prop="from"
        label="from">
        </el-table-column>
        <el-table-column
        prop="amount"
        label="amount1">
        </el-table-column>
        <el-table-column
        prop="amount0"
        label="amount2">
        </el-table-column>
        <el-table-column
        prop="amount1"
        label="amount3">
        </el-table-column>
        <el-table-column
        prop="op"
        label="operation">
        </el-table-column>
        <el-table-column
        prop="to"
        label="to">
        </el-table-column>
        <el-table-column
        prop="timestamp"
        label="time">
        <template slot-scope="scope">
            {{timeEXP(scope.row.timestamp/1e6)}}
      </template>
        </el-table-column>
        <el-table-column
        prop="successful"
        label="status">
        <template slot-scope="scope">
            {{scope.row.successful ? 'success' : 'failed'}}
      </template>
        </el-table-column>
    </el-table>
    </div>
</template>

<script>
export default {
    name: 'defi3',
    methods: {

    },
    data() {
        return {
            principal: '',
            index: '',
            key: '',
            sym: '',
            tableData: [{
                date: '2016-05-02',
                name: '王小虎',
                address: 'defi3'
                }]
        }
    },
    created(){
        this.principal = this.$route.params.id
        this.index = this.$route.params.index
        this.key = this.$route.params.key
        this.sym = this.$route.params.sym
        let path = 'dfitrans'
        if (this.index == '1'){
            path = 'dfidswap'
        }
        this.fetchData(path, {
            principal: this.principal,
            canister_id: this.key,
            from: 0,
            to: 100
        }).then((res)=>{
            this.tableData = res
        })
    },
}
</script>