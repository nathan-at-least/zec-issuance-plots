use crate::idealtime::DateTime;
use crate::DataSet;
use std::collections::BTreeSet;

pub(super) fn write(stem: &str, datasets: &[DataSet<DateTime, f32>]) -> std::io::Result<()> {
    use std::io::Write;

    let path = format!("plots/{}.csv", stem);
    println!("Writing {:?}...", &path);
    let mut f = std::fs::File::create(path)?;

    write!(f, "time")?;
    for colname in datasets.iter().map(|ds| ds.name) {
        write!(f, ",{}", colname)?;
    }
    writeln!(f)?;

    let pts: BTreeSet<PtInner> = datasets
        .iter()
        .enumerate()
        .flat_map(|(col, ds)| ds.points.iter().map(move |(t, v)| PtInner(*t, col, *v)))
        .collect();

    let mut optrow: Option<(DateTime, Vec<f32>)> = None;
    for PtInner(t, col, v) in pts {
        if let Some((rowt, vs)) = optrow.take() {
            assert!(t >= rowt);

            if t > rowt {
                write!(f, "{}", rowt)?;
                for rowv in vs {
                    write!(f, ",{}", rowv)?;
                }
                writeln!(f)?;

                optrow = Some((t, vec![0f32; datasets.len()]));
            }
        } else {
            optrow = Some((t, vec![0f32; datasets.len()]));
        }

        let (t, mut vs) = optrow.take().unwrap();
        vs[col] = v;
        optrow = Some((t, vs));
    }

    if let Some((rowt, vs)) = optrow.take() {
        write!(f, "{}", rowt)?;
        for rowv in vs {
            write!(f, ",{}", rowv)?;
        }
        writeln!(f)?;
    }

    Ok(())
}

struct PtInner(DateTime, usize, f32);

impl std::cmp::Ord for PtInner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0, self.1).cmp(&(other.0, other.1))
    }
}

impl std::cmp::PartialOrd for PtInner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for PtInner {}

impl std::cmp::PartialEq for PtInner {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
