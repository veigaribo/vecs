# TODO:

General:

- Find out what features are needed;
- Allow specifying what operations are possible for each type of component to save
  memory if needed;
- Maybe allow operations such as updating and removing on temporary components, as
  well as on permanently stored ones, if that's found to be useful;
- Get rid of default `frame` event;
- Improve presentation of error messages (spans mainly);
- Put parameters to system functions in structs for ease of declaration;
- Maybe add macros to shorten the names of functions in system definitions, for
  convenience;
- Extract parser and resolver as a library for use in other crates;
- Recheck for memory leaks;

OS-specific:

Things that will require interacting with the OS and as such will not be trivially
portable.

- Make system execution multithreaded:
- - In a state systems definition, `{ par { a, par b }, c, par d }` means: first run,
    at the same time, systems `a` and `b`. When running `a`, run for each node
    sequentially, but, for `b`, run as much of them as possible at the same time.
    Then, run system `c` serially, then run system `d` in the same manner as `b`;
- - How do we then make it so that systems that operate on the same components run
    as often as possible on the same core to optimize cache usage?
- Make dynamic arrays always allocate in cache-line-aligned chunks;
