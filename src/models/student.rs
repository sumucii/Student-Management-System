// src/models/student.rs
#[derive(Debug, Clone)]
pub struct Student { // 定义一个Student结构体，包含学号、姓名、年龄和年级，分别为u32、String、u8和String类型，使用derive宏生成Debug和Clone trait的实现，方便调试和克隆。pub作用：使结构体Student对外可见。
    pub id: u32, // 学号
    pub name: String, // 姓名
    pub age: u8, // 年龄
    pub grade: String, // 年级
}