#!/bin/bash
# trap 'kill %1' SIGINT
 
# sh watch_app.sh | sed -e 's/^/[App] /' &
# sh watch_libs.sh | sed -e 's/^/[Libs] /'

sh -c "sh watch_app.sh | sed -e 's/^/[App] /' & sh watch_libs.sh | sed -e 's/^/[Libs] /' & wait"
# Press Ctrl+C to kill them all