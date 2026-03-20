#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <pwd.h>
#include <unistd.h>

#include "tomlc17/src/tomlc17.h"

#define FILE_SIZE 1024

char* get_home_dir() {
  char* home_dir;
  if ((home_dir = getenv("HOME")) == NULL) {
    home_dir = getpwuid(getuid())->pw_dir;
  }
  return home_dir;
}

int main() {
  char* full_path = malloc(sizeof(char*) * 50);
  sprintf(full_path, "%s%s", get_home_dir(), "/.passport/config.toml");
  toml_result_t result = toml_parse_file_ex(full_path);
  if (!result.ok) {
    printf("Error: %s\n", result.errmsg);
    exit(-1);
  }
  const char* rustport_path = toml_seek(result.toptab, "global.source_path").u.s;
  chdir(rustport_path);
  int git_pull_status = system("git pull");
  if (git_pull_status != 0) {
    printf("Something went wrong (git pull)! Status: %d\n", git_pull_status);
    exit(git_pull_status);
  }
  int just_update = system("just update");
  if (just_update != 0) {
    printf("Something went wrong (just update)! Status: %d\n", just_update);
    exit(just_update);
  }
  return 0;
}
