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
    pub fn novel_page(&self) -> &'static [&'static str] {
        match self {
            Novel::Novel1 => &text_data::NOVEL1,
            Novel::Novel2 => &text_data::NOVEL2,
        }
    }
}

pub fn get_message(novel: Novel, count: usize) -> String {
    novel
        .novel_page()
        .get(count)
        .unwrap_or(&"Not found")
        .to_string()
}

impl NovelImg {
    pub fn nimgpath(&self, count: usize) -> Option<&'static str> {
        match self {
            NovelImg::Novel1 => match count {
                1 => Some("assets/images/ouch.webp"),
                2 => Some("assets/images/doup.webp"),
                5 => Some("assets/images/ikuzo.webp"),
                _ => None,
            },
            NovelImg::Novel2 => match count {
                0 => Some("assets/images/zenbuyaku.webp"),
                1 => Some("assets/images/nattsuya-de.webp"),
                2 => Some("assets/images/hakkou.webp"),
                3 => Some("assets/images/otosan.webp"),
                _ => None,
            },
        }
    }
}
