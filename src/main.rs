// src/main.rs
mod models; // 导入models模块

use std::io::{stdin, Write}; // 导入标准库中的stdin和Write
use colored::*; // 导入colored库
use models::student::Student; // 导入Student结构体
use models::manager::StudentManager; // 导入StudentManager结构体

fn main() {
    let mut manager = StudentManager::new(); // 创建一个新的StudentManager实例
    let filename = "students.txt"; // 定义文件名

    loop {
        println!("{}", "*********************************学生管理系统*********************************".cyan()); // 打印系统标题
        println!("{}", "请选择你的操作：".blue()); // 提示用户选择操作
        println!("{}", "1.添加学生".green()); // 选项1：添加学生
        println!("{}", "2.删除学生".green()); // 选项2：删除学生
        println!("{}", "3.更新学生信息".green()); // 选项3：更新学生信息
        println!("{}", "4.查找学生".green()); // 选项4：查找学生
        println!("{}", "5.显示所有学生信息".green()); // 选项5：显示所有学生信息
        println!("{}", "6.保存学生的信息到文件中".green()); // 选项6：保存学生信息到文件
        println!("{}", "7.从文件中加载学生信息".green()); // 选项7：从文件中加载学生信息
        println!("{}", "8.退出".green()); // 选项8：退出系统
        println!("{}", "注：请先加载文件，再进行操作".yellow()); // 提示用户先加载文件
        println!("{}", "********************************************************************************".cyan()); // 打印分隔线

        let mut choice = String::new(); // 定义变量存储用户选择
        stdin().read_line(&mut choice).expect("无法读取输入"); // 读取用户输入
        let choice = choice.trim().parse().expect("请输入数字"); // 解析用户输入为数字

        match choice {
            1 => {
                let mut id = String::new(); // 定义变量存储学号
                let mut name = String::new(); // 定义变量存储姓名
                let mut age = String::new(); // 定义变量存储年龄
                let mut grade = String::new(); // 定义变量存储年级

                println!("{}", "请输入学号：".blue()); // 提示用户输入学号
                stdin().read_line(&mut id).expect("无法读取输入信息"); // 读取学号
                let id = id.trim().parse().expect("请输入数字"); // 解析学号为数字

                println!("{}", "请输入姓名：".blue()); // 提示用户输入姓名
                stdin().read_line(&mut name).expect("无法读取输入"); // 读取姓名

                println!("{}", "请输入年龄：".blue()); // 提示用户输入年龄
                stdin().read_line(&mut age).expect("无法读取输入"); // 读取年龄
                let age = age.trim().parse().expect("请输入正确的数字"); // 解析年龄为数字

                println!("{}", "请输入年级".blue()); // 提示用户输入年级
                stdin().read_line(&mut grade).expect("无法读取输入"); // 读取年级

                manager.add_student(Student { id, name: name.trim().to_string(), age, grade: grade.trim().to_string() }); // 添加学生信息
            }

            2 => {
                let mut id = String::new(); // 定义变量存储学号
                println!("{}", "输入要删除的学生学号：".blue()); // 提示用户输入要删除的学生学号
                stdin().read_line(&mut id).expect("无法读取输入"); // 读取学号
                let id = id.trim().parse().expect("请输入数字"); // 解析学号为数字
                manager.remove_student(id); // 删除学生信息
            }

            3 => {
                let mut id = String::new(); // 定义变量存储学号
                let mut name = String::new(); // 定义变量存储姓名
                let mut age = String::new(); // 定义变量存储年龄
                let mut grade = String::new(); // 定义变量存储年级

                println!("{}", "输入要更新的学生学号：".blue()); // 提示用户输入要更新的学生学号
                stdin().read_line(&mut id).expect("无法读取输入"); // 读取学号
                let id = id.trim().parse().expect("请输入数字"); // 解析学号为数字

                println!("{}", "输入新的名字：".blue()); // 提示用户输入新的名字
                stdin().read_line(&mut name).expect("无法读取输入"); // 读取名字

                println!("{}", "请输入新的年龄：".blue()); // 提示用户输入新的年龄
                stdin().read_line(&mut age).expect("无法读取输入"); // 读取年龄
                let age = age.trim().parse().expect("请输入数字"); // 解析年龄为数字

                println!("{}", "请输入新的年级：".blue()); // 提示用户输入新的年级
                stdin().read_line(&mut grade).expect("无法读取输入"); // 读取年级

                manager.update_student(id, name.trim().to_string(), age, grade.trim().to_string()); // 更新学生信息
            }

            4 => {
                let mut id = String::new(); // 定义变量存储学号
                println!("{}", "请输入要查找的学生学号：".blue()); // 提示用户输入要查找的学生学号
                stdin().read_line(&mut id).expect("无法读取输入"); // 读取学号
                let id = id.trim().parse().expect("请输入数字"); // 解析学号为数字

                if let Some(student) = manager.find_student(id) { // 查找学生信息
                    println!("{},{:?}", "找到学生".yellow(), student); // 打印找到的学生信息
                } else {
                    println!("{}", "未找到该学生，请重新选择".red()); // 提示未找到学生
                }
            }

            5 => {
                manager.show_all_students(); // 显示所有学生信息
                println!("{}", "所有学生信息已展示".yellow()); // 提示所有学生信息已展示
            }

            6 => {
                manager.save_to_file(filename).expect("无法保存文件"); // 保存学生信息到文件
                println!("{}", "学生信息已保存到文件".yellow()); // 提示学生信息已保存到文件
            }

            7 => {
                manager.load_from_file(filename).expect("无法加载文件"); // 从文件中加载学生信息
                println!("{}", "学生信息已从文件中加载成功".yellow()); // 提示学生信息已从文件中加载成功
            }

            8 => {
                break; // 退出系统
            }

            _ => {
                println!("{}", "无效的选择，请输入正确的数字！！！".red()); // 提示无效的选择
            }
        }
    }
}