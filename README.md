# postx
![Minimum Supported Rust Version](https://img.shields.io/badge/nightly-1.85+-ab6000.svg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/postx.svg?color=fc8d62&logo=rust" height="20" style=flat-square>](https://crates.io/crates/postx)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs&style=flat-square" height="20">](https://docs.rs/postx)
![Lines Of Code](https://img.shields.io/badge/LoC-395-lightblue)
<br>
[<img alt="ci errors" src="https://img.shields.io/github/actions/workflow/status/valeratrades/postx/errors.yml?branch=master&style=for-the-badge&style=flat-square&label=errors&labelColor=420d09" height="20">](https://github.com/valeratrades/postx/actions?query=branch%3Amaster) <!--NB: Won't find it if repo is private-->
[<img alt="ci warnings" src="https://img.shields.io/github/actions/workflow/status/valeratrades/postx/warnings.yml?branch=master&style=for-the-badge&style=flat-square&label=warnings&labelColor=d16002" height="20">](https://github.com/valeratrades/postx/actions?query=branch%3Amaster) <!--NB: Won't find it if repo is private-->

A small little thing to redirect posts from Telegram to Twitter.

<!-- markdownlint-disable -->
<details>
  <summary>
    <h2>Installation</h2>
  </summary>
	<pre>
		<code class="language-sh">nix build</code></pre>
</details>
<!-- markdownlint-restore -->

## Usage
```sh
postx -p PASSWORD -u USERNAME -t TG_BOT_TOKEN follow "CryptoAttack_en"
```

Errors are an after-thought, can panic in case of poor connection / re-login rate-limit, really the only way to somewhat run it is
```sh
while true; do
	postx <args>
	echo "something went wrong, tweet on which it did will be skipped"
done
```
		

<br>

<sup>
	This repository follows <a href="https://github.com/valeratrades/.github/tree/master/best_practices">my best practices</a> and <a href="https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md">Tiger Style</a> (except "proper capitalization for acronyms": (VsrState, not VSRState) and formatting).
</sup>
	

#### License

<sup>
	Licensed under <a href="LICENSE">Blue Oak 1.0.0</a>
</sup>

<br>

<sub>
	Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be licensed as above, without any additional terms or conditions.
</sub>
	

