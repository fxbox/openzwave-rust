#include <openzwave/value_classes/ValueID.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef OpenZWave::ValueID ValueID;
typedef ValueID::ValueGenre ValueGenre;
typedef ValueID::ValueType ValueType;

// constructors
ValueID * value_id_from_packed_id(uint32 home_id, uint64 id);
ValueID * value_id_from_values(uint32 home_id,
                               uint8 node_id,
                               ValueGenre genre,
                               uint8 command_class_id,
                               uint8 instance,
                               uint8 value_index,
                               ValueType type);

// instance methods
uint32 value_id_get_home_id(ValueID *);
uint8 value_id_get_node_id(ValueID *);
ValueGenre value_id_get_genre(ValueID *);
uint8 value_id_get_command_class_id(ValueID *);
uint8 value_id_get_instance(ValueID *);
uint8 value_id_get_index(ValueID *);
ValueType value_id_get_type(ValueID *);
uint64 value_id_get_id(ValueID *);

// Comparison Operators
bool value_id_eq(ValueID * self, ValueID * other);
bool value_id_less_than(ValueID * self, ValueID * other);

#ifdef __cplusplus
}  // extern "C"
#endif
