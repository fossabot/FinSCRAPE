//this will be used to parse json into structs
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

//this will be used to get json from server
extern crate reqwest;

//this will be used to create and add to databases
/* HARDEST TASK */
extern crate rusqlite;

//this will be used to query the storage devices available
extern crate systemstat;

//this will send notifications to operator email
extern crate lettre;

//this will be used to read/write csv files
extern crate csv;

use std::option;
use std::time::Instant;
use std::collections::HashMap;

//before running this as production, the pi should be set up running off the usb A port on the powerbar
//with the hard drive plugged in to the AC port. should use the 80gb spinner and the 64gb usb as the initial
//pair of hot swaps, this will give you a month on each

/*
fn set_disk(mut db: DB) -> DB {
    //check conf file for current db dir in binary directory
    //get a list of storage devices
    //get a list of storage utilization and capacity
    //if current disk is > 2/3 full, notify() and
    //excluding the current db disk, find a disk that has greater than 33 gb capacity
    //if none continue
    //if some set db path field to new disk, notify(changed db path)

    //leave a note of current dir in binary directory
    //set path in db struct

    //conf file should be blank on first run,
    //and at the beginnig, if blank, use first disk larger than 33 gb

    //set_labels()
}
*/

/*
fn notify(reason: &Notify) {
    //this will send emails or other correspondence to the operator
}
*/

/*0th
fn get_data() -> (HashMap<String, CryptoFiat>, i64) {
    //sleep till time is a multiple of 30
    //get response
        //https://min-api.cryptocompare.com/data/pricemultifull?fsyms=BTC,ETH,BCH,LTC,EOS,BNB,XMR,DASH,VEN,NEO,ETC,ZEC,WAVES,BTG,DCR,REP,GNO,MCO,FCT,HSR,DGD,XZC,VERI,PART,GAS,ZEN,GBYTE,BTCD,MLN,XCP,XRP,MAID&tsyms=USD,EUR,JPY,GBP,AUD,CHF,CAD,CNY,KRW&api_key={6cbc5ffe92ca7113e33a5f379e8d73389d6f8a1ba30d10a003135826b0f64815}
    //deserialize the response.text using json into the frame map
        //let data: Value = serde_json::from_str(response.text)?;
        //for crypto in data["RAW"].keys():
        //  for fiat in data["RAW"][crypto].keys():
        //      let pair_block: CryptoFiat = serde_json::from_str(data["RAW"][crypto][fiat])?;
        //      frame[format!("{}-{}", crypto, fiat)] = pair_block;

    //set_labels()
    //return the frame map
    //a frame will be a unordered mut map of
    //<<"$crypto-$fiat">, <CryptoFiat>>
    //assert_eq(frame["BTC-USD"].price, 3626.4) (for instance)

    //can be converted to immutable after get_data
}
*/

/* 2nd
fn write_data(frame: &HashMap<String, CryptoFiat>, timestamp: &i64) {
    //for each write, do checks if db, table, etc exist
    //that way if the disk is changed it can write a new db
    //rather than loosing a row
    //for pair in frame.keys():
    //  writeVEC = arrange_vec(frame[pair], timestamp)
    //  create table called pair if none
    //  write new row to table using writevec
    //set_labels() if the write to single db takes to long, this can be par_itered with multiple dbs instead of tables
    //if new db/tables etc then notify(new db established) as soon as the first row is written
    //that should be the safe to unmount notification for the previous drive
}
*/

/* 1st
fn arrange_vec(pair: &CryptoFiat, timestamp: &i64) -> Vec<String> {
    //because we use this functionality twice, it will be called
    //from queue frames and write data
    //  let writeVEC = [];
    //  writeVEC.append(timestamp.to_string())
    //  writeVEC.append(pair.last_update.to_string())
    //  writeVEC.append(pair.price.to_string())
    //  writeVEC.append(pair.last_trade_id.to_string())
    //  writeVEC.append(pair.last_market.to_string())
    //  writeVEC.append(pair.last_volume_crypto.to_string())
    //  writeVEC.append(pair.volume_hour_crypto.to_string()) 
    //  writeVEC.append(pair.volume_day_crypto.to_string())
    //  writeVEC.append(pair.volume_24_hour_crypto.to_string())
    //  writeVEC.append(pair.total_volume_24_hour_crypto.to_string())
    //  writeVEC.append(pair.last_volume_fiat.to_string())
    //  writeVEC.append(pair.volume_hour_fiat.to_string())
    //  writeVEC.append(pair.volume_day_fiat.to_string())
    //  writeVEC.append(pair.volume_24_hour_fiat.to_string())
    //  writeVEC.append(pair.total_volume_24_hour_fiat.to_string())
    //  writeVEC.append(pair.change_day.to_string())
    //  writeVEC.append(pair.change_pct_day.to_string())
    //  writeVEC.append(pair.change_24_hour.to_string())
    //  writeVEC.append(pair.change_pct_24_hour.to_string())
    //  writeVEC.append(pair.supply.to_string())
    //  writeVEC.append(pair.market_cap.to_string())
    //  writeVEC.append(pair.open_hour.to_string())
    //  writeVEC.append(pair.high_hour.to_string())
    //  writeVEC.append(pair.low_hour.to_string())
    //  writeVEC.append(pair.open_day.to_string())
    //  writeVEC.append(pair.high_day.to_string())
    //  writeVEC.append(pair.low_day.to_string())
    //  writeVEC.append(pair.open_24_hour.to_string())
    //  writeVEC.append(pair.high_24_hour.to_string())
    //  writeVEC.append(pair.low_24_hour.to_string())

}
*/

/* 5th
fn queue_frames(mut queue: HashMap<String, Vec<Vec<String>>>, 
                frame: &HashMap<String, CryptoFiat>, 
                timestamp: &i64
                ) -> HashMap<String, Vec<Vec<String>>> {
    //this should read the agent conf file and set window_size and interval
    //push each new frame to the queue until the queue is == 10 frames
    //then remove the 0th frame each time a frame is pushed to the queue

    //it should get a writeVEC for each pair in the frame
    //then assemble the writeVECS in the following fashion

    //for pair in frame.keys():
    //  let writeVEC = arrange_vec(frame[pair], timestamp)
    //  if queue[pair][-1][0] - writeVEC[0] >= interval:
    //      queue[pair].append(writeVEC)
    //      if queue[pair].len() > window_size:
    //          queue[pair].remove(0)
    //
    //queue is hashmap<String, Vec<Vec<String>>> (
    //                                      "BTC-USD": [writeVEC0, writeVEC1], 
    //                                      "ETH-USD": [writeVEC0, writeVEC1]
    //                                    )
    //with each subkey a hashmap (of different pairs) at a different timestamp
}
*/

/* 3rd
fn set_labels(mut metricVEC: Vec<i64>, duration: i64) -> Vec<i64> {
    //this will be called from inside functions to update the metrics struct
    //framestamp, set_disk, get_agent_config, get_data, queue_frames, inform_agent, write_data, agent_action, main_loop
    //each field will be an int calculated by timecomplete - timestart
    
}
*/

/* 4th
fn measure(metricVEC: Vec<i64>, db: DB) {
    //for each write do checks if db, table, etc exist
    //that way if the disk is changed it can write a new db
    //rather than loosing a row

    //framestamp, storage_device, set_disk, get_agent_config, get_data, queue_frames, inform_agent, write_data, agent_action, main_loop
    //each field will be an int calculated by timecomplete - timestart

    //another rust script could be created which goes over the metrics database
    //and notifies if things get out of bounds or exceed expectations (usually not for free)
}
*/

/* 6th
fn inform_agent(queue: &HashMap<String, Vec<Vec<String>>>) {
    //this should write a csv file named by each key in queue
    //write hardcoded header
    //one writevec per line following that, comma seperated per index
    //using the required coin pair's csv, the agent should read these and act whenever a new timestamp is found at the last index
    //the agent should check every 2-5s
    //and set a third file with a read->action_complete pair of time stamps for metrics
    //set_labels()
}
*/

/* 7th
fn get_agent_metrics() {
    //this will read the agent's read->action_complete timestamps and replace the previous ones
    //the agent's metric file should have headings framestamp, duration
    //the agent should append to this file instead of rewriting, so that this function
    //can update a previous row if the agents metric is missing
    //set_labels()
}
*/

enum Notify {
    ChangedDB,
    FirstWrite,
    LowStorage,
    ChangedConfig
}

struct DB {
    path: Option<String>,
    storage_device: Option<String>

}

#[derive(Serialize, Deserialize)]
struct CryptoFiat {
    //data["RAW"]["$CRYPTO"]["$FIAT"]
    //this is where we put the json after it is broken down untyped into crypto-fiat pairs
    class: String,
    market:String,
    crypto_symbol: String,
    fiat_symbol:String,
    flags: String,
    price: f64,
    last_update: i64,
    last_volume_crypto: f64,
    last_volume_fiat: f64,
    last_trade_id:i64,
    volume_day_crypto: f64,
    volume_day_fiat: f64,
    volume_24_hour_crypto: f64,
    volume_24_hour_fiat: f64,
    open_day: f64,
    high_day: f64,
    low_day: f64,
    open_24_hour: f64,
    high_24_hour: f64,
    low_24_hour: f64,
    last_market: String,
    volume_hour_crypto: f64,
    volume_hour_fiat: f64,
    open_hour: f64,
    high_hour: f64,
    low_hour: f64,
    change_24_hour: f64,
    change_pct_24_hour: f64,
    change_day: f64,
    change_pct_day: f64,
    supply: i64,
    market_cap: i64,
    total_volume_24_hour_crypto: f64,
    total_volume_24_hour_fiat: f64

}

fn main() {
//perf: keys can be str, 
//vecs and hashmaps all have length known, and can be defined

    let mut db = DB{
        path: None,
        storage_device: None
    };
    let mut queue: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    loop{
        let mut metricVEC: Vec<i64> = vec![];
        let start = Instant::now();
        //set_disk(db);
        let duration = start.elapsed().as_secs() as i64;
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //let (frame, timestamp) = get_data();
        let duration = start.elapsed().as_secs() as i64;
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //queue = queue_frames(queue, &frame, &timestamp);
        let duration = start.elapsed().as_secs() as i64;
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //inform_agent(&queue);
        let duration = start.elapsed().as_secs() as i64;
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //write_data(&frame, &timestamp);
        let duration = start.elapsed().as_secs() as i64;
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //get_agent_metrics();
        let duration = start.elapsed().as_secs() as i64;
        //metricVEC = set_labels(metricVEC, duration);

        //measure(metricVEC, db);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //utils
    fn get_fake_data()-> (frame, timestamp) {
        
    }

    //unit tests
    #[test]
    fn set_disk_group(){

    }

    #[test]
    fn notify_group(){

    }

    #[test]
    fn get_data_group(){

    }

    #[test]
    fn write_data_group(){

    }

    #[test]
    fn queue_frames_group(){

    }

    #[test]
    fn set_labels_group(){

    }

    #[test]
    fn measure_group(){

    }

    #[test]
    fn inform_agent_group(){

    }

    #[test]
    fn get_agent_metrics_group(){

    }

    #[test]
    fn get_agent_config_group(){

    }
}