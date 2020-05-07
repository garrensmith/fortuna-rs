// let lib = {};
// let map_funs = [];
//
// function init(libJSON, mapFunsJSON) {
//     try {
//         lib = JSON.parse(libJSON);
//     } catch (ex) {
//         const ret = {"error": "invalid_library", "reason": ex.toString()};
//         return JSON.stringify(ret);
//     }
//
//     try {
//         mapFuns = Array.from(JSON.parse(mapFunsJSON), (source) => {
//             return eval(source)
//         })
//     } catch (ex) {
//         const ret = {"error": "invalid_map_functions", "reason": ex.toString()};
//         return JSON.stringify(ret);
//     }
//
//     return true;
// }
//
// let doc_results = [];
//
// function emit(key, value) {
//     doc_results.push([key, value]);
// }
//
// function mapEach(mapFun, doc) {
//     try {
//         doc_results = [];
//         mapFun(doc);
//         return doc_results;
//     } catch (ex) {
//         return ex.toString();
//     }
// };
//
// function mapDoc(docJSON) {
//     const doc = JSON.parse(docJSON);
//     const mapResults = Array.from(mapFuns, (mapFun) => {
//         return mapEach(mapFun, doc);
//     });
//
//     return mapResults;
// }
