from lexer import *
from parser import *
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
        parser = Parser(lexer)
        interpreter = Interpreter(parser)
        result = interpreter.interpret()
        print(result)

if __name__ == '__main__':
    main()