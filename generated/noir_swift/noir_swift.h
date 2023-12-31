// File automatically generated by swift-bridge.
#include <stdint.h>
typedef struct Proof Proof;
void __swift_bridge__$Proof$_free(void* self);

void* __swift_bridge__$Vec_Proof$new(void);
void __swift_bridge__$Vec_Proof$drop(void* vec_ptr);
void __swift_bridge__$Vec_Proof$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Proof$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Proof$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Proof$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Proof$len(void* vec_ptr);
void* __swift_bridge__$Vec_Proof$as_ptr(void* vec_ptr);

void* __swift_bridge__$prove_swift(void* circuit_bytecode, void* initial_witness_vec_raw);
struct __private__OptionBool __swift_bridge__$verify_swift(void* circuit_bytecode, void* proof);


