mkdir -p ./dist/RunIGV
mkdir -p ./dist/RunIGV/Resources
mkdir -p ./dist/RunIGV/Resources/simple-http-server
cp    ./target/release/run_igv          ./dist/RunIGV/run_igv
cp    ./resources/igvAppIcon.icns       ./dist/RunIGV/Resources/
cp -r ./resources/igv-webapp            ./dist/RunIGV/Resources/
cp    ./resources/*simple-http-server*  ./dist/RunIGV/Resources/simple-http-server
