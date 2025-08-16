#ifndef values_h
#define values_h

extern "C" {
  enum ValueType {
    DOUBLE,
    BOOL,
  };


  struct Value {
    ValueType type;
    union {
      double i;
      bool b;
    } data;
  };

  Value*  getDoubleValue(double i);

  Value* getBoolValue(bool b);

  struct ValueArray {
    Value* values;
    int size;
    int capacity;
  };

  ValueArray* createValueArray(int capacity);
  void addValueToArray(ValueArray* array, Value value);
  void freeValueArray(ValueArray* array);
  void increaseValueArrayCapacity(ValueArray* array);

}

#endif