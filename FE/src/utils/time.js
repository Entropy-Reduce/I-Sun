/*
 * @Author: QHGG
 * @Date: 2021-12-18 05:16:28
 * @LastEditTime: 2021-12-18 05:35:04
 * @LastEditors: QHGG
 * @Description: 
 * @FilePath: /ii/src/utils/time.js
 */

function to2(i){
    return parseInt(i) < 10 ? '0'+i.toString(): i
}
function timeEXP(time){
    let md = new Date(time)
    let str = [md.getFullYear(), md.getMonth()+1, md.getDate()].join('/')
    return str + ' ' + to2(md.getHours())+ ':' + to2(md.getMinutes())
}

export default timeEXP