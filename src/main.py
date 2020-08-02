from lexer import *
from interpreter import *

def main():
    while True:
        try:
            text = input('calc> ')
        except EOFError:
            break
        
        if not text:
            continue

        lexer = Lexer(text)
        interpreter = Interpreter(lexer)
        result = interpreter.expr()
        print(result)

if __name__ == '__main__':
    main()