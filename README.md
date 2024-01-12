# Squishy

Squishy is a simple tool for filtering double soft-clipped reads from BAM files.

## Why?

With the reads coming from common short read sequencing (i.e. NGS) from human samples,
we find that a modest number of mapping arise that have soft clips on both ends. These
often arise from contamination, or other technical issues with samples, and can be
problematic for correctly identifying insertion, duplication and translocation sites,
as most methods for identifying these features rely on soft-clipped read mappings as
starting points for detection.

While there are circumstances where these mappings can be true, and valuable, for
most purposes, they merely confuse, and so we provide this filter to remove them.
