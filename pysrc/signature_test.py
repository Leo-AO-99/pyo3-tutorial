from pyo3_tutorial import signature

if __name__ == "__main__":
    print(signature.kwds_args())
    print(signature.kwds_args(x=1, y=2))
    signature.none_default(None)
    signature.none_default(1)
    signature.args_args(1, 2, 3)