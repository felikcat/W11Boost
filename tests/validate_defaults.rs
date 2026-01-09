#[cfg(test)]
mod integration_test
{
        use std::collections::HashSet;
        use w11boost::gui::tweaks::{defaults::RECOMMENDED_TWEAKS, get_all_tweaks};

        #[test]
        fn validate_no_duplicate_default_tweaks()
        {
                let mut seen = HashSet::new();
                let mut duplicates = Vec::new();

                for &id in RECOMMENDED_TWEAKS {
                        if !seen.insert(id) {
                                duplicates.push(id);
                        }
                }

                if !duplicates.is_empty() {
                        panic!("Found duplicate IDs in RECOMMENDED_TWEAKS: {:?}", duplicates);
                }
        }

        #[test]
        fn validate_default_tweaks_exist()
        {
                let all_tweaks = get_all_tweaks();
                let all_ids: HashSet<&str> = all_tweaks.iter().map(|t| t.id).collect();
                let mut missing = Vec::new();

                for &id in RECOMMENDED_TWEAKS {
                        if !all_ids.contains(id) {
                                missing.push(id);
                        }
                }

                if !missing.is_empty() {
                        panic!("Found default tweaks that are not defined in the code: {:?}", missing);
                }
        }
        #[test]
        fn test_no_duplicate_tweak_ids()
        {
                let mut ids = HashSet::new();
                let mut duplicates = Vec::new();

                for tweak in get_all_tweaks() {
                        if !ids.insert(tweak.id) {
                                duplicates.push(tweak.id);
                        }
                }

                if !duplicates.is_empty() {
                        panic!("Found duplicate tweak IDs in codebase: {:?}", duplicates);
                }
        }
}
