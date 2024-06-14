use warp::Filter;

mod blockchain;

use crate::blockchain::Blockchain;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Tambahkan blok-blok awal
    {
        let mut blockchain = blockchain.lock().unwrap();
        blockchain.add_block("First block after genesis".to_string());
        blockchain.add_block("Second block".to_string());
        blockchain.add_block("Third block".to_string());

        // Cetak seluruh blockchain
        for block in blockchain.chain.iter() {
            println!("{:#?}", block);
        }

        // Periksa apakah blockchain valid
        if blockchain.is_valid() {
            println!("Blockchain is valid!");
        } else {
            println!("Blockchain is not valid!");
        }
    }

    // API untuk menambahkan blok
    let add_block = {
        let blockchain = blockchain.clone();
        warp::path!("add_block" / String)
            .map(move |data: String| {
                let mut blockchain = blockchain.lock().unwrap();
                blockchain.add_block(data);
                warp::reply::json(&*blockchain)
            })
    };

    // API untuk take dpe block
    let get_blocks = {
        let blockchain = blockchain.clone();
        warp::path!("blocks")
            .map(move || {
                let blockchain = blockchain.lock().unwrap();
                warp::reply::json(&*blockchain)
            })
    };
// route API 
    let routes = add_block.or(get_blocks);

    //run warp API
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
