use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    sync::{LazyLock, Mutex},
};

static FREE_APP_IDS: LazyLock<Mutex<Vec<&'static str>>> = LazyLock::new(|| {
    Mutex::new(Vec::from_iter([
        "1420250165960507492",
        "1398619981377175663",
        "1409423773362421820",
        "1454860736646877386",
    ]))
});
static USED_APP_IDS: LazyLock<Mutex<Vec<&'static str>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Clone, Eq, PartialEq, Default, Hash)]
pub struct ClientId(pub Cow<'static, str>);

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for ClientId {
    fn from(id: &'static str) -> Self {
        ClientId(id.into())
    }
}

impl From<String> for ClientId {
    fn from(id: String) -> Self {
        ClientId(id.into())
    }
}

impl Borrow<str> for ClientId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Drop for ClientId {
    fn drop(&mut self) {
        tracing::debug!("`Drop` trait called for ClientId: {:?}", self.0);
        let removed = {
            let mut vec = USED_APP_IDS.lock().unwrap();
            let used_index = vec.iter().position(|i| *i == self.0);

            used_index.map(|index| vec.remove(index))
        };

        if let Some(id) = removed {
            let mut vec = FREE_APP_IDS.lock().unwrap();
            if !vec.contains(&id) {
                vec.push(id);
            }
        }
    }
}

impl ClientId {
    pub fn new() -> Self {
        let id = FREE_APP_IDS.lock().unwrap().pop();
        if let Some(id) = id {
            USED_APP_IDS.lock().unwrap().push(id);
            tracing::debug!("Acquired `{id}`");
            return id.into();
        }

        let id = USED_APP_IDS.lock().unwrap().pop().unwrap();
        id.into()
    }

    pub fn new_from_free() -> Option<Self> {
        let id = FREE_APP_IDS.lock().unwrap().pop();
        if let Some(id) = id {
            USED_APP_IDS.lock().unwrap().push(id);
            tracing::debug!("Acquired `{id}`");
            return Some(id.into());
        }
        None
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
