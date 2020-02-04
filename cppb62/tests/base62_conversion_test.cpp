// Copyright (c) Spotify AB

#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MAIN  // in only one cpp file

#include <cstdio>
#include <cstring>
#include <boost/range/size.hpp>
#include <boost/test/unit_test.hpp>
#include "gid.h"
#include "base62_conversion_test_data.h"

namespace spotify::tl::detail {

BOOST_AUTO_TEST_SUITE(Base62ConversionTestSuite)

using namespace test;

const int kBase62MaxLength = 22;

// Note: *TestData test cases assume that gid::from_hex always does the right thing

BOOST_AUTO_TEST_CASE(ConvertToBase62UsingTestData) {
  for (int data_it = 0; data_it < kNumBase62TestDatas; ++data_it) {
    const Base62TestData &test_data = g_base62_test_datas[data_it];
    gid<16> test_data_binary_id;
    test_data_binary_id.fill(0);
    // Verify that test data is correct
    BOOST_REQUIRE(test_data_binary_id.from_hex(test_data.hex_rep));
    BOOST_REQUIRE(strlen(test_data.base62_rep) == kBase62MaxLength);
    BOOST_REQUIRE(test_data.base62_rep[kBase62MaxLength] == '\0');

    // Verify that ConvertToBase62 works
    char base62_rep[kBase62MaxLength + 1];
    memset(base62_rep, 0, sizeof(base62_rep));

    convert_to_base62(base62_rep, test_data_binary_id.data());
    BOOST_CHECK(base62_rep[kBase62MaxLength] == '\0');
    BOOST_CHECK(strlen(base62_rep) == kBase62MaxLength);
    BOOST_CHECK_MESSAGE(strcmp(base62_rep, test_data.base62_rep) == 0,
                        "'" << base62_rep << "' != '" << test_data.base62_rep << "'");
  }
}

BOOST_AUTO_TEST_CASE(ConvertFromBase62UsingTestData) {
  for (int data_it = 0; data_it < kNumBase62TestDatas; ++data_it) {
    const Base62TestData &test_data = g_base62_test_datas[data_it];
    gid<16> test_data_binary_id;
    test_data_binary_id.fill(0);
    // Verify that test data is correct
    BOOST_REQUIRE(test_data_binary_id.from_hex(test_data.hex_rep));
    BOOST_REQUIRE(strlen(test_data.base62_rep) == kBase62MaxLength);
    BOOST_REQUIRE(test_data.base62_rep[kBase62MaxLength] == '\0');

    // Verify that ConvertFromBase62 works
    gid<16> binary_id;
    binary_id.fill(0);
    convert_from_base62(binary_id.data(), test_data.base62_rep);
    BOOST_CHECK_MESSAGE(binary_id == test_data_binary_id,
                        "'" << binary_id.to_hex().c_str() << "' != '"
                            << test_data_binary_id.to_hex().c_str() << "'");
  }
}

BOOST_AUTO_TEST_CASE(ConvertFromBase62WithInvalidData) {
  const char *invalid_base62_reps[] = {
      //"000000000000000000000+",  // Invalid characters (+)
      //"000000000000000000001",   // String is too short (should be at least 22 characters long)
      //"7N42dgm5tFLK9N8MT7fHC8",  // Too large (max is 7N42dgm5tFLK9N8MT7fHC7)
      //"ZZZZZZZZZZZZZZZZZZZZZZ",  // Definately too large to fit in 128 bits
      "6N62dgm5tFLK9N8MT7fHC8",
  };

  const size_t num_reps = boost::size(invalid_base62_reps);
  for (int rep_it = 0; rep_it < num_reps; ++rep_it) {
    const char *invalid_base62_rep = invalid_base62_reps[rep_it];

    // Verify that ConvertFromBase62 fails on the invalid
    gid<16> binary_id;
    BOOST_CHECK_MESSAGE(!convert_from_base62(binary_id.data(), invalid_base62_rep),
                        "Converting base62 string \"" << invalid_base62_rep << "\" should fail");
  }
}

BOOST_AUTO_TEST_SUITE_END()  // Base62ConversionTestSuite

}  // namespace spotify::tl::detail
