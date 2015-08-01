initSidebarItems({"constant":[["IOC_NONE","/// Indicates that the ioctl data pointer is not used"],["IOC_READ","/// Indicates tha the ioctl data pointer contains data that /// will be populated by the operating system to be consumed /// by userspace"],["IOC_WRITE","/// Indicates that the ioctl data pointer contains data that /// will be consumed by the operating system"]],"fn":[["execute","Ioctl call for which no data pointer is provided to the kernel. That is, the kernel has sufficient information about what to do based on the op alone."],["op","Build an ioctl op with the provide parameters.  This is a helper function for IOCTLs in the Linux kernel using the newer conventions for IOCTLs operations.  Many ioctls do not use this newer convention and the constants for those should just be used as-is."],["op_none","Build an op indicating that the data pointer is not used. That is, the command itself is sufficient."],["op_read","Build an op indicating that the data pointer will be populated with data from the kernel"],["op_read_write","Build an op indicating that the data pointer both contains data to be consumed by the kernel and contains fields that will be populated by the kernel."],["op_write","Build an op indicating that the data pointer contains data to be consumed by the kernel (and not written to)."],["read","Ioctl call that is expected to return a result but which does not take any additional arguments on the input side"],["read_into","Ioctl where the result from the kernel will be written to the provided reference"],["read_into_ptr","Ioctl where the result from the kernel will be written to the provided pointer"],["write","Ioctl call that sends a value to the kernel but does not return anything (pure side effect)."],["write_ptr","Ioctl call that sends a value to the kernel but does not return anything (pure side effect)."]],"struct":[["IoctlDirFlags",""]],"type":[["ioctl_op_t",""]]});