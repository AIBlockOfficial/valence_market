<div id="top"></div>

<!-- PROJECT LOGO -->
<br />

<div align="center">
    <div style="height: 50px; width: 100%"></div>

  <a>
    <img src="https://github.com/ABlockOfficial/weaver_market/blob/main/assets/hero.svg" alt="Logo" width="320px">
  </a>

  <div style="height: 50px; width: 100%"></div>

  <h3>weaver_market</h3>

  <!-- <div>
  <img src="https://img.shields.io/github/actions/workflow/status/Zenotta/Intercom/codeql-analysis.yml?branch=main" alt="Pipeline Status" />
    <img src="https://img.shields.io/github/package-json/v/Zenotta/Intercom" />
  </div> -->

  <p align="center">
    <code>weaver_market</code> is a plug-in for <a href="https://github.com/ABlockOfficial/Weaver">Weaver</a> that lets you set up a web3 marketplace in seconds. Let your users buy and sell assets in a distributed marketplace, where the only fees are the ones you set, and your users are in control of their own trades.
    <br />
    <br />
    <a href="https://a-block.io"><strong>Official documentation »</strong></a>
    <br />
    <br />
  </p>
</div>

<!-- GETTING STARTED -->

## How to Use

`weaver_market` provides routes that you can plug in to your base Weaver node to turn it into a fully functional marketplace. It is designed to be used as a library and can be imported into your Weaver project.

..

### 🔧 Installation

If you have `cargo-add` installed, you can simply run the following command:

```sh
cargo add weaver_market
```

Otherwise, add the following to your `Cargo.toml` file:

```toml
[dependencies]
weaver_market = "0.1.0"
```

<p align="left">(<a href="#top">back to top</a>)</p>

..

### 🔌 Available Routes

#### **<img src="https://img.shields.io/badge/GET-2176FF" alt="GET"/> `/listings`**
Retrieve a list of available assets that users can browse and potentially buy

..

#### **<img src="https://img.shields.io/badge/GET-2176FF" alt="GET"/> `/listings/:id`**
Retrieve a specific listing by its ID

..

#### **<img src="https://img.shields.io/badge/POST-07BEB8" alt="POST"/> `/listings`**
Create a new listing. The structure for the request body will need to follow the `Listing` interface, which looks like:

```json
{
    "_id": "a8f163782fb07c69f511248e",
    "title": "Asset_test",
    "description": "This is a test asset listing",
    "initial_price": 100,
    "quantity": 10
}
```

..

#### **<img src="https://img.shields.io/badge/GET-2176FF" alt="GET"/> `/orders/:id`**
Retrieve a list of orders that have been placed, retrieved by the listing ID

..

#### **<img src="https://img.shields.io/badge/POST-07BEB8" alt="POST"/> `/orders`**
Create a new order. The structure for the request body will need to follow the `Order` interface, which looks like:

```json
{
    "id": "8c6dbdaea24a234fad18eca6",
    "asset_id": "f837cb510db38d9040889e83",
    "price": 100,
    "quantity": 2,
    "is_bid": false,
    "created_at": "20 June 2023",
    "druid": "g092384435098",
    "desired_asset_id": null
}
```

<p align="left">(<a href="#top">back to top</a>)</p>

..

### 🚧 Further Work

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

<p align="left">(<a href="#top">back to top</a>)</p>

..