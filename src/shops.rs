use crate::file_sys::*;

pub fn get_shop(username: String) -> ShopUser {
    let mut data = de_shops();

    let mut shop_user = ShopUser { items: Vec::new(), user: username.clone() };

    if data.users.iter().position(|s| s.user == username).is_none() {
        data.users.push(shop_user.clone());
        data.usernames.push(username.clone());
    }

    let pos = data.users.iter().position(|s| s.user == username).unwrap();

    if data.usernames.contains(&username) { 
        shop_user = data.users[pos].clone();
    }


    // Publish to data
    if data.usernames.contains(&username) {
        data.users.remove(pos);
        data.usernames.remove(pos);
    }

    data.users.push(shop_user.clone());
    data.usernames.push(username);

    ser_shops(data);
    shop_user
}