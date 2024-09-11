#!/bin/bash

## FULL DOCS: https://rust-book.cs.brown.edu/ch11-02-running-tests.html

###  the default way of running all tests
# cargo test

### You can specify command line options to change this default behavior. Some
### command line options go to cargo test, and some go to the resulting test binary.
### To separate these two types of arguments, you list the arguments that go to cargo
### test followed by the separator -- and then the ones that go to the test binary.
### ---------------------------------------------------------------------------------
### Running cargo test --help displays the options you can use with cargo test, and
### running cargo test -- --help displays the options you can use after the separator.#

### If you don’t want to run the tests in parallel or if you want more fine-grained
### control over the number of threads used, you can send the --test-threads flag
### and the number of threads you want to use to the test binary
# cargo test -- --test-threads=1

### If we want to see printed (stdout) values for passing tests, we can tell Rust to
### also show the output of successful tests with --show-output.
### cargo test -- --show-output

### We can pass the name of any test function to cargo test to run only that test:
### We can’t specify the names of multiple tests in this way...
### cargo test -- it_adds_two

### But there is a way to run multiple tests -- We can specify part of a test name,
### and any test whose name matches that value will be run. For example, because two
### of our tests’ names contain add, we can run those two by running cargo test add.
# cargo test add

## Also note that the module in which a test appears becomes part of the test’s name,
## so we can run all the tests in a module by filtering on the module’s name.
# cargo test tests::it

### Sometimes a few specific tests can be very time-consuming to execute, so you might
### want to exclude them during most runs of cargo test using the #[ignore] attribute
### If we want to run only the ignored tests, we can use cargo test -- --ignored:
# cargo test -- --ignored

#  If you want to run all tests whether they’re ignored or not, you can run
cargo test -- --include-ignored
