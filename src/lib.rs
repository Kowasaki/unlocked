
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;


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
        } else {
            let ptr = self.arr.len() / 2;
            self.heapify(ptr, val);
            Ok(())
        } 
    }

    fn heapify(&mut self, ptr: usize, val: i32){
        
        if ptr == 0 {
            if self.arr[ptr] <= val {
                self.arr.insert(ptr, val);
            } 
        } else if ptr == self.arr.len() - 1 {
            if self.arr[ptr] >= val {
                self.arr.push(val);
            }  else if  self.arr[ptr-1] >= val {
                self.arr.insert(ptr, val);
            }
        } else if self.arr[ptr] <= val {
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

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn unlocked(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PriorityQueue>()?;
    Ok(())
}

// Can't get the tests to work in rust...
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_get_max() {
//         let mut priority_q = PriorityQueue::new();
//         priority_q.insert(13);
//         priority_q.insert(25);
//         assert_eq!(priority_q.get_max().unwrap(), 25);
//         assert_eq!(priority_q.len().unwrap(), 1);
//     }
//     #[test] 
//     fn test_get_min() {
//         let mut priority_q = PriorityQueue::new();
//         priority_q.insert(13);
//         priority_q.insert(25);
//         assert_eq!(priority_q.get_min().unwrap(), 13);
//         assert_eq!(priority_q.len().unwrap(), 1);
//     }
//     #[test] 
//     fn test_order() {
//         let mut priority_q = PriorityQueue::new();
//         priority_q.insert(3);
//         priority_q.insert(5);
//         priority_q.insert(4);
//         priority_q.insert(2);
//         priority_q.insert(1);
//         assert_eq!(priority_q.arr, vec![5,4,3,2,1]);

//         let mut priority_q2 = PriorityQueue::new();
//         priority_q2.insert(3);
//         priority_q2.insert(3);
//         priority_q2.insert(3);
//         priority_q2.insert(3);
//         priority_q2.insert(3);
//         assert_eq!(priority_q2.arr, vec![3,3,3,3,3]);
//     }
//     #[test] 
//     #[should_panic]
//     fn test_no_element(){
//         let mut priority_q = PriorityQueue::new();
//         // assert_eq!(priority_q.get_max());
//         priority_q.get_max();

//     }

// }
