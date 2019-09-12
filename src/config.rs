pub mod wikipedia {
    use parse_wiki_text::ConfigurationSource;

    pub fn english<'c>() -> ConfigurationSource<'c> {
        ConfigurationSource {
            category_namespaces: &["category"],
            extension_tags: &[
                "categorytree",
                "ce",
                "charinsert",
                "chem",
                "gallery",
                "graph",
                "hiero",
                "imagemap",
                "indicator",
                "inputbox",
                "mapframe",
                "maplink",
                "math",
                "nowiki",
                "poem",
                "pre",
                "ref",
                "references",
                "score",
                "section",
                "source",
                "syntaxhighlight",
                "templatedata",
                "templatestyles",
                "timeline",
            ],
            file_namespaces: &["file", "image"],
            link_trail: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            magic_words: &[
                "DISAMBIG",
                "EXPECTUNUSEDCATEGORY",
                "FORCETOC",
                "HIDDENCAT",
                "INDEX",
                "NEWSECTIONLINK",
                "NOCC",
                "NOCOLLABORATIONHUBTOC",
                "NOCONTENTCONVERT",
                "NOEDITSECTION",
                "NOGALLERY",
                "NOGLOBAL",
                "NOINDEX",
                "NONEWSECTIONLINK",
                "NOTC",
                "NOTITLECONVERT",
                "NOTOC",
                "STATICREDIRECT",
                "TOC",
            ],
            protocols: &[
                "//",
                "bitcoin:",
                "ftp://",
                "ftps://",
                "geo:",
                "git://",
                "gopher://",
                "http://",
                "https://",
                "irc://",
                "ircs://",
                "magnet:",
                "mailto:",
                "mms://",
                "news:",
                "nntp://",
                "redis://",
                "sftp://",
                "sip:",
                "sips:",
                "sms:",
                "ssh://",
                "svn://",
                "tel:",
                "telnet://",
                "urn:",
                "worldwind://",
                "xmpp:",
            ],
            redirect_magic_words: &["REDIRECT"],
        }
    }

    pub fn simple_english<'c>() -> ConfigurationSource<'c> {
        english()
    }
}
