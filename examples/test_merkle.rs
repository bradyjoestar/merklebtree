extern crate ring;
use ring::digest;
use ring::digest::Digest;

fn main() {
    {
        println!("***********************test2*****************************");
        let key = 1;
        let content_hash = hex::encode(digest::digest(&digest::SHA256, key.to_string().as_ref()));
        println!("content_hash:{}", content_hash);
        let node_hash = hex::encode(digest::digest(&digest::SHA256, content_hash.as_ref()));
        println!("node_hash:{}", node_hash);
    }
    {
        println!("************************test3****************************");
        let key_1 = 1;
        let hash_1 = hex::encode(digest::digest(&digest::SHA256, key_1.to_string().as_ref()));
        let key_2 = 2;
        let hash_2 = hex::encode(digest::digest(&digest::SHA256, key_2.to_string().as_ref()));

        let mut key_3 = String::new();
        key_3.push_str(hash_1.as_str());
        key_3.push_str(hash_2.as_str());

        let hash_3 = hex::encode(digest::digest(&digest::SHA256, key_3.to_string().as_ref()));
        println!("hash_3:{}", hash_3);
    }
    {
        println!("****************************************************");
        let key_1 = 1;
        let content_1 = hex::encode(digest::digest(&digest::SHA256, key_1.to_string().as_ref()));
        let hash_1 = hex::encode(digest::digest(
            &digest::SHA256,
            content_1.to_string().as_ref(),
        ));

        let key_2 = 2;
        let content_2 = hex::encode(digest::digest(&digest::SHA256, key_2.to_string().as_ref()));
        let hash_2 = content_2;

        let key_3 = 3;
        let content_3 = hex::encode(digest::digest(&digest::SHA256, key_3.to_string().as_ref()));
        let hash_3 = hex::encode(digest::digest(
            &digest::SHA256,
            content_3.to_string().as_ref(),
        ));

        println!("hash_1:{}", hash_1);
        println!("hash_2:{}", hash_2);
        println!("hash_3:{}", hash_3);

        let mut key_total = String::new();
        key_total.push_str(hash_2.as_str());
        key_total.push_str(hash_1.as_str());
        key_total.push_str(hash_3.as_str());

        let hash_total = hex::encode(digest::digest(
            &digest::SHA256,
            key_total.to_string().as_ref(),
        ));
        println!("hash_total:{}", hash_total);
    }
}
