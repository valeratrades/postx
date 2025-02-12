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
