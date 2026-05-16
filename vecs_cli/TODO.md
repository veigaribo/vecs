# TODO:

General:

- Find out what features are needed;
- Typedef the struct types so the code is simpler;
- Tidy up `vecs_sparse_array_id` usage;
- Remove components from states (make them global) for now;
- Consider making index arrays O(log n) sometimes somehow;

OS-specific:

- Make system execution multithreaded;
- Make dynamic arrays always allocate in cache-line-sized chunks;
