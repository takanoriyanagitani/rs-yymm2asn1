#!/bin/sh

python3 \
	-c 'import asn1tools; bd = asn1tools.compile_files("./yymm.asn")'
