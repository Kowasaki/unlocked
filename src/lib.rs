use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client, Error, Region};

use futures::StreamExt;
use futures::stream::FuturesUnordered;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use std::collections::HashMap;
use std::path::Path;
use std::process;

/// This is a rust implementation of a priority queue data structure.
#[pyclass]
struct PriorityQueue {
    arr: Vec<i32>,
}

#[pymethods]
impl PriorityQueue {
    #[new]
    fn new() -> Self {
        Self{arr:Vec::new()}
    }

    #[getter]
    fn get_arr(&self) -> PyResult<Vec<i32>> {
        Ok(self.arr.clone())
    }
    
    pub fn insert(&mut self, val: i32) -> PyResult<()>{
        if self.arr.len() == 0{
            self.arr.push(val);
            Ok(())
        } else if val >= self.arr[0]{
            self.arr.insert(0, val);
            Ok(())
        } else if val <= self.arr[self.arr.len()-1]{
            self.arr.push(val);
            Ok(())
        } else {
            let ptr = self.arr.len() / 2;
            self.heapify(ptr, val);
            Ok(())
        } 
    }

    fn heapify(&mut self, ptr: usize, val: i32){
        
        if self.arr[ptr] <= val {
            if self.arr[ptr-1] >= val {
                self.arr.insert(ptr, val);
            } else {
                let ptr = (self.arr.len() - ptr) / 2;
                self.heapify(ptr, val);
            }
        } else {
            let ptr = ptr + ((self.arr.len() - ptr) / 2);
            self.heapify(ptr, val);
        }
    }

    pub fn get_max(&mut self) -> PyResult<i32>{
        match self.arr.len() {
            0 => Err(PyValueError::new_err("Priority Queue is empty")),
            _ => Ok(self.arr.remove(0))
        }
    }

    pub fn get_min(&mut self) -> PyResult<i32>{
        match self.arr.len() {
            0 => Err(PyValueError::new_err("Priority Queue is empty")),
            _ => Ok(self.arr.pop().unwrap()) 
        }
    }

    pub fn view(&mut self){
        println!("{:?}", self.arr);
    }

    pub fn len(&mut self) -> PyResult<usize>{
        Ok(self.arr.len())
    }

}


// (copied from https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rust_dev_preview/s3/src/bin/s3-helloworld.rs)
// Upload a file to a bucket.
// snippet-start:[s3.rust.s3-helloworld]
async fn upload_object(
    client: &Client,
    bucket: &str,
    filename: &str,
    key: &str,
) -> Result<(), Error> {

    let body = ByteStream::from_path(Path::new(filename)).await;

    match body {
        Ok(b) => {
            let _resp = client
                 .put_object()
                 .bucket(bucket)
                 .key(key)
                 .body(b)
                 .send()
                 .await?;

            println!("{} uploaded successfully", filename);

        }
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{}", e);
            process::exit(1);
        }
    }

    Ok(())
}

pub async fn batch_upload(file_key_map: &HashMap<String, String>, bucket: &str, region: Option<String>) -> Result<(), Error> {

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
    .or_default_provider()
    .or_else(Region::new("us-east-1"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let t_start = std::time::Instant::now();
    let futures = FuturesUnordered::new();
    for (filename, key) in file_key_map {
        futures.push(upload_object(&client, &bucket, &filename, &key));
    }

    let _res: Vec<_> = futures.collect().await;   
    println!("Uploaded {} files in {} seconds", file_key_map.len(), t_start.elapsed().as_secs());
    Ok(())
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn unlocked(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PriorityQueue>()?;
    Ok(())
}