use std::result::Result;

pub struct WriteBatch {

}

pub struct CFIterator<'a> {

}

pub struct DB {

}

impl DB {
    pub fn view<T, F>(&self, func: F) -> Result<T, Error>
    where
        F: FnOnce(&Txn) -> Result<T, Error>,
    {

    }

    pub fn update<F>(&self, func: F) -> Result<(), Error>
    where
        F: FnOnce(&Txn) -> Result<(), Error>,
    {
        todo!()
    }

    pub fn new_transaction(&self, read_only: bool) -> Txn {

    }
}

pub struct Txn {

}

impl Txn {
    pub fn discard(&self) {

    }

    pub fn get_cf(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        todo!()
    }

    pub fn set_cf(&self, cf: &str, key: &[u8], val: &[u8]) -> Result<(), Error> {
        todo!()
    }

    pub fn delete_cf(&self, cf: &str, key: &[u8]) -> Result<(), Error> {
        todo!()
    }

    pub fn commit(self) -> Result<(), Error> {
        todo!()
    }
}

pub struct Item {

}

impl Item {
    pub fn key_copy(&self, dst: &mut [u8]) -> &[u8] {

    }

    pub fn value_copy(&self, dst: &mut [u8]) -> Result<usize, Error> {

    }
}

pub fn key_with_cf(cf: &str, key: &[u8]) -> Vec<u8> {
    todo!()
}

pub fn get_cf(db: &DB, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    todo!()
}

pub fn get_cf_from_txn(txn: &Txn, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    todo!()
}

pub fn put_cf(db: &DB, cf: &str, key: &[u8], val: &[u8]) -> Result<(), Error> {
    todo!()
}

pub fn get_meta(db: &DB, key: &[u8], msg: &mut dyn protobuf::Message) -> Result<(), Error> {
    todo!()
}

pub fn get_meta_from_txn(txn: &Txn, key: &[u8], msg: &mut dyn protobuf::Message) -> Result<(), Error> {
    todo!()
}

pub fn put_meta(db: &DB, key: &[u8], msg: &dyn protobuf::Message) -> Result<(), Error> {
    todo!()
}

pub fn delete_cf(db: &DB, cf: &str, key: &[u8]) -> Result<(), Error> {
    todo!()
}

pub fn delete_range(db: &DB, start_key: &[u8], end_key: &[u8]) -> Result<(), Error> {
    todo!()
}

pub fn delete_range_cf(txn: &Txn, batch: &mut WriteBatch, cf: &str, start_key: &[u8], end_key: &[u8]) {
}

pub fn exceed_end_key(current: &[u8], end_key: &[u8]) -> bool {
    todo!()
}
