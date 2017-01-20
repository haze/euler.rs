from os import sys

CARGO_FILE = "../Cargo.toml"
SRC_FOLDER = "../src/"

"""
[[bin]]
name = "euler-rs-p4"
path = "src/euler_four.rs"
"""

if len(sys.argv) != 3:
    print "need two arguents: add_problem.py five 5"
else:
    NEW_FILE_PATH = (SRC_FOLDER + "euler_{0}.rs").format(sys.argv[1])
    NEW_RUST_SRC = open(NEW_FILE_PATH, "w+")
    CARGO_FILE = open(CARGO_FILE, "a+")
    CARGO_FILE.write('\n[[bin]]\nname = "euler-rs-p{}"\npath = "{}"'
                     .format(sys.argv[2], NEW_FILE_PATH[3:]))
    CARGO_FILE.close()
    NEW_RUST_SRC.close()
