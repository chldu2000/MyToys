use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(Default)]
pub struct GrammerInfo {
    pub start: String,                        // 起始符号
    pub terminals: Vec<String>,               // 终结符
    pub non_terminals: Vec<String>,           // 非终结符
    pub productions: Vec<String>,             // 产生式
    pub first: HashMap<String, Vec<String>>,  // first
    pub follow: HashMap<String, Vec<String>>, // follow
}
impl GrammerInfo {
    pub fn init_grammer(&mut self, path: String) {
        // 初始化文法信息
        self.start = String::new();
        self.terminals = vec![];
        self.non_terminals = vec![];
        self.productions = vec![];
        self.first = HashMap::new();
        self.follow = HashMap::new();

        // 按行读取文件
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut count = 0;
        for line in reader.lines() {
            if count < 1 {
                // 起始符号
                self.start = line.unwrap();
            } else if count < 2 {
                // 非终结符
                let non_terminal_symbols: Vec<String> =
                    line.unwrap().split(" ").map(|s| s.to_string()).collect();
                self.non_terminals = non_terminal_symbols;
            } else if count < 3 {
                // 终结符
                let terminal_symbols: Vec<String> =
                    line.unwrap().split(" ").map(|s| s.to_string()).collect();
                self.terminals = terminal_symbols;
            } else {
                // 产生式，将带有“|”的产生式分割成多个
                let line_split: Vec<String> =
                    line.unwrap().split("->").map(|l| l.to_string()).collect();
                let exp_post: Vec<String> = line_split
                    .get(1)
                    .unwrap()
                    .split("|")
                    .map(|e| e.to_string())
                    .collect();
                for exp in exp_post {
                    self.productions
                        .push(line_split.get(0).unwrap().to_string() + "->" + &exp);
                }
            }
            count += 1;
        }
    }

    pub fn get_first(&mut self) -> String {
        let mut first = String::new();
        // 给非终结符分配空的 first 集合
        for symbol in &self.non_terminals {
            self.first.insert(symbol.clone(), vec![]);
        }
        // symbol 是终结符，它的 first 集合中只有自己
        for symbol in &self.terminals {
            self.first
                .insert(symbol.clone(), Vec::from([symbol.to_string()]));
        }
        // 求解非终结符的 first 集合，把结果拼接成字符串形式
        // solve_first 函数内会进行递归求解，不用关注 first 是否发生变化
        for symbol in self.non_terminals.clone() {
            first = first + "First(" + &symbol + "): ";
            self.solve_first(&symbol);
            let first_vec = self.first.get(&symbol);
            for i in 0..first_vec.unwrap().len() {
                first = first + first_vec.unwrap().get(i).unwrap() + " ";
            }
            first += "\r\n";
        }
        first
    }

    // 求 symbol 的 first 集合
    fn solve_first(&mut self, symbol: &String) {
        if self.first.get(symbol).is_some() && self.first.get(symbol).unwrap().len() > 0 {
            // 已经算出 symbol 符号的 first
            return;
        }
        for production in self.productions.clone() {
            let prod_split: Vec<String> = production.split("->").map(|p| p.to_string()).collect();
            if prod_split.get(0).unwrap().eq(symbol) {
                let prod_post = prod_split.get(1); // 产生式右侧
                let n = prod_post.unwrap().len();
                let mut k: usize = 0;
                let mut continue_b = true;
                let epsilon_str = "null".to_string(); // epsilon
                if epsilon_str.eq(prod_post.unwrap()) {
                    // 产生式右边是 epsilon（这里用字符串 null 表示），把 epsilon 加入 symbol 的 first集合
                    let mut first_vec = self.first.get(symbol).unwrap().clone();
                    if !first_vec.contains(&epsilon_str) {
                        first_vec.push(epsilon_str.clone());
                    }
                    self.first.insert(symbol.to_string(), first_vec);
                } else {
                    while continue_b && k < n {
                        // 产生式右侧的 x0 x1 ... xn-1
                        let xk = prod_post.unwrap().chars().nth(k).unwrap().to_string();
                        // 求出 xk 的 first 集合
                        self.solve_first(&xk);
                        // 把 xk 的 first 加入到 symbol 的 first
                        let first_xk = self.first.get(&xk).unwrap().clone();
                        for first_str in &first_xk {
                            if first_str.ne(&epsilon_str) {
                                let mut first_vec = self.first.get(symbol).unwrap().clone();
                                if !first_vec.contains(&first_str) {
                                    first_vec.push(first_str.to_string());
                                }
                                self.first.insert(symbol.to_string(), first_vec);
                            }
                        }
                        // xk 不能导出 epsilon，就不再向 First(symbol) 中加入符号
                        if !first_xk.contains(&epsilon_str) {
                            continue_b = false;
                        }
                        k += 1;
                    }
                    // 从 x0 能够推出 epsilon，将 epsilon 加入到 First(symbol)
                    if continue_b {
                        let mut first_vec = self.first.get(symbol).unwrap().clone();
                        if !first_vec.contains(&epsilon_str) {
                            first_vec.push(epsilon_str);
                        }
                        self.first.insert(symbol.to_string(), first_vec);
                    }
                }
            }
        }
    }

    pub fn get_follow(&mut self) -> String {
        let mut follow = String::new();
        // 初始化 follow 集合
        for symbol in &self.non_terminals {
            self.follow.insert(symbol.clone(), vec![]);
        }

        for symbol in self.non_terminals.clone() {
            // 对每个非终结符求 follow
            let mut follow_changed = self.solve_follow(&symbol);
            // follow 集合有变化，对所有非终结符再求一次 follow
            while follow_changed {
                for symbol in self.non_terminals.clone() {
                    follow_changed = self.solve_follow(&symbol);
                }
            }
        }

        // 把结果拼接成字符串
        for symbol in self.non_terminals.clone() {
            follow = follow + "Follow(" + &symbol + "): ";
            let follow_vec = self.follow.get(&symbol);
            for i in 0..follow_vec.unwrap().len() {
                follow = follow + follow_vec.unwrap().get(i).unwrap() + " ";
            }
            follow += "\r\n";
        }
        follow
    }

    fn solve_follow(&mut self, symbol: &String) -> bool {
        let mut changed = false; // 任意 follow 集合是否有变化
        let end = "$".to_string();
        if self.start.eq(symbol) && !self.follow.get(symbol).unwrap().contains(&end) {
            // Follow(start_symbol) 一定包含 $
            self.follow.insert(symbol.to_string(), Vec::from([end]));
            changed = true;
        }

        // 对任意非终结符
        if self.non_terminals.contains(symbol) {
            for production in self.productions.clone() {
                let prod_split: Vec<String> =
                    production.split("->").map(|p| p.to_string()).collect();
                if prod_split.get(0).unwrap().eq(symbol) {
                    let prod_post = prod_split.get(1); // 产生式右侧
                    let n = prod_post.unwrap().len();
                    let mut i: usize = 0;
                    let epsilon_str = "null".to_string(); // epsilon

                    // 对于产生式右侧的任意 xi
                    while i < n {
                        let mut contains_epsilon = false;
                        let xi = prod_post.unwrap().chars().nth(i).unwrap().to_string();
                        if self.non_terminals.contains(&xi) {
                            // 当 xi 在产生式末尾，Follow(symbol) 中所有元素都在 Follow(xi) 中
                            if i == n - 1 {
                                let follow_symbol = self.follow.get(symbol).unwrap().clone();
                                let mut follow_xi = self.follow.get(&xi).unwrap().clone();
                                for follow_str in follow_symbol {
                                    if !follow_xi.contains(&follow_str) {
                                        follow_xi.push(follow_str.to_string());
                                        changed = true;
                                    }
                                }
                                self.follow.insert(xi.clone(), follow_xi);
                                // changed = true;
                            }
                            // First(xi+1 xi+2 ...), 即 First(xi+1) 中所有元素都在 Follow(xi) 中
                            // epsilon 除外
                            if i < n - 1 {
                                let xip1 =
                                    prod_post.unwrap().chars().nth(i + 1).unwrap().to_string();
                                let first_xip1 = self.first.get(&xip1).unwrap().clone();
                                let mut follow_xi = self.follow.get(&xi).unwrap().clone();
                                for first_str in first_xip1 {
                                    if !follow_xi.contains(&first_str)
                                        && !epsilon_str.eq(&first_str)
                                    {
                                        follow_xi.push(first_str);
                                        changed = true;
                                    } else if epsilon_str.eq(&first_str) {
                                        // First(xi+1 xi+2 ...) 包含 epsilon
                                        contains_epsilon = true;
                                    }
                                }

                                self.follow.insert(xi.to_string(), follow_xi);
                                // changed = true;
                            }
                            // First(xi+1 xi+2 ...) 包含 epsilon
                            // Follow(symbol) 中所有元素都在 Follow(xi) 中
                            if contains_epsilon {
                                let follow_symbol = self.follow.get(symbol).unwrap().clone();
                                let mut follow_xi = self.follow.get(&xi).unwrap().clone();
                                for follow_str in follow_symbol {
                                    if !follow_xi.contains(&follow_str) {
                                        follow_xi.push(follow_str.to_string());
                                        changed = true;
                                    }
                                }
                                self.follow.insert(xi, follow_xi);
                                // changed = true;
                            }
                        }
                        i += 1;
                    }
                }
            }
        }
        changed
    }

    pub fn get_table(&self) -> String {
        let mut table_str = String::new();
        let mut table = HashMap::new();
        for non_tmn in self.non_terminals.clone() {
            for tmn in self.terminals.clone() {
                let value: Vec<String> = Vec::new();
                table.insert(Vec::from([non_tmn.clone(), tmn.clone()]), value);
            }
        }
        // 对于每个产生式
        for prod in self.productions.clone() {
            let prod_split: Vec<String> = prod.split("->").map(|s| s.to_string()).collect();
            let non_tmn = prod_split.get(0).unwrap().to_string(); // 产生式左侧非终结符
            let prod_post = prod_split.get(1).unwrap().to_string(); // 产生式右侧

            // 产生式右侧不是 epsilon
            if !prod_post.eq(&"null".to_string()) {
                let alpha = prod_post.chars().nth(0).unwrap().to_string();
                let first_alpha_vec = self.first.get(&alpha).unwrap();

                for first_str in first_alpha_vec {
                    // 对于 First(产生式右侧) 中的每个终结符 first_str
                    if self.terminals.contains(first_str) {
                        // 将该产生式加入到 M[non_tmn, first_str]
                        let vec_temp = Vec::from([non_tmn.clone(), first_str.clone()]);
                        let mut table_content = table.get(&vec_temp).unwrap().clone();
                        if !table_content.contains(&prod) {
                            table_content.push(prod.clone());
                        }
                        table.insert(vec_temp, table_content);
                    } else if first_str.eq(&"null".to_string()) {
                        // 如果 epsilon 在 First(产生式右侧) 中
                        // 对 Follow(non_tmn) 中的每个终结符 follow_str，包括 $
                        // 将产生式加入到 M[non_tmn, follow_str]
                        for follow_str in self.follow.get(&non_tmn).unwrap() {
                            if self.terminals.contains(follow_str) {
                                let vec_temp = Vec::from([non_tmn.clone(), follow_str.clone()]);
                                let mut table_content = table.get(&vec_temp).unwrap().clone();
                                if !table_content.contains(&prod) {
                                    table_content.push(prod.clone());
                                }
                                table.insert(vec_temp, table_content);
                            }
                        }
                    }
                }
            } else {
                // epsilon 在产生式右侧，处理方式同 epsilon 在 First(产生式右侧) 中的情况
                for follow_str in self.follow.get(&non_tmn).unwrap() {
                    if self.terminals.contains(follow_str) {
                        let vec_temp = Vec::from([non_tmn.clone(), follow_str.clone()]);
                        let mut table_content = table.get(&vec_temp).unwrap().clone();
                        if !table_content.contains(&prod) {
                            table_content.push(prod.clone());
                        }
                        table.insert(vec_temp, table_content);
                    }
                }
            }
        }

        // 将分析表拼成实验九使用的格式并返回
        // A a M[A, a]
        for non_tmn in self.non_terminals.clone() {
            for tmn in self.terminals.clone() {
                table_str = table_str + &non_tmn + " " + &tmn;
                let vec_temp = Vec::from([non_tmn.clone(), tmn.clone()]);
                let table_content = table.get(&vec_temp).unwrap().clone();
                for content in table_content {
                    table_str = table_str + " " + &content;
                }
                table_str = table_str + "\r\n";
            }
        }
        table_str
    }
}
