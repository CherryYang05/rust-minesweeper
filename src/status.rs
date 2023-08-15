/// 处理地图中的一系列状态事件
/// 例如 Debug 模式，游戏开始和结束状态
///

use bitflags::bitflags;

bitflags! {
    struct f: u8 {
        const A = 0b1;
        const B = 0b10;
        const AB = Self::A.bits();
        const C = Self::AB.bits();
    }
}


fn get() {
    let x = f{bits: 1};
    
}

use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize)]
struct Animal {
    name: String,
    age: u8
}

pub fn ser() {
    let animal = Animal {
        name: "cat".to_owned(),
        age: 3
    };
    let json = serde_json::to_string(&animal).unwrap();
    println!("{}", json);
}