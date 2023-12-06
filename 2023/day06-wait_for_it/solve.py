from sympy import symbols, solve

button, allowed_time, record = symbols('button allowed record')

equation = button * (allowed_time - button) - record

solutions = solve(equation, button)
print(solutions)