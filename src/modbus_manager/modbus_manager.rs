extern crate modbus_iiot;
extern crate mongodb;
use std::borrow::BorrowMut;
use std::collections::HashMap;

use modbus_iiot::tcp::master::TcpClient;
use modbus_iiot::tcp::masteraccess::{CoilValue, MasterAccess};
// use mongodb::bson::to_document;
use mongodb::Client;
use serde::{Deserialize, Serialize};
// use serde_json::{Map,value};
// use std::result;
// use mongodb::bson::Document;
// use mongodb::{Client, options::ClientOptions, bson::doc};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModMetaData {
    pub data: Vec<u16>,
    pub address: u16,
}

pub fn reading_input_registers(client: &mut TcpClient, address: u16, quantity: u16) -> Vec<u16> {
    let response = client.read_input_registers(address, quantity);
    println!("Read  {:?}", response);
    response
}
pub fn write_single_register(client: &mut TcpClient, address: u16, value: u16) {
    let response = client.write_single_register(address, value);
    println!("Response WSRE: {:?}", response);
}

pub fn write_multiple_registers(client: &mut TcpClient, address: u16, values: Vec<u16>) {
    let response = client.write_multiple_registers(address, values);
    println!("Response WMRE: {:?}", response);
}

pub fn reading_holding_registers(client: &mut TcpClient, address: u16, quantity: u16) {
    let response = client.read_holding_registers(address, quantity);
    println!("Read  {:?}", response)
}

pub async fn create_collection(db_client: &Client, db_name: &str, collection_name: &str) {
    let db = db_client.database(db_name);
    db.create_collection(collection_name, None).await.unwrap();
}

// pub async fn insert_document(db_client: &Client, db_name: &str, collection_name: &str) {
//     let db = db_client.database(db_name);
//     let coll = db.collection(collection_name);
//     let data_test = vec![
//         ModMetaData {
//             data: vec![1,2,3],
//             address: 2,
//         },
//     ];
//     coll.insert_one(data_test, None).await.unwrap();


// }


// pub async fn write_to_collection(
//     db_client: &Client,
//     db_name: &str,
//     collection_name: &str,
//     data: Vec<u16>,
// ) {
//     let db = db_client.database(db_name);
//     let coll: mongodb::Collection<String> = db.collection(collection_name);

//     // modData.insert("data", String::from_utf16(data.as_ref()).unwrap());

//     let modMetaData =ModMetaData {
//         data: vec![10],
//     };
//     let doc = to_document(&modMetaData);
//     println!("doc : {:?}", doc);

//     // match coll
//     //     .insert_one(
//     //         serde_json::to_string(&modMetaData).unwrap().borrow_mut(),
//     //         None,
//     //     )
//     //     .await
//     // {
//     //     Ok(data) => {
//     //         // do something with data
//     //         println!("Data {:?}", data);
//     //     }
//     //     Err(e) => println!("Error: {:?}", e),
//     // }
// }
