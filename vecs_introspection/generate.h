#ifndef VECS_INTROSPECTION_GENERATE
#define VECS_INTROSPECTION_GENERATE

#include "sparse/symbol.h"
#include <stdio.h>

// Generators output code that outputs a given data structure in some way.
// This is C code that outputs C code. It does get a bit confusing.

void generate_header_dump(FILE *file, const char *fn_name,
                          struct symbol *symbol);
void generate_impl_dump(FILE *file, const char *fn_name, struct symbol *symbol);

#endif // !VECS_INTROSPECTION_GENERATE
