#!/usr/bin/env bash
# Add NOPASSWD for casey to run resize2fs, e2fsck, tune2fs
# This is a marker script - will be written to WSL via mounting later
echo "casey ALL=(ALL) NOPASSWD: /sbin/resize2fs, /sbin/e2fsck, /sbin/tune2fs, /sbin/fstrim" > /tmp/sudoers_fragment
chmod 440 /tmp/sudoers_fragment
echo "Fragment written to /tmp/sudoers_fragment:"
cat /tmp/sudoers_fragment
