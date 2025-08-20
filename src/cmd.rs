use std::path::Path;
use std::process::Command;
use std::fs;
use std::io;
use crate::{COLOR_GRAY, COLOR_RESET, COLOR_RED};

pub struct Cmd<'a> {
    pub name: &'a str,
    pub cmd: Command,
    pub related_files: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    pub fn new(cmd_str: &'a str, related_files: Vec<&'a str>) -> Self {
        let mut command = Command::new(cmd_str);
        command.args(["clean"]);
        Self {
            name: cmd_str,
            cmd: command,
            related_files,
        }
    }
    
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) {
        self.cmd.current_dir(dir);
    }
    
    pub fn run(&mut self) -> std::io::Result<std::process::Output> {
        self.cmd.output()
    }
    
    // 检查是否为需要特殊处理的命令
    pub fn is_special_clean_command(&self) -> bool {
        self.name == "nodejs"
    }
    
    // 执行特殊清理逻辑
    pub fn run_special_clean(&self, dir: &Path) -> io::Result<u32> {
        match self.name {
            "nodejs" => self.clean_nodejs_project(dir),
            _ => Ok(0)
        }
    }
    
    // 清理 Node.js 项目
    fn clean_nodejs_project(&self, dir: &Path) -> io::Result<u32> {
        let mut cleaned_count = 0;
        
        // 删除 node_modules 文件夹
        let node_modules = dir.join("node_modules");
        if node_modules.exists() && node_modules.is_dir() {
            cleaned_count += 1;
            println!("{}remove:{} node_modules{} {}", COLOR_GRAY, COLOR_RESET, COLOR_RED, dir.display());
            if let Err(e) = fs::remove_dir_all(&node_modules) {
                eprintln!("{dir:?} > {e:?}");
                return Err(e);
            }
        }
        
        Ok(cleaned_count)
    }
    
}

#[cfg(test)]
mod tests {
    use crate::constant::get_cmd_map;
    use crate::utils::command_exists;
    use super::*;
    
    #[test]
    fn test_cmd() {
        let cmd = Cmd::new("cargo", vec!["Cargo.toml"]);
        assert_eq!(cmd.name, "cargo");
        assert_eq!(cmd.related_files, vec!["Cargo.toml"]);
    }
    
    #[test]
    fn test_init_cmd_list() {
        let map = get_cmd_map();
        let mut cmd_list = vec![];
        //遍历map
        for (key, value) in map {
            if command_exists(key) {
                cmd_list.push(Cmd::new(key, value.clone()));
            }
        }
        // 现在有7个命令：cargo, go, gradle, npm, yarn, pnpm, mvn/mvn.cmd
        // 但测试环境中可能只有部分命令可用，所以检查总数而不是固定值
        assert!(cmd_list.len() >= 1); // 至少应该有cargo可用
        assert!(cmd_list.len() <= 7); // 最多7个命令
    }
    
    #[test]
    fn test_special_clean_commands() {
        let nodejs_cmd = Cmd::new("nodejs", vec!["package.json"]);
        let cargo_cmd = Cmd::new("cargo", vec!["Cargo.toml"]);
        
        assert!(nodejs_cmd.is_special_clean_command());
        assert!(!cargo_cmd.is_special_clean_command());
    }
}