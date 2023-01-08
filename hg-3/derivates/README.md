# 1. Derivated number and tangent
## a) Variation rate
$$\frac{f(b) - f(a)}{b-a}$$
$$\frac{\Delta{f}}{\Delta{x}}(a;b)$$
## b) Derivated number of a function in a real $\mathbb{R}$
f is derivable only if:
$$f'(a)=\lim_{h\to0}{\frac{f(a+h)-f(a)}{h}}$$
## c) Tangent
A line passing through the point A, with a director coefficient of $f'(a)$
Equation:
$$y=f'(a)(x-a)+f(a)$$
# 2. Derivated functions
## a) On the $I$ interval
A function is derivable on $I$ if derivable for every $\mathbb{R}\in I$. Derivated function of $f(x)$ is noted $f'(x)$.
## b) Reference functions
|$f(x)=C$|$f'(x) = 0$|
|---|---|
|$f(x)=x$|$f'(x) = 1$|
|$f(x)=x^2$|$f'(x) = 2x$|
|$f(x)=\frac{1}{x}$|$f'(x) = -\frac{\normalsize1}{\normalsize{x^2}}$|
|$f(x)=\sqrt{x}$|$f'(x) = \frac{\normalsize1}{\normalsize2\sqrt{x}}$|
|$f(x)=x^n$|$f'(x) = nx^{n-1}$|
### Demonstration for $f(x) = x^2$
$$\frac{(a+h)^2 - a^2}{h} = \frac{a^2+2ah+h^2-a^2}{h} = \frac{2ah+h^2}{h} = 2a$$
# 3. Operations on derivation
## a) Derivation of a sum of functions
If $u$ and $v$ are functions and are derivable on $I$, the sum of $(u+v)' = u'+v'$
## b) Derivation of a multiplication of functions
If $u$ and $v$ are functions and are derivable on $I$, the product of $(u \times v)' = u'v + v'u$
## b) Derivation of a quotient of functions
If $u$ and $v$ are functions and are derivable on $I$, the product of $(\frac{\normalsize u}{\normalsize v})' = \frac{\normalsize{u'v - v'u}}{\normalsize{v^2}}$
# 4. Composition of functions and derivation
$$f\in J, g \in I$$
$$h(x)=f(g(x))$$
$$x \longrightarrow  g(x) = X \longrightarrow f(X)$$
$$h(x) = x \longrightarrow f(g(x))$$
## Derivation of a composed function
$h'(x) = axf'a(ax+b)$ where $g(x) = ax+b$