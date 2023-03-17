/// `KvStore` is a  is HashMap backed.
///
/// it stores value in memory and are not persisted to disk
///
/// ```rust
/// use kvs::KvStore;
///
/// let mut kv = KvStore::new();
/// kv.set("alpha".to_owned(), "beta".to_owned());
///
/// let value = kv.get("alpha".to_owned());
/// assert_eq!(value, Some("beta".to_owned()));
///
/// let value = kv.get("nothing".to_owned());
/// assert_eq!(value, None);
/// ```
///
///
use failure::Fail;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Operation {
    Set(String, String),
    Rm(String),
}

#[derive(Fail, Debug)]
pub enum Errors {
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "Unable to read log file")]
    LogCorrupted,
}

pub type Result<X> = std::result::Result<X, Errors>;
#[derive(Debug)]
pub struct KvStore<'a> {
    log: Vec<Operation>,
    dir: &'a Path,
}

impl<'a> KvStore<'a> {
    pub fn open(path: &'a Path) -> Result<Self> {
        Ok(KvStore {
            log: vec![],
            dir: path,
        })
    }

    fn refill_log(&mut self) -> Result<()> {
        let path = self.dir.join("kvs.log");
        if path.exists() {
            let fh = std::io::BufReader::new(
                std::fs::File::open(path).map_err(|_| Errors::LogCorrupted)?,
            );
            let logs = vec![];
            let mut logs = fh.lines().try_fold(logs, |mut logs, line| match line {
                Ok(line) => {
                    logs.push(serde_json::from_str(&line).map_err(|_| Errors::LogCorrupted)?);
                    Ok(logs)
                }
                _ => Err(Errors::LogCorrupted),
            })?;
            self.log.clear();
            self.log.append(&mut logs);
        };
        Ok(())
    }

    /// set a key to a value in store
    ///
    /// any pre-existing values for the key will be overwritten
    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        //         self.refill_log()?;
        let op = Operation::Set(k, v);
        self.log.push(op);
        Ok(())
    }

    /// return the value of a key from the strore
    ///
    /// returns `None` if key is not present
    pub fn get(&mut self, k: String) -> Result<Option<String>> {
        self.refill_log()?;
        let opr: Vec<&Operation> = self
            .log
            .iter()
            .filter(|&opr| match opr {
                Operation::Set(kk, _) => *kk == k,
                Operation::Rm(kk) => *kk == k,
            })
            .collect();
        match opr.last() {
            Some(Operation::Set(_, val)) => Ok(Some(val.clone())),
            Some(Operation::Rm(..)) => Ok(None),
            None => Ok(None),
        }
    }

    /// Remove the given key from the store
    pub fn remove(&mut self, k: String) -> Result<()> {
        if let Ok(Some(_)) = self.get(k.clone()) {
            self.log.push(Operation::Rm(k));
            Ok(())
        } else {
            Err(Errors::KeyNotFound)
        }
    }
}

impl<'a> Drop for KvStore<'a> {
    fn drop(&mut self) {
        let file = std::fs::File::options()
            .append(true)
            .create(true)
            .open(self.dir.join("kvs.log"))
            .unwrap();
        let mut fh = std::io::BufWriter::new(file);

        let _ = self.log.iter().for_each(|log| {
            fh.write(serde_json::to_string(log).unwrap().as_bytes());
            fh.write("\n".as_bytes());
        });
    }
}
