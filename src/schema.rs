#![allow(non_snake_case)]

table! {
    doc(doc_id){
        doc_id -> BigSerial,
    }
}


table! {
    world (id) {
        id -> Integer,
        randomnumber -> Integer,
    }
}

table! {
    fortune (id) {
        id -> Integer,
        message -> Text,
    }
}