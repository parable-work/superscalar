/* A tiny C consumer of the Acme assembly: one scalar_parse call per process.
 *
 *   c_consumer <scalar-id> <input>
 *
 * Exit 0 and the canonical value on stdout when the input is accepted; exit 1
 * and the error message on stdout when it is rejected; exit 2 on usage errors.
 * scripts/run_vectors.py drives it over the conformance vectors. The header is
 * the generic one: the C ABI does not change with the registry behind it. */
#include <stdio.h>
#include <stdlib.h>

#include "superscalar.h"

int main(int argc, char **argv) {
  if (argc != 3) {
    fputs("usage: c_consumer <scalar-id> <input>\n", stderr);
    return 2;
  }
  char *end = NULL;
  unsigned long id = strtoul(argv[1], &end, 10);
  if (*argv[1] == '\0' || *end != '\0' || id > 0xffffffffUL) {
    fputs("scalar-id must be a decimal u32\n", stderr);
    return 2;
  }

  ScalarResult result = scalar_parse((uint32_t)id, argv[2]);
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
