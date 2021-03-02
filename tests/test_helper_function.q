// test_helper_function.q

// Open namespace test
\d .test

// --------------- TEST GLOBALS --------------- //

// Define enum representing status of executing a function
EXECUTION_STATUS__:`Ok`Error;
EXECUTION_ERROR__:`.test.EXECUTION_STATUS__$`Error;
EXECUTION_OK__:`.test.EXECUTION_STATUS__$`Ok;

// Counter of pass and failure.
PASSED__: 0;
FAILED__: 0;

// List of test items.
MODULES__: `$();

/
* @brief Check if two objects are identical.
* @param test_name {symbol}: Name of the test item.
* @param lhs: left hand side of comparison.
* @param rhs: left hand side of comparison.
\
ASSERT_EQ:{[test_name; lhs; rhs]
  $[-11h ~ type test_name; MODULES__,: test_name; '"test name must be symbol"];
  result:lhs ~ rhs;
  $[result;
    [
      PASSED__+:1;
      (::)
    ];
    [
      FAILED__+:1;
      message:"assertion failed.\n\tleft:", (-3!lhs), "\n\tright:", -3!rhs;
      -2 message;
    ]
  ]
 }

/
* @brief Check if two objects are alike.
* @param test_name {symbol}: Name of the test item.
* @param lhs {string|symbol}: left hand side of comparison.
* @param rhs {string}: left hand side of comparison.
\
ASSERT_LIKE:{[test_name; lhs; rhs]
  $[-11h ~ type test_name; MODULES__,: test_name; '"test name must be symbol"];
  result:lhs like rhs;
  $[result;
    [
      PASSED__+:1;
      (::)
    ];
    [
      FAILED__+:1;
      message:"assertion failed.\n\tleft:", (-3!lhs), "\n\tright:", -3!rhs;
      -2 message;
    ]
  ]
 }

/
* @brief Check if two objects are identical.
* @param test_name {symbol}: Name of the test item.
* @param expr {bool}: Give `1b` for expected result.
\
ASSERT:{[test_name;expr]
  $[-11h ~ type test_name; MODULES__,: test_name; '"test name must be symbol"];
  $[expr;
    [
      PASSED__+:1;
      (::)
    ];
    [
      FAILED__+:1;
      -2 "assertion failed.\n\tleft:1b\n\tright:0b";
    ]
  ]
 }

/
* @brief Check if execution fails and teh returned error matches a specified message.
* @param test_name {symbol}: Name of the test item.
* @param func: interface function to apply
* @param args: list of arguments to pass to the function
* @param errkind {string}: string error kind message to expect. ex.) "Invalid scalar type"
\
ASSERT_ERROR:{[test_name;func; args; errkind]
  res:.[func; args; {[err] (EXECUTION_ERROR__; err)}];
  $[EXECUTION_ERROR__ ~ first res; 
    ASSERT_LIKE[test_name; res[1]; errkind,"*"];
    ASSERT[test_name; 0b]
  ]
 }

DISPLAY_RESULT:{[]
  result:$[FAILED__; "FAILED"; "ok"];
  if[FAILED__; show ([] failed: MODULES__)];
  -1 "test result: ", result, ". ", string[PASSED__], " passed; ", string[FAILED__], " failed";
 }

// ------------------- END -------------------- //

// Close namespace
\d .