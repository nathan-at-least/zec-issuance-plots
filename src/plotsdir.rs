use crate::PLOTS_DIR;
use std::path::Path;

pub fn refresh() -> std::io::Result<()> {
    refresh_dir(PLOTS_DIR)
}

fn refresh_dir<P>(dir: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let path = dir.as_ref();
    remove_dir_if_present(path)?;
    std::fs::create_dir_all(path)?;
    Ok(())
}

fn remove_dir_if_present<P>(dir: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    std::fs::remove_dir_all(dir).or_else(|err| {
        use std::io::ErrorKind::NotFound;
        match err.kind() {
            // It's ok if it didn't exist:
            NotFound => Ok(()),

            // Propagate other errors:
            _ => Err(err),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::refresh_dir;
    use std::path::PathBuf;

    fn prep_test_plots_dir(testname: &str) -> std::io::Result<PathBuf> {
        let pb = PathBuf::from("target")
            .join("test")
            .join(module_path!())
            .join(testname);
        refresh_dir(&pb)?;
        Ok(pb.join("plots"))
    }

    #[test]
    fn new_dir() -> std::io::Result<()> {
        let plotsdir = prep_test_plots_dir("new_dir")?;
        refresh_dir(&plotsdir)?;
        assert!(plotsdir.is_dir());
        Ok(())
    }

    #[test]
    fn existing_dir() -> std::io::Result<()> {
        let plotsdir = prep_test_plots_dir("existing_dir")?;
        std::fs::create_dir(&plotsdir)?;

        refresh_dir(&plotsdir)?;

        assert!(plotsdir.is_dir());
        Ok(())
    }

    #[test]
    fn propagated_error() -> std::io::Result<()> {
        let plotsdir = prep_test_plots_dir("propagated_error")?;

        // Write a file, which should cause rmdir to fail:
        std::fs::write(&plotsdir, &[])?;

        let r = refresh_dir(plotsdir);

        assert!(r.is_err());
        Ok(())
    }
}
