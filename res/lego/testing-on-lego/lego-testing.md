# Testing an upgrade on relaychain.totem.live and lego.totem.live

Run the `cleanup.sh` file on the server.

What we are about to do is replace the binary of lego on the relaychain. First we must make lego operational.

Visit:
[https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/explorer)

1. Go to `Network>Parachains` in polkadotjs apps.

2. Select parathreads sub-menu bar

3. press paraid button.

Using Alice create paraid `2000` with a transaction.

The parathread button should appear. Now you can add the wasm binary and the state for the lego chain. 

4. Select these from the totem-parachains github repo

These can be found in the res folder:
Code is called `parachain-genesis-wasm.wasm` 
the initial genesis state is found in the lego folder: `lego-genesis-state.state`

5. Submit the transaction. 

The onboarding will take around 1.30 mins. This is not the same as Polkadot, which is much longer.

Once the parathread is on-boarded, you should now be able to upgrade to a parachain.

6. Go to `Developer>Sudo`

7. Select Slots and select ForceLease.

```
paraid = `2000`
leaser = Alice
amount = 100000000000000
period begin = 0
period count = 501
```

As Alice is also Sudo this should work. 

8. Next Go to `Network>Parachains` and you will see that the parathread is upgrading to a parachain. It takes about a minute. Again on Polkadot it's much slower.

After a minute lego should be producing blocks. 

Check this by going to  
[https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Flego.totem.live#/explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Flego.totem.live#/explorer)

Next, handle the correction of validation code. This can only happen after 10 blocks have passed on lego.

9. Again go back to the relaychain:
[https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/explorer)


10. Go to `Developer>Sudo`

11. Select `paras` and `forceSetCurrentCode()`

12. You will need to select a new binary. You can find it in the `res` folder called `parachain-code-substitution-wasm.wasm`

This is the validation code that will replace the one the lego chain is now running. Don't forget to use paraid = `2000`

13. Visit the explorer to see the `paras.CurrentCodeUpdated` event. Check when the actual change will occur:

14. Check when the code was updated against these results:

```
# initial code hash:
0x17c5e9811962a4f99d5a2e4a418ab2fe85224fbf7108ce50af198a65a87ea4af

# code substitution hash:
0x6246196532313a2ba5d4ea72ab7e49f0b5d0b6e6973fb198a7da80529ca73f87
```
To do this go to chainstate (https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/chainstate)[https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/chainstate]

and select the following:

```
# paras.currentCodeHash
0x6246196532313a2ba5d4ea72ab7e49f0b5d0b6e6973fb198a7da80529ca73f87


# paras.pastCodeMeta
PolkadotRuntimeParachainsParasParaPastCodeMeta
[
  [
    [
      2,000
    ]
    {
      upgradeTimes: [
        {
          expectedAt: 306
          activatedAt: 306
        }
      ]
      lastPruned: null
    }
  ]
]

# paras.pastCodeHash
[
  [
    [
      [
        2,000
        306
      ]
    ]
    0x17c5e9811962a4f99d5a2e4a418ab2fe85224fbf7108ce50af198a65a87ea4af
  ]
]
```

In this example the code was updated at block 306.

We are now ready to perform an uprade on the Lego chain. You will need the Lego Sudo key to perfom this.

15. Visit:

[https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Flego.totem.live#/extrinsics](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Flego.totem.live#/extrinsics)

16. Select the account with the Sudo key in it.

* select the sudo pallet, and `sudo(call)` from the dropdown. This basically makes the sudo account execute a sudo transacion. If not you will get a `bad origin` error.

* in the call section select `parachainSystem` and `authorizeUpgrade()`.

* you should select the upgrade code you want to upgrade to. 

    * you can find this in the `totem-parachains/upgrades` folder

In this case it is `kapex-v1.4.5` which has this hash `0x8747a3d9c213ae568db9395196d878ab2723417d420c0432c777bf3b3f6778af`

17. Once this has been set and the events in lego have been seen any account can enact the upgrade.

Visit:

[https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Flego.totem.live#/extrinsics](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Flego.totem.live#/extrinsics)

* in the call section select `parachainSystem` and `enactUpgrade()`.

* you will need to select the exact same file that was selected in the authorizeUpdrage step.

* YOU CANNOT UPGRADE UNTIL THE RELAYCHAIN GIVES PERMISSION. YOU WILL SEE THAT IN THE EVENTS.

* monitor the relaychain in another browser tab for events: [https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frelaychain.totem.live#/explorer)

18. In the Lego explorer you will see the event message : `parachainSystem.ValidationFunctionStored`

19. in the Relaychain explorer you will see the messages: `paras.PvfCheckStarted` and `paras.PvfCheckAccepted` however the upgrade has not happened at this point. Check when it will occur:

`paras.futureCodeUpgrades`

and `paras.upgradeCooldowns`

These values indicate the block numbers upon which the upgrade is scheduled to occur on the parachain, not the relaychain.

On the parachain itself, upgrading is locked until it occurs. You can check if there is a pending upgrade using `parachainSystem.pendingValidationCode`. If a value exists here then the parachain is in the process of being upgraded.


