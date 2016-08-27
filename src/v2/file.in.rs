// This is not a normal Rust module! It's included directly into v2.rs,
// possibly after build-time preprocessing.  See v2.rs for an explanation
// of how this works.

/// A `docker-compose.yml` file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    /// The individual services which make up this app.
    pub services: BTreeMap<String, Service>,
}

impl File {
    /// Read a file from an input stream containing YAML.
    pub fn read<R>(r: R) -> Result<Self, Error>
        where R: io::Read
    {
        Ok(try!(serde_yaml::from_reader(r)))
    }

    /// Write a file to an output stream as YAML.
    pub fn write<W>(&self, w: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        Ok(try!(serde_yaml::to_writer(w, self)))
    }

    /// Read a file from the specified path.
    pub fn read_from_path<P>(path: P) -> Result<Self, Error>
        where P: AsRef<Path>
    {
        Self::read(try!(fs::File::open(path)))
    }

    /// Write a file to the specified path.
    pub fn write_to_path<P>(&self, path: P) -> Result<(), Error>
        where P: AsRef<Path>
    {
        self.write(&mut try!(fs::File::create(path)))
    }
}

impl FromStr for File {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<File, Self::Err> {
        serde_yaml::from_str(&s)
    }
}

#[test]
fn file_can_be_converted_from_and_to_yaml() {
    let yaml = r#"---
"services":
  "foo":
    "build": "."
"#;

    assert_roundtrip!(File, yaml);

    let file: File = serde_yaml::from_str(&yaml).unwrap();
    let foo = file.services.get("foo").unwrap();
    assert_eq!(foo.build.as_ref().unwrap().context, Context::new("."));
}
