use std::time::Duration;

use crate::rpc::common::*;
use avalanche_types::subnet::rpc::database::{
    corruptabledb::Database as CorruptableDb,
    memdb::Database as MemDb,
    rpcdb::{client::DatabaseClient, server::Server as RpcDb},
};

use tokio::net::TcpListener;
use tonic::transport::Channel;

// Test to make sure the database iterates over the database
// contents lexicographically.
#[tokio::test]
async fn iterator_test() {
    let server = RpcDb::new(MemDb::new());

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        serve_test_database(server, listener).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client_conn = Channel::builder(format!("http://{}", addr).parse().unwrap())
        .connect()
        .await
        .unwrap();

    let mut db = CorruptableDb::new(DatabaseClient::new(client_conn));

    let key1 = "hello1".as_bytes();
    let value1 = "world1".as_bytes();
    let key2 = "hello2".as_bytes();
    let value2 = "world2".as_bytes();

    let _ = db.put(key1, value1).await.unwrap();
    let _ = db.put(key2, value2).await.unwrap();

    let resp = db.new_iterator().await;
    assert!(resp.is_ok());

    let mut iterator = resp.unwrap();

    // first
    assert!(iterator.next().await.unwrap());
    assert_eq!(iterator.key().await.unwrap(), key1);
    assert_eq!(iterator.value().await.unwrap(), value1);

    // second
    assert!(iterator.next().await.unwrap());
    assert_eq!(iterator.key().await.unwrap(), key2);
    assert_eq!(iterator.value().await.unwrap(), value2);

    assert_eq!(iterator.next().await.unwrap(), false);

    // cleanup
    let _ = iterator.release().await;
}

// Test to make sure the the iterator can be configured to
// start mid way through the database.
#[tokio::test]
async fn iterator_start_test() {
    let server = RpcDb::new(MemDb::new());

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        serve_test_database(server, listener).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client_conn = Channel::builder(format!("http://{}", addr).parse().unwrap())
        .connect()
        .await
        .unwrap();

    let mut db = CorruptableDb::new(DatabaseClient::new(client_conn));

    let key1 = "hello1".as_bytes();
    let value1 = "world1".as_bytes();
    let key2 = "goodbye".as_bytes();
    let value2 = "world2".as_bytes();

    let _ = db.put(key1, value1).await.unwrap();
    let _ = db.put(key2, value2).await.unwrap();

    let resp = db.new_iterator_with_start(key2).await;
    assert!(resp.is_ok());

    let mut iterator = resp.unwrap();

    assert!(iterator.next().await.unwrap());
    assert_eq!(iterator.key().await.unwrap(), key2);
    assert_eq!(iterator.value().await.unwrap(), value2);

    // cleanup
    let _ = iterator.release().await;
}

// Test to make sure the iterator can be configured to skip
// keys missing the provided prefix.
#[tokio::test]
async fn iterator_prefix_test() {
    let server = RpcDb::new(MemDb::new());

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        serve_test_database(server, listener).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client_conn = Channel::builder(format!("http://{}", addr).parse().unwrap())
        .connect()
        .await
        .unwrap();

    let mut db = CorruptableDb::new(DatabaseClient::new(client_conn));

    let key1 = "hello1".as_bytes();
    let value1 = "world1".as_bytes();
    let key2 = "goodbye".as_bytes();
    let value2 = "world2".as_bytes();
    let key3 = "joy".as_bytes();
    let value3 = "world3".as_bytes();

    let _ = db.put(key1, value1).await.unwrap();
    let _ = db.put(key2, value2).await.unwrap();
    let _ = db.put(key3, value3).await.unwrap();

    let resp = db.new_iterator_with_prefix("h".as_bytes()).await;
    assert!(resp.is_ok());

    let mut iterator = resp.unwrap();

    assert!(iterator.next().await.unwrap());
    assert_eq!(iterator.key().await.unwrap(), key1);
    assert_eq!(iterator.value().await.unwrap(), value1);

    assert_eq!(iterator.next().await.unwrap(), false);

    // cleanup
    let _ = iterator.release().await;
}
