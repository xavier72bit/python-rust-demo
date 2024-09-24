import python_rust_demo

if __name__ == '__main__':
    test_num1 = 123543
    test_num2 = 154663

    print(python_rust_demo.add_i32(test_num1, test_num2))
    print(type(python_rust_demo.add_i32(test_num1, test_num2)))

    test_metrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [10, 11, 12],
    ]

    print(python_rust_demo.dimensional_reduction(test_metrix))
