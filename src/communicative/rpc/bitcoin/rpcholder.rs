/// RPC holder.
#[derive(Clone)]
pub struct RPCHolder {
    url: String,
    user: String,
    password: String,
}

impl RPCHolder {
    pub fn new(url: String, user: String, password: String) -> RPCHolder {
        RPCHolder {
            url,
            user,
            password,
        }
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn user(&self) -> String {
        self.user.clone()
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }
}
