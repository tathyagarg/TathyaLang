/*use {
    std::{
        collections
    },
    crate::{
        character_reader,
        character_reader::ReaderFunctionality
    }
};

mod toolbox {
    pub struct Syntax<'a> {
        pub text: &'a str,
        pub ending_char: char
    }

    const keyword: Syntax = Syntax { text: "!!", ending_char: ':' };
    const seperator: Syntax = Syntax { text: ":", ending_char: '-' }; // Null ending char
    
    pub fn query(text: &str) -> Syntax {
        match text {
            "keyword" => keyword,
            "seperator" => seperator,
            &_ => todo!()
        }
    }

    pub fn token_list() -> Vec<&'static str> {
        vec!["keyword", "seperator"]
    }
}

/*
Tokens are defined as HashMap<&str, &str>. They are in the blueprint:
token = HashMap::from([
    ("type", "token type"),
    ("value", "the text of the token"),
    ("position", "start>end") or ("position", "position")
])
Forced to adapt the position tuple into a &str
*/

pub struct Lexer<'a> {
    pub reader: &'a character_reader::Reader<'a>,
    pub tokens: &'a mut Vec<&'a collections::HashMap<&'a str, &'a str>>,
    pub position: u16,
    pub string_position: &'a str
}

pub trait LexerFunctionality<'a> {
    fn new(reader: &'a character_reader::Reader, vector: &'a mut Vec<&'a collections::HashMap<&'a str, &'a str>>) -> Self;
    fn update_string_position(&mut self) -> ();
    fn lex(&mut self) -> ();
}

impl<'a> LexerFunctionality<'a> for Lexer<'a> {
    fn new(reader: &'a character_reader::Reader, vector: &'a mut Vec<&'a collections::HashMap<&'a str, &'a str>>) -> Lexer<'a> {
        Lexer {
            reader: reader,
            tokens: vector,
            position: 0,
            string_position: "0"
        }
    }

    fn update_string_position(&mut self) -> () {
        let position = &self.position.to_string();
        self.string_position = position;
    }

    fn lex(&mut self) -> () {
        for token in toolbox::token_list() {
            if self.reader.contents.starts_with(token) {
                let token_obj: toolbox::Syntax = toolbox::query(token);
                let string_position: &str = if token_obj.ending_char == '-' { self.string_position } else { format!("{}>{}", self.string_position, (self.position + self.reader.first_instance_of(token_obj.ending_char))).as_str() };
                let value: &str = &self.reader.contents[..((if token_obj.ending_char == '-' { 0u16 } else { self.reader.first_instance_of(token_obj.ending_char) }) as usize)];
                let appended_item: collections::HashMap<&'a str, &'a str> = collections::HashMap::from([
                    ("type", token_obj.text),
                    ("value", value),
                    ("position", string_position)
                ]);
                self.tokens.push(&appended_item);
            }
        }
        /*
        for token in toolbox::token_list() {
            if self.reader.contents.starts_with(token) {
                let token_obj: toolbox::Syntax = toolbox::query(token);
                let ending_pos: u16 = self.reader.first_instance_of(token_obj.ending_char);
                let appended_item: collections::HashMap<&str, &str> = if token_obj.ending_char == '-' {
                    collections::HashMap::from([
                        ("type", token_obj.text),
                        ("value", token_obj.text),
                        ("position", self.string_position)
                    ])
                } else {
                    let value = &self.reader.contents[..(ending_pos as usize)];
                    collections::HashMap::from([
                        ("type", token_obj.text),
                        ("value", value),
                        ("position", format!("{}>{}", self.position, self.position + ending_pos).as_str())
                    ])
                };
                if ending_pos == u16::MAX { panic!("Syntax error: '{}' expected after '{}'", token_obj.ending_char, appended_item.get("value").unwrap()) }

                self.tokens.push(&appended_item);
                self.position += ending_pos;
                return Some( &appended_item );
            }
        }
        return None; */
    }
}
*/
