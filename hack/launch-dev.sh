#!/bin/bash

wasm-pack build --out-name wasm --out-dir ./www/pkg
cd www
npm install
npm start
