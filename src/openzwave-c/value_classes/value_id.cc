#include <openzwave/value_classes/ValueID.h>
#include "value_id.h"

extern "C" {

// constructors
ValueID * value_id_from_packed_id(uint32 home_id, uint64 id) {
  return new ValueID(home_id, id);
}

ValueID * value_id_from_values(uint32 home_id,
                               uint8 node_id,
                               ValueGenre genre,
                               uint8 command_class_id,
                               uint8 instance,
                               uint8 value_index,
                               ValueType type) {
  return new ValueID(home_id, node_id, genre, command_class_id, instance, value_index, type);
}

// instance methods
uint32 value_id_get_home_id(ValueID * value) {
  return value->GetHomeId();
}

uint8 value_id_get_node_id(ValueID * value) {
  return value->GetNodeId();
}

ValueGenre value_id_get_genre(ValueID * value) {
  return value->GetGenre();
}

uint8 value_id_get_command_class_id(ValueID * value) {
  return value->GetCommandClassId();
}

uint8 value_id_get_instance(ValueID * value) {
  return value->GetInstance();
}

uint8 value_id_get_index(ValueID * value) {
  return value->GetIndex();
}

ValueType value_id_get_type(ValueID * value) {
  return value->GetType();
}

uint64 value_id_get_id(ValueID * value) {
  return value->GetId();
}

// Comparison Operators
bool value_id_eq(ValueID * self, ValueID * other) {
  return self == other;
}

bool value_id_less_than(ValueID * self, ValueID * other) {
  return self < other;
}

}  // extern "C"
