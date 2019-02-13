// vscode-fold=1
#[macro_use]
extern crate serde_derive;

use serde_json::{Value, from_str};
use serde::{Deserialize, Serialize};

use rusqlite::{Connection, NO_PARAMS, MappedRows, Row};

use serial_test_derive::serial;

use std::str;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::env;
use std::option;
use std::result;
use std::io::prelude::*;
use std::{thread, time};
use std::time::{Duration, Instant};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::time::{SystemTime, UNIX_EPOCH};


//before running this as production, the pi should be set up running off the usb A port on the powerbar
//with the hard drive plugged in to the AC port. should use the 80gb spinner and the 64gb usb as the initial
//pair of hot swaps, this will give you a month on each

 #[allow(bad_style)]

/* 9th
fn set_disk(mut master: &DB, mut metrics: &DB) {
    //check conf file for current master dir in binary directory
    //get a list of storage devices
    //get a list of storage utilization and capacity
    //if current disk is > 2/3 full, notify() and
    //excluding the current master disk, find a disk that has greater than 33 gb capacity
    //if none continue
    //if some set master path field to new disk, notify(changed master path)

    //leave a note of current dir in binary directory
    //set path in master struct

    //conf file should be blank on first run,
    //and at the beginnig, if blank, use first disk larger than 33 gb

    //set_labels()
}
*/

//this is pretest
fn notify(notification: &Notify) {
    //this will send emails or other correspondence to the operator
    match notification {
        Notify::ChangedDB(info) => println!("the DB has been changed to: {}", info),
        Notify::FirstWrite(timestamp) => println!("the new DB has been written to at: {}", timestamp),
        Notify::LowStorage(info) => println!("the storage is low on this volume: {}", info),
        Notify::ChangedConfig(config) => println!("the config has successfully been changed to: {}", config),
        Notify::InvalidConfig(config) => println!("reverting to previous config, new config failed to parse: {}", config)
    }

}

fn get_data() -> (HashMap<String, CryptoFiat>, u64) {
    let mut json = "".to_string();
    let mut timestamp = 0;
    loop {
        timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if timestamp % 30 == 0 {
            json = reqwest::get("https://min-api.cryptocompare.com/data/pricemultifull?fsyms=BTC,ETH,BCH,LTC,EOS,BNB,XMR,DASH,VEN,NEO,ETC,ZEC,WAVES,BTG,DCR,REP,GNO,MCO,FCT,HSR,DGD,XZC,VERI,PART,GAS,ZEN,GBYTE,BTCD,MLN,XCP,XRP,MAID&tsyms=USD&api_key={6cbc5ffe92ca7113e33a5f379e8d73389d6f8a1ba30d10a003135826b0f64815}")
                //this should default to a frame of default primatives if the connection times out or other
                .expect("the request to the cryptocompare api failed")
                .text().expect("unable to get text from the cryptocompare api response");

            break
        } else {
            let sleep_time = time::Duration::from_secs(1);
            thread::sleep(sleep_time);
        }
    }

    let mut frame = HashMap::new();
    let data: Value = serde_json::from_str(&json).expect("unable to convert response text to untyped object");
    let object = data.as_object().expect("unable to convert outer values to map");
    let object = object["RAW"].as_object().expect("unable to convert inner values to map");
    for crypto in object.keys() {
        for fiat in object[crypto].as_object().unwrap().keys() {
            let pair_block: CryptoFiat = serde_json::from_value(object[crypto][fiat].clone()).expect("failed to convert untyped map to typed struct");
            frame.entry(format!("{}and{}", crypto, fiat)).or_insert(pair_block);
        }
    }

    (frame, timestamp)

}

fn write_data(frame: &HashMap<String, CryptoFiat>, timestamp: &u64, master: &DB) {
    let db_path = master.path.to_owned();
    let db_path = db_path.unwrap();
    let storage = Connection::open(db_path).expect("failed to open or create master");
    
    for table_name in frame.keys() {
            let table_statement = format!("CREATE TABLE IF NOT EXISTS {} (
                    timestamp              INTERGER NOT NULL,
                    last_update            INTEGER NOT NULL,
                    price    REAL NOT NULL,
                    last_market    TEXT NOT NULL,
                    last_volume_crypto    REAL NOT NULL,
                    volume_hour_crypto    REAL NOT NULL,
                    volume_day_crypto    REAL NOT NULL,
                    volume_24_hour_crypto    REAL NOT NULL,
                    total_volume_24_hour_crypto REAL NOT NULL,
                    last_volume_fiat    REAL NOT NULL,
                    volume_hour_fiat    REAL NOT NULL,
                    volume_day_fiat    REAL NOT NULL,
                    volume_24_hour_fiat    REAL NOT NULL,
                    total_volume_24_hour_fiat    REAL NOT NULL,
                    change_day    REAL NOT NULL,
                    change_pct_day    REAL NOT NULL,
                    change_24_hour    REAL NOT NULL,
                    change_pct_24_hour    REAL NOT NULL,
                    supply    REAL NOT NULL,
                    market_cap    REAL NOT NULL,
                    open_hour    REAL NOT NULL,
                    high_hour    REAL NOT NULL,
                    low_hour    REAL NOT NULL,
                    open_day    REAL NOT NULL,
                    high_day    REAL NOT NULL,
                    low_day    REAL NOT NULL,
                    open_24_hour    REAL NOT NULL,
                    high_24_hour    REAL NOT NULL,
                    low_24_hour    REAL NOT NULL
                  )", table_name);
            storage.execute(&table_statement, NO_PARAMS).expect("failed to create table");
    }
    for key in frame.keys(){
        let pair = &frame[key];
        let writeVEC = arrange_vec(&pair, &timestamp);
        let table_statement = format!("INSERT INTO {} (
                    timestamp,
                    last_update,
                    price,
                    last_market,
                    last_volume_crypto,
                    volume_hour_crypto,
                    volume_day_crypto,
                    volume_24_hour_crypto,
                    total_volume_24_hour_crypto,
                    last_volume_fiat,
                    volume_hour_fiat,
                    volume_day_fiat,
                    volume_24_hour_fiat,
                    total_volume_24_hour_fiat,
                    change_day,
                    change_pct_day,
                    change_24_hour,
                    change_pct_24_hour,
                    supply,
                    market_cap,
                    open_hour,
                    high_hour,
                    low_hour,
                    open_day,
                    high_day,
                    low_day,
                    open_24_hour,
                    high_24_hour,
                    low_24_hour
                    ) VALUES (
                        ?1,
                        ?2,
                        ?3,
                        ?4,
                        ?5,
                        ?6,
                        ?7,
                        ?8,
                        ?9,
                        ?10,
                        ?11,
                        ?12,
                        ?13,
                        ?14,
                        ?15,
                        ?16,
                        ?17,
                        ?18,
                        ?19,
                        ?20,
                        ?21,
                        ?22,
                        ?23,
                        ?24,
                        ?25,
                        ?26,
                        ?27,
                        ?28,
                        ?29
                    )", key
            );
            storage.execute(&table_statement, writeVEC).expect("failed to write to master");
    }
    storage.close().expect("failed to close the db");
}

fn arrange_vec(pair: &CryptoFiat, timestamp: &u64) -> Vec<String> {    
    let mut writeVEC: Vec<String> = vec![];
    writeVEC.push(timestamp.to_string());
    writeVEC.push(pair.last_update.to_string());
    writeVEC.push(pair.price.to_string());
    writeVEC.push(pair.last_market.to_string());
    writeVEC.push(pair.last_volume_crypto.to_string());
    writeVEC.push(pair.volume_hour_crypto.to_string()); 
    writeVEC.push(pair.volume_day_crypto.to_string());
    writeVEC.push(pair.volume_24_hour_crypto.to_string());
    writeVEC.push(pair.total_volume_24_hour_crypto.to_string());
    writeVEC.push(pair.last_volume_fiat.to_string());
    writeVEC.push(pair.volume_hour_fiat.to_string());
    writeVEC.push(pair.volume_day_fiat.to_string());
    writeVEC.push(pair.volume_24_hour_fiat.to_string());
    writeVEC.push(pair.total_volume_24_hour_fiat.to_string());
    writeVEC.push(pair.change_day.to_string());
    writeVEC.push(pair.change_pct_day.to_string());
    writeVEC.push(pair.change_24_hour.to_string());
    writeVEC.push(pair.change_pct_24_hour.to_string());
    writeVEC.push(pair.supply.to_string());
    writeVEC.push(pair.market_cap.to_string());
    writeVEC.push(pair.open_hour.to_string());
    writeVEC.push(pair.high_hour.to_string());
    writeVEC.push(pair.low_hour.to_string());
    writeVEC.push(pair.open_day.to_string());
    writeVEC.push(pair.high_day.to_string());
    writeVEC.push(pair.low_day.to_string());
    writeVEC.push(pair.open_24_hour.to_string());
    writeVEC.push(pair.high_24_hour.to_string());
    writeVEC.push(pair.low_24_hour.to_string());
    writeVEC
}

#[allow(dead_code)]
fn queue_frames(mut queue: HashMap<String, Vec<Vec<String>>>, 
                frame: &HashMap<String, CryptoFiat>, 
                timestamp: &u64
                ) -> HashMap<String, Vec<Vec<String>>> {

    match File::open("agent_conf.txt") {
        Err(_) => {
            let file = File::create("agent_conf.txt").expect("failed to create conf file in queue_frames");
            file.sync_all().expect("failed to sync changes after creating conf in queue_frames");
            },
        Ok(_) => ()
    };
    //default conf
    let default_conf = Configuration {
        pairs: vec![
                        "BCHandUSD".to_string(),
                        "BNBandUSD".to_string(),
                        "BTCDandUSD".to_string(),
                        "BTCandUSD".to_string(),
                        "BTGandUSD".to_string(),
                        "DASHandUSD".to_string(),
                        "DCRandUSD".to_string(),
                        "DGDandUSD".to_string(),
                        "EOSandUSD".to_string(),
                        "ETCandUSD".to_string(),
                        "ETHandUSD".to_string(),
                        "FCTandUSD".to_string(),
                        "GASandUSD".to_string(),
                        "GBYTEandUSD".to_string(),
                        "GNOandUSD".to_string(),
                        "HSRandUSD".to_string(),
                        "LTCandUSD".to_string(),
                        "MAIDandUSD".to_string(),
                        "MCOandUSD".to_string(),
                        "MLNandUSD".to_string(),
                        "NEOandUSD".to_string(),
                        "PARTandUSD".to_string(),
                        "REPandUSD".to_string(),
                        "VENandUSD".to_string(),
                        "VERIandUSD".to_string(),
                        "WAVESandUSD".to_string(),
                        "XCPandUSD".to_string(),
                        "XMRandUSD".to_string(),
                        "XRPandUSD".to_string(),
                        "XZCandUSD".to_string(),
                        "ZECandUSD".to_string(),
                        "ZENandUSD".to_string()
                    ], 
        window: 60, 
        interval: 60, 
        //this is the path of the output files for the agent not the conf file
        path: "".to_string()
    };

    let conf_json = fs::read_to_string("agent_conf.txt").expect("failed to read conf");

    //this should set to default in any case of malformed conf
    let agent_conf: Configuration = match serde_json::from_str(&conf_json) {
        Ok(conf) =>  conf,
        Err(err) => {println!("used default_conf, error was {}", err); default_conf},
    };

    //here is where we would check the well formed conf for validity
    //            window should be greater than 0 and present
    //        interval should be greater than 30, mulitple of 30 and present
     //       paires should be present, and len greater than 0


    //this inserts a key and a blank timestep vec if there are none
    for pair in frame.keys() {
        let mut timesteps = vec![];
        queue.entry(pair.to_string()).or_insert(timesteps);
    }
    
    //this does the add/remove mutation business
    for pair in queue.clone() {
        let key = pair.0.to_string();

        //remove all entries which are not aligned with the configured interval
        queue.entry(key.clone()).and_modify(|timesteps| {
            timesteps.retain(|step| step[0].parse::<i64>().expect("failed to parse timestamp during retain") % agent_conf.interval == 0);
        });

        //this checks to see if there are more timesteps than requested in the conf
        if queue[&key].len() as i64 >= agent_conf.window {
            let mut difference = 0;

            //this makes sure not to delete too many frames if the current frame is non interval
            if *timestamp as i64 % agent_conf.interval == 0 {
                difference = queue[&key].len() as i64 - agent_conf.window + 1;            
            } else {
                difference = queue[&key].len() as i64 - agent_conf.window;
            }
            //front pop all entries that are over the configured window size
            let range = std::ops::Range{start: 0, end: difference};
            for _each in range {
                queue.entry(key.clone()).and_modify(|timesteps|{
                    timesteps.remove(0);
                });
            }
        }

        //add the new timestep
        let writeVEC = arrange_vec(&frame[&key], &timestamp);
        queue.entry(key).and_modify(|timesteps| {
            //this make sure not to push if the current frame is non interval    
            if *timestamp as i64 % agent_conf.interval == 0 {
                timesteps.push(writeVEC);
            }
            
        });
    }

    queue
}


/* 5th
fn measure(metricVEC: Vec<u64>, master: DB) {
    //for each write do checks if master, table, etc exist
    //that way if the disk is changed it can write a new master
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
    //the agent should push to this file instead of rewriting, so that this function
    //can update a previous row if the agents metric is missing
    //set_labels()
}
*/

enum Notify {
    //provide path of new db and volume info
    ChangedDB(String),
    //provide timestamp of first new write
    FirstWrite(String),
    //provide volume info and remanining storage
    //as well as previous frame size
    LowStorage(String),
    //provide new params set
    ChangedConfig(String),
    //provide field names and values that did not parse properly
    InvalidConfig(String)
}

struct DB {
    path: Option<String>,
    storage_device: Option<String>
}

fn default_string() -> String {
    "MISSING".to_string()
}
fn default_int() -> i64 {
    424242
}
fn default_float() -> f64 {
    4242.42
}

#[derive(Serialize, Deserialize)]
struct CryptoFiat {
    //data["RAW"]["$CRYPTO"]["$FIAT"]
    //this is where we put the json after it is broken down untyped into crypto-fiat pairs
    #[serde(rename="TYPE")]
    #[serde(default="default_string")]
    class: String,
    #[serde(rename="MARKET")]
    #[serde(default="default_string")]
    market:String,
    #[serde(rename="FROMSYMBOL")]
    #[serde(default="default_string")]
    crypto_symbol: String,
    #[serde(rename="TOSYMBOL")]
    #[serde(default="default_string")]
    fiat_symbol:String,
    #[serde(rename="FLAGS")]
    #[serde(default="default_string")]
    flags: String,
    #[serde(rename="PRICE")]
    #[serde(default="default_float")]
    price: f64,
    #[serde(rename="LASTUPDATE")]
    #[serde(default="default_int")]
    last_update: i64,
    #[serde(rename="LASTVOLUME")]
    #[serde(default="default_float")]
    last_volume_crypto: f64,
    #[serde(rename="LASTVOLUMETO")]
    #[serde(default="default_float")]
    last_volume_fiat: f64,
    #[serde(skip_deserializing)]
    //this one comes out as a string sometimes
    LASTTRADEID: i64,
    #[serde(rename="VOLUMEDAY")]
    #[serde(default="default_float")]
    volume_day_crypto: f64,
    #[serde(rename="VOLUMEDAYTO")]
    #[serde(default="default_float")]
    volume_day_fiat: f64,
    #[serde(rename="VOLUME24HOUR")]
    #[serde(default="default_float")]
    volume_24_hour_crypto: f64,
    #[serde(rename="VOLUME24HOURTO")]
    #[serde(default="default_float")]
    volume_24_hour_fiat: f64,
    #[serde(rename="OPENDAY")]
    #[serde(default="default_float")]
    open_day: f64,
    #[serde(rename="HIGHDAY")]
    #[serde(default="default_float")]
    high_day: f64,
    #[serde(rename="LOWDAY")]
    #[serde(default="default_float")]
    low_day: f64,
    #[serde(rename="OPEN24HOUR")]
    #[serde(default="default_float")]
    open_24_hour: f64,
    #[serde(rename="HIGH24HOUR")]
    #[serde(default="default_float")]
    high_24_hour: f64,
    #[serde(rename="LOW24HOUR")]
    #[serde(default="default_float")]
    low_24_hour: f64,
    #[serde(rename="LASTMARKET")]
    #[serde(default="default_string")]
    last_market: String,
    //this and the nearly all the following
    //have no data in a or all currencies other than USD
    #[serde(rename="VOLUMEHOUR")]
    #[serde(default="default_float")]
    volume_hour_crypto: f64,
    #[serde(rename="VOLUMEHOURTO")]
    #[serde(default="default_float")]
    volume_hour_fiat: f64,
    #[serde(rename="OPENHOUR")]
    #[serde(default="default_float")]
    open_hour: f64,
    #[serde(rename="HIGHHOUR")]
    #[serde(default="default_float")]
    high_hour: f64,
    #[serde(rename="LOWHOUR")]
    #[serde(default="default_float")]
    low_hour: f64,
    #[serde(rename="CHANGE24HOUR")]
    #[serde(default="default_float")]
    change_24_hour: f64,
    #[serde(rename="CHANGEPCT24HOUR")]
    #[serde(default="default_float")]
    change_pct_24_hour: f64,
    #[serde(rename="CHANGEDAY")]
    #[serde(default="default_float")]
    change_day: f64,
    #[serde(rename="CHANGEPCTDAY")]
    #[serde(default="default_float")]
    change_pct_day: f64,
    #[serde(rename="SUPPLY")]
    #[serde(default="default_float")]
    supply: f64,
    #[serde(rename="MKTCAP")]
    #[serde(default="default_float")]
    market_cap: f64,
    #[serde(rename="TOTALVOLUME24H")]
    #[serde(default="default_float")]
    total_volume_24_hour_crypto: f64,
    #[serde(rename="TOTALVOLUME24HTO")]
    #[serde(default="default_float")]
    total_volume_24_hour_fiat: f64,
    #[serde(default="default_string")]
    IMAGEURL: String

}

#[derive(Serialize, Deserialize)]
struct Configuration {
    pairs: Vec<String>,
    window: i64,
    interval: i64,
    path: String
}

fn main() {
//perf: keys can be str, 
//vecs and hashmaps all have length known, and can be defined

    let mut master = DB{
        path: Some("fake_api.db".to_string()),
        storage_device: None
    };

    let mut metrics = DB{
        path: None,
        storage_device: None
    };

    let mut queue: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    let mut count = 0;

    loop{
        let mut metricVEC: Vec<u64> = vec![];
        let start = Instant::now();
        //set_disk(&master, &metrics);
        let duration = start.elapsed().as_secs();
        metricVEC.push(duration);

        let start = Instant::now();
        let (frame, timestamp) = get_data();
        let duration = start.elapsed().as_secs();
        metricVEC.push(duration);

        let start = Instant::now();
        //queue = queue_frames(queue, &frame, &timestamp);
        let duration = start.elapsed().as_secs();
        metricVEC.push(duration);

        let start = Instant::now();
        //inform_agent(&queue);
        let duration = start.elapsed().as_secs();
        metricVEC.push(duration);

        let start = Instant::now();
        //this takes 9s for create and write, 3s for write
        write_data(&frame, &timestamp, &master);
        let duration = start.elapsed().as_secs();
        metricVEC.push(duration);

        let start = Instant::now();
        //get_agent_metrics();
        let duration = start.elapsed().as_secs();
        metricVEC.push(duration);

        //measure(metricVEC, metrics);
        println!("{} frames captured", count +1);
        println!("this function took {}s", metricVEC[4]);
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MiniCryptoFiat {
        timestamp: i64,
        last_update: i64,
        price: f64,
        last_market: String,
        last_volume_crypto: f64,
        volume_hour_crypto: f64,
        volume_day_crypto: f64,
        volume_24_hour_crypto: f64,
        total_volume_24_hour_crypto: f64,
        last_volume_fiat: f64,
        volume_hour_fiat: f64,
        volume_day_fiat: f64,
        volume_24_hour_fiat: f64,
        total_volume_24_hour_fiat: f64,
        change_day: f64,
        change_pct_day: f64,
        change_24_hour: f64,
        change_pct_24_hour: f64,
        supply: f64,
        market_cap: f64,
        open_hour: f64,
        high_hour: f64,
        low_hour: f64,
        open_day: f64,
        high_day: f64,
        low_day: f64,
        open_24_hour: f64,
        high_24_hour: f64,
        low_24_hour: f64
    }


    //utils
    fn mini_struct_to_full_struct(mini_frame: HashMap<String, tests::MiniCryptoFiat>) -> HashMap<String, CryptoFiat> {
        let mut frame = HashMap::new();
        for key in mini_frame.keys() {
            let pair = CryptoFiat {
                class: "MISSING".to_string(),
                market: "MISSING".to_string(),
                crypto_symbol: "MISSING".to_string(),
                fiat_symbol:"MISSING".to_string(),
                flags: "MISSING".to_string(),
                price: mini_frame[key].price,
                last_update: mini_frame[key].last_update,
                last_volume_crypto: mini_frame[key].last_volume_crypto,
                last_volume_fiat: mini_frame[key].last_volume_fiat,
                LASTTRADEID: 424242,
                volume_day_crypto: mini_frame[key].volume_day_crypto,
                volume_day_fiat: mini_frame[key].volume_day_fiat,
                volume_24_hour_crypto: mini_frame[key].volume_24_hour_crypto,
                volume_24_hour_fiat: mini_frame[key].volume_24_hour_fiat,
                open_day: mini_frame[key].open_day,
                high_day: mini_frame[key].high_day,
                low_day: mini_frame[key].low_day,
                open_24_hour: mini_frame[key].open_24_hour,
                high_24_hour: mini_frame[key].high_24_hour,
                low_24_hour: mini_frame[key].low_24_hour,
                last_market: mini_frame[key].last_market.to_owned(),
                volume_hour_crypto: mini_frame[key].volume_hour_crypto,
                volume_hour_fiat: mini_frame[key].volume_hour_fiat,
                open_hour: mini_frame[key].open_hour,
                high_hour: mini_frame[key].high_hour,
                low_hour: mini_frame[key].low_hour,
                change_24_hour: mini_frame[key].change_24_hour,
                change_pct_24_hour: mini_frame[key].change_pct_24_hour,
                change_day: mini_frame[key].change_day,
                change_pct_day: mini_frame[key].change_pct_day,
                supply: mini_frame[key].supply,
                market_cap: mini_frame[key].market_cap,
                total_volume_24_hour_crypto: mini_frame[key].total_volume_24_hour_crypto,
                total_volume_24_hour_fiat: mini_frame[key].total_volume_24_hour_fiat,
                IMAGEURL: "MISSING".to_string()
            };
            frame.insert(key.to_string(), pair);
        }
        frame
    }

    fn get_one_fake_frame()-> (HashMap<String, CryptoFiat>, u64) {
        let json = fs::read_to_string("fake_frame.txt")
        .expect("Something went wrong reading the file");

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let mut frame = HashMap::new();
        let data: Value = serde_json::from_str(&json).expect("unable to convert response text to untyped object");
        let object = data.as_object().expect("unable to convert outer values to map");
        let object = object["RAW"].as_object().expect("unable to convert inner values to map");
        for crypto in object.keys() {
            for fiat in object[crypto].as_object().unwrap().keys() {
                let pair_block: CryptoFiat = serde_json::from_value(object[crypto][fiat].clone()).expect("failed to convert untyped map to typed struct");
                frame.entry(format!("{}and{}", crypto, fiat)).or_insert(pair_block);
            }
        }

        (frame, timestamp)


    }

    #[test]
    fn get_one_fake_frame_returns_valid_frame() {
        let (frame, timestamp) = get_one_fake_frame();
        if frame["BTCandUSD"].crypto_symbol != "BTC" ||
           frame["BTCandUSD"].fiat_symbol != "USD" {
               panic!("get_one_fake_frame returned an invalid frame");
           }
    }

    //when using this in a loop make sure to remove test_timestamp.txt before the loop
    fn get_many_fake_frames() -> (HashMap<String, tests::MiniCryptoFiat>, u64) {
        //this should access the timestamp file in a thread_safe way, to be able to run the tests in parallel
        let index: Box<Fn() -> String> = match File::open("test_timestamp.txt") {
            //this was literally hitler to write, but its all mine from scratch
            Err(e) => Box::new(|| {
                let mut file = File::create("test_timestamp.txt").expect("failed to create test_timestamp.txt");
                file.write(&"1548299340".to_string().into_bytes()).expect("failed to write index to test_timestamp.txt");
                file.sync_all().expect("failed to sync file changes after writing test_timestamp.txt");
                "1548299340\u{0}\u{0}".to_string()
                }),

            Ok(file) => Box::new(|| {
                //it will be a good day for my program if this 12 byte buffer is exceeded by a unix timestamp
                let mut index: [u8; 12] = [0; 12];
                let mut file = File::open("test_timestamp.txt").unwrap();
                file.read(&mut index).expect("failed to read test_timestamp.txt");
                let output = str::from_utf8(&index).expect("failed to convert test_timestamp.txt bytes to str");
                output.to_string()
                })
        };

        let index = &*index;
        //this string is "1548299370\u{0}\u{0}"
        //when i try to chop off the last two bytes, the valid frame test fails
        //even though it should be using a fresh file each time
        let index: String = index().trim().to_string();
        let index: String = index[..index.len()-2].to_string();
        let mut index: i64 = index.clone().parse().expect("failed to convert index to i64");

        let table_vec = vec![
             "BCHandUSD".to_string(),
            "BNBandUSD".to_string(),
            "BTCDandUSD".to_string(),
            "BTCandUSD".to_string(),
            "BTGandUSD".to_string(),
            "DASHandUSD".to_string(),
            "DCRandUSD".to_string(),
            "DGDandUSD".to_string(),
            "EOSandUSD".to_string(),
            "ETCandUSD".to_string(),
            "ETHandUSD".to_string(),
            "FCTandUSD".to_string(),
            "GASandUSD".to_string(),
            "GBYTEandUSD".to_string(),
            "GNOandUSD".to_string(),
            "HSRandUSD".to_string(),
            "LTCandUSD".to_string(),
            "MAIDandUSD".to_string(),
            "MCOandUSD".to_string(),
            "MLNandUSD".to_string(),
            "NEOandUSD".to_string(),
            "PARTandUSD".to_string(),
            "REPandUSD".to_string(),
            "VENandUSD".to_string(),
            "VERIandUSD".to_string(),
            "WAVESandUSD".to_string(),
            "XCPandUSD".to_string(),
            "XMRandUSD".to_string(),
            "XRPandUSD".to_string(),
            "XZCandUSD".to_string(),
            "ZECandUSD".to_string(),
            "ZENandUSD".to_string()
        ];

        let storage = Connection::open("fake_api.db").expect("failed to open fake_api.db");
        let mut frame = HashMap::new();
        let mut timestamp: u64 = 0;

        for table in table_vec {
            let query = format!("SELECT * FROM {} WHERE timestamp > ?", &table);
            if index > 1548314400 {
                index = 1548299310;
            }
            let query_index = &[index];

            let mut stmt = storage.prepare(&query).expect("failed to prepare query");

            let mut pair_iter = stmt.query_map(query_index, |row| MiniCryptoFiat {
                timestamp: row.get(0),
                last_update: row.get(1),
                price: row.get(2),
                last_market: row.get(3),
                last_volume_crypto: row.get(4),
                volume_hour_crypto: row.get(5),
                volume_day_crypto: row.get(6),
                volume_24_hour_crypto: row.get(7),
                total_volume_24_hour_crypto: row.get(8),
                last_volume_fiat: row.get(9),
                volume_hour_fiat: row.get(10),
                volume_day_fiat: row.get(11),
                volume_24_hour_fiat: row.get(12),
                total_volume_24_hour_fiat: row.get(13),
                change_day: row.get(14),
                change_pct_day: row.get(15),
                change_24_hour: row.get(16),
                change_pct_24_hour: row.get(17),
                supply: row.get(18),
                market_cap: row.get(19),
                open_hour: row.get(20),
                high_hour: row.get(21),
                low_hour: row.get(22),
                open_day: row.get(23),
                high_day: row.get(24),
                low_day: row.get(25),
                open_24_hour: row.get(26),
                high_24_hour: row.get(27),
                low_24_hour: row.get(28)
            }).expect("failed to run query");

            let single = pair_iter.nth(0).expect("failed to index pair_iter").expect("second result for indexing pair iter has failed");
            timestamp = single.timestamp as u64;

            frame.insert(table, single);
        }
        let mut file = OpenOptions::new().create(false).write(true).append(false).open("test_timestamp.txt").expect("failed to open timestamp file for increment");
        //this is adding 60 to the timestamp all the sudden
        let writestamp = index + 30;
        let writestamp = writestamp.to_string();
        file.write(&writestamp.into_bytes()).expect("failed to write to file for increment");
        file.sync_all().expect("failed to sync file changes after writing test_timestamp.txt");
        return (frame, timestamp);

    }

    fn get_many_fake_frames_returns_valid_data() -> Result<(), ()>{
        clean_up_confs();

        let (frame, timestamp) = get_many_fake_frames();
        if frame["BTCandUSD"].timestamp != 1548299370 {
            match File::open("test_timestamp.txt") {
                Err(_) => (),
                Ok(_) => clean_up_confs()
            };
            return Err(());
        } else if frame["MAIDandUSD"].price != 0.1203174445 {
            match File::open("test_timestamp.txt") {
                Err(_) => (),
                Ok(_) => clean_up_confs()
            };
            return Err(());
        }
        Ok(())
    }

    fn get_many_fake_frames_resets_after_db_exhausted() -> Result<(), ()> {
        clean_up_confs();

        //this may need to be 505 because its upper bound is not inclusive
        for iteration in 0..504 {
            let (frame, timestamp) = get_many_fake_frames();
        }
        let (frame, timestamp) = get_many_fake_frames();
        //this should equal the second timestamp, because the get_many will never return the first
        //as the SELECT is > timestamp (which defaults to the first)
    
        if timestamp != 1548299370 {
            clean_up_confs();
            return Err(());
        }

        clean_up_confs();
        Ok(())

    }

    #[test]
    #[serial]
    fn get_many_fake_frames_group_with_2() {
        //mutation/deletion of the shared file in get_many_fake_frames prevents any of these tests from being run in parallel
        get_many_fake_frames_returns_valid_data().expect("get_many_fake returned invalid data");
        get_many_fake_frames_resets_after_db_exhausted().expect("get_many_fake did not reset after db was exhausted");
    }

    fn clean_up_confs() {
        //these should be run in a thread safe way along with get_many_fake_frames
        match File::open("agent_conf.txt") {
            Err(_) => (),
            Ok(_) => fs::remove_file("agent_conf.txt").expect("failed to remove file after open succeeded")
        };

        match File::open("test_timestamp.txt") {
            Err(_) => (),
            Ok(_) => fs::remove_file("test_timestamp.txt").expect("failed to remove file after open succeeded")
        };

    }

    //unit tests
    /*
    #[test]
    fn set_disk_group(){
        panic!("not implemented");
    }
    */

    /*
    #[test]
    fn notify_group(){
        panic!("not implemented");
    }
    */


    fn get_data_sleeps_till_30() -> Result<(), ()>{
        let (frame, timestamp) = get_data();
        if timestamp % 30 == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    fn get_data_creates_valid_frame() -> Result<(), ()> {
        let (frame, timestamp) = get_data();
        if frame["BTCandUSD"].crypto_symbol == "BTC" &&
           frame["BTCandUSD"].fiat_symbol == "USD"
        {
            Ok(())
        }
        else {
            Err(())
        }
    }

    fn get_data_frame_has_all_crypto() -> Result<(), ()> {
        let (frame, timestamp) = get_data();
        if frame.len() == 32 {
            Ok(())
        } else {
            Err(())
        }
    }

    fn get_data_survives_bad_connection_and_api() -> Result<(),()> {
        Err(())
    }

    #[test]
    #[ignore]
    fn get_data_group_with_3(){
        get_data_sleeps_till_30().expect("the request did not happen on a round 30 seconds");
        get_data_creates_valid_frame().expect("get_data returned an invalid frame");
        get_data_frame_has_all_crypto().expect("frame does not contain enough crypto-USD pairs");
    }


    fn arrange_vec_has_29_items() -> Result<(), ()> {
        let (frame, timestamp) = get_one_fake_frame();
        let pair = &frame["BTCandUSD"];
        let writeVEC = arrange_vec(&pair, &timestamp);
        if writeVEC.len() == 29 {
            Ok(())
        } else {
            Err(())
        }
    }

    fn arrange_vec_returns_valid_writevec() -> Result<(), ()> {
        let (frame, timestamp) = get_one_fake_frame();
        let pair = &frame["BTCandUSD"];
        let writeVEC = arrange_vec(&pair, &timestamp);
        if writeVEC[0].len() == 10 &&
            //market
           writeVEC[3] == "Coinbase" &&
           //volume24h
           writeVEC[7] == "37533.51939446323" &&
           //volume_day_fiat
           writeVEC[11] == "140675918.74609685" &&
           //change_pct_day
           writeVEC[15] == "2.3316949881989917" &&
           //market_cap
           writeVEC[19] == "65291977762.5" &&
           //low_24h
           writeVEC[28] == "3643.41"
        {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test]
    fn arrange_vec_test_group_with_2(){
        arrange_vec_has_29_items().expect("arrange_vec returns an incorrect number of items");
        arrange_vec_returns_valid_writevec().expect("arrange_vec returns an invalid writeVEC");
    }


    fn write_data_creates_db_when_none() -> Result <(), ()> {
        let master = DB {
            path: Some("test.db".to_string()),
            storage_device: None
        };

        //get paths
        match File::open("test.db") {
            Err(_) => (),
            Ok(_) => fs::remove_file("test.db").expect("failed to remove file after open succeeded")
        };

        let (frame, timestamp) = get_one_fake_frame();
        write_data(&frame, &timestamp, &master);

        match File::open("test.db") {
            Err(_) => return Err(()),
            Ok(_) => fs::remove_file("test.db").expect("failed to remove file after open succeeded")
        };

        return Ok(());
    }

    fn write_data_adds_valid_tables_to_db() -> Result <(), ()> {
        let master = DB {
            path: Some("test.db".to_string()),
            storage_device: None
        };

        let (frame, timestamp) = get_one_fake_frame();
        write_data(&frame, &timestamp, &master);
        //BTC,ETH,BCH,LTC,EOS,BNB,XMR,DASH,VEN,NEO,ETC,ZEC,WAVES,BTG,DCR,REP,GNO,MCO,FCT,HSR,DGD,XZC,VERI,PART,GAS,ZEN,GBYTE,BTCD,MLN,XCP,XRP,MAID
        let storage = Connection::open("test.db").expect("failed to open the database");
        let mut table_vec: HashSet<String> = [].iter().cloned().collect();
        let expect_vec: HashSet<String> = [
                                            "BCHandUSD".to_string(),
                                            "BNBandUSD".to_string(),
                                            "BTCDandUSD".to_string(),
                                            "BTCandUSD".to_string(),
                                            "BTGandUSD".to_string(),
                                            "DASHandUSD".to_string(),
                                            "DCRandUSD".to_string(),
                                            "DGDandUSD".to_string(),
                                            "EOSandUSD".to_string(),
                                            "ETCandUSD".to_string(),
                                            "ETHandUSD".to_string(),
                                            "FCTandUSD".to_string(),
                                            "GASandUSD".to_string(),
                                            "GBYTEandUSD".to_string(),
                                            "GNOandUSD".to_string(),
                                            "HSRandUSD".to_string(),
                                            "LTCandUSD".to_string(),
                                            "MAIDandUSD".to_string(),
                                            "MCOandUSD".to_string(),
                                            "MLNandUSD".to_string(),
                                            "NEOandUSD".to_string(),
                                            "PARTandUSD".to_string(),
                                            "REPandUSD".to_string(),
                                            "VENandUSD".to_string(),
                                            "VERIandUSD".to_string(),
                                            "WAVESandUSD".to_string(),
                                            "XCPandUSD".to_string(),
                                            "XMRandUSD".to_string(),
                                            "XRPandUSD".to_string(),
                                            "XZCandUSD".to_string(),
                                            "ZECandUSD".to_string(),
                                            "ZENandUSD".to_string()
                                            ].iter().cloned().collect();
        //this statement works when run alone in sqlite3 prompt
        let mut statement = storage.prepare("SELECT name FROM sqlite_master WHERE type='table';").expect("failed to prepare statement");
        let table_iter = statement.query_map(NO_PARAMS, |row| row.get(0)).expect("failed to map rows");

        for row in table_iter {
            table_vec.insert(row.expect("row error"));
        }

        if expect_vec == table_vec {
            fs::remove_file("test.db").expect("failed to remove file after match");
            Ok(())
        } else {
            fs::remove_file("test.db").expect("failed to remove file after match");
            Err(())
        }
        
    }

    fn write_data_adds_valid_row_to_one_table() -> Result <(), ()> {
        let master = DB {
            path: Some("test.db".to_string()),
            storage_device: None
        };

        let (frame, timestamp) = get_one_fake_frame();

        let pair = &frame["BTCandUSD"];

        write_data(&frame, &timestamp, &master);
        //want to test all columns in all tables, but there is a inference issue when query string is formatted
        //and there is a no such var as row issue when closure adds each column to result_vec
        let storage = Connection::open("test.db").expect("failed to open the database");
        let mut statement = storage.prepare("SELECT * FROM BTCandUSD;").expect("failed to prepare statement");
        let row_iter = statement.query_map(NO_PARAMS, |row| row.get(28)).expect("failed to map rows");

        let mut result = 0.0;
        for row in row_iter {
            result = row.expect("unable to unwrap row from row_iter");
        }

        if result == pair.low_24_hour {            
            fs::remove_file("test.db").expect("failed to remove file after match");
            Ok(())
        } else {
            fs::remove_file("test.db").expect("failed to remove file after match");
            Err(())
        }

    }

    fn write_data_adds_valid_columns() -> Result <(), ()> {
        let master = DB {
            path: Some("test.db".to_string()),
            storage_device: None
        };

        let (frame, timestamp) = get_one_fake_frame();

        let expect_set: HashSet<&str> = [
            "timestamp",
            "last_update",
            "price",
            "last_market",
            "last_volume_crypto",
            "volume_hour_crypto",
            "volume_day_crypto",
            "volume_24_hour_crypto",
            "total_volume_24_hour_crypto",
            "last_volume_fiat",
            "volume_hour_fiat",
            "volume_day_fiat",
            "volume_24_hour_fiat",
            "total_volume_24_hour_fiat",
            "change_day",
            "change_pct_day",
            "change_24_hour",
            "change_pct_24_hour",
            "supply",
            "market_cap",
            "open_hour",
            "high_hour",
            "low_hour",
            "open_day",
            "high_day",
            "low_day",
            "open_24_hour",
            "high_24_hour",
            "low_24_hour",
        ].iter().cloned().collect();

        write_data(&frame, &timestamp, &master);
        //want to test all columns in all tables, but there is a inference issue when query string is formatted
        //and there is a no such var as row issue when closure adds each column to result_vec
        let storage = Connection::open("test.db").expect("failed to open the database");
        let mut statement = storage.prepare("SELECT * FROM BTCandUSD;").expect("failed to prepare statement");
        let column_vec = statement.column_names();

        let column_set: HashSet<&str> = column_vec.iter().cloned().collect();

        if expect_set == column_set {
            Ok(())
        } else {
            Err(())
        }

    }

    #[test]
    fn write_data_group_with_4(){
        write_data_creates_db_when_none().expect("write_data failed to create master");
        write_data_adds_valid_tables_to_db().expect("write_data failed to add tables to DB");
        write_data_adds_valid_columns().expect("write_data failed to add valid columns");
        write_data_adds_valid_row_to_one_table().expect("write_data failed to add a valid row to the  first table");
    }

    fn queue_frames_returns_all_keys() -> Result <(), ()> {
        let (mini_frame, timestamp) = get_many_fake_frames();
        let frame = mini_struct_to_full_struct(mini_frame);
        let mut queue = HashMap::new();
        queue = queue_frames(queue, &frame, &timestamp);

        let mut table_vec: HashSet<String> = [].iter().cloned().collect();
        let expect_vec: HashSet<String> = [
                                            "BCHandUSD".to_string(),
                                            "BNBandUSD".to_string(),
                                            "BTCDandUSD".to_string(),
                                            "BTCandUSD".to_string(),
                                            "BTGandUSD".to_string(),
                                            "DASHandUSD".to_string(),
                                            "DCRandUSD".to_string(),
                                            "DGDandUSD".to_string(),
                                            "EOSandUSD".to_string(),
                                            "ETCandUSD".to_string(),
                                            "ETHandUSD".to_string(),
                                            "FCTandUSD".to_string(),
                                            "GASandUSD".to_string(),
                                            "GBYTEandUSD".to_string(),
                                            "GNOandUSD".to_string(),
                                            "HSRandUSD".to_string(),
                                            "LTCandUSD".to_string(),
                                            "MAIDandUSD".to_string(),
                                            "MCOandUSD".to_string(),
                                            "MLNandUSD".to_string(),
                                            "NEOandUSD".to_string(),
                                            "PARTandUSD".to_string(),
                                            "REPandUSD".to_string(),
                                            "VENandUSD".to_string(),
                                            "VERIandUSD".to_string(),
                                            "WAVESandUSD".to_string(),
                                            "XCPandUSD".to_string(),
                                            "XMRandUSD".to_string(),
                                            "XRPandUSD".to_string(),
                                            "XZCandUSD".to_string(),
                                            "ZECandUSD".to_string(),
                                            "ZENandUSD".to_string()
                                            ].iter().cloned().collect();


        for key in queue.keys() {
            table_vec.insert(key.to_string());
        }
        if table_vec == expect_vec {
            return Ok(());
        } else {
            return Err(())
        }
    }

    fn queue_frames_returns_valid_data() -> Result <(), ()> {
        clean_up_confs();
        
        let mut queue = HashMap::new();
        //this fails if done once due to the 60s interval default
        for each in 0..2 {
            let (mini_frame, timestamp) = get_many_fake_frames();
            let frame = mini_struct_to_full_struct(mini_frame);
            queue = queue_frames(queue, &frame, &timestamp);
        }

        clean_up_confs();

        //this is the only indexing operation
        let thisBOX = &queue["BTCandUSD"][0][0];
        let thisBOX: u64 = match thisBOX.parse::<u64>() {
            Err(_) => return Err(()),
            Ok(_) => return Ok(())
        };
    }

    fn queue_frames_returns_more_than_one_vec() -> Result <(), ()> {
        clean_up_confs();

        let mut queue = HashMap::new();

        //this should be four because of the default interval of 60s (two 30s frame are skipped)
        for _vec in 0..4 {
            let (mini_frame, timestamp) = get_many_fake_frames();
            let frame = mini_struct_to_full_struct(mini_frame);
            queue = queue_frames(queue, &frame, &timestamp);
        }

        clean_up_confs();

        if queue["BTCandUSD"].len() < 2 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn queue_frames_notifies_when_specified_window_is_complete() -> Result <(),()> {
        Err(())
    }


    #[test]
    #[serial(mut_timestamp)]
    fn queue_frames_group_with_3(){
        queue_frames_returns_all_keys().expect("queue_frames did not return the expected keys");
        queue_frames_returns_valid_data().expect("queue_frames did not return a parsable timestamp at [0][0] position");
        queue_frames_returns_more_than_one_vec().expect("queue_frames did not return multiple timesteps");
    }

    fn queue_frames_creates_conf_when_none() -> Result<(), ()> {
        //use previous conf if current is invalid,
        //should set a previous file each time a valid conf is accepted
        // valid conf:
        /*
        {
            "pairs": Vec<String<CRYPTOandFIAT>>,
            "window": i64<0..Any>,
            "interval": i64<30..Any*30>,
            "path": String<Path>
        }

        */
        clean_up_confs();

        let (mini_frame, timestamp) = get_many_fake_frames();
        let frame = mini_struct_to_full_struct(mini_frame);
        let mut _queue = HashMap::new();
        _queue = queue_frames(_queue, &frame, &timestamp);

        match File::open("agent_conf.txt") {
            Err(_) => return Err(()),
            Ok(_) => ()
        };

        clean_up_confs();

        //no comments allowed in json so we are going to skip the info header
        return Ok(())
    }

    fn queue_frames_survives_blank_conf_and_caps_at_defaults() -> Result<(), ()> {
        clean_up_confs();

        let mut queue = HashMap::new();

        //the upper bound is odd to make sure that queue_frames keeps any current non interval frame from being pushed
        for _each in 0..121 {
            let (mini_frame, timestamp) = get_many_fake_frames();
            let frame = mini_struct_to_full_struct(mini_frame);
            queue = queue_frames(queue, &frame, &timestamp);
        }

        if queue["BTCandUSD"].len() != 60 {
            println!("the default queue length was not 10");
            return Err(())
        }
        
        //this indexes the last values to make sure queue_frames keeps any current non interval frame from being pushed
        let timestamp0: i64 = queue["BTCandUSD"][8][0].parse().expect("failed to parse timestamp0");
        let timestamp1: i64 = queue["BTCandUSD"][9][0].parse().expect("failed to parse timestamp1");
    
        clean_up_confs();

        if timestamp1 - timestamp0 != 60 {
            println!("the default queue interval was not 60 seconds apart");
            return Err(())
        }

        Ok(())
    }

    fn queue_frames_survives_invalid_conf() -> Result<(), ()> {
        /*
            window should be greater than 0 and present
            interval should be greater than 30, mulitple of 30 and present
            path should be present and create if not valid
            paires should be present, and len greater than 0
            by deserializing all typing errors should be tested and results handled to survive properly

            this test should create a file with no data,
            create a file with invalid types,
            create a file with 0 len types,
            clean up file and any directories created

            should default to 60/60

            "{\n    \"pairs\": [\n                  \"HAMandEGG\",\n                  \"BOBandMARTHA\",\n],\n    \"window\": 60,\n    \"interval\": 60,\n    \"path\": \"/agent_output/\"\n}\n"

        */
        clean_up_confs();
        let mut file = File::create("agent_conf.txt").expect("failed to create agent_conf.txt");
        file.write(&"{\n    \"pairs\": [\n                  \"HAMandEGG\",\n                  \"BOBandMARTHA\"\n],\n    \"window\": 60,\n    \"interval\": 60,\n    \"path\": \"/agent_output/\"\n}\n".to_string().into_bytes()).expect("failed to write invalid pairs to agent_conf.txt");
        file.sync_all().expect("failed to sync file changes after writing agent_conf.txt");

        let expect_vec: HashSet<String> = [
                                            "BCHandUSD".to_string(),
                                            "BNBandUSD".to_string(),
                                            "BTCDandUSD".to_string(),
                                            "BTCandUSD".to_string(),
                                            "BTGandUSD".to_string(),
                                            "DASHandUSD".to_string(),
                                            "DCRandUSD".to_string(),
                                            "DGDandUSD".to_string(),
                                            "EOSandUSD".to_string(),
                                            "ETCandUSD".to_string(),
                                            "ETHandUSD".to_string(),
                                            "FCTandUSD".to_string(),
                                            "GASandUSD".to_string(),
                                            "GBYTEandUSD".to_string(),
                                            "GNOandUSD".to_string(),
                                            "HSRandUSD".to_string(),
                                            "LTCandUSD".to_string(),
                                            "MAIDandUSD".to_string(),
                                            "MCOandUSD".to_string(),
                                            "MLNandUSD".to_string(),
                                            "NEOandUSD".to_string(),
                                            "PARTandUSD".to_string(),
                                            "REPandUSD".to_string(),
                                            "VENandUSD".to_string(),
                                            "VERIandUSD".to_string(),
                                            "WAVESandUSD".to_string(),
                                            "XCPandUSD".to_string(),
                                            "XMRandUSD".to_string(),
                                            "XRPandUSD".to_string(),
                                            "XZCandUSD".to_string(),
                                            "ZECandUSD".to_string(),
                                            "ZENandUSD".to_string()
                                            ].iter().cloned().collect();

        let mut queue = HashMap::new();
        let (mini_frame, timestamp) = get_many_fake_frames();
        let frame = mini_struct_to_full_struct(mini_frame);
        queue = queue_frames(queue, &frame, &timestamp);

        let returned_vec: HashSet<String> = queue.keys().map(|key| key.to_owned()).collect();
        if returned_vec == expect_vec {
            //this worked with no changes to queue_frames
            clean_up_confs();
            return Ok(());
        } else {
            println!("queue frames did not revert to default config when given invalid pairs\n it returned {:?}", returned_vec);
            clean_up_confs();
            return Err(());
        }

    }

    fn queue_frames_removes_many_when_interval_is_changed() -> Result<(),()> {
        Err(())
    }

    fn queue_frames_notifies_invalid_conf_params() -> Result<(), ()> {
        Err(())
    }

    fn queue_frames_returns_pairs_specified_in_conf() -> Result <(), () > {
        /*
        if pairs.len() > performance_number of pairs then pairs is invalid
        */
        clean_up_confs();
        let mut file = File::create("agent_conf.txt").expect("failed to create agent_conf.txt");
        file.write(&"{\n    \"pairs\": [\n                  \"LTCandUSD\",\n                  \"MAIDandUSD\"\n],\n    \"window\": 60,\n    \"interval\": 60,\n    \"path\": \"/agent_output/\"\n}\n".to_string().into_bytes()).expect("failed to write invalid pairs to agent_conf.txt");
        file.sync_all().expect("failed to sync file changes after writing agent_conf.txt");

        let expect_vec: HashSet<String> = [
                                            "LTCandUSD".to_string(),
                                            "MAIDandUSD".to_string(),
                                            ].iter().cloned().collect();

        let mut queue = HashMap::new();
        let (mini_frame, timestamp) = get_many_fake_frames();
        let frame = mini_struct_to_full_struct(mini_frame);
        queue = queue_frames(queue, &frame, &timestamp);

        let returned_vec: HashSet<String> = queue.keys().map(|key| key.to_owned()).collect();
        if returned_vec == expect_vec {
            //this worked with no changes to queue_frames
            clean_up_confs();
            return Ok(());
        } else {
            println!("queue frames did not return specified config when given valid pairs\n it returned {:?}", returned_vec);
            clean_up_confs();
            return Err(());
        }
    }

    #[test]
    #[serial(mut_timestamp)]
    fn queue_frames_conf_group_with_2(){
        queue_frames_creates_conf_when_none().expect("queue_frames failed to create a blank conf file");
        queue_frames_survives_blank_conf_and_caps_at_defaults().expect("queue_frames did not use defaults when conf was blank");
        queue_frames_survives_invalid_conf().expect("queue_frames did not revert to default when given an invalid config");
        queue_frames_returns_pairs_specified_in_conf().expect("queue_frames failed to return pairs given in a valid conf");
        //what is this test, does it take user input???
        //queue_frames_notifies_invalid_conf_params().expect("queue_frames failed to notify");
    }

    /*
    #[test]
    fn set_labels_group(){
        panic!("not implemented");
    }
    */

    /*
    #[test]
    fn measure_group(){
        panic!("not implemented");
    }
    */

    /*
    #[test]
    fn inform_agent_group(){
        panic!("not implemented");
    }
    */

    /*
    #[test]
    fn get_agent_metrics_group(){
        panic!("not implemented");
    }
    */

    /*
    #[test]
    fn get_agent_config_group(){
        panic!("not implemented");
    }
    */
}