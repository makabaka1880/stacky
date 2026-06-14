#import "@preview/curryst:0.6.0": rule, prooftree, rule-set


= Stacky: Syntax & Semantics

Stacky is a stack-based language for educational purposes. It is isomorphic to RPN when not using `pop` — a nice example of using programs to describe expressions.

== Abstract Syntax

$
  p ::= s; p | "skip" \
  s ::= "push"(a) | "pop" | "squish"(a, n)
$

== Concrete Syntax (S-expression)

The reference interpreter parses an S-expression surface syntax:

$
  "stmt" ::= ("push", "value") | ("pop") | ("squish", "value", "int") \
  "prog" ::= ("begin", "prog"_1, "prog"_2, dots) | "stmt"
$

Where `value` is any valid identifier. A program is a statement or the concatenation of programs via `begin`; the parser flattens nested `begin` blocks into a linear program. The terminal `skip` is not written explicitly — the program terminates when it reaches the end of the linear expansion.

== Configuration

Stacky maintains a global stack of _string-labeled trees_.

$
  sigma ::= t : sigma | nothing \
  t ::= "Node"(v) | "Branch"_v (v_1, v_2, dots)
$

The domain of $v$ matches `value` in the concrete syntax above. The configuration $D$ of the operational semantics is a tuple:

$
  D = angle.l P, space sigma angle.r
$

== Small-Step Semantics

The evaluation relation is heterogeneous over the configuration:

$
  arrow : D times D union {"Err"}
$

#let push  = prooftree(rule(name: [Push],  $ angle.l "push"(v); P, space sigma angle.r  arrow  angle.l P, space "Node"(v) : sigma angle.r $))
#let pop   = prooftree(rule(name: [Pop],   $ sigma = t : sigma' $, $ angle.l "pop"; P, space sigma angle.r  arrow  angle.l P, space sigma' angle.r $))
#let pop-err = prooftree(rule(name: [Pop-Err], $ angle.l "pop"; P, space emptyset angle.r  arrow  "Err" $))
#let squish = prooftree(rule(name: [Squish],
  $ sigma = t_1 : t_2 dots : t_n : sigma' $,
  $ angle.l "squish"(v, n); P, space sigma angle.r  arrow  angle.l P, space "Branch"_v(t_1, t_2, dots, t_n) : sigma' angle.r $,
))
#let squish-err = prooftree(rule(name: [Squish-Err],
  $ angle.l "squish"(v, n); P, space sigma angle.r  arrow  "Err" quad "when " |sigma| < n $,
))

#v(0.5em)
#align(center, rule-set(
  push,
  pop,
  pop-err,
  squish,
  squish-err,
))

*Stasis* — terminal configuration:

#v(0.3em)
$
  angle.l "skip", space sigma angle.r arrow.not
$

== Expressiveness

#block[
  *Theorem.* For any tree $t$ and stack $sigma$, there exists a program $P$ such that
  $
    angle.l P, space sigma angle.r arrow^* angle.l "skip", space t : sigma angle.r
  $
]

*Proof.* By structural induction on $t$.

*Leaf.* Let $t = "Node"(v)$. Then $P = "push"(v); "skip"$:

$
  angle.l "push"(v); "skip", space sigma angle.r arrow^( "Push" ) angle.l "skip", space "Node"(v) : sigma angle.r arrow.not
$

*Tree.* Let $t = "Branch"_v (t_1, t_2, dots, t_n)$. By the induction hypothesis, for each $t_i$ and any stack $sigma_i$ there exists $P_i$ such that $angle.l P_i, space sigma_i angle.r arrow^* angle.l "skip", space t_i : sigma_i angle.r$.

Because the IH holds for _any_ stack, chain the children in reverse order ($t_n$ down to $t_1$):

1. Let $sigma_n = sigma$. There exists $P_n$ where $angle.l P_n, space sigma angle.r arrow^* angle.l "skip", space t_n : sigma angle.r$.
2. Let $sigma_(n-1) = t_n : sigma$. There exists $P_(n-1)$ where $angle.l P_(n-1), space t_n : sigma angle.r arrow^* angle.l "skip", space t_(n-1) : t_n : sigma angle.r$.
3. Continue for all children. At the final step for $t_1$, let $sigma_1 = t_2 : dots : t_n : sigma$. There exists $P_1$ where $angle.l P_1, space t_2 : dots : t_n : sigma angle.r arrow^* angle.l "skip", space t_1 : t_2 : dots : t_n : sigma angle.r$.

Sequencing these subprograms gives $P_("children") = P_n; P_(n-1); dots ; P_1$:

$
  angle.l P_"children", space sigma angle.r arrow^* angle.l "skip", space t_1 : t_2 : dots : t_n : sigma angle.r
$

Construct the total program $P = P_"children"; "squish"(v, n); "skip"$:

$
  angle.l P_n; P_(n-1); dots ; P_1; "squish"(v, n); "skip", space sigma angle.r \
  arrow^* angle.l "squish"(v, n); "skip", space t_1 : t_2 : dots : t_n : sigma angle.r \
  arrow^( "Squish" ) angle.l "skip", space "Branch"_v(t_1, t_2, dots, t_n) : sigma angle.r  arrow.not
$

Thus the claim holds for all trees. $square$
