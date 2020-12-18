# RapidVM
A quick web-based management interface for KVM virtual machines
using QEMU.

## Why?
My final year of my BS degree involves a project which is intended to
run on a fairly long list of platforms: me being me, my code should run
as close to *everywhere* as is possible. Naturally, to compile for 
NTOS x86-64/ARM64 (ReactOS PPC64?), macOS x86-64/ARM64, 
Linux x86-64/ARMHF/ARM64/PPC64, and FreeBSD x86-64/ARMHF/ARM64/PPC64/RV64GC, 
virtual machines for each of these targets needs to be made.

Since without KVM, performance is pretty bad for virtual machines, real hardware
is always preferable. For x86, this is very easy. For ARM64, this is a little
harder, and the most reasonable option for me is a Raspberry Pi 4B 8GB. For
the other targets being PPC64 and RV64GC, since I can't get my hands on the
real hardware, I don't want to put in the effort to really support them anyway
at the moment.

Unfortunately, while the Pi does support KVM and all, and doesn't have the weird
interrupt controller, it doesn't have a native port of Proxmox over to it.
For me, since I don't really care about LXD and Ceph and all that, I only really
need support for network-based file storage for holding onto the virtual machine
images, and then some easy way to access them for management. The easy way was
to just use the same tools: xterm.js and NoVNC. ZFS would be nice, but I doubt
it would play well with a single 32GB SD card.

Given a short break in between semesters, it seemed reasonable to give a try
to this whole web development thing and use Rust for it since it has nice
frameworks for that sort of thing, and since there's no GC or virtual machine,
it's likely to not bother me with memory consumption: 8GB isn't a whole lot when
it needs to run 3 VMs, and more expensive ARM boards I could realistically get
don't really help with memory capacity. Something that just took off the shelf
DDR3 or DDR4 SODIMMS or something would be nice, since I don't care about ECC.
VM (or physical host) crashes I'll just reboot it, and no job should take more
than a couple minutes anyway.
