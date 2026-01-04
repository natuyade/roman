use crate::text_data;



//Copyの内部処理にCloneが自動で付与されるようになっているが
//deriveのルール上Copy単体で書いてもCloneは使えない
//Cloneを覚えていないとCopyが使えない上位魔法だと思えばいい。

//ノベルリスト
#[derive(Copy, Clone)]
pub enum Novel {
    Novel1,
    Novel2,
}

//挿絵リスト
#[derive(Copy, Clone)]
pub enum NovelImg {
    Novel1,
    Novel2,
}

impl Novel {
    pub fn page(&self) -> &'static[&'static str] {
        match self {
            Novel::Novel1 => &text_data::NOVEL1,
            Novel::Novel2 => &text_data::NOVEL2,
        }
    }
}

pub fn get_message(novel: Novel, count: usize) -> String {
    novel
        .page()
        .get(count)
        .unwrap_or(&"Not found")
        .to_string()
}

impl NovelImg {
    pub fn path(&self, count:usize) -> &'static str{
        match self{
            NovelImg::Novel1 => match count {
                1 => "/image/ouch.webp",
                _ => "",
            },
            NovelImg::Novel2 => match count {
                0 => "/image/temmie.webp",
                _ => "",
                
            },
        }
    }
}
