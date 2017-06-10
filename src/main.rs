// #![feature(plugin)]
// #![plugin(clippy)]

#![allow(unused_imports)]
#![allow(unreachable_code)]

#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;
extern crate chrono;
#[macro_use]
extern crate url;
extern crate crypto;
extern crate base64;
extern crate xml;
extern crate mime;
extern crate time;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate uuid;

use azure::core::lease::{LeaseState, LeaseStatus, LeaseAction};
use azure::storage::client::Client;
use azure::storage::blob::{Blob, BlobType, LIST_BLOB_OPTIONS_DEFAULT, PUT_OPTIONS_DEFAULT,
                           PUT_BLOCK_OPTIONS_DEFAULT, PUT_PAGE_OPTIONS_DEFAULT,
                           LEASE_BLOB_OPTIONS_DEFAULT};
use azure::storage::container::{Container, PublicAccess, LIST_CONTAINER_OPTIONS_DEFAULT};
use azure::core::ba512_range::BA512Range;

use std::fs;
use time::Duration;
use chrono::TimeZone;

use azure::storage::table::TableService;

// use azure::storage::container::PublicAccess;

#[macro_use]
pub mod azure;

// use chrono::datetime::DateTime;
use chrono::UTC;

use mime::Mime;

use azure::cosmos::client::ResourceType;
use azure::cosmos::authorization_token::{AuthorizationToken, TokenType};

#[allow(unused_variables)]
fn main() {
    env_logger::init().unwrap();

    //let time = chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00").unwrap();

    //let time = time.with_timezone(&chrono::UTC);
    //println!("{}", time);

    //let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
    //let time = format!("{}", time.format("%a, %d %h %Y %T GMT"));

    //let time = time.with_timezone(&chrono::UTC);
    //let ret = azure::cosmos::string_to_sign("GET",
    //                                        ResourceType::Databases,
    //                                        "dbs/MyDatabase/colls/MyCollection",
    //                                        &time);
    //println!("{}", ret);

    //let time = chrono::UTC::now();

    //let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T00:00:00.000000000+00:00").unwrap();
    //let time = time.with_timezone(&chrono::UTC);

    //let time = format!("{}", time.format("%a, %d %h %Y %T GMT"));

    //let auth = generate_authorization("8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==",
    //                                  "GET",
    //                                  TokenType::Master,
    //                                  ResourceType::Databases,
    //                                  "dbs/MyDatabase/colls/MyCollection",
    //                                  &time);

    //println!("auth == {}", auth);

    let master_key = std::env::var("COSMOS_MASTER_KEY")
        .expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");


    let authorization_token = AuthorizationToken::new(&account, TokenType::Master, master_key)
        .unwrap();

    let c = azure::cosmos::client::Client::new(&authorization_token).unwrap();

    let colls = c.list_collections("test_db").unwrap();
    println!("colls == {:?}", colls);

    // delete mycollection if exists
    if let Some(coll_to_del) = colls.iter().find(|ref c| c.id == "mycollection") {
        c.delete_collection("test_db", coll_to_del).unwrap();
        println!("collection deleted!");
    }

    let ep = azure::cosmos::collection::ExcludedPath { path: "".to_owned() };

    let indexes = azure::cosmos::collection::IncludedPathIndex {
        kind: azure::cosmos::collection::KeyKind::Hash,
        data_type: azure::cosmos::collection::DataType::String,
        precision: Some(3),
    };

    let ip = azure::cosmos::collection::IncludedPath {
        path: "/*".to_owned(),
        indexes: vec![indexes],
    };


    let ip = azure::cosmos::collection::IndexingPolicy {
        automatic: true,
        indexing_mode: azure::cosmos::collection::IndexingMode::Consistent,
        included_paths: vec![ip],
        excluded_paths: vec![],
    };


    let coll = azure::cosmos::collection::Collection::new("mycollection", ip);

    let created_coll = c.create_collection("test_db", 400, &coll).unwrap();
    println!("collection created == {:?}", created_coll);


    //// now let's get back the created collection and add an index
    //// using replace_collection
    //let mut created_coll = c.get_collection("test_db", &created_coll).unwrap();
    //println!("\nretrieved coll = {:?}", created_coll);
    //created_coll.indexing_policy.included_paths[0]
    //    .indexes
    //    .push(azure::cosmos::collection::IncludedPathIndex {
    //              data_type: azure::cosmos::collection::DataType::Point,
    //              precision: Some(-1),
    //              kind: azure::cosmos::collection::KeyKind::Hash,
    //          });

    //c.replace_collection("test_db", &created_coll).unwrap();
    //println!("collection replaced!");
    return;

    let tdb = c.get_database("test_db").unwrap();
    let coll = c.get_collection(&tdb, "test_collection").unwrap();
    println!("coll == {:?}", coll);

    return;

    let new_db = c.create_database("palazzo").unwrap();
    println!("palazzo created");

    let my_db = c.get_database("palazzo").unwrap();
    println!("{:?}", my_db);


    let dbs = c.list_databases().unwrap();

    println!("dbs.len() == {}", dbs.len());

    c.delete_database("palazzo").unwrap();

    println!("palazzo deleted");

    return;



    let azure_storage_account = match std::env::var("AZURE_STORAGE_ACCOUNT") {
        Ok(val) => val,
        _ => {
            panic!("Please set AZURE_STORAGE_ACCOUNT env variable first!");
        }
    };

    let azure_storage_key = match std::env::var("AZURE_STORAGE_KEY") {
        Ok(val) => val,
        _ => {
            panic!("Please set AZURE_STORAGE_KEY env variable first!");
        }
    };

    let client = Client::new(&azure_storage_account, &azure_storage_key, true);

    let policy_name = match std::env::var("AZURE_POLICY_NAME") {
        Ok(val) => val,
        _ => {
            panic!("Please set AZURE_POLICY_NAME env variable first!");
        }
    };

    let policy_key = match std::env::var("AZURE_POLICY_KEY") {
        Ok(val) => val,
        _ => {
            panic!("Please set AZURE_POLICY_KEY env variable first!");
        }
    };

    let sb_namespace = match std::env::var("AZURE_SERVICE_BUS_NAMESPACE") {
        Ok(val) => val,
        _ => {
            panic!("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");
        }
    };

    let ev_name = match std::env::var("AZURE_EVENT_HUB_NAME") {
        Ok(val) => val,
        _ => {
            panic!("Please set AZURE_EVENT_HUB_NAME env variable first!");
        }
    };

    let table_service =
        TableService::new(Client::new(&azure_storage_account, &azure_storage_key, true));
    table_service.create_table("mytable").unwrap();


    let mut eh_client = azure::service_bus::event_hub::Client::new(&sb_namespace,
                                                                   &ev_name,
                                                                   &policy_name,
                                                                   &policy_key);
    info!("Enumerating containers");
    Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();
    info!("Enumeration completed");

    info!("Creating sample container");
    match Container::create(&client, "sample", PublicAccess::Blob) {
        Ok(_) => info!("container created"),
        Err(ref ae) => error!("error: {}", ae),
    };


    //client.create_container("balocco3", PublicAccess::Blob).unwrap();
    // println!("{:?}", new);


    info!("Beginning tests");

    for i in 0..20 {
        info!("Sending message {}", i);
        send_event(&mut eh_client);
    }

    // lease_blob(&client);

    // put_block_blob(&client);
    //
    // put_page_blob(&client);

    // {
    //     let vhds = ret.iter_mut().find(|x| x.name == "canotto").unwrap();
    //
    //     let blobs = vhds.list_blobs(&client, true, true, true, true).unwrap();
    //
    //     println!("len == {:?}", blobs.len());
    //
    //     for blob in &blobs {
    //         println!("{}, {} KB ({:?})",
    //                  blob.name,
    //                  (blob.content_length / 1024),
    //                  blob.lease_state)
    //     }
    //
    //     let (blob, mut stream) = vhds.get_blob(&client, "DataCollector01.csv", None, None, None)
    //                                  .unwrap();
    //     println!("blob == {:?}", blob);
    //
    //     let mut buffer = String::new();
    //     stream.read_to_string(&mut buffer).unwrap();
    //
    //     // println!("buffer == {:?}", buffer);
    // }


    // bal2.delete(&client).unwrap();
    // println!("{:?} deleted!", bal2);

    // let ret = client.delete_container("balocco2").unwrap();
    // println!("{:?}", ret);
    // inc_a!("main");
}

#[allow(dead_code)]
fn send_event(cli: &mut azure::service_bus::event_hub::Client) {
    debug!("running send_event");
    let file_name = "/home/mindflavor/samplein.json";

    let metadata = fs::metadata(file_name).unwrap();
    let mut file_handle = fs::File::open(file_name).unwrap();

    cli.send_event(&mut (&mut file_handle, metadata.len()), Duration::hours(1))
        .unwrap();
}

#[allow(dead_code)]
fn lease_blob(client: &Client) {
    println!("running lease_blob");

    let ret = Container::list(client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();
    let vhds = ret.iter().find(|x| x.name == "rust").unwrap();
    let blobs = Blob::list(client, &vhds.name, &LIST_BLOB_OPTIONS_DEFAULT).unwrap();
    let blob = blobs.iter().find(|x| x.name == "go_rust12.txt").unwrap();

    println!("blob == {:?}", blob);

    let mut lbo = LEASE_BLOB_OPTIONS_DEFAULT.clone();
    lbo.lease_duration = Some(30);
    let ret = blob.lease(client, LeaseAction::Acquire, &lbo).unwrap();
    println!("ret == {:?}", ret);

}

#[allow(dead_code)]
fn list_blobs(client: &Client) {
    println!("running list_blobs");

    let mut lbo2 = LIST_BLOB_OPTIONS_DEFAULT.clone();
    lbo2.max_results = 15;

    loop {
        let uc = Blob::list(client, "rust", &lbo2).unwrap();

        println!("uc {:?}\n\n", uc);

        if !uc.is_complete() {
            lbo2.next_marker = Some(uc.next_marker().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[allow(dead_code)]
fn put_block_blob(client: &Client) {
    use std::fs::metadata;
    use std::fs::File;

    println!("\nrunning put_block_blob");

    let blob_name: &'static str = "Win64OpenSSL-1_0_2e.exe";
    let file_name: &'static str = "C:\\temp\\Win64OpenSSL-1_0_2e.exe";
    let container_name: &'static str = "rust";
    let metadata = metadata(file_name).unwrap();
    let mut file = File::open(file_name).unwrap();

    let content_length = metadata.len();

    {
        let containers = Container::list(client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();

        let cont = containers.iter().find(|x| x.name == container_name);
        if cont.is_none() {
            Container::create(client, container_name, PublicAccess::Blob).unwrap();
        }
    }

    let new_blob = Blob {
        name: blob_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: UTC::now(),
        etag: "".to_owned(),
        content_length: content_length,
        content_type: "application/octet-stream".parse::<Mime>().unwrap(),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::BlockBlob,
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
        copy_id: None,
        copy_status: None,
        copy_source: None,
        copy_progress: None,
        copy_completion: None,
        copy_status_description: None,
    };

    new_blob
        .put_block(client,
                   "block_name",
                   &PUT_BLOCK_OPTIONS_DEFAULT,
                   (&mut file, 1024 * 1024))
        .unwrap();

    println!("created {:?}", new_blob);
}

#[allow(dead_code)]
fn put_page_blob(client: &Client) {
    use std::fs::metadata;
    use std::fs::File;

    println!("\nrunning put_page_blob");

    let blob_name: &'static str = "MindDB_Log.ldf";
    let file_name: &'static str = "C:\\temp\\MindDB_Log.ldf";
    let container_name: &'static str = "rust";
    let metadata = metadata(file_name).unwrap();
    let mut file = File::open(file_name).unwrap();

    {
        let containers = Container::list(client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();

        let cont = containers.iter().find(|x| x.name == container_name);
        if cont.is_none() {
            Container::create(client, container_name, PublicAccess::Blob).unwrap();
        }
    }

    // align to 512 bytes
    let content_length = metadata.len() % 512 + metadata.len();

    let new_blob = Blob {
        name: blob_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: UTC::now(),
        etag: "".to_owned(),
        content_length: content_length,
        content_type: "application/octet-stream".parse::<Mime>().unwrap(),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::PageBlob,
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
        copy_id: None,
        copy_status: None,
        copy_source: None,
        copy_progress: None,
        copy_completion: None,
        copy_status_description: None,
    };

    new_blob.put(client, &PUT_OPTIONS_DEFAULT, None).unwrap();

    let range = BA512Range::new(0, 1024 * 1024 - 1).unwrap();

    new_blob
        .put_page(client,
                  &range, // 1MB
                  &PUT_PAGE_OPTIONS_DEFAULT,
                  (&mut file, range.size()))
        .unwrap();

    println!("created {:?}", new_blob);
}
