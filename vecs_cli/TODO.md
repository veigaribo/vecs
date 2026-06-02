# TODO:

General:

- Find out what features are needed;
- Improve presentation of error messages (spans mainly);
- Add methods that operate based on `engine->state` automatically for convenience;
- Add attribute-like things to e.g. omit `init`s of systems;

OS-specific:

- Make system execution multithreaded;
- Make dynamic arrays always allocate in cache-line-sized and cache-line-aligned
  chunks;
