#!/bin/bash
echo "---------------------------------------------------"
echo "Building TargetContract and CallerContract."
fversion=$(forc --version)
echo "forc version = $fversion"
echo ""
echo "---------------------------------------------------"
echo ""
rm ./contracts/caller/Forc.lock
rm -r ./contracts/caller/out/
rm ./contracts/target/Forc.lock
rm -r ./contracts/target/out/

forc build --path ./contracts/target/
forc build --path ./contracts/caller/

echo ""
echo "Done!"

