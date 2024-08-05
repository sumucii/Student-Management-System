// src/models/manager.rs
use std::fs::File; // 导入标准库中的File
use std::io::{self, BufRead, BufReader, Write}; // 导入标准库中的io模块及其子模块BufRead、BufReader和Write
use crate::models::student::Student; // 导入Student结构体

pub struct StudentManager { // 定义一个StudentManager结构体，包含一个学生向量
    pub students: Vec<Student>, // 学生向量
}

impl StudentManager {
    pub fn new() -> Self { // 定义一个new方法，返回一个新的StudentManager实例
        StudentManager { // 返回一个新的StudentManager实例
            students: Vec::new(), // 初始化学生向量为空
        }
    }

    pub fn add_student(&mut self, student: Student) { // 定义一个add_student方法，添加一个学生到学生向量中
        self.students.push(student); // 将学生添加到学生向量中
    }

    pub fn remove_student(&mut self, id: u32) { // 定义一个remove_student方法，根据学号删除学生
        self.students.retain(|student| student.id != id); // 保留所有学号不等于给定学号的学生
    }

    pub fn update_student(&mut self, id: u32, name: String, age: u8, grade: String) { // 定义一个update_student方法，根据学号更新学生信息
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) { // 查找学号等于给定学号的学生
            student.name = name; // 更新学生姓名
            student.age = age; // 更新学生年龄
            student.grade = grade; // 更新学生年级
        }
    }

    pub fn find_student(&self, id: u32) -> Option<&Student> { // 定义一个find_student方法，根据学号查找学生
        self.students.iter().find(|&student| student.id == id) // 返回学号等于给定学号的学生
    }

    pub fn show_all_students(&self) { // 定义一个show_all_students方法，显示所有学生信息
        for student in &self.students { // 遍历所有学生
            println!("{:?}", student); // 打印学生信息
        }
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> { // 定义一个save_to_file方法，将学生信息保存到文件
        let mut file = File::create(filename)?; // 创建文件
        for student in &self.students { // 遍历所有学生
            writeln!(file, "{}---{}---{}---{}", student.id, student.name, student.age, student.grade)?; // 将学生信息写入文件
        }
        Ok(()) // 返回Ok
    }

    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()> { // 定义一个load_from_file方法，从文件中加载学生信息
        let file = File::open(filename)?; // 打开文件
        let reader = BufReader::new(file); // 创建一个BufReader
        self.students.clear(); // 清空学生向量
        for line in reader.lines() { // 遍历文件中的每一行
            let line = line?; // 读取一行
            let parts: Vec<&str> = line.split("---").collect(); // 将行按"---"分割
            if parts.len() == 4 { // 如果分割后的部分有4个
                let id = parts[0].parse().unwrap_or(0); // 解析学号
                let name = parts[1].to_string(); // 解析姓名
                let age = parts[2].parse().unwrap_or(0); // 解析年龄
                let grade = parts[3].to_string(); // 解析年级
                self.students.push(Student { id, name, age, grade }); // 将学生添加到学生向量中
            }
        }
        Ok(()) // 返回Ok
    }
}