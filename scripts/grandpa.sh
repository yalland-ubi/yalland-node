curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "babe",
      "zero crop neutral that garage notable trim always client dry current veteran//1//babe",
      "0x5cd39f695f92f114d33f731283968eb5627d82b20cf9b2357798dbdb271bd01a"
    ]
  }'

curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "gran",
      "zero crop neutral that garage notable trim always client dry current veteran//1//grandpa",
      "0xf3e9d049bf974441e6603e28d216b6f7b01addc12a987224f09f62ccd2df4c9f"
    ]
  }'

curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "babe",
      "zero crop neutral that garage notable trim always client dry current veteran//2//babe",
      "0x206ff0c892b4b1faa0e4051476a997e9b31a28957aff53281a52f43967cf200f"
    ]
  }'

curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "gran",
      "zero crop neutral that garage notable trim always client dry current veteran//2//grandpa",
      "0xa9bc13a350d35a666ab5007f09a90bc2e7f4bcd58b0be8208a03347e46c9395d"
    ]
  }'
curl http://localhost:9935 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "babe",
      "zero crop neutral that garage notable trim always client dry current veteran//3//babe",
      "0x5ac8b49bc092ecf7fe6d5ba8d44af9bfd15d93c2c99cbba5a98caad2f661642e"
    ]
  }'

curl http://localhost:9935 -H "Content-Type:application/json;charset=utf-8" -d \
  '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
      "gran",
      "zero crop neutral that garage notable trim always client dry current veteran//3//grandpa",
      "0x91795288ab550bbb972627fff63567fc1d1d6f1ecfbcec33afe64560186418c4"
    ]
  }'
