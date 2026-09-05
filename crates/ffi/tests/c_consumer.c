/* A tiny C consumer of superscalar.h. Proves the cbindgen header is valid
 * C and that the static library's ABI links and round-trips from C. The codegen
 * crate generates named ScalarId constants; this smoke test hardcodes one frozen
 * discriminant (see core/src/catalog.rs). */
#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "superscalar.h"

/* Contact.Email -- frozen ScalarId discriminant. */
#define SCALAR_CONTACT_EMAIL 8u

int main(void) {
  ScalarResult ok = scalar_parse(SCALAR_CONTACT_EMAIL, "Foo@Bar.com");
  assert(ok.ok);
  assert(ok.value.kind == ValueKind_Str);
  assert(ok.error == NULL);
  assert(ok.value.str_ptr != NULL);
  assert(strcmp(ok.value.str_ptr, "foo@bar.com") == 0);
  assert(ok.value.str_len == strlen("foo@bar.com"));
  scalar_result_free(ok);

  ScalarResult bad = scalar_parse(SCALAR_CONTACT_EMAIL, "not-an-email");
  assert(!bad.ok);
  assert(bad.error_category == ErrorCategory_Pattern);
  assert(bad.error != NULL);
  scalar_result_free(bad);

  /* Batch equals N singles. */
  const char *inputs[3] = {"a@b.io", "bad", "C@D.com"};
  ScalarResultArray batch = scalar_parse_batch(SCALAR_CONTACT_EMAIL, inputs, 3);
  assert(batch.len == 3);
  assert(batch.ptr[0].ok);
  assert(!batch.ptr[1].ok);
  assert(batch.ptr[2].ok);
  scalar_result_array_free(batch);

  printf("c_consumer: ABI ok\n");
  return 0;
}
