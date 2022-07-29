use crate::PLOTS_DIR;

pub fn refresh() -> std::io::Result<()> {
    std::fs::remove_dir_all(PLOTS_DIR).or_else(|err| {
        use std::io::ErrorKind::NotFound;
        match err.kind() {
            // It's ok if it didn't exist:
            NotFound => Ok(()),

            // Propagate other errors:
            other => Err(other),
        }
    })?;

    std::fs::create_dir_all(PLOTS_DIR)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{plotsdir, PLOTS_DIR};
    use testdir::testdir;

    #[test]
    fn new_dir() -> std::io::Result<()> {
        let d = testdir!();
        std::env::set_current_dir(&d)?;
        plotsdir::refresh()?;
        assert!(d.join(PLOTS_DIR).is_dir());
        Ok(())
    }

    #[test]
    fn existing_dir() -> std::io::Result<()> {
        let d = testdir!();
        std::env::set_current_dir(&d)?;

        let plots = d.join(PLOTS_DIR);
        dbg!(&plots);
        std::fs::create_dir(&plots)?;

        plotsdir::refresh()?;

        assert!(plots.is_dir());
        Ok(())
    }

    #[test]
    fn propagated_error() -> std::io::Result<()> {
        let d = testdir!();
        std::env::set_current_dir(&d)?;

        let plots = d.join(PLOTS_DIR);
        // Write a file, which should cause rmdir to fail:
        std::fs::write(&plots, &[])?;

        let r = plotsdir::refresh();

        assert!(r.is_err());
        Ok(())
    }
}
