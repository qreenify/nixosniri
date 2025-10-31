{ config, pkgs, ... }:

{
  # Disk mounts
  fileSystems."/mnt/games" = {
    device = "/dev/disk/by-uuid/396155f3-d8cf-4126-a704-46e4260f96f8";
    fsType = "ext4";
    options = [ "defaults" "nofail" ];
  };

  fileSystems."/mnt/backup" = {
    device = "/dev/disk/by-uuid/1611f6fc-24d1-4f81-b69a-39896784a852";
    fsType = "btrfs";
    options = [ "defaults" "nofail" ];
  };

  # Create mount directories
  systemd.tmpfiles.rules = [
    "d /mnt/games 0755 qreenify users -"
    "d /mnt/backup 0755 qreenify users -"
    "d /mnt/gdrive-wonderland 0755 qreenify users -"
  ];
}
