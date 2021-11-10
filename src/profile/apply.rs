use crate::profile::Profile;
use rhai::Engine;

impl Profile {
    pub fn apply(&self, engine: &Engine) {
        // check for setting conflicts
        if let Some((setting1, setting2, target)) = self.setting_conflict(None) {
            println!("failed to apply profile, found setting conflict!");
            println!(
                "settings {:?} and {:?} both set values for target {:?}",
                setting1.name(),
                setting2.name(),
                target
            );
            std::process::exit(1);
        }

        // go through each setting and apply it
        for setting in &self.settings {
            setting.apply(engine);
        }

        for setting in &self.settings {
            for hook in setting.hooks() {
                hook.run();
            }
        }

        for hook in &self.hooks {
            hook.run();
        }
    }
}
