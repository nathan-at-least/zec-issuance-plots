use crate::idealtime::DateTime;
use crate::{DataSet, PLOTS_DIR};

pub(super) fn write(stem: &str, datasets: &[DataSet<DateTime, f64>]) -> std::io::Result<()> {
    use std::io::Write;

    let path = format!("{}/{}.csv", PLOTS_DIR, stem);
    println!("Writing {:?}...", &path);
    let mut f = std::fs::File::create(path)?;

    write!(f, "time")?;
    for colname in datasets.iter().map(|ds| ds.name) {
        write!(f, ",{}", colname)?;
    }
    writeln!(f)?;

    // We want to write one row for each time, but the datasets may have partially overlapping time
    // axes.
    //
    // 1. Assume datasets are ordered chronologically.
    // 2. Track the next point in each data set in `nexts`.
    // 3. While there are any next points:
    // 3.a Find the earliest time of all next points.
    // 3.b Write a row for that time, leaving any column which does not have a value at that time
    //   empty. While doing so, advance each column that has a point to the next point in `nexts`.

    // Step 1: track peekable iterators for each dataset assuming they are chronological:
    let mut iters: Vec<_> = datasets
        .iter()
        .map(|ds| ds.points.iter().peekable())
        .collect();

    let mut prevtime = None;

    // Step 3: while any next point exists, calculate the minimum time:
    while let Some(nexttime) = iters
        .iter_mut()
        .filter_map(|pts| pts.peek().map(|(t, _)| t))
        .min()
    {
        if let Some(ptime) = prevtime {
            // Ensure the algorithm makes progress:
            assert!(nexttime > ptime);
        }
        prevtime = Some(nexttime);

        write!(f, "{}", nexttime)?;

        for pts in iters.iter_mut() {
            if let Some((coltime, v)) = pts.peek() {
                assert!(
                    coltime >= nexttime,
                    "{:?}, {:?}, {:?}",
                    nexttime,
                    coltime,
                    v
                );
                if coltime == nexttime {
                    write!(f, ",{}", v)?;
                    pts.next(); // Advance this column.
                } else {
                    // Empty cell:
                    write!(f, ",")?;
                }
            }
        }
        writeln!(f)?;
    }

    Ok(())
}
