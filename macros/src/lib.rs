use proc_macro::{Group, Ident, Punct, Span, Spacing, TokenStream, TokenTree};

//use crate::helpers::expect_punct;

fn expect_group(it: &mut impl Iterator<Item = TokenTree>) -> Group {
    if let Some(TokenTree::Group(group)) = it.next() {
        group.clone()
    } else {
        panic!("Expected Group")
    }
}

fn expect_punct(it: &mut impl Iterator<Item = TokenTree>) -> String {
    if let Some(TokenTree::Punct(punct)) = it.next() {
        punct.to_string()
    } else {
        panic!("Expected Group")
    }
}

fn drop_until_punct(it: &mut impl Iterator<Item = TokenTree>, delimiter: &str) {
    while let Some(token) = it.next() {
        match &token {
            TokenTree::Punct(punct) => {
                if delimiter.contains(&punct.to_string()) {
                    break;
                }
            }
            _ => (),
        }
    }
}

struct VersionConfig {
    fields: &'static [&'static str],
    enums: &'static [&'static [&'static str]],
    versions: &'static [&'static [&'static str]],
}

static AGX_VERSIONS: VersionConfig = VersionConfig {
    fields: &["G", "V"],
    enums: &[&["G13G", "G13X", "G14G"], &["V12_3", "V13_0b4"]],
    versions: &[
        &["G13G", "V12_3"],
        &["G13G", "V13_0b4"],
        &["G13X", "V12_3"],
        &["G13X", "V13_0b4"],
        &["G14G", "V13_0b4"],
    ],
};

fn check_version(
    config: &VersionConfig,
    ver: &[usize],
    it: &mut impl Iterator<Item = TokenTree>,
) -> bool {
    let first = it.next().unwrap();
    let val: bool = match &first {
        TokenTree::Group(group) => check_version(config, ver, &mut group.stream().into_iter()),
        TokenTree::Ident(ident) => {
            let key = match config.fields.iter().position(|&r| r == ident.to_string()) {
                Some(s) => s,
                None => panic!("Unknown field {}", ident.to_string()),
            };
            let mut operator = expect_punct(it);
            let mut rhs_token = it.next().unwrap();
            match &rhs_token {
                TokenTree::Punct(punct) => {
                    operator = operator + &punct.to_string();
                    rhs_token = it.next().unwrap();
                }
                _ => (),
            }
            let rhs_name = match &rhs_token {
                TokenTree::Ident(ident) => ident.to_string(),
                _ => panic!("Unexpected token {}", ident.to_string()),
            };

            let rhs = match config.enums[key].iter().position(|&r| r == rhs_name) {
                Some(s) => s,
                None => panic!("Unknown field {}", ident.to_string()),
            };
            let lhs = ver[key];

            match operator.as_str() {
                "==" => lhs == rhs,
                "!=" => lhs != rhs,
                ">" => lhs > rhs,
                ">=" => lhs >= rhs,
                "<" => lhs < rhs,
                "<=" => lhs <= rhs,
                _ => panic!("Unknown operator {}", operator),
            }
        }
        _ => {
            panic!("Unknown token {}", first)
        }
    };

    let boolop = it.next();
    match boolop {
        Some(TokenTree::Punct(punct)) => {
            let right = expect_punct(it);
            if right != punct.to_string() {
                panic!("Unexpected op {}{}", punct.to_string(), right);
            }
            match punct.as_char() {
                '&' => val && check_version(config, ver, it),
                '|' => val || check_version(config, ver, it),
                _ => panic!("Unexpected op {}{}", right, right),
            }
        }
        Some(a) => panic!("Unexpected op {}", a),
        None => val,
    }
}

fn filter_versions(
    config: &VersionConfig,
    tag: &str,
    ver: &[usize],
    tree: impl IntoIterator<Item = TokenTree>,
    is_struct: bool,
) -> Vec<TokenTree> {
    let mut out = Vec::<TokenTree>::new();
    let mut it = tree.into_iter();

    while let Some(token) = it.next() {
        match &token {
            TokenTree::Punct(punct) if punct.to_string() == "#" => {
                let group = expect_group(&mut it);
                let mut grp_it = group.stream().into_iter();
                let attr = grp_it.next().unwrap();
                match attr {
                    TokenTree::Ident(ident) if ident.to_string() == "ver" => {
                        if check_version(config, ver, &mut grp_it) {
                        } else if is_struct {
                            drop_until_punct(&mut it, ",");
                        } else {
                            let first = it.next().unwrap();
                            match &first {
                                TokenTree::Group(_) => (),
                                _ => {
                                    drop_until_punct(&mut it, ",;");
                                }
                            }
                        }
                    }
                    _ => {
                        out.push(token.clone());
                        out.push(TokenTree::Group(group.clone()));
                    }
                }
            }
            TokenTree::Punct(punct) if punct.to_string() == ":" => {
                let next = it.next();
                match next {
                    Some(TokenTree::Punct(punct)) if punct.to_string() == ":" => (),
                    Some(a) => {
                        out.push(token.clone());
                        out.push(a.clone());
                        continue;
                    }
                    None => {
                        out.push(token.clone());
                        continue;
                    }
                }

                let next = it.next();
                match next {
                    Some(TokenTree::Ident(idtag)) if idtag.to_string() == "ver" => {
                        let ident = match out.pop() {
                            Some(TokenTree::Ident(ident)) => ident,
                            a => panic!("$ver not following ident: {:?}", a),
                        };
                        let name = ident.to_string() + tag;
                        let new_ident = Ident::new(name.as_str(), ident.span());
                        out.push(TokenTree::Ident(new_ident));
                    }
                    Some(a) => {
                        out.push(token.clone());
                        out.push(token.clone());
                        out.push(a.clone());
                    }
                    None => {
                        out.push(token.clone());
                        out.push(token.clone());
                    }
                }
            }
            TokenTree::Group(group) => {
                let new_body =
                    filter_versions(config, tag, ver, &mut group.stream().into_iter(), is_struct);
                let mut stream = TokenStream::new();
                stream.extend(new_body);
                let mut filtered_group = Group::new(group.delimiter(), stream);
                filtered_group.set_span(group.span());
                out.push(TokenTree::Group(filtered_group));
            }
            _ => {
                out.push(token.clone());
            }
        }
    }

    out
}

#[proc_macro_attribute]
pub fn versions(attr: TokenStream, item: TokenStream) -> TokenStream {
    let config = match attr.to_string().as_str() {
        "AGX" => &AGX_VERSIONS,
        _ => panic!("Unknown version group {}", attr.to_string())
    };

    let mut it = item.into_iter();
    let mut out = TokenStream::new();
    let mut body: Vec<TokenTree> = Vec::new();
    let mut is_struct = false;

    while let Some(token) = it.next() {
        match token {
            TokenTree::Punct(punct) if punct.to_string() == "#" => {
                body.push(TokenTree::Punct(punct));
                body.push(it.next().unwrap());
            }
            TokenTree::Ident(ident) if ident.to_string() == "struct" => {
                body.push(TokenTree::Ident(ident));
                body.push(it.next().unwrap());
                // This isn't valid syntax in a struct definition, so add it for the user
                body.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
                body.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
                body.push(TokenTree::Ident(Ident::new("ver", Span::call_site())));
                is_struct = true;
                break;
            }
            TokenTree::Ident(ident) if ident.to_string() == "impl" => {
                body.push(TokenTree::Ident(ident));
                break;
            }
            TokenTree::Ident(ident) if ident.to_string() == "fn" => {
                body.push(TokenTree::Ident(ident));
                break;
            }
            _ => {
                body.push(token);
            }
        }
    }

    body.extend(it);

    for ver in config.versions {
        let tag = ver.join("");
        let mut ver_num = Vec::<usize>::new();
        for (i, comp) in ver.iter().enumerate() {
            let idx = config.enums[i]
                .iter()
                .position(|&r| r == comp.to_string())
                .unwrap();
            ver_num.push(idx);
        }
        out.extend(filter_versions(
            config,
            &tag,
            &ver_num,
            body.clone(),
            is_struct,
        ));
    }

    out
}
