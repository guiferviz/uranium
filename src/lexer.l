
%{ 
#include <iostream>


using namespace std;


int operations_count = 0; 
%} 
  
%% 
[0-9]+ {
    cout << yytext << " --> constant number" << endl; 
    return 2;
} 
"+" {
    cout << yytext << " --> add operator" << endl; 
    operations_count++;
    return 1;
}
\n {
    cout << "New line" << endl;
    return 0;
} 
<<EOF>> {
    cout << "End of file!" << endl;
    return -1;
}
%% 
 
int yywrap() {
    return 1;
}

int main() {
    int i = 0;
    int token = -1;
    do {
        token = yylex();
        cout << "Iteration: " << i << ". Token: " << token << endl;
        i++;
    } while (token != -1);

    cout << "Total number of operations: " << operations_count << endl;

    return 0;
}

