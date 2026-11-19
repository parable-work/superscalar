/* A tiny C consumer of the Acme assembly: one call per process.
 *
 *   c_consumer <canonical-name> <input>
 *   c_consumer parse|magnitude|parts <canonical-name> <input>
 *
 * The first form is scalar_parse. Exit 0 and the answer on stdout when the
 * input is accepted; exit 1 and the error message on stdout when it is
 * rejected; exit 2 on usage errors. scripts/run_vectors.py drives it over the
 * conformance vectors. The header is the generic one: the C ABI does not
 * change with the registry behind it. */
#include <stdio.h>
#include <string.h>

#include "superscalar.h"

static int usage(void) {
  fputs("usage: c_consumer [parse|magnitude|parts] <canonical-name> <input>\n", stderr);
  return 2;
}

int main(int argc, char **argv) {
  if (argc != 3 && argc != 4) {
    return usage();
  }
  const char *hook = argc == 4 ? argv[1] : "parse";
  const char *name = argv[argc - 2];
  const char *input = argv[argc - 1];

  ScalarResult result;
  if (strcmp(hook, "parse") == 0) {
    result = scalar_parse(name, input);
  } else if (strcmp(hook, "magnitude") == 0) {
    result = scalar_magnitude(name, input);
  } else if (strcmp(hook, "parts") == 0) {
    result = scalar_parts(name, input);
  } else {
    return usage();
  }
  int status;
  if (result.ok) {
    if (result.value.kind == ValueKind_Str && result.value.str_ptr != NULL) {
      fwrite(result.value.str_ptr, 1, result.value.str_len, stdout);
    }
    status = 0;
  } else {
    fputs(result.error != NULL ? result.error : "scalar error", stdout);
    status = 1;
  }
  scalar_result_free(result);
  return status;
}
