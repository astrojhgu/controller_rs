#!/usr/bin/env python

import matplotlib
matplotlib.use('AGG')
import sys
import time
import yaml
import struct
import numpy as np
import matplotlib.pylab as plt
from matplotlib.gridspec import GridSpec
from matplotlib.ticker import MultipleLocator, FormatStrFormatter
import subprocess

nboards=16
ports_per_board=8
nch=2048
nports=nboards*ports_per_board

def hide_tick_labels(ax, hide_x=False, hide_y=False):
    ticks = []
    if hide_x:
        ticks += ax.get_xticklabels()
    if hide_y:
        ticks += ax.get_yticklabels()
    for tl in ticks:
        tl.set_visible(False)



def calc_corr(fft_data, ref_port_id):
    result=np.zeros_like(fft_data, dtype='complex')
    for i in range(0, fft_data.shape[0]):
        #print("calculating port {0}".format(i))
        result[i,:]=(fft_data[i, :])*np.conj(fft_data[ref_port_id, :])
    #print("fsdfa:{0}".format((fft_data[127,:]==fft_data[135,:]).all()))
    #print("*****:{0}".format((result[127,:]==result[135,:]).all()))
    #result[:, result.shape[1]/2]=0
    return result


dev_name=sys.argv[1]
cfg_name=sys.argv[2]


spec_data=np.array([[0. for i in range(0,nch)] for i in range(nports)])
fft_data=np.array([[0.+0.0j for i in range(0,nch)] for i in range(nports)])
corr_data=np.array([[0.+0.0j for i in range(0,nch)] for i in range(nports)])
mean_fft_data=np.array([[0.+0.0j for i in range(0,nch)] for i in range(nports)])
mean_k=0.9

update_ratio=0.9
fft_update_ratio=0.99

gs=GridSpec(ports_per_board,nboards)
gs.update(hspace=0, wspace=0)
cmd=["../target/debug/fetch_fft", dev_name, cfg_name]
request=subprocess.Popen(cmd)

cnt=0

ref_port=int(sys.argv[3])

print(spec_data.shape)
while True:
    #request=subprocess.Popen(["../target/debug/fetch_fft", dev_name, cfg_name])
    request.wait()
    request.communicate()
    rc=request.returncode
    if rc!=0:
        request=subprocess.Popen(cmd)
        continue

    raw_data=open("fft_data.dat","rb").read()
    ndata=len(raw_data)/4 # 4 bytes each integer
    raw_data=np.array(struct.unpack("={0}i".format(ndata), raw_data))
    fft_data=(raw_data[1::2]+raw_data[::2]*1j).reshape([nports,-1])
    mean_fft_data=fft_update_ratio*mean_fft_data+(1.0-fft_update_ratio)*fft_data
    spec_data+=np.real(fft_data*np.conj(fft_data))
    corr_data=corr_data*update_ratio+(1.-update_ratio)*calc_corr(fft_data, ref_port)
    cnt+=1
    if cnt%10==0:
        
        print("plotting")
        ymax=10*np.max(np.log10(spec_data[spec_data>0]))
        ymin=10*np.min(np.log10(spec_data[spec_data>0]))
        dy=ymax-ymin
        ymax+=dy*0.1
        ymin-=dy*0.1
        print(ymin, ymax)

        plt.close()
        fig=plt.figure(figsize=(40,20))
        for j in range(nboards):
            print("ploting board {0}".format(j));
            for i in range(ports_per_board):
                bid=j
                cid=i
                port_id=bid*ports_per_board+cid
                ax=plt.subplot(gs[i,j])
                hide_tick_labels(ax, hide_x=True, hide_y=True)
                ax.set_ylim(ymin,ymax)
                ax.set_xlim(-1, 2050)
                ax.plot(10*np.log10(spec_data[port_id,:]), linewidth=0.5)

        plt.tight_layout()
        plt.savefig('auto.png')
        print("fig1 saved")
        spec_data*=0.0

        plt.close()

        #ymax=max(np.max(corr_data.real),np.max(corr_data.imag))
        #ymin=min(np.min(corr_data.real),np.min(corr_data.imag))
        ymax=max(np.std(corr_data.real)*5, np.std(corr_data.imag)*5)
        ymin=-ymax
        ymax*=1.1
        ymin*=1.1
        ymax=max(ymax, abs(ymin))
        ymin=-ymax
        print(ymin,ymax)
        fig=plt.figure(figsize=(40,20))

        for j in range(nboards):
            print("ploting board {0}".format(j))
            for i in range(ports_per_board):
                bid=j
                cid=i
                port_id=bid*ports_per_board+cid
                ax=plt.subplot(gs[i,j])
                ax.set_ylim(ymin, ymax)

                hide_tick_labels(ax, hide_x=True, hide_y=True)
                ax.plot(corr_data[port_id,:].real, linewidth=0.5)
                ax.plot(corr_data[port_id,:].imag,'.', markersize=0.5)
        plt.tight_layout()
        plt.savefig('corr.png')
        print("fig2 saved")

        print("plotting")
        ymax=np.pi
        ymin=-np.pi
        print(ymin, ymax)

        plt.close()
        fig=plt.figure(figsize=(40,20))
        for j in range(nboards):
            print("ploting board {0}".format(j));
            for i in range(ports_per_board):
                bid=j
                cid=i
                port_id=bid*ports_per_board+cid
                ax=plt.subplot(gs[i,j])
                hide_tick_labels(ax, hide_x=True, hide_y=True)
                ax.set_ylim(ymin,ymax)
                ax.set_xlim(-1, 2050)
                ax.plot(np.angle(corr_data[port_id,:]), linewidth=0.5)

        plt.tight_layout()
        plt.savefig('args.png')
        print("fig2.5 saved")
        #spec_data*=0.0
        plt.close()
        fig=plt.figure(figsize=(40,20))
        ymax=max(np.max(mean_fft_data.real),np.max(mean_fft_data.imag))
        ymin=min(np.min(mean_fft_data.real),np.min(mean_fft_data.imag))
        ymax*=1.1
        ymin*=1.1
        ymax=max(ymax, abs(ymin))
        ymin=-ymax
        print(ymin,ymax)
        for j in range(nboards):
            print("ploting board {0}".format(j))
            for i in range(ports_per_board):
                bid=j
                cid=i
                port_id=bid*ports_per_board+cid
                ax=plt.subplot(gs[i,j])
                ax.set_ylim(ymin, ymax)

                hide_tick_labels(ax, hide_x=True, hide_y=True)
                ax.plot(mean_fft_data[port_id,:].real, linewidth=2.5)
                ax.plot(np.zeros_like(mean_fft_data[port_id,:]), linewidth=2.5)
                #ax.plot(mean_fft_data[port_id,:].imag,'.')
        plt.tight_layout()
        plt.savefig('mean_fft.png')
        print("fig3 saved")
        plt.close()
        request=subprocess.Popen(cmd)

