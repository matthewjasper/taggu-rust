use std::path::Path;
use std::path::PathBuf;
use std::path::Component;

pub fn normalize(p: &Path) -> PathBuf {
    let mut stack: Vec<Component> = vec![];

    // We assume .components() removes redundant consecutive path separators.
    // Note that .components() also does some normalization of '.' on its own anyways.
    // This '.' normalization happens to be compatible with the approach below.
    for component in p.components() {
        match component {
            // Drop CurDir components, do not even push onto the stack.
            Component::CurDir => {},

            // For ParentDir components, we need to use the contents of the stack.
            Component::ParentDir => {
                // Look at the top element of stack, if any.
                let top = stack.last().cloned();

                match top {
                    // A component is on the stack, need more pattern matching.
                    Some(c) => {
                        match c {
                            // Push the ParentDir on the stack.
                            Component::Prefix(_) => { stack.push(component); },

                            // The parent of a RootDir is itself, so drop the ParentDir (no-op).
                            Component::RootDir => {},

                            // A CurDir should never be found on the stack, since they are dropped when seen.
                            Component::CurDir => { unreachable!(); },

                            // If a ParentDir is found, it must be due to it piling up at the start of a path.
                            // Push the new ParentDir onto the stack.
                            Component::ParentDir => { stack.push(component); },

                            // If a Normal is found, pop it off.
                            Component::Normal(_) => { let _ = stack.pop(); }
                        }
                    },

                    // Stack is empty, so path is empty, just push.
                    None => { stack.push(component); }
                }
            },

            // All others, simply push onto the stack.
            _ => { stack.push(component); },
        }
    }

    // If an empty PathBuf would be return, instead return CurDir ('.').
    if stack.is_empty() {
        return PathBuf::from(Component::CurDir.as_ref());
    }

    let mut norm_path = PathBuf::new();

    for item in &stack {
        norm_path.push(item.as_ref());
    }

    norm_path
}

#[cfg(test)]
mod tests {
    use super::normalize;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(Path::new("")), PathBuf::from("."));
        assert_eq!(normalize(Path::new("/")), PathBuf::from("/"));
        assert_eq!(normalize(Path::new("/..")), PathBuf::from("/"));
        assert_eq!(normalize(Path::new(".")), PathBuf::from("."));
        assert_eq!(normalize(Path::new("./foo")), PathBuf::from("foo"));
        assert_eq!(normalize(Path::new("foo")), PathBuf::from("foo"));
        assert_eq!(normalize(Path::new("/.")), PathBuf::from("/"));
        assert_eq!(normalize(Path::new("/..")), PathBuf::from("/"));
        assert_eq!(normalize(Path::new("foo/bar/")), PathBuf::from("foo/bar"));
        assert_eq!(normalize(Path::new("foo//bar///")), PathBuf::from("foo/bar"));
        assert_eq!(normalize(Path::new("foo/bar/./baz/")), PathBuf::from("foo/bar/baz"));
        assert_eq!(normalize(Path::new("foo/bar/../baz/")), PathBuf::from("foo/baz"));
        assert_eq!(normalize(Path::new("../foo")), PathBuf::from("../foo"));
    }
}
