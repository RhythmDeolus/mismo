#include "values.h"


Value* getDoubleValue(double i) {
  Value* v = new Value;
  v->type = DOUBLE;
  v->data.i = i;
  return v;
}

Value* getBoolValue(bool b) {
  Value* v = new Value;
  v->type = BOOL;
  v->data.b = b;
  return v;
}


ValueArray* createValueArray(int capacity) {
  ValueArray* array = new ValueArray;
  array->values = new Value[capacity];
  array->size = 0;
  array->capacity = capacity;
  return array;
}

void addValueToArray(ValueArray* array, Value value) {
  if (array->size >= array->capacity) {
    increaseValueArrayCapacity(array);
  }
  array->values[array->size++] = value;
}

void freeValueArray(ValueArray* array) {
  delete[] array->values;
  delete array;
}

void increaseValueArrayCapacity(ValueArray* array) {
  int newCapacity = array->capacity * 2;
  Value* newValues = new Value[newCapacity];
  for (int i = 0; i < array->size; ++i) {
    newValues[i] = array->values[i];
  }
  delete[] array->values;
  array->values = newValues;
  array->capacity = newCapacity;
}

