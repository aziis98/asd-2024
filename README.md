# Progetto ASD 2023/2024

## Usage

-   `cargo run -- --help`

    Display help message.

-   `cargo run -- show -i ./dataset/example.gfa`

    Parses the GFA file and prints the graph and various information.

-   Visualize the graph using EGUI, go to `examples/configurable` and run:

    `cargo run -- ../../dataset/example.gfa`

    `cargo run -- ../../dataset/DRB1-3123_unsorted.gfa`

-   Running the project with all the flags:

    `cargo run --release -- show -i dataset/chrX.hprc-v1.0-pggb.local.gfa -c 2 -p ACGT -k 3`

## Example GFA

```
H	VN:Z:1.0
S	11	G
S	12	A
S	13	T
S	14	T
S	15	A
S	16	C
S	17	A
S	21	G
S	22	A
S	23	T
S	24	T
S	25	A
L	11	+	12	+	*
L	11	+	13	+	*
L	12	+	14	+	*
L	13	+	14	+	*
L	14	+	15	+	*
L	14	+	16	+	*
L	15	+	17	+	*
L	16	+	17	+	*
L	21	+	22	+	*
L	21	+	23	+	*
L	22	+	24	+	*
L	23	+	24	-	*
L	24	+	25	+	*
P	A	11+,12+,14+,15+,17+	*,*,*,*
P	B	21+,22+,24+,25+	*,*,*
W	sample	1	A	0	5	>11>12>14>15>17
W	sample	2	A	0	5	>11>13>14>16>17
W	sample	1	B	0	5	>21>22>24<23<21
W	sample	2	B	0	4	>21>22>24>25
```

## Note

-   Documentazione del formato GFA: http://gfa-spec.github.io/GFA-spec/GFA1.html

-   Data set:

    -   [example.gfa](https://github.com/jltsiren/gbwt-rs/blob/main/test-data/example.gfa)

    -   [drb1.gfa](https://github.com/pangenome/odgi/blob/master/test/DRB1-3123_unsorted.gfa)

    -   [chrY.gfa](https://s3-us-west-2.amazonaws.com/human-pangenomics/pangenomes/freeze/freeze1/pggb/chroms/chrY.hprc-v1.0-pggb.gfa.gz)

    -   [chrX.gfa](https://s3-us-west-2.amazonaws.com/human-pangenomics/pangenomes/freeze/freeze1/pggb/chroms/chrX.hprc-v1.0-pggb.gfa.gz)
