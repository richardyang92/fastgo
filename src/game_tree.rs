use std::{vec, fs::{File, OpenOptions}, io::{BufReader, Read, Write}, rc::Rc, cell::RefCell, collections::LinkedList, borrow::BorrowMut, cmp::Ordering};

use json::JsonValue;

use crate::go_move::GoMove;

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub enum SgfToken {
    SEGSTART, SPLIT, SEGEND, VALSTART, VALEND,
    CA, FF, AP, GM, SZ, PB, PW, BR, WR, RE,
    KM, HA, TM, DT, EV, RO, PC, RU, GN, ON, SO,
    US, AN, CP, GC,
    AB, AW, AE, PL, B, W, C,
    CR, MA, SQ, TR, LB, TB, TW, VAL(String),
}

impl Default for SgfToken {
    fn default() -> Self {
        SgfToken::VAL("".to_string())
    }
}

impl SgfToken {
    fn is_prop_sgf_key(&self) -> bool {
        match self {
            SgfKey::SEGSTART
            | SgfKey::SPLIT
            | SgfKey::SEGEND
            | SgfKey::VALSTART
            | SgfKey::VALEND
            | SgfKey::VAL(_) => false,
            _ => true,
        }
    }
}

pub trait Parse<Token> {
    type Output;
    fn parse(&self) -> Self::Output;
}

pub trait ReadFile: Sized {
    fn read_from(filename: String) -> Result<Self, String>;
}

pub struct SgfReader {
    content: String
}

impl ReadFile for SgfReader {
    fn read_from(filename: String) -> Result<Self, String> {
        if !filename.ends_with(".sgf") {
            return Err("this isn't a sgf file!".to_string());
        }
        let sfg_file = File::open(filename)
            .map_err(|e| e.to_string()).unwrap();
        let mut sgf_reader = BufReader::new(sfg_file);
        let mut sgf_buf: Vec<u8> = vec![];
        match sgf_reader.read_to_end(&mut sgf_buf) {
            Ok(_) => {
                if let Ok(content) = String::from_utf8(sgf_buf) {
                    Ok(SgfReader { content })
                } else {
                    Err("read sgf content failed!".to_string())
                }
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Parse<SgfToken> for SgfReader {
    type Output = Vec<SgfToken>;
    fn parse(&self) -> Vec<SgfToken> {
        let mut sgf_tokens = vec![];
        let content_chs = self.content.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < content_chs.len() {
            let cur_ch = content_chs[i];
            if self.is_seg_start_token(cur_ch) {
                sgf_tokens.push(SgfToken::SEGSTART);
            } else if self.is_seg_end_token(cur_ch) {
                sgf_tokens.push(SgfToken::SEGEND);
            } else if self.is_seg_split_token(cur_ch) {
                sgf_tokens.push(SgfToken::SPLIT);
            } else if self.is_val_start_token(cur_ch) {
                sgf_tokens.push(SgfToken::VALSTART);
                let mut token_chs = vec![];
                let mut j = i + 1;
                while j < content_chs.len() && !self.is_val_end_token(content_chs[j]) {
                    token_chs.push(content_chs[j]);
                    j += 1;
                }
                let token_str = token_chs.iter().collect::<String>();
                sgf_tokens.push(SgfToken::VAL(token_str));
                sgf_tokens.push(SgfToken::VALEND);

                i += token_chs.len() + 2;
                continue;
            } else {
                let mut token_chs = vec![];
                token_chs.push(cur_ch);
                let mut j = i + 1;
                while j < content_chs.len() && self.is_big_character_ch(content_chs[j]) {
                    token_chs.push(content_chs[j]);
                    j += 1;
                }
                let token_str = token_chs.iter().collect::<String>();
                match token_str.as_str() {
                    "CA" => sgf_tokens.push(SgfToken::CA),
                    "FF" => sgf_tokens.push(SgfToken::FF),
                    "AP" => sgf_tokens.push(SgfToken::AP),
                    "GM" => sgf_tokens.push(SgfToken::GM),
                    "SZ" => sgf_tokens.push(SgfToken::SZ),
                    "PB" => sgf_tokens.push(SgfToken::PB),
                    "PW" => sgf_tokens.push(SgfToken::PW),
                    "BR" => sgf_tokens.push(SgfToken::BR),
                    "WR" => sgf_tokens.push(SgfToken::WR),
                    "RE" => sgf_tokens.push(SgfToken::RE),
                    "KM" => sgf_tokens.push(SgfToken::KM),
                    "HA" => sgf_tokens.push(SgfToken::HA),
                    "TM" => sgf_tokens.push(SgfToken::TM),
                    "DT" => sgf_tokens.push(SgfToken::DT),
                    "EV" => sgf_tokens.push(SgfToken::EV),
                    "RO" => sgf_tokens.push(SgfToken::RO),
                    "PC" => sgf_tokens.push(SgfToken::PC),
                    "RU" => sgf_tokens.push(SgfToken::RU),
                    "GN" => sgf_tokens.push(SgfToken::GN),
                    "ON" => sgf_tokens.push(SgfToken::ON),
                    "SO" => sgf_tokens.push(SgfToken::SO),
                    "US" => sgf_tokens.push(SgfToken::US),
                    "AN" => sgf_tokens.push(SgfToken::AN),
                    "CP" => sgf_tokens.push(SgfToken::CP),
                    "GC" => sgf_tokens.push(SgfToken::GC),
                    "AB" => sgf_tokens.push(SgfToken::AB),
                    "AW" => sgf_tokens.push(SgfToken::AW),
                    "AE" => sgf_tokens.push(SgfToken::AE),
                    "PL" => sgf_tokens.push(SgfToken::PL),
                    "B" => sgf_tokens.push(SgfToken::B),
                    "W" => sgf_tokens.push(SgfToken::W),
                    "C" => sgf_tokens.push(SgfToken::C),
                    "CR" => sgf_tokens.push(SgfToken::CR),
                    "MA" => sgf_tokens.push(SgfToken::MA),
                    "SQ" => sgf_tokens.push(SgfToken::SQ),
                    "TR" => sgf_tokens.push(SgfToken::TR),
                    "LB" => sgf_tokens.push(SgfToken::LB),
                    "TB" => sgf_tokens.push(SgfToken::TB),
                    "TW" => sgf_tokens.push(SgfToken::TW),
                    _ => {},
                }
                i += token_chs.len();
                continue;
            }
            i += 1;
        }
        sgf_tokens
    }
}

impl SgfReader {
    fn is_seg_start_token(&self, ch: char) -> bool {
        ch == '('
    }

    fn is_seg_end_token(&self, ch: char) -> bool {
        ch == ')'
    }

    fn is_seg_split_token(&self, ch: char) -> bool {
        ch == ';'
    }

    fn is_val_start_token(&self, ch: char) -> bool {
        ch == '['
    }
    
    fn is_val_end_token(&self, ch: char) -> bool {
        ch == ']'
    }

    fn is_big_character_ch(&self, ch: char) -> bool {
        ch >= 'A' && ch <= 'Z'
    }
}

pub type SgfKey = SgfToken;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct GameTree {
    is_root: bool,
    selected: bool,
    nodes: Option<Rc<RefCell<Vec<SgfNode>>>>,
    sub_game_trees: Option<Rc<RefCell<Vec<GameTree>>>>,
}

impl Ord for GameTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.selected {
            true => {
                if other.selected {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            },
            false => {
                if other.selected {
                    Ordering::Greater
                } else {
                    self.nodes.partial_cmp(&other.nodes).unwrap()
                }
            },
        }
    }
}

impl GameTree {
    pub fn create(
        km: f32,
        sz: i32,
        dt: String,
        pb: String,
        pw: String) -> Self {
        let game_tree = GameTree {
            is_root: true,
            selected: true,
            nodes: Some(Rc::new(RefCell::new(vec![]))),
            sub_game_trees: None,
        };
        let mut nodes_ref = game_tree.nodes.as_ref().unwrap().borrow_mut().take();
        let gm_node = SgfNode::new(SgfKey::GM, 1.to_string());
        nodes_ref.push(gm_node);
        let ff_node = SgfNode::new(SgfKey::FF, 4.to_string());
        nodes_ref.push(ff_node);
        let ca_node = SgfNode::new(SgfKey::CA, "utf-8".to_string());
        nodes_ref.push(ca_node);
        let ap_node = SgfNode::new(SgfKey::AP, "fastgo".to_string());
        nodes_ref.push(ap_node);
        let km_node = SgfNode::new(SgfKey::KM, km.to_string());
        nodes_ref.push(km_node);
        let sz_node = SgfNode::new(SgfKey::SZ, sz.to_string());
        nodes_ref.push(sz_node);
        let dt_node = SgfNode::new(SgfKey::DT, dt);
        nodes_ref.push(dt_node);
        let pb_node = SgfNode::new(SgfKey::PB, pb);
        nodes_ref.push(pb_node);
        let pw_node = SgfNode::new(SgfKey::PW, pw);
        nodes_ref.push(pw_node);
        game_tree.nodes.as_ref().unwrap().borrow_mut().replace(nodes_ref);
        game_tree
    }

    pub fn from_sgf_tokens(sgf_tokens: &Vec<SgfToken>, start: usize, end: usize, selected: bool, is_root: bool) -> Option<Self> {
        let mut i = start + 1;
        let mut idx_stack: LinkedList<usize> = LinkedList::new();
        let mut game_tree = GameTree {
            is_root,
            selected,
            nodes: None,
            sub_game_trees: None,
        };

        while i <= end - 1 {
            if sgf_tokens[i] == SgfKey::SEGSTART {
                idx_stack.push_front(i);
                let mut j = i + 1;
                while j <= end - 1 {
                    if sgf_tokens[j] == SgfKey::SEGSTART {
                        idx_stack.push_front(i);
                    } else if sgf_tokens[j] == SgfKey::SEGEND {
                        let k = idx_stack.pop_front().unwrap();
                        if idx_stack.len() == 0 {
                            let mut selected = false;
                            if let None = game_tree.sub_game_trees {
                                game_tree.sub_game_trees = Some(Rc::new(RefCell::new(vec![])));
                                selected = true;
                            }
                            let sub_game_tree = GameTree::from_sgf_tokens(sgf_tokens, k, j, selected, false);
                            if let Some(sub_game_tree) = sub_game_tree {
                                let mut sub_game_tree_ref = game_tree.sub_game_trees.as_ref().unwrap().borrow_mut().take();
                                sub_game_tree_ref.push(sub_game_tree);
                                game_tree.sub_game_trees.as_ref().unwrap().borrow_mut().replace(sub_game_tree_ref);
                            }
                            i = j;
                            break;
                        }
                    }
                    j += 1;
                }
            } else {
                if sgf_tokens[i] == SgfKey::SPLIT
                    && sgf_tokens[i + 1].is_prop_sgf_key() {
                    let sgf_node = game_tree.parse_sgf_node(sgf_tokens, i + 1);
                    if let Some(sgf_node) = sgf_node {
                        if let None = game_tree.nodes {
                            game_tree.nodes = Some(Rc::new(RefCell::new(vec![])));
                        }
                        let mut nodes_ref = game_tree.nodes.as_ref().unwrap().borrow_mut().take();
                        nodes_ref.push(sgf_node);
                        game_tree.nodes.as_ref().unwrap().borrow_mut().replace(nodes_ref);
                    }
                    i += 4;
                } else if sgf_tokens[i].is_prop_sgf_key() {
                    let sgf_node = game_tree.parse_sgf_node(sgf_tokens, i);
                    if let Some(sgf_node) = sgf_node {
                        if let None = game_tree.nodes {
                            game_tree.nodes = Some(Rc::new(RefCell::new(vec![])));
                        }
                        let mut nodes_ref = game_tree.nodes.as_ref().unwrap().borrow_mut().take();
                        nodes_ref.push(sgf_node);
                        game_tree.nodes.as_ref().unwrap().borrow_mut().replace(nodes_ref);
                    }
                    i += 3;
                }
            }
            i += 1;
        }
        Some(game_tree)
    }

    fn record_move<'a>(game_tree: &'a mut GameTree, res_move: i32, go_move: GoMove) -> Option<&'a GameTree> {
        if res_move < 0 {
            return None;
        }
        let nodes = game_tree.nodes.as_ref().unwrap().borrow_mut().take();
        let mut move_count = 0;
        let mut skip_count = 0;
        for node in nodes.clone() {
            match node.node_key {
                SgfKey::W | SgfKey::B => move_count += 1,
                _ => skip_count += 1,
            }
        }
        game_tree.nodes.as_ref().unwrap().borrow_mut().replace(nodes);
        if res_move <= move_count {
            // println!("find brach {:?}", game_tree);
            let mut nodes = game_tree.nodes.as_ref().unwrap().borrow_mut().take();
            if res_move == move_count {
                let sgf_node = SgfNode::from(go_move);
                nodes.push(sgf_node);
            } else {
                let origin = nodes.clone();
                let (old, new) = origin.split_at(res_move as usize + skip_count);
                // println!("old={:?}, new={:?}", old, new);
                let mut old_branch = Vec::from(old);
                let sgf_node = SgfNode::from(go_move);
                old_branch.push(sgf_node);
                nodes = old_branch;

                let new_branch = Vec::from(new);
                let new_sub_game_tree = GameTree {
                    is_root: false,
                    selected: false,
                    nodes: Some(Rc::new(RefCell::new(new_branch))),
                    sub_game_trees: None,
                };
                if let None = game_tree.sub_game_trees {
                    game_tree.sub_game_trees = Some(Rc::new(RefCell::new(vec![])));
                }
                let mut sub_game_trees = game_tree.sub_game_trees.as_ref().unwrap().borrow_mut().take();
                for mut sub_game_tree in sub_game_trees.clone() {
                    sub_game_tree.selected = false;
                }
                sub_game_trees.push(new_sub_game_tree);
                game_tree.sub_game_trees.as_ref().unwrap().borrow_mut().replace(sub_game_trees);
            }
            game_tree.nodes.as_ref().unwrap().borrow_mut().replace(nodes);
            // println!("record: {:?}", game_tree);
            return Some(game_tree);
        } else {
            match game_tree.sub_game_trees.as_ref() {
                Some(mut sub_game_trees_ref) => {
                    let mut sub_game_trees = sub_game_trees_ref.borrow_mut().take();
                    let mut i = 0;
                    for sub_game_tree in sub_game_trees.clone() {
                        if !sub_game_tree.selected {
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    let mut sub_game_tree = sub_game_trees[i].clone();
                    GameTree::record_move(&mut sub_game_tree, res_move - move_count, go_move);
                    // println!("sub: {:?}", sub_game_trees);
                    sub_game_trees[i] = sub_game_tree;
                    sub_game_trees_ref.borrow_mut().replace(sub_game_trees);
                },
                None => {},
            }
            // println!("{:?}", game_tree.sub_game_trees);
            Some(game_tree)
        }
    }
}

impl GameTree {
    fn parse_sgf_node(&mut self, sgf_tokens: &Vec<SgfToken>, idx: usize) -> Option<SgfNode> {
        let mut sgf_node = SgfNode::default();
        if sgf_tokens[idx + 1] == SgfKey::VALSTART
            && sgf_tokens[idx + 3] == SgfKey::VALEND {
            match &sgf_tokens[idx + 2] {
                SgfToken::VAL(val) => {
                    sgf_node.node_key = sgf_tokens[idx].clone();
                    sgf_node.node_val = val.clone();
                    return Some(sgf_node);
                },
                _ => {},
            }
        }
        None
    }

    fn to_json(&self) -> Option<JsonValue> {
        let mut root = json::JsonValue::new_object();
        let mut nodes = json::JsonValue::new_array();
        let mut sub_game_trees_json = json::JsonValue::new_array();
        let sgf_nodes = self.nodes.as_ref().unwrap().borrow_mut().take();
        for sgf_node in sgf_nodes.clone() {
            let node_key = format!("{:?}", sgf_node.node_key);
            let mut node = json::JsonValue::new_object();
            node[node_key] = sgf_node.node_val.into();
            nodes.push(node).expect("push node failed");
        }
        self.nodes.as_ref().unwrap().borrow_mut().replace(sgf_nodes);
        root["selected"] = json::JsonValue::Boolean(self.selected);
        root["nodes"] = nodes;
        match self.sub_game_trees.as_ref() {
            Some(mut sub_game_trees_ref) => {
                let sub_game_trees = sub_game_trees_ref.borrow_mut().take();
                for sub_game_tree in sub_game_trees.clone() {
                    let sub_game_tree_json = sub_game_tree.to_json().expect("parse sub tree failed");
                    sub_game_trees_json.push(sub_game_tree_json).expect("push sub tree failed!");
                }
                sub_game_trees_ref.borrow_mut().replace(sub_game_trees);
                root["sub_trees"] = sub_game_trees_json;
            },
            None => {},
        }
        Some(root)
    }

    fn to_string(&self) -> Option<String> {
        let mut sgf_str = if self.is_root { String::from("(;") } else { String::from("(") };
        let nodes = self.nodes.as_ref().unwrap().borrow_mut().take();
        for node in nodes.clone() {
            if node.node_key == SgfKey::B
                || node.node_key == SgfKey::W {
                sgf_str += &";".to_string().clone();
            }
            sgf_str += &SgfNode::to_string(node).clone();
        }
        self.nodes.as_ref().unwrap().borrow_mut().replace(nodes);
        match self.sub_game_trees.as_ref() {
            Some(mut sub_game_trees_ref) => {
                let sub_game_trees = sub_game_trees_ref.borrow_mut().take();
                for sub_game_tree in sub_game_trees.clone() {
                    let sub_game_tree_str = sub_game_tree.to_string().unwrap();
                    sgf_str += &sub_game_tree_str.clone();
                }
                sub_game_trees_ref.borrow_mut().replace(sub_game_trees);
            },
            None => {},
        }
        sgf_str += &")".to_string().clone();
        Some(sgf_str)
    }

    fn save_sgf(&self, filename: &str) -> std::io::Result<()> {
        let sgf_str = self.to_string();
        match sgf_str {
            Some(str) => {
                let mut file = match OpenOptions::new()  
                    .write(true)  
                    .create(true)  
                    .truncate(true)  
                    .open(filename) {  
                        Ok(file) => file,  
                        Err(why) => {  
                            println!(" couldn't open file: {}", why);  
                            return Ok(());  
                        }  
                    }; 
                file.write_all(str.as_bytes())  
                    .expect("failed to write to file");  
                file.flush()  
                    .expect("failed to flush file");
            },
            None => {},
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct SgfNode {
    node_key: SgfKey,
    node_val: String,
}

impl SgfNode {
    fn new(key: SgfKey, val: String) -> Self {
        SgfNode { node_key: key, node_val: val }
    }
}

impl Default for SgfNode {
    fn default() -> Self {
        Self { node_key: Default::default(), node_val: Default::default() }
    }
}

impl From<GoMove> for SgfNode {
    fn from(go_move: GoMove) -> Self {
        let (x, y, player) = go_move.move_pos();

        let mut sgf_node = SgfNode::default();
        if player == 1 {
            sgf_node.node_key = SgfKey::B;
        } else if player == -1 {
            sgf_node.node_key = SgfKey::W;
        }
        sgf_node.node_val = format!("{}{}", SgfNode::convert_pos(x), SgfNode::convert_pos(y));
        // println!("{:?}", sgf_node);
        sgf_node
    }
}

impl SgfNode {
    fn convert_pos(pos: usize) -> char {
        match pos {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            8 => 'i',
            9 => 'j',
            10 => 'k',
            11 => 'l',
            12 => 'm',
            13 => 'n',
            14 => 'o',
            15 => 'p',
            16 => 'q',
            17 => 'r',
            18 => 's',
            _ => ' ',
        }
    }
}

impl SgfNode {
    fn to_string(sgf_node: SgfNode) -> String {
        let mut node_str = String::new();
        let key_str = match sgf_node.node_key {
            SgfToken::CA => "CA",
            SgfToken::FF => "FF",
            SgfToken::AP => "AP",
            SgfToken::GM => "GM",
            SgfToken::SZ => "SZ",
            SgfToken::PB => "PB",
            SgfToken::PW => "PW",
            SgfToken::BR => "BR",
            SgfToken::WR => "WR",
            SgfToken::RE => "RE",
            SgfToken::KM => "KM",
            SgfToken::HA => "HA",
            SgfToken::TM => "TM",
            SgfToken::DT => "DT",
            SgfToken::EV => "EV",
            SgfToken::RO => "RO",
            SgfToken::PC => "PC",
            SgfToken::RU => "RU",
            SgfToken::GN => "GN",
            SgfToken::ON => "ON",
            SgfToken::SO => "SO",
            SgfToken::US => "US",
            SgfToken::AN => "AN",
            SgfToken::CP => "CP",
            SgfToken::GC => "GC",
            SgfToken::AB => "AB",
            SgfToken::AW => "AW",
            SgfToken::AE => "AE",
            SgfToken::PL => "PL",
            SgfToken::B => "B",
            SgfToken::W => "W",
            SgfToken::C => "C",
            SgfToken::CR => "CR",
            SgfToken::MA => "MA",
            SgfToken::SQ => "SQ",
            SgfToken::TR => "TR",
            SgfToken::LB => "LB",
            SgfToken::TB => "TB",
            SgfToken::TW => "TW",
            _ => "",
        };
        node_str += &key_str.to_string().clone();
        node_str += &"[".to_string().clone();
        node_str += &sgf_node.node_val.clone();
        node_str += &"]".to_string().clone();
        node_str
    }
}

#[cfg(test)]
mod test {
    use crate::go_move::GoMove;

    use super::{SgfReader, ReadFile, Parse, GameTree};

    #[test]
    pub fn test_scan_sgf() {
        let sgf_path = "sgf/test.sgf".to_string();
        if let Ok(sgf_reader) = SgfReader::read_from(sgf_path) {
            let sgf_tokens = sgf_reader.parse();
            let game_tree = GameTree::from_sgf_tokens(&sgf_tokens, 0, sgf_tokens.len() - 1, true, true);
            let json = match game_tree {
                Some(game_tree) => Some(game_tree.to_json()),
                None => None,
            };
            println!("json={}", json::stringify(json));
        }
    }

    #[test]
    pub fn create_new_sgf() {
        let game_tree = GameTree::create(
            7.5,
            19,
            "2023-06-03 17:21:30".to_string(),
            "a".to_string(),
            "b".to_string());
        println!("{:?}", json::stringify(game_tree.to_json().unwrap()));
    }

    #[test]
    pub fn test_record_move() {
        let sgf_path = "sgf/test.sgf".to_string();
        if let Ok(sgf_reader) = SgfReader::read_from(sgf_path) {
            let sgf_tokens = sgf_reader.parse();
            let mut game_tree = GameTree::from_sgf_tokens(&sgf_tokens, 0, sgf_tokens.len(), true, true).unwrap();
            println!("before={}", json::stringify(game_tree.to_json()));
            GameTree::record_move(&mut game_tree, 5, GoMove::new(9, 3, 10, -1));
            println!("after={}", json::stringify(game_tree.to_json()));
            let _ = game_tree.save_sgf("sgf/test2.sgf");
        }
    }

    #[test]
    pub fn test_to_string() {
        let sgf_path = "sgf/test.sgf".to_string();
        if let Ok(sgf_reader) = SgfReader::read_from(sgf_path) {
            let sgf_tokens = sgf_reader.parse();
            let game_tree = GameTree::from_sgf_tokens(&sgf_tokens, 0, sgf_tokens.len() - 1, true, true).unwrap();
            println!("{}", game_tree.to_string().unwrap());   
        }
    }
}