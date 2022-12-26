# Generalities on sequences

## Definiton
> sequence is a function $u$ defined on $\N$ or $\{n\in\N, n\ge{n_0}\}$ where $n_0\in\N$ fixed. For all unsigned integer $n$ associates a real $u(n)$ noted $u_n$

## Notation
A sequence function $\N$ describes a n-step as $u_n$
### Example
$(u_n)$ defined $\forall n \in\N$ by $u_n=n^2-1$

### Note
When $u_n$ is expressed based on $n$ we can calculate by replacing with the rank. Is is called *defined by an explicit form*.

## Defintion, defined by recurrence
> Relation allowing to calculate a term based on previous term(s)
### Example
$$(u_n)\space u_0=-6 \space \forall n \in \N \space u_{n+1}=3u_n+15$$

# Arithmetic sequence
## Definition
> A sequence $(u_n)$ is arithmetic if a real $r$ exists, called common difference, such as for all $n\in\N$ we have $u_{n+1}=u_n+r$
## Note
To demonstrate that a sequence $(u_n)$ is arithmetic, we can search to prove that for all $n\in\N$, $u_{n+1}-u_n = r$ where $r$ is a constant.
## Property: Expression of the general term
Let $(u_n)$ an arithmetic sequence of common difference $r$

> For all $n\in\N,u_n=u_0+r\times n$
> For all $n\in\N,u_n=u_1+r\times (n-1)$
> For all $n\in\N,\space p\in\N\space u_n=u_p+r\times (n-p)$

### Graphical reference
> Points of graphical representation are placed on the line $y=r\times x + u_0$

# Geometrical sequence
## Definition
> A sequence $(u_n)$ is geometric if a real $q$ exists called common ratio, such as for all $n\in\N$ we have $u_{n+1}=q\times u_n$

## Note
* To demonstrate that a sequence $(u_n)$ is geometric, we need to prove that $u_{n+1}=q\times u_n$
If terms aren't null, we can prove that $\frac{u_{n+1}}{u_n} = q$ where $q$ is constant.
* Variation between two consecutive terms is constant.

## Property: Defintion in general terms
Let $(u_n)$ a geometrical sequence of common ration $q$.

> For all $n\in\N, u_n = u_0\times{q^n}$
> For all $n\in\N, u_{n-1} = u_0\times{q^{n-1}}$
> For all $n\in\N,\space p\in\N\space u_n=u_p\times q^{n-p}$

### Graphical reference
Exponential

# Calulus of sums
## Property: Sum of n firsts integers
> For all integer $n \ge 1$, we have $1+2+...+n=\Large\frac{n\times{(n+1)}}{2}$

### Demonstration
$$S = 1 + 2 + ... + (n-1) + n$$
$$S = n + (n-1) + ... + 2 + 1$$
If we add both equalities we obtain:
$$S + S = (1+n) + (1+n) + ... + (1+n) + (1+n)$$
$$2\times S= n \times (1+n)$$
$$S = \frac{n(n+1)}{2}$$

## Property: Sum if n first powers
> For all real $q\ne1$ and for all integer $n\ge1$, we have:
> $q^0+q^1+q^2+...+q^n=\Large\frac{1-q^{n+1}}{1-q}$

### Demonstration
$$S = 1+q+q^2+...+q^n$$
$$q \times S = q+q^2+q^3+...+q^{n+1}$$
$$S - q \times S = 1+q+q^2+...+q^n-q-q^2-q^3-...-q^{n+1}$$
So $S - q \times S = 1 - q^{n+1}$
$$S \times (1-q) = 1 - q^{n+1}$$
Because $q\ne1$, we have $S=\Large\frac{1-q^{n+1}}{1-q}$

# Direction of variation of a sequence
## Defition: Direction of variation of a sequence
> Let $(u_n)$ a sequence and $k$ an interger.
> - The sequence $(u_n)$ is **increasing** from rank $k$ if, for all $n \ge k, u_{n+1} \ge u_n$
> - The sequence $(u_n)$ is **decreasing** from rank $k$ if, for all $n \ge k, u_{n+1} \le u_n$
> - The sequence $(u_n)$ is **monotone** from rank $k$ if it is either increasing from rank $k$, or either decreasing from rank $k$.
> - The sequence $(u_n)$ is **constant** from rank $k$ if, for all integer $n\ge k,u_{n+1}=u_n$.

### Note
* As with function if we replace $\ge, \le$ with $<, >$ we talk about **srictly increasing**, **strictly decreasing**, and **strictly monotone**.

## Propery: Study of the sign of $u_{n+1} - u_n$
> Let $(u_n) a sequence:$
> - if $u_{n+1}-u_n > 0$, then the sequence is strictly increasing
> - if $u_{n+1}-u_n < 0$, then the sequence is strictly decreasing

### Example
The sequence $(u_n), u_0=5,\forall n \in \N,u_{n+1}=u_n+n^2+1$ is strictly increasing because $u_{n+1}-u_n=n^2+1$ and $n^2+1 > 0$.

## Property: Comparison between $\Large\frac{u_{b+1}}{u_n}$ and $1$
> Let $(u_n)$ a sequence of which all terms are strictly positives
> - If $\Large\frac{u_{n+1}}{u_n}\small>1$, so ${u_{n+1}} > u_{n'}$, sequence strictly increasing
> - If $\Large\frac{u_{n+1}}{u_n}\small<1$, so ${u_{n+1}} < u_{n'}$, sequence strictly decreasing

## Property: Direction of variation of an arithmetic sequence
> Let $(u_n)$ be an arithmetic sequence of common difference $r$.
> - If $r > 0$, so the sequence is strictly increasing
> - If $r < 0$, so the sequence is strictly decreasing
> - If $r = 0$, so the sequence is constant

## Property: Direction of variation of a geometric sequence
> Let $(u_n)$ be a geometric sequence of common ratio $q$ and of first term $u_0 \ne 0$.
> - If $q > 1$:
>   - if $u_0>0$, so the sequence is strictly increasing
>   - if $u_0<0$, so the sequence is strictly decreasing
> - If $0 < q < 1$:
>   - if $u_0>0$, so the sequence is strictly decreasing
>   - if $u_0<0$, so the sequence is strictly increasinbg
> - If $q =0$ or $q=1$, the sequence is constant
> - If $q<0$, the sequence isn't monotone

# Limit of a sequence
## Definition: Sequence having a real as limit
> A sequence $(u_n)$ has for limit a real $l$ when $n$ tends to $+\infty$, if the terms $u_n$ become as close to $l$ as we want by taking $n$ sufficiently big. We can say that $(u_n)$ converges to $l$ and we note it:

$$\lim_{n\to+\infty}u_n=l$$

## Definition: Sequence having $+\infty$ as limit
> A sequence $(u_n)$ has for limit $+\infty$ when $n$ tends to $+\infty$, if the $u_n$ terms become as big as we want by taking a $n$ sufficiently big.
> We can say that $(u_n)$ diverges and we note it:
$$\lim_{n\to+\infty}u_n=+\infty$$

## Definition: Sequence having $-\infty$ as limit
> A sequence $(u_n)$ has for limit $-\infty$ when $n$ tends to $+\infty$, if the $u_n$ terms become as small as we want by taking a $n$ sufficiently big.
> We can say that $(u_n)$ diverges and we note it:
$$\lim_{n\to+\infty}u_n=-\infty$$

### Note
Some sequences do not have limits. In this case they diverge.
Let the sequence $(u_n)$ defined by $u_n=(-1)^n$
![graph representation for $(u_n)$ defined by $u_n=(-1)^n$](assets/screenshot1.png)