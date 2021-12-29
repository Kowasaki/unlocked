use std::collections::HashMap;

mod lib;


#[tokio::main]
async fn main(){

    let mut filename_key_map = HashMap::new();
    let paths = std::fs::read_dir("./tests/test_artifacts").unwrap();
    for f in paths{
        let filename = f.unwrap().path().into_os_string().into_string().unwrap(); 
        let key = filename.split("./tests/test_artifacts/").collect::<Vec<&str>>()[1].to_string();
        filename_key_map.insert(filename, key);
    }
    for (f,k) in &filename_key_map {
        println!("filename: {} Key: {}", f, k)
    }
    let _res = lib::batch_upload(&filename_key_map, "data-testing-ground", Some("us-east-1".to_string())).await;
}