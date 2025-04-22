/// Modeling inflation attack
use cvlr::prelude::*;

// -- number of shares reserved to protect the protocol
// -- set to 0 to remove protection
const RESERVED_SHARES: u64 = 10_000;
// -- minimal profit that Bob wants to make
// -- set to 0 if the goal is simply not to lose any funds
const MIN_PROFIT: u64 = 0;

#[rule]
pub fn inflate_profit() {


    //  -- initial state. Ratio is 1:1.
    let shares: u64 = nondet();
    let assets: u64 = nondet();
    cvlr_assume!(shares == assets);

    // -- Bob has some shares
    let bob_shares: u64 = nondet();
    // -- Bob has at most all the shares
    cvlr_assume!(shares >= bob_shares);
    // -- Limited by initial reserved shares for protection
    cvlr_assume!(shares.checked_sub(bob_shares).unwrap() >= RESERVED_SHARES);

    // -- Bob plans to donate this amount to create inflation
    let bob_donate: u64 = nondet();
    // -- Donation is non-zero. Otherwise Bob is honest and has no loss
    cvlr_assume!(bob_donate > 0);

    // -- Bob inflates assets, before Alice gets shares
    let assets0 = assets.checked_add(bob_donate).unwrap();
    let shares0 = shares;

    // -- Assets that alice wants to use to buy shares
    let alice_assets: u64 = nondet();

    // -- Shares that Alice bought
    let alice_shares: u64 = alice_assets
        .checked_mul(shares0)
        .unwrap()
        .checked_div(assets0)
        .unwrap();

    // update assets and shares
    let assets1 = assets0.checked_add(alice_assets).unwrap();
    let shares1 = shares0.checked_add(alice_shares).unwrap();

    // -- assets that Bob gets at the end
    let bob_assets: u64 = bob_shares
        .checked_mul(assets1)
        .unwrap()
        .checked_div(shares1)
        .unwrap();

    // update assets and shares to reflect current state
    let assets2 = assets1.checked_sub(bob_assets).unwrap();
    let shares2 = shares1.checked_sub(bob_shares).unwrap();

    // -- Bobs total assets used. Assume that price was 1:1, so `bob_shares`
    // -- were bought at face value
    let bob_pre_value = bob_shares.checked_add(bob_donate).unwrap();

    // -- After an attack, Alice redeems her shares
    // -- It does not matter how many shares Alice has, but how much they are
    // -- worth
    let alice_assets_post = alice_shares
        .checked_mul(assets2)
        .unwrap()
        .checked_div(shares2)
        .unwrap();

    let assets3 = assets2.checked_sub(alice_assets_post).unwrap();
    let shares3 = shares2.checked_sub(alice_shares);

    clog!(
        shares,
        assets,
        bob_shares,
        bob_donate,
        assets0,
        alice_assets,
        alice_shares,
        shares1,
        assets1,
        bob_assets,
        shares2,
        assets2,
        bob_pre_value,
        alice_assets_post,
        assets3,
        shares3,
    );

    // -- Alice wants to buy at 1:1, but might get less than she expects 
    cvlr_assert!(alice_shares <= alice_assets);

    // under this condition, Bob is not losing (but maybe not profiting either)
    if bob_assets >= bob_pre_value {
        // -- if Bob did attacked and came out ahead, alice did not lose more than 5 tokens
        // -- XXX find actual lower bound, it is likely to be 1
        // verified in 9m: https://prover.certora.com/output/175561/0b016925eab04d369f2c2848bcca13d2?anonymousKey=af8e6dccf566b3842e717ba2c9f46e023b34dc4f
        // -- comment out because this assertion takes significant time
        // cvlr_assert!(alice_assets_post.checked_add(5).unwrap() >= alice_assets);
    }

    // Assert that attack is NOT profitable for Bob and vault is protected
    // MIN_PROFIT controls the minimal profit: 0 means Bob did not lose
    // anything, but also did not gain
    // -- cex in few seconds. with MIN_PROFIT=1, verified in ~10s
    cvlr_assert_lt!(bob_assets, bob_pre_value.checked_add(MIN_PROFIT).unwrap());
}
