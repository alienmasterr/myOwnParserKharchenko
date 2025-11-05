## my_own_parser_Kharchenko

this parser is designed to parse simple arythmetic expresions that have simple operators (+-*/) and numbers and show operators, operands and results of the operations

the examples of the expresions to be parsed are here:

1+2 ; 1+ 2 ; 1-2 ; 1/2 ; 1+(-2) ; 1-(-2) ; 1*(-2) ; 1/(-2) ; -(1+2) ; ((1+2))

the parser works in a way that it removes the extra brackets, changes operators' concatanations (exmpl -(-a) = a) and than devides the expresion tooperators and operands and performs arythmetic operations afterwards. it also returns the operators and operands of the expresion after the mentioned previously expression simplification.

link to crates.io : https://crates.io/crates/my_own_parser_Kharchenko
