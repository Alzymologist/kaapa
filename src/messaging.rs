pub struct CallConstructorEvent {
    pub index: Vec<usize>,
    pub ccei: CCEI,
}

pub struct CCEI {
    pub id: usize,
    pub message: String,
}
