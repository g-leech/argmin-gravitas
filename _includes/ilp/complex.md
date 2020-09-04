## Complexity


Subsumption is NP-complete \citep{cheng}. This puts ILP in $\Sigma_2^P$-complete, the class of problems that are in NP even if we have access to an oracle for NP \citep{gottlob1997}. The intuition behind this is simple: recall that in ILP we don't just try to subsume a single clause, but find an $H$ subsuming all clauses in $E^{+}$ and none in $E^{-}$. We can do this with two oracles, one for guessing $H$ and one for finding $\theta$. The complexity result simply states that these two oracles are in NP and that they are necessary and sufficient.<br>

Can we do any better by relaxing some of the assumptions? Yes, but not by much. Instead of aiming for exact, deterministic learnability, we can try to efficiently learn a hypothesis that is close enough to the true one with high probability. Making 'efficiently', 'close enough', and 'high probability' more precise leads to the definition of the class of problems that are "polynomially probably approximately correct learnable" or polynomially PAC-learnable. These are the problems for which there is an algorithm with the following properties:
%
\begin{itemize}
    \item It runs in polynomial time in input size and inverse error.
    \item The coverage of the output hypothesis is highly likely to have substantial overlap with that of the true hypothesis.
\end{itemize}

This definition is not strictly about complexity since it also depends on the sampling distribution $D$. However, it serves as an intermediate step to establish whether a problem lies in $RP$, the class of decision problems for which a polynomial randomised algorithm with success probability $>1/2$ exists if the answer is yes (see theorem 1 in \citep{kietz1994}). The upshot is that polynomial PAC-learnability implies a problem is in $RP$ [CITATION needed?].<br>

- \citep{gottlob1997} contains a lot of negative results showing that many tweaks don't suffice to take ILP decision problems out of $\Sigma_2^P$<br>

- \citep{kietz1994} contains a single positive result (theorem 13): i j-determinate k-discriminative nonrecursive function-free predicate definitions are polynomially PAC-learnable under arbitrary distributions. This implies the problem is in RP (randomised polynomial).

%So far we have only considered FOL. What happens if we allow higher-order predicates? %TODO


#### Sample complexity

The strong inductive bias imposed by the background knowledge and the program template allow ILP systems to generalise well from small sample sizes, sometimes even managing one-shot learning \citep{lin-shot}.<br>

% but SOMETIMES so does DL
% http://www.cs.cmu.edu/~rsalakhu/papers/oneshot1.pdf


The sample complexity of ILP in this setting has been rigorously studied by Cropper and Muggleton in the context of their proposed algorithm meta-interpretive learning (MIL). MIL is an approach to ILP that handles predicate invention through a series of metarules (second-order Horn clauses) \citep{muggleton14}. Within this framework, the sample complexity of MIL for PAC-learning is linear in the hypothesis size and number of body literals contained in the metarules and logarithmic in the number of background predicates and the number of metarules (see Proposition 2 in \citep{cropper}). <br>
% this makes it PAC-leranable and thus in RP, right?

% Bayesian approaches have also supported sample complexity analysis of predicate invention within the framework of repeat learning. In this framework it is assumed that the learner's prior is not equivalent to the distribution from which the teacher is sampling targets. By providing a series of sessions the learner is able to update the initial prior by adding and deleting background predicates." \citep{mugg98}

%"Additionally we show that Knuth–Bendix ordering of the hypothesis space together with logarithmic clause bounding allows our MIL implementation Metagol to PAC-learn minimal cardinality $H^2_2$ definitions" \citep{metagolPaper}

In deep learning, the relation between sample size and learning is more complex. Conventional wisdom in statistical learning theory roughly says that if a model has more parameters than it has training data, it will overfit [CITATION NEEDED]. However, empirical results in deep learning seem to contradict this, with deep neural networks having low test error even when trained on relatively small datasets. A recent attempt to reconcile these two phenomena invokes what has been called \textit{double descent} \citep{doubledescent}. According to this hypothesis, the test error is non-monotonic with respect to sample size in some settings: there is \textit{critically parametrised regime} that happens when the number of training samples is approximately equal to some measure of model complexity. In this regime, more training or larger models hurt test error instead of improving it, as opposed to what happens in the under- and over-parametrised regimes. This phenomenon has been shown to hold across several popular deep learning models like CNNs or transformers.
% conventional wisdom means without regularisation

% neural ILP
% consider the 50,000 examples used in ILP (page 17)
% https://arxiv.org/abs/1904.11694

% What the fuck was that 200 sample computer vision paper???
% https://openreview.net/pdf?id=HkxLXnAcFQ
% https://arxiv.org/abs/1805.10123
% https://arxiv.org/abs/1909.02729

% DL                - Sejnowski: Deep learning should have bad sample complexity, from convex opt proof                - but then GP

% Paper covering many learnability results
% https://dl.acm.org/doi/abs/10.1145/181668.181674


\subsubsection{TODO: Time complexity}     
\label{sec:time}

Finding a hypothesis $h$ with background $B$ has complexity in $O\left(\, (|h|\cdot|B|)^{|h|} \,\right)$ \citep{orallo}.<br>

'function free universally quantified Horn expressions are exactly learnable... possible with membership and equivalence queries with resources polynomial in the number of clauses in the expression, though superpolynomial in the number of universally quantified variables. Similar results for related models including entailment queries and ILP are also derived.' \citep{khardon}<br>

%        - "a large majority of result concerning PAC learnability for ILP are negative."
%            - existentially quantified formulas are not PAC-learnable
        
%        - single definite clauses with bounded indeterminacy and polynomial literal support are PAC-predictable

Time or proof-depth bounding ...

% "Apart from a few special cases PAC-learning results have been largely negative for ILP. This is in large part due to the fact that testing satisfiability is intractable for most interesting subsets of first-order Horn logic. The development of Bayesian approaches to ILP supported the development of U-learnability, which allows classes of distributions over the hypotheses. Here it was shown that for any exponential-decay distribution the class of time-bounded logic-programs is polynomially U-learnable.

% The main issue preventing ILP from taking over the AI landscape is its time-complexity [haha what an overstatement].

% Some actual Big-O results here, p.6:
% http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.34.3280&rep=rep1&type=pdf

% TODO: https://people.cs.kuleuven.be/~luc.deraedt/acai1.pdf

% \subsubsection{Space complexity}
% \label{sec:space}
% The space complexity of Cautious Inductive Logic System (CILS) [TODO: what is this??] is upper bounded by ??. The space complexity of Progol is upper bounded by ?? (theorems 18-19) \citep{anthony97}. % TODO add the dILP handwaving about RAM