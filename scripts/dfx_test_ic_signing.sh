cd ./test_canister
dfx deploy test_canister
SIGN_RESULT=$(dfx canister call test_canister sign_and_check)

if [ "$SIGN_RESULT" != "()" ]; 
then
    echo "Unexpected result: $SIGN_RESULT"
    exit 1
else
    echo "Signing works correctly"
fi