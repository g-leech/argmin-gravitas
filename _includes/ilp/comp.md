## Computability

% LIL is finite time convergence to correct hypothesis
%\href{https://en.wikipedia.org/wiki/Language_identification_in_the_limit}{LIL}
% Constraint ILP \citep{richardIL} is identifiable in the limit given heavy conditions
% 1994: "Current learnability results address only the definite and the example settings of inductive logic programming." 
% Nandi: Does this deal with completeness as well?

As noted in Section 2, ILP is a search for clauses that entail given positive examples, but entailment in general is semi-decidable [CITATION NEEDED]. This means that whenever $H_1\vdash H_2$, there is an algorithm to prove it, but no algorithm exists that can prove $H_1\not\vdash H_2$ when $H_1\not\vdash H_2$. If we restrict ourselves to Horn clauses in function-free FOL we can even give a sharper characterisation: finding out whether $H_1\vdash H_2$ is undecidable if $H_1$ has more than one atom in its body \citep{marcinkowski92}. To circumvent this issue, we must provide a suitable notion of provability ($\models$) to the ILP algorithm.<br> 

A reasonable candidate is implication, which is both correct 
    $$(A\models B) \,\Rightarrow\, (A\vdash B)$$ 

and complete 
    $$(A\vdash B) \,\Rightarrow\, (A\models B).$$

However, even in this setting, the problem remains undecidable in general and even for smaller sets of FOL clauses (see \citep{kietz1994}, theorems 2 through 6).<br>

A better-behaved alternative to implication is $\theta$-subsumption. Given a variable substitution $\theta$, we say $H$ $\theta$-subsumes $e$, and write $H\leq_{\theta} e$, if the literals in $H\theta$ are a subset of those in $e$. For example, if $H=P(X,Y)$ and $e=P(a,b)$ we can show $H\leq_{\theta} e$ simply taking $\theta=\{X/a, Y/b\}$.<br>

% Soundness and completeness do not imply decidability.

It can be shown that $\theta$-subsumption is correct, but not complete, since it is decidable [CITATION NEEDED]. However, completeness is guaranteed if we restrict ourselves to non self-resolving clauses, which means we cannot consider predicates like married(X,Y) :- married(Y,X)\citep{kietz1994}. [TODO: this is incomplete, see theorem 10 in \citep{kietz1994}]. Given these results, we restrict the following sections to ILP problems using $\theta$-subsumption.

% the subsumption theorem states that logical implication between clauses can be divided in two separate steps: a derivation by resolution, and then a subsumption. Hence the theorem provides a natural \bridge" between logical implication and subsumption.

% refutation completeness of resolution for unsatisfiable sets of clauses

% Robinson’s 1965 resolution algorithm is a complete proof procedure for FOL
% https://dl.acm.org/doi/10.1145/321250.321253

% [Is this true? By Church-Turing [citation needed] Entailment for FOL is semidecidable: algorithms exist that say yes to every entailed sentence, but no algorithm exists that also says no to every nonentailed sentence]

% partial correctness proof
% if an answer is returned, it will be correct
% http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.34.3280&rep=rep1&type=pdf
