# Clippord

Clippord is a lightweight & fast clipboard api for node that's built on rust

## Example

``` javascript
import * as clippord from "clippord"
console.log(clippord.getClipboard()); // prints the current copied text
```

## Contributers

* [darkdarcool](https://github.com)

## API

``` ts
function getClipboard(): string;
function setClipboard(text: string): undefined
```