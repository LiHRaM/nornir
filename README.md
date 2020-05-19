# Nornir
Inspired by [kanshi](https://github.com/emersion/kanshi), `nornir` will handle the creation of workspaces when using [SwayWM](swaywm.org) with several monitors.
`nornir` will do this based on simple sway-ish configuration profiles.

## Motivation
Currently, Sway does not do handle this explicitly.
Instead, the user is expected to control the placement of workspaces by changing the focus to the screen where the new workspace is supposed to be created.

In an ideal world, we should be able to set our workspaces up with minimal fuss based on some preconfigured setting, such as alternating between screens when creating workspaces.

## Nornir...?
In norse mythology, the fates of men are controlled by the nornir, Urður, Verðandi and Skuld.
On Linux, however, they only control the placement of your workspaces.
