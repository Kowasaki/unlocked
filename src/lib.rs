use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::{Client, Region, PKG_VERSION};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use rayon::prelude::*;

use std::error::Error;
use std::time::Duration;

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


// Copied from (https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rust_dev_preview/s3/src/bin/get-object-presigned.rs)
// Get object using presigned request.
// snippet-start:[s3.rust.get-object-presigned]
async fn get_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<(), Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(())
}

// (copied from https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rust_dev_preview/s3/src/bin/put-object-presigned.rs)
// Adds an object to a bucket and returns a public URI.
// snippet-start:[s3.rust.put-object-presigned]
async fn put_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<(), Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in);

    let presigned_request = client
        .put_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(())
}

#[pyfunction]
fn batch_download(uris: Vec<String>, bucket:string, ) -> PyResult<Vec<bool>> {

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
    .or_default_provider()
    .or_else(Region::new("us-east-1"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    uris.iter()
        .map(|uri| get_object(&client, &bucket, &uri, expires_in.unwrap_or(900)).await)
        .collect::<Vec<_>>()

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