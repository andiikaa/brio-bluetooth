# brio bluetooth

In this project the focus is not on the complete reverse engineering of the brio protocol, I just wanted to learn how to reverse engineer bluetooth protocols in general.

Some [BRIO](https://www.brio.de/de-DE/produkte/eisenbahn/smart-tech-sound) products allow the connection via bluetooth and the controll of their toys via iOS app.
The current implementation will just change the colors for the headlights of [this train](https://www.brio.de/de-DE/produkte/eisenbahn/smart-tech-sound/smart-tech-soundlok-m-aufnahmef-63397100).

## spec

A nice overview about the spec can be found [in this project](https://github.com/cpetrich/Smart-Tech-Sound-BLE)

## run

```bash
$ cargo run
```

## reverse engineer bluetooth

To get the required data out of the bluetooth connection between toy and iOS app, i mainly followed these steps:

https://www.bluetooth.com/blog/a-new-way-to-debug-iosbluetooth-applications/
