### Docs soon to follow

In the meantime, you'll have to look at the code. So sorry.

Some lazy examples of usage:

```bash
lgster-wake -m 29:00:8B:10:20:02 -t 192.168.1.255

lgster -k 0KEYC0DE -t 192.168.1.50 query volume # output: VOL:5
lgster -k 0KEYC0DE -t 192.168.1.50 set volume 10 # output: OK
lgster -k 0KEYC0DE -t 192.168.1.50 query volume # output: VOL:10
lgster -k 0KEYC0DE -t 192.168.1.50 query mute # output: MUTE:off
lgster -k 0KEYC0DE -t 192.168.1.50 custom command "POWER off" # output: OK
```
