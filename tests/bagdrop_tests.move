#[test_only]
module bagdrop::bagdrop_tests {
    use bagdrop::bagdrop as bg;
    use sui::test_scenario as ts;

    const ENotImplemented: u64 = 0;

    const USER: address = @0x5354085bc8a8d3f96383483a9ba42410476af916d42ff5dd4f05bad55608f2ce;

    #[test]
    fun test_bagdrop() {
        let mut test = ts::begin(USER);

        bg::test_proof(
            x"7b5fb9efc66b648b6c578cd2caaed1ec04ff8d2dda9dfee2a063f231d67faa0e6c50a714487e389da14053009cd82b9bf83e1da7fa3f1641c5b044af31b0572209f4f72fafbc8eebd54129753c0d596082ce209f114c4e176b45077d2095470dbacf044fd3761b1c431f669ea19de49e47ccab38aa051efa77cf0c0f35ff4a1f3d1b7d2bd2bfe50fa46218c014a05a2353100f913681ec0abb7a347a3453e5a4cce0ab3107ca43446fa46820f6848637495672578c69a7cd6bf6c512dc77702a5ed628803908f8c4d177cdaca6ea5431ddde4644c5095330e4efc7e91506a80f0300000000000000077da5446a45251e3d2a2c50533a383acba970b52395f0e9657b0b6721e0c3aefff6edcf9771d14dcc0f68569c4d4454ab90b992b0e7a81226fde4e9ee0c9c91840759366c9d1c5dbfeb7c354db90a67537663e83a781dfe16fc116165258810",
            x"8c4a6e8ec2e12595f2f7365a1c2f4dd17c7984b6ec9dd52098bc990f89f4ed960f0c7640e5f13fa42828797d4af1b1f58354244990267bb1b5d4af87be2185271123984edda5e1c06aa50201e7db52e87b739da4126021d95132fdb738a7c013fef73a21ff85785dd70980a018c369792fe2832579c73d4012ada9509c79f113",
            x"86059120aed4929dfef2c7f59974efe2701a882975bc1c4d290fca169fe2c125",
            test.ctx(),
        );

        test.end();
    }

    #[test, expected_failure(abort_code = ::bagdrop::bagdrop_tests::ENotImplemented)]
    fun test_bagdrop_fail() {
        abort ENotImplemented
    }
}
