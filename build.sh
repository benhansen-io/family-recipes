rm -rf public
mkdir public
cp -r recipes public/recipes
cp site/* public
pushd indexer
cargo run ../public/recipes ../public/index.json
