# TODO:

General:

- Find out what features are needed;
- Improve presentation of error messages (spans mainly);
- Ask for state systems in a simple list and optimize it internally;
- Add methods that operate based on `engine->state` automatically for convenience;
- Consider making index arrays O(log n) sometimes somehow;
- - IDEA: Store indices(1) in actually 2 structures. Initially, use bucketed arrays,
    like we currently do. When the structure gets to a certain amount of items, we
    store the biggest index(2) + gen value we currently store. From that point on, we
    start storing items in a regular B-tree, but only if the index(2) + gen is greater
    than what we have stored. When accessing an index(1), we know whether to look on
    the arrays or the trees by checking if index(2) + gen is greater than the
    threshold. This should be fast for few items and for a lot of items at the same
    time, without needing reallocations of existing items. Still need to make sure
    this is sound. (1): Index in the book/database sense; (2): Index in the pointer
    offset sense.

OS-specific:

- Make system execution multithreaded;
- Make dynamic arrays always allocate in cache-line-sized and cache-line-aligned chunks;
