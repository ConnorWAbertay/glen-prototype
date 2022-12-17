use mysql::prelude::*;
use mysql::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AssetInfo {
    id: i32,
    name: String,
    model: String,
    modelid: i32,
    manu: String,
    typeid: i32,
    glentype: String,
    ip: String,
}

const SELECT_STR: &str = "SELECT glenasset.id, glenasset.name, glenmodel.name as 'model', glenmodel.id as 'modelid', glenmanu.name as 'manu', glentype.id as 'typeid', glentype.name as 'glentype', ip FROM glenasset 
JOIN glenmodel ON glenasset.modelid = glenmodel.id 
JOIN glentype ON glenasset.typeid = glentype.id 
JOIN glenmanu ON glenmodel.manuid = glenmanu.id 
WHERE ";

const URL: &str = "///";

pub fn query_assets(query_string: &str) -> Vec<AssetInfo> {
    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    return conn
        .query_map(
            format!("{} {}", SELECT_STR, query_string),
            |(id, name, model, modelid, manu, typeid, glentype, ip)| AssetInfo {
                id,
                name,
                model,
                modelid,
                manu,
                typeid,
                glentype,
                ip,
            },
        )
        .unwrap();
}

#[derive(serde::Deserialize)]
pub struct UpdateInfo {
    id: i32,
    name: String,
    modelid: i32,
    typeid: i32,
    ip: String,
}

pub fn update_asset(info: UpdateInfo) {
    println!("{}", info.name);

    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    conn.exec_drop("UPDATE `glenasset` SET `name`=:name, `modelid`=:modelid, `typeid`=:typeid, `ip`=:ip WHERE `id` = :id",
    params! {
        "id" => info.id,
        "name" => info.name,
        "modelid" => info.modelid,
        "typeid" => info.typeid,
        "ip" => info.ip
    }).unwrap();
}

#[tauri::command]
pub fn get_name(query : &str) -> Vec<(i32, String)>
{
    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    return conn.query(query).unwrap();
}


#[tauri::command]
pub fn execute_drop(query : &str, params: Vec<serde_json::Value>)
{
    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    let mut converted_params: Vec<mysql::Value> = Vec::new();
    for p in params {
        match p {
            serde_json::Value::Number(i) => {
                let result: i64 = i.as_i64().unwrap();
                converted_params.push(mysql::Value::Int(result))
              }
            serde_json::Value::String(i) => {
                let result: &str = i.as_str();
                converted_params.push(mysql::Value::Bytes(result.as_bytes().to_vec()))
            }
            _ => {
                //UNHANDLED EXCEPTION
            }
        }
    }

    conn.exec_drop(query, converted_params).unwrap();
}
