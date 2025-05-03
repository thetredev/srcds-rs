// Opaque type for Plugin
pub enum PluginC {}

// Import the C++ functions
extern "C" {
    pub fn get_plugin_instance() -> *mut PluginC;
    pub fn call_plugin_hello(plugin: *mut PluginC);
}

// Safe wrapper for Plugin
pub struct Plugin {
    ptr: *mut PluginC,
}

impl Plugin {
    // Get the global Plugin instance
    pub fn get_instance() -> Self {
        unsafe {
            Plugin {
                ptr: get_plugin_instance(),
            }
        }
    }

    // Call the hello method

    fn internal_hello(&self) {
        unsafe {
            call_plugin_hello(self.ptr);
        }
    }

    pub fn hello(&self, count: usize) {
        let mut handles = vec![];

        for i in 0..count {
            // Clone the Plugin reference for each thread
            let plugin = self.clone();

            let handle = std::thread::spawn(move || {
                println!("Thread {} starting work", i);
                // This is safe because we implemented Send and Sync
                plugin.internal_hello();

                println!("Thread {} finished work", i);
            });

            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

// You would need to implement Clone for Plugin
impl Clone for Plugin {
    fn clone(&self) -> Self {
        // This just copies the pointer, not the underlying C++ object
        Self { ptr: self.ptr }
    }
}

// Ensure Plugin is Send and Sync
unsafe impl Send for Plugin {}
unsafe impl Sync for Plugin {}
