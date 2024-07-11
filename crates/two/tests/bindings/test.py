import unittest

from one import *
from two import *

# BROKEN: error "attempted relative import with no known parent package"

class TestCoverall(unittest.TestCase):

    def test(self):
        # self.assertEqual(new_record_default(), new_record_default())
        print("Works?")
                         
if __name__=='__main__':
    unittest.main()