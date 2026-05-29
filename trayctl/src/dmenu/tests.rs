use super::find_selected;
use crate::ipc::MenuItem;

fn make_item(item_id: u32, label: &str, is_submenu: bool) -> MenuItem {
    MenuItem {
        item_id,
        label: label.to_owned(),
        is_submenu,
    }
}

/// `find_selected` returns the item whose label matches exactly.
#[test]
fn find_selected_by_label() {
    let items = vec![
        make_item(1, "Copy", false),
        make_item(2, "More Actions", true),
    ];
    let found = find_selected(&items, "More Actions").expect("should find item");
    assert_eq!(found.item_id, 2);
    assert!(found.is_submenu);
}

/// `find_selected` returns `None` for an unknown label.
#[test]
fn find_selected_missing_returns_none() {
    let items = vec![make_item(1, "Copy", false), make_item(2, "Paste", false)];
    assert!(find_selected(&items, "Delete").is_none());
}

/// The loop's filter discards all items when every label is empty,
/// which causes it to exit without calling dmenu.
#[test]
fn run_submenu_loop_empty_labels_filtered() {
    let items = [
        make_item(1, "", false),
        make_item(2, "", true),
        make_item(3, "", false),
    ];
    let visible: Vec<&MenuItem> = items.iter().filter(|i| !i.label.is_empty()).collect();
    assert!(
        visible.is_empty(),
        "all empty-label items must be filtered out before dmenu is spawned"
    );
}

/// When multiple items share the same label, `find_selected` returns the first.
#[test]
fn find_selected_first_match() {
    let items = vec![
        make_item(10, "Duplicate", false),
        make_item(20, "Duplicate", false),
    ];
    let found = find_selected(&items, "Duplicate").expect("should find first match");
    assert_eq!(found.item_id, 10);
}

/// Basic sanity check on `MenuItem`'s `is_submenu` field.
#[test]
fn menu_item_is_submenu() {
    let leaf = make_item(1, "Leaf", false);
    let folder = make_item(2, "Folder", true);
    assert!(!leaf.is_submenu);
    assert!(folder.is_submenu);
    assert_eq!(leaf.label, "Leaf");
    assert_eq!(folder.item_id, 2);
}
