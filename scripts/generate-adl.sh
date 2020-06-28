#!/bin/bash

ROOTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." >/dev/null 2>&1 && pwd )"

ADLC=$ROOTDIR/scripts/adlc

cd $ROOTDIR

$ADLC rust \
  --searchdir adl \
  --outputdir cli-client/src \
  --manifest cli-client/src/.adl-manifest \
  --package adl \
  --runtime-module adlrt \
  --generate-transitive \
  adl/*.adl
