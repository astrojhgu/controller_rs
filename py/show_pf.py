#!/usr/bin/env python

import numpy as np
import matplotlib.pylab as plt
import sys
import struct

ants=np.array([[0.000000, -15.417587, 0], [-1.515544, -14.132788, 0], [1.515544, -14.132788, 0], [-3.031089, -12.847989, 0], [0.000000, -12.847989, 0], [3.031089, -12.847989, 0], [-4.546633, -11.563190, 0], [-1.515544, -11.563190, 0], [1.515544, -11.563190, 0], [4.546633, -11.563190, 0], [-6.062178, -10.278391, 0], [-3.031089, -10.278391, 0], [0.000000, -10.278391, 0], [3.031089, -10.278391, 0], [6.062178, -10.278391, 0], [-7.577722, -8.993592, 0], [-4.546633, -8.993592, 0], [-1.515544, -8.993592, 0], [1.515544, -8.993592, 0], [4.546633, -8.993592, 0], [7.577722, -8.993592, 0], [-9.093267, -7.708793, 0], [-6.062178, -7.708793, 0], [-3.031089, -7.708793, 0], [0.000000, -7.708793, 0], [3.031089, -7.708793, 0], [6.062178, -7.708793, 0], [9.093267, -7.708793, 0], [-7.577722, -6.423994, 0], [-4.546633, -6.423994, 0], [-1.515544, -6.423994, 0], [1.515544, -6.423994, 0], [4.546633, -6.423994, 0], [7.577722, -6.423994, 0], [-9.093267, -5.139196, 0], [-6.062178, -5.139196, 0], [-3.031089, -5.139196, 0], [0.000000, -5.139196, 0], [3.031089, -5.139196, 0], [6.062178, -5.139196, 0], [9.093267, -5.139196, 0], [-7.577722, -3.854397, 0], [-4.546633, -3.854397, 0], [-1.515544, -3.854397, 0], [1.515544, -3.854397, 0], [4.546633, -3.854397, 0], [7.577722, -3.854397, 0], [-9.093267, -2.569598, 0], [-6.062178, -2.569598, 0], [-3.031089, -2.569598, 0], [0.000000, -2.569598, 0], [3.031089, -2.569598, 0], [6.062178, -2.569598, 0], [9.093267, -2.569598, 0], [-7.577722, -1.284799, 0], [-4.546633, -1.284799, 0], [-1.515544, -1.284799, 0], [1.515544, -1.284799, 0], [4.546633, -1.284799, 0], [7.577722, -1.284799, 0], [-9.093267, 0.000000, 0], [-6.062178, 0.000000, 0], [-3.031089, 0.000000, 0], [0.000000, 0.000000, 0], [3.031089, 0.000000, 0], [6.062178, 0.000000, 0], [9.093267, 0.000000, 0], [-7.577722, 1.284799, 0], [-4.546633, 1.284799, 0], [-1.515544, 1.284799, 0], [1.515544, 1.284799, 0], [4.546633, 1.284799, 0], [7.577722, 1.284799, 0], [-9.093267, 2.569598, 0], [-6.062178, 2.569598, 0], [-3.031089, 2.569598, 0], [0.000000, 2.569598, 0], [3.031089, 2.569598, 0], [6.062178, 2.569598, 0], [9.093267, 2.569598, 0], [-7.577722, 3.854397, 0], [-4.546633, 3.854397, 0], [-1.515544, 3.854397, 0], [1.515544, 3.854397, 0], [4.546633, 3.854397, 0], [7.577722, 3.854397, 0], [-9.093267, 5.139196, 0], [-6.062178, 5.139196, 0], [-3.031089, 5.139196, 0], [0.000000, 5.139196, 0], [3.031089, 5.139196, 0], [6.062178, 5.139196, 0], [9.093267, 5.139196, 0], [-7.577722, 6.423994, 0], [-4.546633, 6.423994, 0], [-1.515544, 6.423994, 0], [1.515544, 6.423994, 0], [4.546633, 6.423994, 0], [7.577722, 6.423994, 0], [-9.093267, 7.708793, 0], [-6.062178, 7.708793, 0], [-3.031089, 7.708793, 0], [0.000000, 7.708793, 0], [3.031089, 7.708793, 0], [6.062178, 7.708793, 0], [9.093267, 7.708793, 0], [-7.577722, 8.993592, 0], [-4.546633, 8.993592, 0], [-1.515544, 8.993592, 0], [1.515544, 8.993592, 0], [4.546633, 8.993592, 0], [7.577722, 8.993592, 0], [-6.062178, 10.278391, 0], [-3.031089, 10.278391, 0], [0.000000, 10.278391, 0], [3.031089, 10.278391, 0], [6.062178, 10.278391, 0], [-4.546633, 11.563190, 0], [-1.515544, 11.563190, 0], [1.515544, 11.563190, 0], [4.546633, 11.563190, 0], [-3.031089, 12.847989, 0], [0.000000, 12.847989, 0], [3.031089, 12.847989, 0], [-1.515544, 14.132788, 0], [1.515544, 14.132788, 0], [0.000000, 15.417587, 0]])
ant_max_x=np.max(ants[:,1])
ant_min_x=np.min(ants[:,1])

ant_max_y=np.max(ants[:,2])
ant_min_y=np.min(ants[:,2])

nch=2048
xscale=1.0/nch
yscale=1.0/360.0

phase_data=[]
phase_file=open(sys.argv[1], 'rb')
for i in range(0, 127):
    d=phase_file.read(2048*(2+2))
    ant_x, ant_y, ant_z=ants[i, :]
    raw_phase=np.array(struct.unpack('<4096h', d))
    phase_factor=raw_phase[0::2]+1j*raw_phase[1::2]
    plt.plot((np.arange(0, nch)-nch/2)*xscale+ant_x, np.degrees(np.angle(phase_factor))*yscale+ant_y)
    plt.text(ant_x, ant_y, "{0}".format(i))
plt.show()    