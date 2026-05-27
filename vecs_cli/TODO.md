# TODO:

General:

- Find out what features are needed;
- Improve presentation of error messages (spans mainly);
- Ask for state systems in a simple list and optimize it internally;
- Add methods that operate based on `engine->state` automatically for convenience;
- Add one byte of flags to every element in a sparse array. Sparse arrays will use it
  to store if a value has been deleted, saving us from having to check the holes array
  (which is still useful for insertions). There may be another flag to tell if a
  component has been disabled, which does not apply to entities.

OS-specific:

- Make system execution multithreaded;
- Make dynamic arrays always allocate in cache-line-sized and cache-line-aligned
  chunks;
