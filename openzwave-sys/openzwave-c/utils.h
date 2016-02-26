#ifdef __cplusplus
extern "C" {
#endif

typedef char * (*RustStringCreator) (char const *);
typedef void * (*RustU8VecCreator) (uint8 const *, const size_t length);
typedef void * (*RustStringVecCreator) (char const **, const size_t length);

#ifdef __cplusplus
}  // extern "C"
#endif
