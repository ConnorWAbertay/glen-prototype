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

const URL: &str = "mysql://sql1904676:cKDwBoIwnvSo@lochnagar.abertay.ac.uk/sql1904676";

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
pub fn insert_manu(manu : &str)
{
    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    conn.exec_drop("INSERT INTO `glenmanu`(`name`) VALUES (:new_manu)", 
    params! {
        "new_manu" => manu
    }).unwrap();
}

#[tauri::command]
pub fn insert_type(glentype : &str)
{
    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    conn.exec_drop("INSERT INTO `glentype`(`name`) VALUES (:new_type)", 
    params! {
        "new_type" => glentype
    }).unwrap();
}

#[tauri::command]
pub fn insert_model(model : &str, manuid : i32)
{
    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    conn.exec_drop("INSERT INTO `glenmodel`(`name`, `manuid`) VALUES (:model,:manuid)", 
    params! {
        "model" => model,
        "manuid" => manuid
    }).unwrap();
}

#[tauri::command]
pub fn insert_asset(info:UpdateInfo)
{
    let opts = Opts::from_url(URL).unwrap();
    let mut conn = Conn::new(opts).unwrap();

    conn.exec_drop("INSERT INTO `glenasset`(`name`, `modelid`, `typeid`, `ip`) VALUES (:name,:modelid,:typeid,:ip)", 
    params! {
        "name" => info.name,
        "modelid" => info.modelid,
        "typeid" => info.typeid,
        "ip" => info.ip
    }).unwrap();
}
