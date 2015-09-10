use std::sync::mpsc::{Receiver, Sender};
use std::path::PathBuf;
use std::fs::Metadata;
use std::io::Result;
use std::thread::spawn;
use std::sync::mpsc::channel;

fn worker_loop(files: Receiver<PathBuf>,
               results: Sender<(PathBuf, Result<Metadata>)>) {
    for path_buf in files {    	
        let md = std::fs::metadata(&path_buf);
        results.send((path_buf, md)).unwrap();
    }
}

fn main() {
    let paths = vec!["/tmp", "/usr/local/etc", "/dev/null"];
    let worker_thr;
    //WSender, MReceiver
    let (worker_tx, main_rx) = channel();
    {
        //MSender, WReceiver
        let (main_tx, worker_rx) = channel();
        worker_thr = spawn(|| {
            worker_loop(worker_rx, worker_tx);
        });
        //Send the paths
        for path in paths {
            main_tx.send(PathBuf::from(path)).unwrap();
        }
    }

    //Iterate over received Result
    for (path, result) in main_rx {
        match result {
            Ok(metadata) =>
                println!("Size of {:?}: {}", &path, metadata.len()),
            Err(err) =>
                println!("Error for {:?}: {}", &path, err)
        }
    }
    worker_thr.join().unwrap();
}
