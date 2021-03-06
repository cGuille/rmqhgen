# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET

    # Test the generate subcommand:
    cross run --target $TARGET generate 'password'
    cross run --target $TARGET generate -a sha512 'password'
    cross run --target $TARGET generate --algorithm=sha256 'password'

    # Test the validate subcommand:
    cross run --target $TARGET validate --quiet '7HAPNn2PxK6dI6XaY3IDHELAjR/tac9pkgHaeQ0Txpvvk69D' 'password'
    cross run --target $TARGET validate --quiet -a sha256 '7HAPNn2PxK6dI6XaY3IDHELAjR/tac9pkgHaeQ0Txpvvk69D' 'password'
    cross run --target $TARGET validate --quiet -a md5 'cH4ItNINimvBXIJPvL7cU5ZSxr8=' 'password'
    cross run --target $TARGET validate --quiet --algorithm=sha512 'ATe3LFxFP+US2SFBZ9sLKpeSNS+/ywgrYPFFLCuK28/+Ioz+jQPUegAgjaqHcxxumWnIywgOl8rGwTWQnOq/7i6TIps=' 'password'
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
