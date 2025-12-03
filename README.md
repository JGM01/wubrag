# Wubrag

index with
```bash
grpcurl -plaintext -d '{"path": "ABSOLUTE PATH TO SOME DIRECTORY" }' localhost:5001 wubrag.WubRAG/index
```

search with
```bash
grpcurl -plaintext -d '{"text": "SEARCH QUERY"}' localhost:5001 wubrag.WubRAG/search | jq -r '"Found \(.stringAmt) results:\n" + (.resultStrings | join("\n═══════════════════\n"))'
```
