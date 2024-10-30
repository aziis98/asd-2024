# Progetto ASD 2023/2024

Creazione e analisi di un Pangenome Graph.

## Obbiettivi / Features

-   [x] Caricare un Pangenome Graph dal formato GFA

-   [x] Classificare i nodi del grafo in base al tipo tree, back, forward, cross

-   [x] Rimuovere tutti i nodi di tipo back per rendere il grafo un DAG

-   [x] Restringere il grafo alla componente connessa più grande

-   [x] Ricostruire le sequenze dei nodi del grafo in base ai cammini ed alla
        direzione di percorrenza dei nodi (forward o reverse)

-   [x] Ricerca di un pattern k-mer in queste sequenze utilizzando il rolling
        hash

-   [ ] (WIP) (Opzionale) Calcolare le frequenze di occorrenza di tutti i k-mer
        presenti nel grafo

## CLI Options

-   `-i, --input <input>`: file to read

-   `-c, --path_count <path_count>`: number of paths to visit when searching for
    the pattern (default: 1)

-   `-p, --pattern <pattern>`: k-mer pattern to search (default: "ACGT")

-   `-k, --kmer_size <kmer_size>`: k-mer length (default: 4)

## Usage

-   To show help message:

    ```
    cargo run -- --help`
    ```

-   For example to try out the `chrX` dataset:

    ```bash
    GFA_URL='https://s3-us-west-2.amazonaws.com/human-pangenomics/pangenomes/freeze/freeze1/pggb/chroms/chrX.hprc-v1.0-pggb.gfa.gz'
    wget $GFA_URL -O dataset/chrX.hprc-v1.0-pggb.local.gfa.gz
    gunzip dataset/chrX.hprc-v1.0-pggb.local.gfa.gz
    cargo run --release -- -i dataset/chrX.hprc-v1.0-pggb.local.gfa -c 2 -p ACGT -k 3
    ```

    altri dataset sono elencati in [Note](#note)

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

## Stats

| Language  | Files  | Lines    | Code     | Comments | Blanks  |
| --------- | ------ | -------- | -------- | -------- | ------- |
| Rust      | 21     | 3606     | 2452     | 464      | 690     |
| Python    | 2      | 249      | 164      | 40       | 45      |
| TOML      | 8      | 110      | 99       | 2        | 9       |
| Markdown  | 4      | 131      | 0        | 82       | 34      |
| **Total** | **35** | **4096** | **2715** | **588**  | **778** |
