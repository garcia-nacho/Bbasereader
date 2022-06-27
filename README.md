## Bbasereader
[![Linux](https://svgshare.com/i/Zhy.svg)](https://svgshare.com/i/Zhy.svg)   [![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)   
      
Retrieve all bases aligned to particular positions of sorted bam files.
bbasereader provides read id, base and position.

# Installing Bbasereader:   
Compile the binary from the source and copy the binary to your favourite location.    
See [here](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html) about how to compile Rust source code.   

# Running Bbasereader:   
<code>bbasereader -p Position-of-interest -f Bam.file.bam > output.tsv</code>   
Where -p is the position you want to retrieve (with regards to the reference) and -f is the indexed bam file you want to explore.   
   
# Output
You will get a tsv with 2 columns, the read-id and the base it contains on the position of interest.
