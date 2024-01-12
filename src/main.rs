const USAGE: &'static str = "squishy: filter BAM files to remove doubly soft-clipped reads.

Usage: squishy [options] <src-bam> <dest-bam>
       squishy (-h | --help)

Options:
    -h --help               Show this help message.
    -v                      Produce verbose output.
    -w NUM                  Minimum clip width to consider [default: 25]
    -t NUM                  Maximum mapped length to consider [default: 50]
";

use std::{fs::File, time::Duration};

use docopt::Docopt;
use indicatif::ProgressBar;
use noodles::{
    bam,
    sam::{alignment::Record, record::cigar::op::Kind},
};

fn record_is_double_clipped(rec: &Record, w: usize) -> bool {
    let cigar = rec.cigar();
    let parts = Vec::from_iter(cigar.iter());
    let n = parts.len();
    if n > 2 && parts[0].kind() == Kind::SoftClip && parts[n - 1].kind() == Kind::SoftClip {
        return parts[0].len() >= w && parts[n - 1].len() >= w;
    }
    false
}

fn main() -> std::io::Result<()> {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    eprintln!("{:?}", args);
    let src = args.get_str("<src-bam>");
    let dst = args.get_str("<dest-bam>");
    let w: usize = args.get_str("-w").parse().unwrap();
    let t: i32 = args.get_str("-t").parse().unwrap();
    let verbose = args.get_bool("-v");

    let mut reader = bam::reader::Builder::default().build_from_path(src)?;
    let hdr = reader.read_header()?;

    let out = File::create(dst)?;
    let mut writer = bam::writer::Writer::new(out);
    writer.write_header(&hdr)?;

    let mut i: usize = 0;
    let mut j: usize = 0;

    let mpb = if verbose {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message("Calculating...");
        Some(pb)
    } else {
        None
    };

    for rec_res in reader.records(&hdr) {
        let rec = rec_res?;
        i += 1;
        if record_is_double_clipped(&rec, w) && rec.template_length().abs() <= t {
            j += 1;
            continue;
        }
        writer.write_record(&hdr, &rec)?;
        if let Some(pb) = &mpb {
            pb.tick();
            if (i & 0xffff) == 0 {
                pb.set_message(format!("after {} records, {} filtered", i, j));
            }
        }
    }
    if let Some(pb) = mpb {
        pb.finish_with_message("Done.");
    }

    Ok(())
}
