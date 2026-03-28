use std::sync::{LazyLock, Mutex};

static FREE_APP_IDS: LazyLock<Mutex<Vec<&'static str>>> = LazyLock::new(|| {
    Mutex::new(Vec::from_iter([
        "1420250165960507492",
        "1398619981377175663",
        "1409423773362421820",
        "1454860736646877386",
    ]))
});
static USED_APP_IDS: LazyLock<Mutex<Vec<&'static str>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub(crate) fn get() -> &'static str {
    let id = FREE_APP_IDS.lock().unwrap().pop();
    if let Some(id) = id {
        USED_APP_IDS.lock().unwrap().push(id);
        tracing::debug!("`{id}` in USED");
        return id;
    }

    let id = USED_APP_IDS.lock().unwrap().pop().unwrap();
    tracing::debug!("`Recycle {id}` from USED");
    id
}

pub(crate) fn free(id: &'static str) {
    {
        let mut vec = USED_APP_IDS.lock().unwrap();
        let used_index = vec.iter().position(|i| *i == id);

        if let Some(index) = used_index {
            vec.remove(index);
            tracing::debug!("Removed `{id}` from USED");
        }
    }

    {
        let mut vec = FREE_APP_IDS.lock().unwrap();
        if !vec.contains(&id) {
            vec.push(id);
            tracing::debug!("`{id}` in FREE");
        }
    }
}
