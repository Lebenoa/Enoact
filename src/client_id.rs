use std::sync::LazyLock;

use dashmap::DashMap;

static APP_IDS: LazyLock<DashMap<u8, Vec<&'static str>>> = LazyLock::new(|| {
    let map = DashMap::new();
    map.insert(
        0,
        vec![
            "1420250165960507492",
            "1398619981377175663",
            "1409423773362421820",
            "1454860736646877386",
        ],
    );
    map.insert(1, vec![]);

    map
});

pub(crate) fn get() -> &'static str {
    let id = APP_IDS.get_mut(&0).unwrap().pop();
    if let Some(id) = id {
        APP_IDS.get_mut(&1).unwrap().push(id);
        return id;
    }

    APP_IDS.get(&1).unwrap().first().unwrap()
}

pub(crate) fn free(id: &'static str) {
    {
        let mut vec = APP_IDS.get_mut(&1).unwrap();
        let used_index = vec.iter().position(|i| *i == id);

        if let Some(index) = used_index {
            vec.remove(index);
        }
    }

    {
        let mut vec = APP_IDS.get_mut(&0).unwrap();
        if !vec.contains(&id) {
            vec.push(id);
        }
    }
}
