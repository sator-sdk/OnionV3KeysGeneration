# OnionV3KeysGeneration
Rust script that generate the pair of keys (public and private), required by Tor as authentication credential in order to connect to the Onion Service, for v3 Onion Services only.
## Client Auth
As already explained in the official tor [page](https://community.torproject.org/onion-services/advanced/client-auth/) the mechanism of authentication for Onion V3 Services is a method to make an Onion Service private and authenticated. It requires Tor clients to provide an authentication credential in order to connect to the Onion Service. For v3 Onion Services, this method works with a pair of keys (a public and a private). The service side is configured with a public key and the client can only access it with a private key.
Read their guide on how to set properly up the configuration and all the location for the configuration files.

## x25519-keygen
I developed this scirpt in Rust for its efficiency. Even though as they already pointed out its preatty easy to manually generate those pair of keys in multiple ways and also notable scripts already exists.
The most notables are one in [Pyhton](https://github.com/pastly/python-snippits/blob/master/src/tor/x25519-gen.py) and one in [Bash](https://gist.github.com/mtigas/9c2386adf65345be34045dace134140b#file-onion-svc-v3-client-auth-sh), but another in [Rust](https://github.com/ppopth/torkeygen/tree/master) also exist but i've had some issues with that even if its a good script an simple.
So I decided to make a more competitive Rsut script just because Rust language deserve it!

---

## Usage

```shell
x25519-keygen <name> <onion-addres>
# Example
./keygenx25519 bob rh5d6reakhpvuxe2t3next6um6iiq4jf43m7gmdrphfhopfpnoglzcyd      
bob public key:  WU7FGD3CZWVFUBBVHRJLSRZC6ZMIWJITJX5LEYHI5UOUZBYTOIXA
bob private key: BN4M2IT6DQWFLF26EF6L3OIHWXXRGHECGXOQ7E7UPO5CL5Z7IJDA
Public key saved to bob.auth
Private key saved to bob_onion.auth_private
```
The `name` is the name of hte cliente you want to generate the key for, and the `onion anddress` provided is the site for which you are generating the auth pair of keys for (not necessary for the generation but for the file creation needed later when reactivating the HiddenService), it should be stripped without the `.onion` but during execution is stripped anyway so it dosent matter.

### Files

After that the results are saved to a file with the Tor requirements syntax:

**Server side**

```shell
# authorized client file
cat /var/lib/tor/hidden_service/authorized_clients/alice.auth
descriptor:x25519:N2NU7BSRL6YODZCYPN4CREB54TYLKGIE2KYOQWLFYC23ZJVCE5DQ
```

**Client side**

Make sure you have `ClientOnionAuthDir` set in your `torrc`. For example, add this line to `/etc/tor/torrc`: `ClientOnionAuthDir /var/lib/tor/onion_auth`

Then, in the `<ClientOnionAuthDir>` directory, create an `.auth_private` file for the Onion Service corresponding to this key (i.e. 'bob_onion.auth_private'). The content of the `<ClientOnionAuthDir>/<user>.auth_private` file should look like this:

```shell
 <56-char-onion-addr-without-.onion-part>:descriptor:x25519:<x25519 private key in base32>
```
For example: `rh5d6reakhpvuxe2t3next6um6iiq4jf43m7gmdrphfhopfpnoglzcyd:descriptor:x25519:ZDUVQQ7IKBXSGR2WWOBNM3VP5ELNOYSSINDK7CAUN2W`

---

## Releases

---

## Build it from source

```shell
# Intall Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
#
rustup update
#
git clone https://github.com/sator-sdk/OnionV3KeysGeneration.git
#
cd OnionV3KeysGeneration
#
cargo build --release
```


















