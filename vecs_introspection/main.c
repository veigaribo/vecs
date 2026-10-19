#include "generate.h"
#include "sparse/lib.h"
#include "sparse/ptrlist.h"
#include "sparse/scope.h"
#include "sparse/symbol.h"
#include <stdio.h>

void action(struct symbol_list *symbols) {
  struct symbol *symbol;
  FOR_EACH_PTR(symbols, symbol) {
    printf("symbol %s\n", show_typename(symbol));
  }
  END_FOR_EACH_PTR(symbol);
}

static inline bool valid_namespace(enum namespace ns) {
  return (ns == NS_TYPEDEF || ns == NS_MACRO || ns == NS_UNDEF ||
          ns == NS_STRUCT || ns == NS_SYMBOL);
}

char *IMPL_FILENAME = ".ignore/introspect.c";
char *HEADER_FILENAME = ".ignore/introspect.h";

int main(int argc, char *argv[]) {
  struct string_list *filelist = NULL;
  char *filename;

  // action(sparse_initialize(1, &filename, &filelist));

  // FOR_EACH_PTR(filelist, file) { action(sparse(file)); }
  // END_FOR_EACH_PTR(file);

  sparse_initialize(argc, argv, &filelist);

  // Write header:
  FILE *header_file = fopen(HEADER_FILENAME, "w");

  FOR_EACH_PTR(filelist, filename) {
    sparse_keep_tokens(filename);

    fprintf(header_file, "#ifndef VECS_INTROSPECTION_\n");
    fprintf(header_file, "#define VECS_INTROSPECTION_\n");
    fprintf(header_file, "\n");

    fprintf(header_file, "#include <stdio.h>\n");
    fprintf(header_file, "#include \"%s\"\n", filename);
    fprintf(header_file, "\n");

    struct symbol *symbol;
    FOR_EACH_PTR(file_scope->symbols, symbol) {
      const char *member_name = show_ident(symbol->ident);
      generate_header_dump(header_file, member_name, symbol);
    }
    END_FOR_EACH_PTR(symbol);

    fprintf(header_file, "#endif // !VECS_INTROSPECTION_\n");
  }
  END_FOR_EACH_PTR(filename);

  fclose(header_file);

  // Write implementation:
  FILE *impl_file = fopen(IMPL_FILENAME, "w");

  FOR_EACH_PTR(filelist, filename) {
    sparse_keep_tokens(filename);

    fprintf(impl_file, "#include \"%s\"\n", HEADER_FILENAME);
    fprintf(impl_file, "\n");

    struct symbol *symbol;
    FOR_EACH_PTR(file_scope->symbols, symbol) {
      const char *member_name = show_ident(symbol->ident);
      generate_impl_dump(impl_file, member_name, symbol);
    }
    END_FOR_EACH_PTR(symbol);
  }
  END_FOR_EACH_PTR(filename);

  fclose(impl_file);

  return 0;
}
