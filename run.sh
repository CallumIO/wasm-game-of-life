#!/bin/sh
wasm-pack build
cd www/
npm install
npm run start