use pyo3::prelude::*;

#[pyclass]
pub struct MyClass {
    pub x: f64,
    pub y: f64,
    pub name: String,
}

#[pymethods]
impl MyClass {
    #[new]
    pub fn new(name: String, x: f64, y: f64) -> PyResult<Self> {
        Ok(MyClass {
            x: x,
            y: y,
            name: name,
        })
    }
    pub fn print_list(&self, lst: Vec<i64>) -> Option<String> {
        if lst.len() == 0 {
            None
        } else {
            let mut s = String::new();
            for n in lst.iter() {
                s.push_str(format!("{}", n).as_str());
            }
            Some(s)
        }
    }
    pub fn show(&self) -> String {
        format!(
            "MyClass(name={}, x={:.2}, y={:.2})",
            self.name, self.x, self.y
        )
    }
}
