#!/bin/bash
cat > /etc/sudoers.d/01-resize-nopasswd <<'EOF'
eileen ALL=(ALL) NOPASSWD: /sbin/resize2fs, /sbin/e2fsck, /sbin/tune2fs, /sbin/fdisk, /sbin/mkfs.ext4, /sbin/mount, /sbin/umount, /usr/sbin/resize2fs, /usr/sbin/e2fsck, /usr/sbin/tune2fs, /usr/bin/mount, /usr/bin/umount
EOF
chmod 440 /etc/sudoers.d/01-resize-nopasswd
visudo -c -f /etc/sudoers.d/01-resize-nopasswd
echo "--- content ---"
cat /etc/sudoers.d/01-resize-nopasswd
