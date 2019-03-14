#!/usr/bin/env python

import matplotlib
matplotlib.use('AGG')
import pcap
import sys
import time
import yaml
import struct
import numpy as np
import matplotlib.pylab as plt
from matplotlib.gridspec import GridSpec
from matplotlib.ticker import MultipleLocator, FormatStrFormatter
import subprocess


def hide_tick_labels(ax, hide_x=False, hide_y=False):
    ticks = []
    if hide_x:
        ticks += ax.get_xticklabels()
    if hide_y:
        ticks += ax.get_yticklabels()
    for tl in ticks:
        tl.set_visible(False)
                


dev_name=sys.argv[1]
cfg_name=sys.argv[2]


p = pcap.pcapObject()

p.open_live(dev_name, 1600, True , 0)

cfg=yaml.load(open(cfg_name))

mac_list=cfg['mac']

mac_idx_map={tuple(x[1]):x[0] for x in enumerate(mac_list)} 

fft_data=np.array([[0. for i in range(0,2048)] for i in range(len(mac_idx_map)*8)])

update_ratio=0.9

gs=GridSpec(8,16)
gs.update(hspace=0, wspace=0)

request=subprocess.Popen(["../target/release/fetch_fft", "enp3s0", "../param1.yaml"])

cnt=0

print(fft_data.shape)
try:
    while True:
        pktlen, data, timestamp=next(p)
        raw_data=bytearray(data)
        src_mac=tuple(i for i in raw_data[6:12])
        if src_mac in mac_idx_map and pktlen==1024+12+2+4+1:
            bid=mac_idx_map[src_mac]
            data_order=struct.unpack('=B',data[18])[0]-1
            fft_data1=np.array(struct.unpack('=256i', data[19:]))
            fft_data1=fft_data1[::2]+1.j*fft_data1[1::2]
            ps=np.real(fft_data1*np.conj(fft_data1))
            
            chip_id=int(data_order/16)
            port_id=bid*8+chip_id
            
            ch1=(data_order-chip_id*16)*128
            ch2=ch1+128

            #fft_data[port_id,ch1:ch2]=fft_data[port_id, ch1:ch2]*update_ratio+(1.-update_ratio)*ps
            fft_data[port_id,ch1:ch2]+=ps
            #print "%d %s %d %d" % ( pktlen, src_mac, bid, data_order)

            if bid==15 and data_order==127:
                request.wait()
                cnt+=1
                if cnt%10==0:
                    print("ploting")
                    ymax=10*np.max(np.log10(fft_data[fft_data>0]))
                    ymin=10*np.min(np.log10(fft_data[fft_data>0]))
                    dy=ymax-ymin
                    ymax+=dy*0.1
                    ymin-=dy*0.1
                    print(ymin, ymax)

                    plt.close()
                    fig=plt.figure(figsize=(40,20))
                    for j in range(16):
                        print("ploting board {0}".format(j));
                        for i in range(8):
                            bid=j
                            cid=i
                            port_id=bid*8+cid
                            ax=plt.subplot(gs[i,j])
                            hide_tick_labels(ax, hide_x=True, hide_y=True)
                            ax.set_ylim(ymin,ymax)
                            ax.set_xlim(-1, 2050)
                            ax.plot(10*np.log10(fft_data[port_id,:]))

                    plt.tight_layout()
                    plt.savefig('a.png')
                    print("fig saved")
                    fft_data*=0.0;
                    time.sleep(5)
                request=subprocess.Popen(["../target/release/fetch_fft", "enp3s0", "../param1.yaml"])
        
except KeyboardInterrupt:
    print '%s' % sys.exc_type
    print 'shutting down'
    print ('%d packets received, %d packets dropped %d packets dropped by interface)' % p.stats())
    
    
