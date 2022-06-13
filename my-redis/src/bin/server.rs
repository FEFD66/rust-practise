use std::{collections::{HashMap, hash_map::DefaultHasher}, sync::{Mutex, Arc}, hash::{Hash, Hasher}};

use bytes::Bytes;
use mini_redis::{Connection, Frame, Command};
use tokio::{net::{TcpListener,TcpStream}, task::JoinHandle};

// type Db = Arc<Mutex<HashMap<String,Bytes>>>;
// 将锁分片
type ShardedDb = Arc<Vec<Mutex<HashMap<String,Bytes>>>>;
/** new_sharded_db(num_shards:usize)
 num_shards:分片数
 */
fn new_sharded_db(num_shards: usize)->ShardedDb{
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards{
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = new_sharded_db(10);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        let _handle:JoinHandle<_> = tokio::spawn(async move {
            process(socket,db).await;
        });
    }
}
async fn process(socket: TcpStream,db:ShardedDb){
    // use HashMap to store data
    // let mut db = HashMap::new();
    // mini-redis 提供的函数，可用于从socket中读取数据并解析为数据帧
    let mut connection = Connection::new(socket);
    // 获取数据帧：一条命令+数据
    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(){
            Command::Set(cmd) => {
                let mut s = DefaultHasher::new();
                cmd.key().hash(&mut s);
                let mut shard = db[s.finish() as usize % db.len()].lock().unwrap();
                shard.insert(cmd.key().to_string(),cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Command::Get(cmd) => {
                let mut s = DefaultHasher::new();
                cmd.key().hash(&mut s);
                let shard = db[s.finish() as usize % db.len()].lock().unwrap();
                if let Some(value)=shard.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                }else{
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented command: {:?}",cmd),
            // Command::Publish(_) => todo!(),
            // Command::Subscribe(_) => todo!(),
            // Command::Unsubscribe(_) => todo!(),
            // Command::Unknown(_) => todo!(),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
