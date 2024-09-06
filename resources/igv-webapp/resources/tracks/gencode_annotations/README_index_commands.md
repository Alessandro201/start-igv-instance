# How to make the index of gtf files

The first step is to download the coprehensive gene annotation from [GENCODE](https://www.gencodegenes.org/human/), currently the latest version is v46.

```bash
wget https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_46/gencode.v46.chr_patch_hapl_scaff.annotation.gtf.gz
```

Then sort the file and rezip it using samtool bgzip to create a compressed file compatible with gzip but that can be used to create an index. And then use samtools tabix to create the index

```bash
gzcat gencode.v46.chr_patch_hapl_scaff.annotation.gtf.gz | sort -k1,1 -k4,4n | bgzip -c > gencode.v46.chr_patch_hapl_scaff.annotation.gtf.sorted.gz
tabix -p ./gencode.v46.chr_patch_hapl_scaff.annotation.sorted.gtf.gz
```

If IGV says "offset is outside the bound of the DataView" it may be because the tabix tbi index can handle up to 2^29 bases. If you have longer positions you must use the `csi` index with the `-C` flag. For mode details check the [documentation](https://www.htslib.org/doc/tabix.html)
