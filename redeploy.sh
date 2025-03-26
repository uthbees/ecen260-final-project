set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR

git pull

cd web-client
npm install
npm run build

cd ../web-server
cargo build --release

cd ..

rm -rf ../../public/iot-fan
rm -f ../../protected/iot-fan-server

cp -R web-client/dist ../../public/iot-fan
cp web-server/target/release/web-server ../../protected/iot_fan_server

echo "Successfully updated deployment."
