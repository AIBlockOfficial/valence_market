# 💸 weaver_market

`weaver_market` is a plug-in for `weaver` that lets you set up a web3 marketplace in seconds. Let your users buy and sell assets in a distributed marketplace, where the only fees are the ones you set, and your users are in control of their own trades.

## TODOs

- [ ] Paginate orders
- [ ] Add cache and cuckoo filter functionality
- [ ] Separate ID from Listing and Order structs (create MongoDB wrapper struct with ID)
- [ ] Construct initial order when new listing is created (does this form part of the listing POST call?)
- [ ] Add tests
- [ ] Add logging
- [ ] Refactor and improve error messages for call failures
- [ ] Create user functionality
- [ ] With user functionality enabled, add user ID to listing and order structs
- [ ] With user functionality enabled, add PUT/DELETE calls for listings and orders