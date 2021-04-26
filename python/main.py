                # Python is an interpreted language, so its entry point is the 
                # beginning of the script file. There is no main function in 
                # python programs.

class Fibonacci:
    def __init__(self, n):
        self.value = 0
        self.prev = 1
                # Python is a dynamically typed language, meaning that the 
                # types of variables need not be declared. Above, we can see
                # that two integer variables are being created without a type
                # specifier.
        for _ in range(n):
            temp = self.value
            self.value += self.prev
            self.prev = temp
    
    def __repr__(self):
        return self.value
    
    def __str__(self):
        return str(self.value)

if (__name__ == "__main__"):
    n = int(input("Find the fibonacci number at index: "))
    fib = Fibonacci(n)
                # We can see in the line above that we are passing an argument
                # to a function. In python, passing happens by object
                # reference. This means that primitive types are passed by
                # value, but objects are passed by reference.
    print(f"The {n}th fibonacci number is {fib}")