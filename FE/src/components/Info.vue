<!--
 * @Author: QHGG
 * @Date: 2021-12-04 21:52:40
 * @LastEditTime: 2021-12-18 05:30:10
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/components/Info.vue
-->
<template>
  <div>
    <!-- {{ infoid }}
    {{ index }} -->
    <ve-line :data="chartData"></ve-line>
    <el-table :data="tableData" stripe style="width: 100%">
      <el-table-column prop="0" label="from"> </el-table-column>
      <el-table-column prop="1" label="to" > </el-table-column>
      <el-table-column prop="2" label="price"> </el-table-column>
      <el-table-column prop="3" label="time">
        <template slot-scope="scope">
            {{timeEXP(scope.row[3]/1e6)}}
      </template>
         </el-table-column>
    </el-table>
  </div>
</template>

<script>
export default {
  name: "Info",
  methods: {},
  data() {
    return {
      index: '',
      infoid: '',
      chartSettings: {
        xAxisType: 'time'
      },
      tableData: [],
      chartData: {
          columns: ['time', 'price'],
          rows: []
        }
      
    };
  },
  created() {
    this.dapp_id = this.$route.params.dapp_id;
    this.index = this.$route.params.index;
    this.fetchData('info', {
      index: this.dapp_id, 
      dapp_id: this.index
    }).then((res)=>{
        this.tableData = res
        this.chartData.rows =res.map(item=>{
          return {
            'price': item[2],
            'time': this.timeEXP(item[3]/1e6)
        }})
    })
  },
};
</script>
