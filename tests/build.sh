# Build all smart contract cargo projects structured two folder levels below this file. 

for dir in */ ; do
    cd ./$dir
    for subdir in */ ; do
        cd ./$subdir
        cargo build --target thumbv6m-none-eabi
        cargo build --target thumbv6m-none-eabi --release
        cd ../
    done
    cd ../
done
