<div align="center">

# GTA V solo

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Linux](https://img.shields.io/badge/%E2%80%8B-Linux-green.svg?logo=linux&style=flat&logoColor=white)](https://github.com/eonm/gtav-solo/releases/latest/download/gtav-solo)
[![Windows](https://img.shields.io/badge/%E2%80%8B-Windows-blue.svg?logo=Windows&style=flat&logoColor=white)](https://github.com/eonm/gtav-solo/releases/latest/download/gtav-solo.exe)

</div>

> Can't handle the heat of GTAV online ? Want to farm without being annoyed ?

GTA V solo creates an __empty public session__ on GTA online for you!

<div align="center">

![alt text](./gtav-solo.gif "GTA V solo")

</div>

## Download

GTA V solo can be downloaded from the [release page](https://github.com/eonm/gtav-solo/releases/latest) of this repo.

GTA V solo works on Linux and Windows.

## How it's work ?

No need to unplug your network cable anymore, GTA V solo freezes GTA5.exe process for a few seconds, which reconnects you to a new empty public session.

### Pro

This approach doesn't require admin privileges.

### Cons

This approach only create a temporary public solo lobby. After player other player can join.

In the future a firewall based approach will be will be added.

## Usage

You should bind this script to a shortcut.

## Motivations

Providing a rust implementation which run on Linux (proton player) and windows.

## Other implementation (not limited)

### Suspend based implementations

* [GTASuspender](https://github.com/Jyrka98/GTASuspender) created by [@Jyrka98](https://github.com/Jyrka98) and written in C++

* [GTAO-PublicSessionBlocker](https://github.com/FaZeIlLuMiNaTi/GTAO-PublicSessionBlocker) created by [@FaZeIlLuMiNaTi](https://github.com/FaZeIlLuMiNaTi) and written in C#

* [GTAV Online Lobby Kicker](https://github.com/saltpouch/GTAV-Online-Lobby-Kicker) created by [@saltpouch](https://github.com/saltpouch) and written in python

### Firewall based implementations

* [GTAV Solo Lobby Script](https://github.com/thronic-gh/GTAV-Solo-Lobby-Script) created by [@thronic-gh](https://github.com/thronic-gh)

* [GTA5Online-Private_Public_Lobby](https://github.com/CodeSwine/GTA5Online-Private_Public_Lobby) created by [@CodeSwine](https://github.com/CodeSwine) and written in C#