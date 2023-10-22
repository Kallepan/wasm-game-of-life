#!/bin/bash

wasm-pack build
cd www
npm install
npm start
