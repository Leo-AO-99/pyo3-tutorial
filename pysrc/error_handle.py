from pyo3_tutorial import math

if __name__ == "__main__":
    try:
        print(math.divide(1, 0))
    except ZeroDivisionError as e:
        print(e)
        
    try:
        print(math.parse_int("1aa23"))
    except ValueError as e:
        print(e)