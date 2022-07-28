use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(Default)]
pub struct TableInfo {
    pub non_terminal: Vec<String>, // 非终结符
    pub terminal: Vec<String>,     // 终结符/输入符号
    pub table: Vec<Vec<String>>,   // 产生式
}
impl TableInfo {
    // 根据文件初始化，返回值：是否是 LL(1) 文法
    pub fn init_table(&mut self, file_path: &String) -> bool {
        // 初始化存储内容
        self.non_terminal = vec![];
        self.terminal = vec![];
        self.table = vec![];

        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        // 逐行读取文件
        for line in reader.lines() {
            // 获取一行：非终结符、终结符、产生式
            let words: Vec<String> = line.unwrap().split(" ").map(|s| s.to_string()).collect();
            // 有多重定义，不是 LL(1) 文法
            if words.len() > 3 {
                return false;
            }

            // 读取非终结符和终结符
            let non_tmn_symbol = words.get(0).unwrap();
            if !self.non_terminal.contains(non_tmn_symbol) {
                self.non_terminal.push(non_tmn_symbol.to_string());
                // 读取到生成表的新一行，增加一个产生式 vector
                self.table.push(vec![]);
            }
            let tmn_symbol = words.get(1).unwrap();
            if !self.terminal.contains(tmn_symbol) {
                self.terminal.push(tmn_symbol.to_string());
            }

            // 写入产生式
            let mut expression = " ";
            if words.len() > 2 {
                expression = words.get(2).unwrap();
            }
            self.table.last_mut().unwrap().push(expression.to_string());
        }

        true
    }

    // 读取产生式
    // fn read_analyze_table(&mut self, file_path: &String) {
    //     let row_num = self.non_terminal.len(); // 行数
    //     let col_num = self.terminal.len(); // 列数
    //     for _i in 0..row_num {
    //         self.table.push(vec![]);
    //     }
    //     let mut count: usize = 0;

    //     let file = File::open(file_path).unwrap();
    //     let reader = BufReader::new(file);
    //     for line in reader.lines() {
    //         // 读取一行
    //         let words: Vec<String> = line.unwrap().split(" ").map(|s| s.to_string()).collect();
    //         let mut expression: String = " ".to_string();
    //         if words.len() > 2 {
    //             // 产生式不为空
    //             expression = words.get(2).unwrap().to_string();
    //         }
    //         self.table
    //             .get_mut(count / col_num)
    //             .unwrap()
    //             .push(expression.clone());
    //         count += 1;
    //     }
    // }

    // 将分析表格式化并拼接后返回
    pub fn get_analyze_table(&self) -> Option<String> {
        let mut table = format!("{:<10}", "TABLE");

        // 列标题为输入符号/终结符
        for i in 0..self.terminal.len() {
            let tmn_symbol = format!("{:<10}", self.terminal.get(i).unwrap());
            table = format!("{}{}", table, tmn_symbol);
        }
        table = format!("{}\r\n", table);
        // 行标题为非终结符，后面跟上各行的表项
        for i in 0..self.non_terminal.len() {
            let non_tmn_symbol = format!("{:<10}", self.non_terminal.get(i).unwrap());
            table = format!("{}{}", table, non_tmn_symbol);

            for j in 0..self.terminal.len() {
                let exp = format!("{:<10}", self.table.get(i).unwrap().get(j).unwrap());
                table = format!("{}{}", table, exp);
            }
            table = format!("{}\r\n", table);
        }
        return Some(table);
    }

    // 分析输入串，将过程拼接成字符串形式，返回值：(是否符合文法, 分析过程)
    pub fn analyze_input(&self, input_str: &String) -> (bool, Option<String>) {
        // 分析过程
        let mut process = format!(
            "{:<16}  {:<16}  {:<16}  {:<16}\r\n",
            "Matched", "Stack", "Input", "Action"
        );

        // 栈
        let mut stack: Vec<String> = vec![];
        // 依次把 $ 和 起始符号压入栈中
        stack.push("$".to_string());
        stack.push(self.non_terminal.get(0).unwrap().clone());
        // 下标，代替指向输入内容的指针
        let mut ip = 0;
        // 分割输入内容
        let input_symbols: Vec<String> = input_str.split(" ").map(|s| s.to_string()).collect();

        // 分析步骤中已匹配、栈、输入和动作部分的内容
        let mut matched = String::default();
        stack.reverse();
        let mut stack_content = stack.concat();
        stack.reverse();
        let mut input = input_symbols.concat() + "$";
        let mut action = String::default();
        // 开始分析前记录一次分析步骤
        process = format!(
            "{}{:<16}  {:>16}  {:>16}  {:<16}\r\n",
            process, matched, stack_content, input, action
        );

        // x = 栈顶符号
        let mut x = stack.last().unwrap().clone();
        while !x.eq("$") {
            let a: String;
            if ip < input_symbols.len() {
                // a = ip 指向的符号
                a = input_symbols.get(ip).unwrap().clone();
            } else {
                // ip 越界，此时 a = $
                a = "$".to_string();
            }

            if a.eq(&x) {
                // 若 x 等于 a，弹出，ip 向前移动一位
                matched += &stack.pop().unwrap();
                input.remove(0);
                action = format!("匹配 {}", &a);
                ip += 1;
            } else if self.terminal.contains(&x) {
                // X 是终结符
                process = format!("{}X: {} 是终结符，且与输入串不匹配。", process, &x);
                return (false, Some(process));
            } else {
                // 找到 M[X, a] 在 table 中的位置
                let (mut i, mut j) = (0, 0);
                for (index, value) in self.non_terminal.iter().enumerate() {
                    if value == &x {
                        i = index;
                    }
                }
                for (index, value) in self.terminal.iter().enumerate() {
                    if value == &a {
                        j = index;
                    }
                }

                // M[X, a]
                let exp = self.table.get(i).unwrap().get(j).unwrap();
                // M[X, a] 是报错条目（空产生式）
                if exp == " " {
                    process = format!("{}Error! M[{}, {}] 是报错条目。", process, &x, &a);
                    return (false, Some(process));
                } else if exp.starts_with(&x) {
                    // M[X, a] = X->y1y2...yk，弹栈
                    stack.pop();
                    action = format!("输出 {}", exp);
                    // 把 yk yk-1 ... y1 压入栈
                    if !exp.contains("null") {
                        let exp_split: Vec<String> =
                            exp.split("->").map(|s| s.to_string()).collect();
                        let mut exp_right = exp_split.get(1).unwrap().clone();
                        while !exp_right.is_empty() {
                            stack.push(exp_right.pop().unwrap().to_string());
                        }
                    }
                }
            }
            x = stack.last().unwrap().clone();
            // 更新分析过程
            stack.reverse();
            stack_content = stack.concat();
            stack.reverse();
            process = format!(
                "{}{:<16}  {:>16}  {:>16}  {:<16}\r\n",
                process, matched, stack_content, input, action
            );
        }

        (true, Some(process))
    }
}
