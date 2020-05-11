use reqwest::Client;
use futures::{stream, StreamExt};

use prost::Message;
use ateles::{JsRequest};
use std::time::Instant;

pub mod ateles {
    tonic::include_proto!("ateles"); // The string specified here must match the proto package name
}

/*
    steps:
    * rewrite map funs
    * add map.js
    * init map funs
    * map docs
 */

async fn rewrite_map_funs(client: &Client) {
    let js_req = JsRequest {
        action: 0,
        script: "rewriteFun".to_string(),
        args: vec!["\"function(doc) {emit(doc._id, null);}\"".to_string()],
        timeout: 5000
    };

    let mut resp = Vec::<u8>::new();
    js_req.encode(&mut resp).unwrap();

    let _body = client.post("http://localhost:8444/Ateles/Execute")
        .body(resp)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // println!("text: {:?}", body);
}

async fn add_map_js(client: &Client) {
    let js_req = JsRequest {
        action: 1,
        script: MAP_JS.to_string(),
        args: vec!["file=map.js".to_string(), "line=1".to_string()],
        timeout: 5000
    };

    let mut resp = Vec::<u8>::new();
    js_req.encode(&mut resp).unwrap();

    let _body = client.post("http://localhost:8444/Ateles/Execute")
        .body(resp)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // println!("text: {:?}", body);
}


async fn init_map(client: &Client) {
    let js_req = JsRequest {
        action: 2,
        script: "init".to_string(),
        args: vec!["{}".to_string(), MAP_FUNS.to_string()],
        timeout: 5000
    };

    let mut resp = Vec::<u8>::new();
    js_req.encode(&mut resp).unwrap();

    let _body = client.post("http://localhost:8444/Ateles/Execute")
        .body(resp)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // println!("text: {:?}", body);
}

async fn map_doc(client: &Client) {
    let js_req = JsRequest {
        action: 2,
        script: "mapDoc".to_string(),
        args: vec![DOC.to_string()],
        timeout: 5000
    };

    let mut resp = Vec::<u8>::new();
    js_req.encode(&mut resp).unwrap();

    let _body = client.post("http://localhost:8444/Ateles/Execute")
        .body(resp)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // println!("text: {:?}", body);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let reqs = 0..1;

    let fetches = stream::iter(
        reqs.map(|_| {
            async {
                let docs = 0..10000;
                let client = Client::new();
                // let _body = client.get("http://localhost:8444/Health")
                //     .send()
                //     .await
                //     .unwrap()
                //     .text()
                //     .await
                //     .unwrap();

                rewrite_map_funs(&client).await;
                add_map_js(&client).await;
                init_map(&client).await;
                let doc_fetches = stream::iter(
                    docs.map(|_| {
                        async {
                            map_doc(&client).await;
                        }
                    })
                ).buffer_unordered(1).collect::<Vec<()>>();
                doc_fetches.await;

                // println!("text: {:?}", body);
                // Ok(())
            }
        })
    ).buffer_unordered(60).collect::<Vec<()>>();
    println!("Running...");
    fetches.await;

    println!("requests took {:?}", start.elapsed());
    Ok(())
}

const DOC: &str = "{\"_id\":\"foo\",\"value\":1}";

// const MAP_FUNS: &str = "[\"(function (doc) {\\n    emit(null, null);\\n});\"]";
const MAP_FUNS: &str = "
    [
        \"(function (doc) { emit(null, null);});\",
         \"(function (doc) {\
           let val = 0;\
           for(let i = 0; i < 1000; i++) {\
                val = i;\
            }\
            emit(doc._id, val);\
         });\"

    ]";

const MAP_JS: &str = r#"
  let lib = {};
let map_funs = [];

function init(libJSON, mapFunsJSON) {
    try {
        lib = JSON.parse(libJSON);
    } catch (ex) {
        const ret = {"error": "invalid_library", "reason": ex.toString()};
        return JSON.stringify(ret);
    }

    try {
        mapFuns = Array.from(JSON.parse(mapFunsJSON), (source) => {
            return eval(source)
        })
    } catch (ex) {
        const ret = {"error": "invalid_map_functions", "reason": ex.toString()};
        return JSON.stringify(ret);
    }

    return true;
}

let doc_results = [];

function emit(key, value) {
    doc_results.push([key, value]);
}

function mapEach(mapFun, doc) {
    try {
        doc_results = [];
        mapFun(doc);
        return doc_results;
    } catch (ex) {
        return ex.toString();
    }
};

function mapDoc(docJSON) {
    const doc = JSON.parse(docJSON);
    const mapResults = Array.from(mapFuns, (mapFun) => {
        return mapEach(mapFun, doc);
    });

    return mapResults;
}
"#;



