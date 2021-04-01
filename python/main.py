class Fibonacci:
    def __init__(self, n):
        self.value = 0
        self.prev = 1
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
    print(f"The {n}th fibonacci number is {fib}")