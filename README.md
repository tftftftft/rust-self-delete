# rust-self-delete
Self-deletion running executable self-deletion from disk in Rust
The code uses Windows API bindings to achieve the self-deletion functionality. Here's a breakdown of the process:

1. The code retrieves the path of the current executable file and stores it in a buffer.
2. It creates a handle for the file using the `CreateFileW` function.
3. The file is renamed using the `SetFileInformationByHandle` function and the `FILE_RENAME_INFO` structure.
4. After renaming, the code closes the handle.
5. It then retrieves the new path of the renamed file and stores it in another buffer.
6. A new handle is created for the renamed file.
7. The file disposition information is set to indicate that the file should be deleted.
8. The new handle is closed, and the program pauses for 10 seconds.

Ref: https://github.com/LloydLabs/delete-self-poc
