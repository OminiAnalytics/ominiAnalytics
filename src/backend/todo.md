## TO-DO

- [❌] Add relations to some migrations
- [❌] Modify device indexer with hash->uuid[]
> Example : "SELECT unnest(col) as uid, count(*) as count FROM scientist group by uid order by count DESC limit(10);"