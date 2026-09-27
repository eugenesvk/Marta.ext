# Octal permission flags st_mode field
  - `S_IFMT`  	F MT  	17· ··· bit mask for the file type bit fields
  - `S_IFSOCK`	F Sock	14· ··· socket
  - `S_IFLNK` 	F Lnk 	12· ··· symbolic link
  - `S_IFREG` 	F Reg 	1·· ··· regular file
  - `S_IFBLK` 	F Blk 	 6· ··· block device
  - `S_IFDIR` 	F Dir 	 4· ··· directory
  - `S_IFCHR` 	F Chr 	 2· ··· character device
  - `S_IFIFO` 	F IFO 	 1· ··· FIFO
  - `S_ISUID` 	S Uid 	  4 ··· set-user- ID bit
  - `S_ISGID` 	S Gid 	  2 ··· set-group-ID bit (see below)
  - `S_ISVTX` 	S VTX 	  1 ··· sticky bit       (see below)    = "SaVe TeXt-segments"
  - `S_IRWXU` 	RWX U 	    7·· Owner . mask for   permissions of file owner
  - `S_IRUSR` 	R Usr 	    4·· …          read    …
  - `S_IWUSR` 	W Usr 	    2·· …          write   …
  - `S_IXUSR` 	X Usr 	    1·· …          execute …
  - `S_IRWXG` 	RWX G 	     7· Group . mask for   permissions of group
  - `S_IRGRP` 	R Grp 	     4· …          read    …
  - `S_IWGRP` 	W Grp 	     2· …          write   …
  - `S_IXGRP` 	X Grp 	     1· …          execute …
  - `S_IRWXO` 	RWX O 	      7 Others. mask for   permissions for others (not in group)
  - `S_IROTH` 	R Oth 	      4 …          read    …
  - `S_IWOTH` 	W Oth 	      2 …          write   …
  - `S_IXOTH` 	X Oth 	      1 …          execute …

- `sticky` (meaningles for files): a dir with 'sticky bit' becomes append-only = deletion of files is restricted, a file may only be removed or renamed by user with dir write permission and is
  - ¦ owner of file
  - ¦ owner of directory
  - ¦ super-user
  Usefull for /tmp which must be publicly writable but should deny users the license to arbitrarily delete or rename each others' files
