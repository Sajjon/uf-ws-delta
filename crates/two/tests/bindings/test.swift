import two

public func test() throws {
  assert(newRecordDefault() == newRecordDefault())
  assert(newRecordDefault() == newRecord(one: newOneDefault(), two: newTwoDefault()))
  assert(AlphaObject.newDefault() == AlphaObject.newDefault())
  assert(objectRecord(value: AlphaObject.newDefault()) == newRecordDefault())
  assert(recordObject(value: newRecordDefault()) == AlphaObject.newDefault())

  do {
    let r = newRecord(one: newOne(value: true), two: newTwo(value: true))
    assert(r == recordRecord(value: r))
    assert(r == recordRefRecord(value: r))
  }

  do {
    let o = AlphaObject(one: newOne(value: true), two: newTwo(value: true))
    assert(o == objectObject(value: o))
    assert(o == objectRefObject(value: o))
  }

}

try! test()
