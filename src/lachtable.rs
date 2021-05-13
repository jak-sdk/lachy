use notify::{DebouncedEvent, Watcher, RecommendedWatcher, RecursiveMode, Result, watcher};
use std::sync::mpsc::{Sender};
use std::time::Duration;

#[derive(Debug)]
pub struct UnloadedTable {
    path: std::path::PathBuf,
}

impl UnloadedTable {
    pub fn load (&self) -> Table {
        let rule = Rule {name : Some(String::from("test")), 
               path : String::from("/tmp/foo"), 
               command : String::from("echo foo"), 
               notify : String::from("IN_MODIFY"),
               watcher: None,
              };
        let rule2 = Rule {name : Some(String::from("test")), 
               path : String::from("/tmp/foodir/"), 
               command : String::from("echo foo"), 
               notify : String::from("IN_MODIFY"),
               watcher: None,
              };
        Table {name : String::from("test table"),
               rules: vec![rule, rule2]
              }
    }
}

struct WrapRecommendedWatcher {
   inner : RecommendedWatcher
}

impl std::fmt::Debug for WrapRecommendedWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Watcher")
         //.field("", &self.x)
         //.field("y", &self.y)
         .finish()
    }
}

#[derive(Debug)]
struct Rule {
    name: Option<String>,
    path: String,
    command: String,
    notify: String,
    watcher: Option<WrapRecommendedWatcher>,
}

impl Rule {
    fn watch(&mut self, tx : Sender<DebouncedEvent>) -> Result<()> {
        let mut watcher = watcher(tx, Duration::from_secs(2))?;
        watcher.watch(self.path.clone(), RecursiveMode::Recursive)?; 

        self.watcher = Some(WrapRecommendedWatcher { inner : watcher });

        Ok(())
    }
}


#[derive(Debug)]
pub struct Table {
    name: String,
    rules: Vec<Rule>,
}

impl Table {
    pub fn new(pb: std::path::PathBuf) -> UnloadedTable {
        UnloadedTable { path : pb }
    }

    pub fn watch(&mut self, tx : Sender<DebouncedEvent>) -> () {
        self.name = String::from("watched");
        for rule in &mut self.rules {
            rule.watch(tx.clone());
        }
    }
}
