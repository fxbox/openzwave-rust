#include <openzwave/value_classes/ValueID.h>
#include "value_id.h"

extern "C" {

// constructors
ValueID value_id_from_packed_id(uint32 home_id, uint64 id) {
  return ValueID(home_id, id);
}

ValueID value_id_from_values(uint32 home_id,
                             uint8 node_id,
                             ValueGenre genre,
                             uint8 command_class_id,
                             uint8 instance,
                             uint8 value_index,
                             ValueType type) {
  return ValueID(home_id, node_id, genre, command_class_id, instance, value_index, type);
}

// instance methods
uint32 value_id_get_home_id(ValueID *vid) {
  return vid->GetHomeId();
}

uint8 value_id_get_node_id(ValueID *vid) {
  return vid->GetNodeId();
}

ValueGenre value_id_get_genre(ValueID *vid) {
  return vid->GetGenre();
}

uint8 value_id_get_command_class_id(ValueID *vid) {
  return vid->GetCommandClassId();
}

uint8 value_id_get_instance(ValueID *vid) {
  return vid->GetInstance();
}

uint8 value_id_get_index(ValueID *vid) {
  return vid->GetIndex();
}

ValueType value_id_get_type(ValueID *vid) {
  return vid->GetType();
}

uint64 value_id_get_id(ValueID *vid) {
  return vid->GetId();
}

// Comparison Operators
bool value_id_eq(ValueID * self, ValueID * other) {
  return *self == *other;
}

bool value_id_less_than(ValueID * self, ValueID * other) {
  return *self < *other;
}

}  // extern "C"
