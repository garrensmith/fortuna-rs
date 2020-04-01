// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

let lib = {};
const mapFuns = new Map();
let results = [];

function emit(key, value) {
    results.push([key, value]);
}

function setLib(source) {
    lib = JSON.parse(source);
}

function addFun(id, source) {
    print("WOO HOO");
    const fixedSource = rewriteAnonFun(source);
    const fun = eval(fixedSource);
    print(fun);
    if (typeof fun === "function") {
        mapFuns.set(id, fun);
    } else {
        throw "Invalid function";
    }
}

const mapFunRunner = (doc, mapFn) => {
    try {
        results = [];
        mapFn(doc);
        return results;
    } catch (ex) {
        return { error: ex.toString() };
    }
};

function mapDoc(doc_str) {
    const doc = JSON.parse(doc_str);
    const mapResults = Array.from(mapFuns, ([id, mapFun]) => {
        // const mapResult = { id };
        mapResult = [];

        print(`mapping ${mapFun}`);
        const result = mapFunRunner(doc, mapFun);
        if (result.error) {
            // mapResult.error = result.error;
            return [];
            // mapResult.push([]);
        }

        return result;
    });

    return JSON.stringify(mapResults);
}

function clearFuns() {
    mapFuns.clear();
}

if (typeof module !== "undefined") {
    module.exports = {
        mapDoc,
        addFun,
        setLib,
        clearFuns
    };
}
