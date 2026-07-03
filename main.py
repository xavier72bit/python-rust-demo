import example_py_module


if __name__ == '__main__':
    test_num1 = 123543
    test_num2 = 154663

    print(example_py_module.add_i32(test_num1, test_num2))
    print(type(example_py_module.add_i32(test_num1, test_num2)))

    test_metrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [10, 11, 12],
    ]

    print(example_py_module.dimensional_reduction(test_metrix))
    print(type(example_py_module.dimensional_reduction(test_metrix)))
    
    print(example_py_module.sum_as_string(test_num1, test_num2))
    print(type(example_py_module.sum_as_string(test_num1, test_num2)))
