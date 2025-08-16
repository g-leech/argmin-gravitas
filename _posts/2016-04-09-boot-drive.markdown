---
layout:     post
title:      "troubleshooting a boot drive"
baselink:   /boot
permalink:  /boot
date:       2016-04-09
author:     Gavin   
img:        /img/bofh_bofh-copy-2158851190.png

visible:    1
published:  true
quality:    4

summary:    pain
confidence: 
importance: 4
wordcount:  
categories: comput
where:      "Glasgow"
---

Possible points of failure when using a USB boot drive, in order of likelihood:

<ol>
<li>At boot, you're tapping the wrong key to enter BIOS: try <pre style="display: inline;">F2, Del, F9, right-shift, F12, F1, Escape</pre>, or <pre style="display: inline;">Ctrl+Alt+Escape</pre> before proceeding down this list.</li>
<li>Your monitor input is attached to the graphics card, not motherboard. BIOS has no drivers for that.</li>
<li>Either Fast Boot or Secure Boot are enabled. Kill them.</li>
<li>Your wireless keyboard requires an extra driver, not supported. Dig out that horrible beige one.</li>
<li>Boot drive is in wrong format (GPT is wrong when using legacy BIOS, MBR is wrong when using UEFI)</li>
<li>You're using a USB 3.0 device in a BIOS that can't understand it.</li>
<li>Actually it's fine, it's just that your BIOS lists your USB boot drive as a "hard drive", not a "removable device".</li>
<li>Your boot drive <i>file system</i> is wrong; it is NTFS when you need FAT32 for UEFI. Or vice versa.</li>
<li>Boot drive was burned badly, somehow. Use <a href="https://rufus.akeo.ie/">Rufus</a> instead.</li>
<li>The OS ISO was ripped badly, somehow. Use a fresh drive.</li>
<li>You have two identical hard drives, they have the same name in the boot priority screen, so the UEFI is booting you into the completely blank one and fails to respond to any further input and feels very clever in itself. Physically detach one of them until your dual boot is finished.</li>
<li value="xxxx">Your motherboard is fried, or the BIOS corrupted. Cry.</li>
</ol>

