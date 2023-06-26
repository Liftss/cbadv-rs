<p align="center">
    <a href="https://patreon.com/ohkthx" title="Donate to this project using Patreon">
        <img src="https://img.shields.io/badge/patreon-donate-red.svg?style=for-the-badge&color=f38ba8&label=PATREON&logo=patreon&logoColor=f38ba8&labelColor=11111b"
            alt="Patreon donate button"></a>
    <a href="https://ko-fi.com/G2G0J79MY" title="Donate to this project using Ko-fi">
        <img src="https://img.shields.io/badge/kofi-donate-ffffff.svg?style=for-the-badge&color=fab387&label=KOFI&logo=kofi&logoColor=fab387&labelColor=11111b"
            alt="Buy me a coffee! Ko-fi"></a>
<br>
    <a href="https://github.com/ohkthx/xIPL" title="Size of the repo!">
        <img src="https://img.shields.io/github/repo-size/ohkthx/cbadv-rs?style=for-the-badge&color=cba6f7&label=SIZE&logo=codesandbox&logoColor=cba6f7&labelColor=11111b"
            alt="No data."></a>
</p>

# cbadv-rs, Coinbase Advanced API in Rust


The **cbadv-rs** project is designed to help me get my feet wet in Rust. By no means should others consider using this in the near future, especially with the hopes of making money. This is entirely for testing purposes and I am not responsible for your losses. However, you can choose to credit me with any gains made.

I am ambitious with the project and plan on expanding to the entire API. The API reference can be seen at [Coinbase Advanced API](https://docs.cloud.coinbase.com/advanced-trade-api/reference)

## Features
- Easy-to-use Client.
- Configuration file to hold API Key and API Secret.

## Covered API requests

- **Accounts [client.account]**
  - List Accounts [client.account.get_all]
  - Get Account [client.account.get]
- **Products [client.product]**
  - List Products [client.product.get_all]
  - Get Product [client.product.get]
  - Get Product Candles [client.product.candles]
  - Get Market Trades (Ticker) [client.product.ticker]
- **Orders [client.order]**
  - Create Order 
    - Market IOC (untested) [client.order.create_market]
    - Limit GTC [client.order.create_limit_gtc]
    - Limit GTD (untested) [client.order.create_limit_gtd]
  - Cancel Orders [client.order.cancel]
  - List Orders [client.order.get_all]
  - Get Order [client.order.get]
- **Fees [client.fee]**
  - Get Transaction Summary [client.fee.get]

## TODO API Requests

- **Products**
  - Get Best Bid / Ask
  - Get Product Book
- **Orders**
  - Create Order 
    - Stop Limit GTC
    - Stop Limit GTD
  - List Fills

## Configuration

The default configuration is unusable due to the API requiring a Key and Secret. You can create, modify, and delete API Keys and Secrets with this [link](https://www.coinbase.com/settings/api).

Copy the `config.toml.sample` to `config.toml` and add in your API information. The `config.toml` file will automatically be read on launch to access your accounts API information. Unlike the depreciated Coinbase Pro API, there's no longer access to Public API endpoints. All access requires authentication. The key and secret is authentication requirements for HTTP requests to be properly [signed](https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-auth) and accepted by Coinbase.
