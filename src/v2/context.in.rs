// This is not a normal Rust module! It's included directly into v2.rs,
// possibly after build-time preprocessing.  See v2.rs for an explanation
// of how this works.

/// Either a local directory path, or a Git-format "URL" (not necessarily a
/// real URL, alas).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Context {
    /// A regular local directory.
    Dir(PathBuf),
    /// A Git repository, specified using any of the usual git repository
    /// syntaxes.
    GitUrl(GitUrl),
}

impl Context {
    /// Construct a new Context from a string, identifying it as either a
    /// local path or a remote git repository.
    ///
    /// ```
    /// use compose_yml::v2 as dc;
    /// dc::Context::new("https://github.com/docker/docker.git");
    /// dc::Context::new("src/myapp");
    /// ```
    pub fn new<S: AsRef<str>>(s: S) -> Context {
        let s_ref = s.as_ref();
        if GitUrl::should_treat_as_url(s_ref) {
            // unwrap is safe here because of contract on
            // `should_treat_as_url`.
            Context::GitUrl(GitUrl::new(s_ref.to_owned()).unwrap())
        } else {
            Context::Dir(Path::new(&s_ref).to_owned())
        }
    }
}

impl_interpolatable_value!(Context);

impl FromStr for Context {
    type Err = Void;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(Context::new(s))
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Context::Dir(ref path) => write!(f, "{}", path.display()),
            &Context::GitUrl(ref url) => write!(f, "{}", url),
        }
    }
}

#[test]
fn context_may_contain_git_urls() {
    // See http://stackoverflow.com/a/34120821/12089
    let urls =
        vec!("git://github.com/docker/docker",
             "git@github.com:docker/docker.git",
             "git@bitbucket.org:atlassianlabs/atlassian-docker.git",
             "https://github.com/docker/docker.git",
             "http://github.com/docker/docker.git",
             "github.com/docker/docker.git");

    for url in urls {
        let context: Context = FromStr::from_str(url).unwrap();
        assert_eq!(context, Context::GitUrl(GitUrl::new(url.to_string()).unwrap()));
        assert_eq!(context.to_string(), url);
    }
}

#[test]
fn context_may_contain_dir_paths() {
    let paths = vec!(".", "./foo", "./foo/bar/");
    for path in paths {
        let context: Context = FromStr::from_str(path).unwrap();
        assert_eq!(context, Context::Dir(Path::new(path).to_owned()));
        assert_eq!(context.to_string(), path);
    }
}
