#!/bin/bash

long_value=$(printf 'A%.0s' {1..4097})
cat <<EOL > config/long_value_test.conf
endpoint = localhost:3000
# debug = true
log.file = /var/log/console.log
log.name = default.log
value.too.long = $long_value
EOL
echo "create long_value_test.conf🖊️✨"
echo
