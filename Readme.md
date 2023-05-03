# actix-sse-gzip
## Description
Show's the sse throughput reduction when used the compression gzip

## Hardware
Mac M1 10core

## Results for 30s

curl uncompressed
```shell
timeout 30 curl -i http://localhost:9090/sse > out_uncompressed.txt
```
curl compressed
```shell
timeout 30 curl --compressed -i http://localhost:9090/sse -H "accept-encoding: gzip "> out_compressed.txt
```

get message amount for 30s
```shell
cat [file] | grep data: | wc -l
```

Values in messages per second 

| build type | uncompressed | compressed | reduction in percent |
|:-----------|-------------:|-----------:|---------------------:|
| debug      |       358720 |      36248 |                      |
| release    |       899699 |     627421 |               -30,26 |


## Summary
Using the compression slow down the throughput dramatically. While the
uncompressed throughput is perfect.

