import com.sajjon.one.*
import com.sajjon.two.*

fun test() {
    assert(newRecordDefault() == newRecordDefault())
    assert(newRecordDefault() == newRecord(one = newOneDefault(), two = newTwoDefault()))
    assert(AlphaObject.newDefault() == AlphaObject.newDefault())
    assert(objectRecord(value = AlphaObject.newDefault()) == newRecordDefault())
    assert(recordObject(value = newRecordDefault()) == AlphaObject.newDefault())
  
    val r = newRecord(one = newOne(value = true), two = newTwo(value = true))
    assert(r == recordRecord(value = r))
    assert(r == recordRefRecord(value = r))
  
    val o = AlphaObject(one = newOne(value = true), two = newTwo(value = true))
    assert(o == objectObject(value = o))
    assert(o == objectRefObject(value = o))
}

test()