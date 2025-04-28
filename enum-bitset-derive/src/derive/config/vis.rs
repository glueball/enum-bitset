use std::iter::once;

use quote::format_ident;
use syn::{Path, PathSegment, Token, VisRestricted, Visibility, parse_str, punctuated::Punctuated};

pub fn compute_visibility(base_vis: &Visibility) -> Visibility {
    let path = match base_vis {
        Visibility::Public(_) => return base_vis.clone(),
        Visibility::Inherited => return parse_str("pub(super)").unwrap(),
        Visibility::Restricted(vis) => &*vis.path,
    };

    if path.leading_colon.is_some()
        || path
            .segments
            .first()
            .is_some_and(|s| s.ident == "crate")
    {
        return base_vis.clone();
    }

    let prefix: PathSegment = PathSegment {
        ident: format_ident!("super"),
        arguments: Default::default(),
    };

    let segments: Punctuated<_, Token![::]> = once(prefix)
        .chain(path.segments.iter().cloned())
        .collect();

    Visibility::Restricted(VisRestricted {
        pub_token: Default::default(),
        paren_token: Default::default(),
        in_token: Some(Default::default()),
        path: Box::new(Path {
            leading_colon: None,
            segments,
        }),
    })
}

#[cfg(test)]
mod tests {
    use crate::derive::config::{vis::compute_visibility, *};

    #[test]
    fn visibility_pub() {
        let vis: Visibility = parse_str("pub").unwrap();
        let computed = compute_visibility(&vis);

        assert_eq!(computed, vis);
    }

    #[test]
    fn visibility_priv() {
        let vis: Visibility = parse_str("").unwrap();
        let computed = compute_visibility(&vis);

        let expected: Visibility = parse_str("pub(super)").unwrap();
        assert_eq!(computed, expected);
    }

    #[test]
    fn visibility_crate() {
        let vis: Visibility = parse_str("pub(crate)").unwrap();

        let computed = compute_visibility(&vis);

        assert_eq!(computed, vis);
    }

    #[test]
    fn visibility_starting_with_crate() {
        let vis: Visibility = parse_str("pub(in crate::module)").unwrap();

        let computed = compute_visibility(&vis);

        assert_eq!(computed, vis);
    }

    #[test]
    fn visibility_super() {
        let vis: Visibility = parse_str("pub(super)").unwrap();
        let computed = compute_visibility(&vis);

        let expected: Visibility = parse_str("pub(in super::super)").unwrap();
        assert_eq!(computed, expected);
    }

    #[test]
    fn visibility_random_path() {
        let vis: Visibility = parse_str("pub(in abc::xyz)").unwrap();
        let computed = compute_visibility(&vis);

        let expected: Visibility = parse_str("pub(in super::abc::xyz)").unwrap();
        assert_eq!(computed, expected);
    }
}
