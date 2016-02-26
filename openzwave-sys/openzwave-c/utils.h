#ifdef __cplusplus
extern "C" {
#endif

#define DEFINE_RUST_VEC_CREATOR(name, type) \
  typedef void * (*name) (const type *, const size_t length);

typedef char * (*RustStringCreator) (char const *);
DEFINE_RUST_VEC_CREATOR(RustU8VecCreator, uint8)
DEFINE_RUST_VEC_CREATOR(RustStringVecCreator, char*)
DEFINE_RUST_VEC_CREATOR(RustI32VecCreator, int32)

#ifdef __cplusplus
}  // extern "C"
#endif
