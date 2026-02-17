//! Exercise 28: Type Erasure with Box - Dynamic dispatch patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use Box for type erasure
//! - Implement heterogeneous collections
//! - Build plugin systems with trait objects

pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

pub struct UppercasePlugin;

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str  {
        todo!("Implement name")
    }

    fn execute(&self, input: &str) -> String  {
        todo!("Implement execute")
    }
}

pub struct ReversePlugin;

impl Plugin for ReversePlugin {
    fn name(&self) -> &str  {
        todo!("Implement name")
    }

    fn execute(&self, input: &str) -> String  {
        todo!("Implement execute")
    }
}

pub struct RepeatPlugin {
    times: usize,
}

impl RepeatPlugin {
    pub fn new(times: usize) -> Self  {
        todo!("Implement new")
    }
}

impl Plugin for RepeatPlugin {
    fn name(&self) -> &str  {
        todo!("Implement name")
    }

    fn execute(&self, input: &str) -> String  {
        todo!("Implement execute")
    }
}

/// A plugin manager that holds boxed plugins.
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self  {
        todo!("A plugin manager that holds boxed plugins.")
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>)  {
        todo!("Implement register")
    }

    pub fn execute(&self, plugin_name: &str, input: &str) -> Option<String>  {
        todo!("Implement execute")
    }

    pub fn execute_all(&self, input: &str) -> Vec<(String, String)>  {
        todo!("Implement execute_all")
    }

    pub fn plugin_count(&self) -> usize  {
        todo!("Implement plugin_count")
    }

    pub fn list_plugins(&self) -> Vec<String>  {
        todo!("Implement list_plugins")
    }
}

/// A type-erased function wrapper.
pub struct BoxedFunction {
    func: Box<dyn Fn(i32) -> i32>,
}

impl BoxedFunction {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(i32) -> i32 + 'static,
     {
        todo!("Implement new")
    }

    pub fn call(&self, input: i32) -> i32  {
        todo!("Implement call")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_plugin() {
        let plugin = UppercasePlugin;
        assert_eq!(plugin.name(), "uppercase");
        assert_eq!(plugin.execute("hello"), "HELLO");
    }

    #[test]
    fn test_reverse_plugin() {
        let plugin = ReversePlugin;
        assert_eq!(plugin.name(), "reverse");
        assert_eq!(plugin.execute("hello"), "olleh");
    }

    #[test]
    fn test_repeat_plugin() {
        let plugin = RepeatPlugin::new(3);
        assert_eq!(plugin.name(), "repeat");
        assert_eq!(plugin.execute("ab"), "ababab");
    }

    #[test]
    fn test_plugin_manager() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(UppercasePlugin));
        manager.register(Box::new(ReversePlugin));

        assert_eq!(manager.plugin_count(), 2);

        assert_eq!(
            manager.execute("uppercase", "test"),
            Some("TEST".to_string())
        );
        assert_eq!(
            manager.execute("reverse", "test"),
            Some("tset".to_string())
        );
        assert_eq!(manager.execute("unknown", "test"), None);
    }

    #[test]
    fn test_execute_all() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(UppercasePlugin));
        manager.register(Box::new(ReversePlugin));

        let results = manager.execute_all("hi");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("uppercase".to_string(), "HI".to_string())));
        assert!(results.contains(&("reverse".to_string(), "ih".to_string())));
    }

    #[test]
    fn test_list_plugins() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(UppercasePlugin));
        manager.register(Box::new(RepeatPlugin::new(2)));

        let list = manager.list_plugins();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"uppercase".to_string()));
        assert!(list.contains(&"repeat".to_string()));
    }

    #[test]
    fn test_boxed_function() {
        let double = BoxedFunction::new(|x| x * 2);
        assert_eq!(double.call(5), 10);
        assert_eq!(double.call(21), 42);
    }

    #[test]
    fn test_boxed_function_closure() {
        let multiplier = 3;
        let multiply = BoxedFunction::new(move |x| x * multiplier);
        assert_eq!(multiply.call(4), 12);
    }

    #[test]
    fn test_heterogeneous_plugins() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(UppercasePlugin));
        manager.register(Box::new(ReversePlugin));
        manager.register(Box::new(RepeatPlugin::new(2)));

        assert_eq!(manager.plugin_count(), 3);

        // All different types stored in same collection
        assert!(manager.execute("uppercase", "a").is_some());
        assert!(manager.execute("reverse", "a").is_some());
        assert!(manager.execute("repeat", "a").is_some());
    }
}
