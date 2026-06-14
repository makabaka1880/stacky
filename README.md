# Stacky
A simple stack-based language for educational purposes.

> Stacky is isomorphic to RPN when not using pop. I found it a very interesting example of using programs to describe expressions.

## Syntax
The abstract syntax for Stacky is given below:

$$
\begin{aligned}
p & ::= && s; p \; \\
& \mid && \texttt{skip} \\
s & ::= && \texttt{push}(a) \\
& \mid && \texttt{pop} \\
& \mid && \texttt{squish}(a, n) \\
\end{aligned}
$$

In this interpreter I implemented a parser for a s-expression based concrete syntax:

```
stmt    ::= (push <value>)
        |   (pop)
        |   (squish <value> <int>)
prog    ::= (begin <prog1> <prog2> ...)
        |   stmt
```

Where `<value>` is any valid identifier.

A program is a statement or the concatenation of programs via `begin`. That is, the parser flattens out nested `begin` constructs into a linear program. 

You dont explicitly write $\texttt{skip}$; the program terminates when it hits the end of the linear expansion.

## Semantics

### Configuration

Stacky is a stack-based language. The interpreter maintains a global stack of **string labeled trees**. 

$$
\begin{aligned}
\sigma  &::= t : \sigma \;\mid\; \emptyset \\
t  &::= \texttt{Node}(v) \;\mid\; \texttt{Branch}_v(v_1, v_2, \dots) 
\end{aligned}
$$

The domain of $v$ here is the same as `<value>` in the above concrete syntax.

The configuration $D$ of the operational semantics for Stacky is defined as a tuple $\langle P \;,\; \sigma \rangle$ of the remaining program and the global stack.

### Small-Step Semantics

The evaluation transition relation is defined as a heterogeneous relation over the configuration tuple:

$$
\to : D \times D \cup \{\texttt{Err}\}
$$

You can push a value to the stack, forming a node:

$$
\dfrac{}{\langle \texttt{push}(v); P \;,\; \sigma \rangle \to \langle P \;,\; \texttt{Node}(v):\sigma \rangle} \text{(Push)}
$$

Or pop the top element off:

$$
\dfrac{\sigma = t : \sigma'}{
    \langle \texttt{pop};P \;,\; \sigma \rangle \to \langle P \;,\; \sigma' \rangle
} \text{(Pop)}
$$

$$
\dfrac{}{\langle \texttt{pop};P \;,\; \emptyset \rangle \rightarrow \texttt{Err}} \text{(Pop-Err)}
$$

And also "squish" top $n$ elements by popping them off, creating a new tree with a node $v$ as the root, putting those $n$ elements under the root as children, and pushing the new tree back in:

$$
\dfrac{\sigma = t_1 : t_2 \cdots : t_n : \sigma'}{
    \langle \texttt{squish}(v,n);P \;,\; \sigma \rangle \to \langle P \;,\; \texttt{Branch}_v (t_1, t_2, \dots, t_n) : \sigma' \rangle
} \text{(Squish)}
$$

$$
\dfrac{\sigma = t_1 : t_2 \cdots : t_k : \emptyset \quad k < n}{
    \langle \texttt{squish}(v,n);P \;,\; \sigma \rangle \to \texttt{Err}
} \text{(Squish-Err)}
$$

Program termination:

$$
\dfrac{}{\langle \texttt{skip} \;,\; \sigma \rangle \nrightarrow} \texttt{(Stasis)}
$$

## Some Corollaries

> **Theorem**. Consider any tree $t$ and stack $\sigma$. Then there always exist program $P$ such that
> $$ 
> \langle P \;,\; \sigma \rangle \to^\ast \langle \texttt{skip} \;,\; t : \sigma \rangle 
> $$

**Proof**. We proceed by structural induction of $t$.

**Leaf**. Consider when $t = \texttt{Node}(v)$. Then $P = \texttt{push}(v); \texttt{skip}$ evaluates to $\texttt{Node}(v)$:

$$
\langle \texttt{push}(v); \texttt{skip} \;, \; \sigma \rangle \xrightarrow{\text{Push}} \langle \texttt{skip} \;,\; \texttt{Node}(v):\sigma \rangle \nrightarrow
$$

$\square$

**Tree**. Consider when $t = \texttt{Branch}_v(t_1, t_2, \dots, t_n)$. 

By the induction hypothesis, for any tree $t_i$ and any starting stack state $\sigma_i$, there exists a program $P_i$ that constructs $t_i$ and places it on top of the stack:
$$\langle P_i \;,\; \sigma_i \rangle \to^\ast \langle \texttt{skip} \;,\; t_i : \sigma_i \rangle$$

Because the induction hypothesis holds for *any* arbitrary stack, we can chain these programs together by evaluating the children in reverse order, from $t_n$ down to $t_1$:

1. Let $\sigma_n = \sigma$. By IH, there exists $P_n$ such that:
   $$\langle P_n \;,\; \sigma \rangle \to^\ast \langle \texttt{skip} \;,\; t_n : \sigma \rangle$$
2. Let $\sigma_{n-1} = t_n : \sigma$. By IH, there exists $P_{n-1}$ such that:
   $$\langle P_{n-1} \;,\; t_n : \sigma \rangle \to^\ast \langle \texttt{skip} \;,\; t_{n-1} : t_n : \sigma \rangle$$
3. We repeat this sequence for all children. At the final inductive step for $t_1$, let $\sigma_1 = t_2 : \dots : t_n : \sigma$. There exists $P_1$ such that:
   $$\langle P_1 \;,\; t_2 : \dots : t_n : \sigma \rangle \to^\ast \langle \texttt{skip} \;,\; t_1 : t_2 : \dots : t_n : \sigma \rangle$$

By sequencing these subprograms together, we construct a compound program $P_{\text{children}} = P_n ; P_{n-1} ; \dots ; P_1$ that transforms the stack configuration as follows:
$$\langle P_{\text{children}} \;,\; \sigma \rangle \to^\ast \langle \texttt{skip} \;,\; t_1 : t_2 : \dots : t_n : \sigma \rangle$$

We then construct the total program $P = P_{\text{children}} ; \texttt{squish}(v, n) ; \texttt{skip}$. Evaluating $P$ from the initial configuration yields:

$$
\begin{aligned}
\langle P_n ; P_{n-1} ; \dots ; P_1 ; \texttt{squish}(v, n) ; \texttt{skip} \;,\; \sigma \rangle 
& \to^\ast \langle \texttt{squish}(v, n) ; \texttt{skip} \;,\; t_1 : t_2 : \dots : t_n : \sigma \rangle \\
& \xrightarrow{\text{Squish}} \langle \texttt{skip} \;,\; \texttt{Branch}_v(t_1, t_2, \dots, t_n) : \sigma \rangle \nrightarrow
\end{aligned}
$$

Thus, the claim holds for complex trees. $\blacksquare$