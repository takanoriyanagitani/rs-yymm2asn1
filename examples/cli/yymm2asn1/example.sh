#!/bin/sh

der2fq() {
	cat /dev/stdin |
		fq -d asn1_ber
}

der2jer() {
	cat /dev/stdin |
		xxd -ps |
		tr -d '\n' |
		python3 -m asn1tools \
			convert \
			-i der \
			-o jer \
			yymm.asn \
			DateYyMmCompactInfer \
			- |
		jq -c
}

wazero run ./yymm2asn1.wasm 19 5 | der2jer
wazero run ./yymm2asn1.wasm 85 8 | der2jer
