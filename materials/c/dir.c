#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/dir.h>

#ifdef __linux__
  #include <dirent.h>
#elif __APPLE__
  #include <sys/dirent.h>
  #include <sys/types.h>
#endif

/**
 * #define DT_UNKNOWN       0 The file type could not be determined.
 * #define DT_FIFO          1 This is a named pipe (FIFO).
 * #define DT_CHR           2 This is a character device.
 * #define DT_DIR           4 This is a directory.
 * #define DT_BLK           6 This is a block device.
 * #define DT_REG           8 This is a regular file.
 * #define DT_LNK          10 This is a symbolic link.
 * #define DT_SOCK         12 This is a UNIX domain socket.
 * #define DT_WHT          14 This is a dummy `whiteout inode`
 *                            is internal to the implementation
 *                            and should not be seen in normal
 *                            user applications.
 */
#define FOUND 0;
#define ERROR 1;
#define NOT_FOUND 2;
int main() {
  DIR *dirp = opendir(".");
  if (dirp == NULL) {
    printf("dirp: NULL");
    return ERROR;
  }

  struct dirent *dp = NULL;
  while ((dp = readdir(dirp)) != NULL) {
    printf("entry: %d %s\n", dp->d_type, dp->d_name);
  }
  closedir(dirp);
  return NOT_FOUND;
}