# Known issues

The `twitter-stream-2020-12-09.zip` archive is corrupt, but at least 948 entries can be recovered.

The contents of `archiveteam-twitter-stream-2013-12-b.tar` are a subset of `archiveteam-twitter-stream-2013-12.tar`,
so there is no need to download or access the `12-b` archive.

There are 16 entries in three archive files that have invalid extensions and seem to be download artifacts.
All but one of these are exact copies of entries in the same archive with valid names.
The one entry that does not have an exact copy is a corrupted bzip2 archive, and attempting to recover
its contents suggests that it is a subset of a valid file.

```csv
Archive file,Duplicate entry,Valid entry,Size
archiveteam-twitter-stream-2013-04.tar,04/01/00/.09.json.bz2.J1uA1l,04/01/00/09.json.bz2,847641
archiveteam-twitter-stream-2013-04.tar,04/01/01/.36.json.bz2.VbagCN,04/01/01/36.json.bz2,692844
archiveteam-twitter-stream-2013-04.tar,04/01/07/.36.json.bz2.tyJLBj,,1310720
twitter-stream-2017-11-11.tar,2017/11/11/17/.05.json.bz2.PTrPs6,2017/11/11/17/05.json.bz2,1193404
twitter-stream-2017-11-11.tar,2017/11/11/17/.05.json.bz2.aCV9l3,2017/11/11/17/05.json.bz2,1193404
twitter-stream-2017-11-11.tar,2017/11/11/17/.10.json.bz2.dDyEke,2017/11/11/17/10.json.bz2,1169103
twitter-stream-2017-11-11.tar,2017/11/11/17/.51.json.bz2.4xKmdM,2017/11/11/17/51.json.bz2,1279707
twitter-stream-2017-11-11.tar,2017/11/11/17/.nfs00000000050801fe00002afb,2017/11/11/17/05.json.bz2,1193404
twitter-stream-2017-11-11.tar,2017/11/11/19/.08.json.bz2.ciq7xl,2017/11/11/19/08.json.bz2,1235430
twitter-stream-2017-11-11.tar,2017/11/11/19/.51.json.bz2.bEkeVi,2017/11/11/19/51.json.bz2,1269504
twitter-stream-2017-11-11.tar,2017/11/11/23/.20.json.bz2.7QLEpW,2017/11/11/23/20.json.bz2,1021649
twitter-stream-2017-11-11.tar,2017/11/11/23/.25.json.bz2.8wJLNz,2017/11/11/23/25.json.bz2,976495
twitter-stream-2017-11-12.tar,2017/11/12/04/.21.json.bz2.27oMIg,2017/11/12/04/21.json.bz2,1324180
twitter-stream-2017-11-12.tar,2017/11/12/04/.21.json.bz2.5igBTf,2017/11/12/04/21.json.bz2,1324180
twitter-stream-2017-11-12.tar,2017/11/12/04/.22.json.bz2.OQvwhc,2017/11/12/04/22.json.bz2,1328067
twitter-stream-2017-11-12.tar,2017/11/12/04/.22.json.bz2.bQcEGU,2017/11/12/04/22.json.bz2,1328067
```

There are 13 duplicate entries in `archiveteam-twitter-stream-2013-04.tar`.
The entry path format for the archive file includes the month, but the invalid duplicate entries omit that directory.

```csv
Archive file,Duplicate entry,Valid entry,Size
archiveteam-twitter-stream-2013-04.tar,04/00/00.json.bz2,04/04/00/00.json.bz2,911843
archiveteam-twitter-stream-2013-04.tar,04/00/01.json.bz2,04/04/00/01.json.bz2,844149
archiveteam-twitter-stream-2013-04.tar,04/00/02.json.bz2,04/04/00/02.json.bz2,801118
archiveteam-twitter-stream-2013-04.tar,04/00/03.json.bz2,04/04/00/03.json.bz2,808784
archiveteam-twitter-stream-2013-04.tar,04/00/04.json.bz2,04/04/00/04.json.bz2,825389
archiveteam-twitter-stream-2013-04.tar,04/00/05.json.bz2,04/04/00/05.json.bz2,825315
archiveteam-twitter-stream-2013-04.tar,04/00/06.json.bz2,04/04/00/06.json.bz2,782645
archiveteam-twitter-stream-2013-04.tar,04/00/07.json.bz2,04/04/00/07.json.bz2,767995
archiveteam-twitter-stream-2013-04.tar,04/00/08.json.bz2,04/04/00/08.json.bz2,777639
archiveteam-twitter-stream-2013-04.tar,04/00/09.json.bz2,04/04/00/09.json.bz2,805213
archiveteam-twitter-stream-2013-04.tar,04/00/10.json.bz2,04/04/00/10.json.bz2,815588
archiveteam-twitter-stream-2013-04.tar,04/00/11.json.bz2,04/04/00/11.json.bz2,779445
archiveteam-twitter-stream-2013-04.tar,04/00/12.json.bz2,04/04/00/12.json.bz2,799660
```
