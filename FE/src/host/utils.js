
const getQuery = (name, decode) => {
    const reg = new RegExp("(^|&)" + name + "=([^&]*)(&|$)");
    const r = window.location.search.substr(1).match(reg);
    if (r != null) {
        return decode == 'url' ? decodeURIComponent(r[2]) : decodeURI(r[2]);
    }
    return '';
};
export default getQuery;
