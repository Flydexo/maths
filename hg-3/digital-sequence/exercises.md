# Exercises: Digital Sequence

* [source](https://manuel.sesamath.net/numerique/index.php?ouvrage=ms1spe_2019&page_gauche=56)

# TODO
### Corrected exercises 
- [x] Page 56           
- [x] Page 57           
- [x] Page 58           
- [ ] Page 59           
- [ ] Page 60           
- [ ] Page 60           
- [ ] Page 61    
- [ ] Page 62           
- [ ] Page 63            
### Application exercises
- [ ] 34-36 page 64
- [ ] 40-42 page 64
- [ ] 50-54 page 65
- [ ] 61-63 page 66
- [ ] 70-72 page 66
- [ ] 77-79 page 67
- [ ] 83-85 page 67
### Training exercices
- [ ] 88-90 page 68
- [ ] 105-107 page 69
- [ ] 116-118 page 70
- [ ] 128-129 page 71
### Deepening exercises
- [ ] 135, 137, 139 page 73
- [ ] 143,144 page 74
- [ ] 147,148,149,150 page 75
### Practical work
- [ ] 1 page 76
- [ ] 160-162 page 78
- [ ] 178-180 page 79

# Exercises
## Page 56
### 1. Calculate terms defined by an explicit formula
1. Calculate 3 first terms of the sequence $(u_n)$ defined $\forall n \in \N$ by $u_n = 2n +1$

> 1, 3, 5

---

### 1
1. Calculate first 4 terms of $(u_n)$ $u_n = n^3$
> 0, 1, 3, 8
2. 10 first terms
> 0, 1, 3, 8, 27, 64, 125, 216, 343, 512
### 2
Let $(u_b)$ a sequence, defined $u_n = -n + 5$
1. Calculate $u_0$ and $u_1$
> $u_0 = 5$, $u_1=4$

---

### 2 Relation of recurrence
1. 3 first terms, $(u_n)$, defined by $u_0=1$ and $u_{n+1}=2u_n+1$
> 1, 3, 7
2. 3 firsts terms, $(v_n)$, defined by $v_0=2$ and $v_{n+1}=v_n+n+3$
> 2, 5, 9

---

### 3
1. 3 first terms, $(u_n)$, $u_0=2$, $u_{n+1}=u_n^2$
> 2, 4, 16

### 4
Let $(w_n)$, $w_0=-1$, $w_{n+1}=-w_n+n$
1. $w_1, w_2$
> -1, 0

---

### 3 Model with Sequence
Sport club has 500 subs in 2019. Each year 80% stay subscribed and new 20 people subscribe. $(u_n)$ is the number of subs in $2019 + n$.

1. Subs in 2020
$$u_{n+1}=u_n\times\frac{4}{5}+20$$
$$u_{2O20}=500\times\frac{4}{5}+20$$
$$u_{2O20}=420$$
2. Express $u_{n+1}$ based on ${u_n}
$$u_{n+1}=u_n\times\frac{4}{5}+20$$
3. How many subs in 2030 (with calculator)
> 134
4. When the gym hits 101 subs it will close, will it happen and when ?
> Yes, in 2046   

---

### 5
A company has 200 employees in 2019, each year 10% leave and 30 join. $(u_n)$ is the number of employees in $2019 + n$.
1. How many employees in 2020, 2021 ?
$$u_{n+1}=u_n\times\frac{9}{10}+30$$
$$u_{2020}=200\times\frac{9}{10}+30$$
$$u_{2020}=210$$
$$u_{2021}=210\times\frac{9}{10}+30$$
$$u_{2021}=219$$
2. Express based on $n$
$$u_{n+1}=u_n\times\frac{9}{10}+30$$

### 6
In a shop a sweater costs 60$, each period of sale the price decreases by 15%.
1. Price after first period of sale
$$u_{n+1}=u_n\times \frac{85}{100}$$
$$u_1=60\times \frac{85}{100}$$
$$u_1=51$$
2. We note $(u_n)$ the price of the sweater after $n$ period of sale.
    - a. $u_0$ and $u_1$
  
        > 60, 51
    - b. Express based on $n$
        $$u_{n+1}=u_n\times \frac{85}{100}$$