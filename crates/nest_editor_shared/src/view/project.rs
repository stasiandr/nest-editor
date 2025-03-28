use std::{collections::BTreeMap, path::Path};

use bevy_egui::egui::{self, Ui};
use egui_ltreeview::{TreeView, TreeViewBuilder};
use ignore::WalkBuilder;


#[derive(Debug, Clone)]
enum FileNode {
    Directory(BTreeMap<String, FileNode>),
    File,
}


fn build_tree(root: &Path) -> FileNode {
    let mut root_map = BTreeMap::new();

    for result in WalkBuilder::new(root)
        .standard_filters(true)
        .follow_links(false)
        .build()
    {
        let Ok(entry) = result else { continue; };
        let path = entry.path();

        if path == root {
            continue;
        }

        let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());

        let Ok(rel_path) = path.strip_prefix(root) else { continue; };
        if rel_path.as_os_str().is_empty() {
            continue; // Shouldn't happen, but be safe
        }

        insert_into_tree(&mut root_map, rel_path, is_dir);
    }

    FileNode::Directory(root_map)
}

fn insert_into_tree(tree: &mut BTreeMap<String, FileNode>, rel_path: &Path, is_dir: bool) {
    if let Some(first) = rel_path.iter().next() {
        let key = first.to_string_lossy().to_string();
        let remainder = rel_path.strip_prefix(first).unwrap_or(rel_path);
        let entry = tree.entry(key).or_insert_with(|| {
            if is_dir {
                FileNode::Directory(BTreeMap::new())
            } else {
                FileNode::File
            }
        });

        if remainder.components().next().is_none() {
            if is_dir {
                *entry = FileNode::Directory(
                    match entry {
                        FileNode::Directory(ref m) => m.clone(),
                        FileNode::File => BTreeMap::new(),
                    }
                );
            } else {
                *entry = FileNode::File;
            }
        } else if let FileNode::Directory(ref mut subtree) = entry {
            insert_into_tree(subtree, remainder, is_dir);
        } else {
            let mut new_map = BTreeMap::new();
            insert_into_tree(&mut new_map, remainder, is_dir);
            *entry = FileNode::Directory(new_map);
        }
    }
}

fn show_nested_tree(ui: &mut egui::Ui, root: &std::path::Path) {
    let file_node = build_tree(root);
    if let FileNode::Directory(tree_map) = file_node {
        TreeView::new("nested_tree".into()).show(ui, |builder| {
            let mut next_id = 0_usize;
            let root_label = root.to_string_lossy().to_string();
            
            render_tree(builder, &tree_map, &mut next_id, &root_label);
        });
    }
}

fn render_tree(
    builder: &mut TreeViewBuilder<usize>,
    nodes: &std::collections::BTreeMap<String, FileNode>,
    next_id: &mut usize,
    label: &str,
) {
    let this_dir_id = *next_id;
    *next_id += 1;

    builder.dir(this_dir_id, label);
    for (name, node) in nodes {
        match node {
            FileNode::File => {
                let leaf_id = *next_id;
                *next_id += 1;

                builder.leaf(leaf_id, name);
            }
            FileNode::Directory(subtree) => {
                render_tree(builder, subtree, next_id, name);
            }
        }
    }

    builder.close_dir();
}


pub fn show_project_ui(ui: &mut Ui, project_path: &Path) {
    show_nested_tree(ui, project_path);
}