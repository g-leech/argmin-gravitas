---
layout:     math_post
title:      "Notes on inductive logic programming"
baselink:   /ilp
permalink:  /ilp
date:       2020-03-01
author:     Gavin

img:        /img/
published:  true
visible:    1

summary:    A neglected paradigm in AI and its struggles.
quality:    4
categories: AI
confidence: 70%. Not my area. I played with a couple systems and read a dozen papers.
importance: 4
wordcount:  6000
argument:	ilp/argument.html
---

{%  include ilp/links.md        %}


Inductive logic programming (ILP) is a subfield of ML for learning from examples $$E$$ and suitably encoded human "background knowledge" $$B$$, using logic programs to represent both inputs $${E, B}$$ and the output model $$h$$. 

ML took over AI. What ILP shows is that the version of ML which exploded in the last decade is only one restricted form: "statistical ML" or "<a href="{{oral}}">propositional ML</a>".

The (potential) upsides of ILP are in some sense a <i>complement</i> of the benefits of deep learning, which is ubiquitous because of its tolerance of unstructured, noisy, and ambiguous data, and its learning hierarchical feature representations.

The field is tiny. As a suggestive bound on the ratio of investment in ILP vs DL, compare the \~<a href="{{ilp}}">130 researchers</a> (worldwide) listed on the ILP community hub, to the <a href="{{bair}}">400 researchers</a> at a <i>single</i> DL/RL lab, Berkeley AI Research.<br>

This makes comparing it to other paradigms difficult, since "SOTA" means much less. We also don't have theoretical coverage: we don't know the complexity classes of many ILP systems.

<!-- Deep learning (DL) has rapidly increased in capability, without commensurate progress in its interpretability, sample efficiency, or verifiability \citep{xai}, \citep{marcus}. Are there ML paradigms that can substitute or complement DL, to make up for this? <br>
 -->

ILP was motivated by the promise of learning from structured data (for instance, recursive structures), and of better knowledge representation. The resulting approach has interesting properties. For example, ILP yields relatively short, human-readable models, and is often claimed to be sample efficient (though finding comparative data on this was difficult, for me).<br>

<!-- 'predicate invention' promises to allow future ILP systems to learn completely new assumptions necessary to explain the data  -->



<!-- Recent developments are promising: work on learning higher-order programs dramatically improves on previous first-order logic (FOL) systems \citep{hoilp}; probabilistic forms of ILP attempt to handle noise and ambiguity in the data \citep{pilp}; and there is a plethora of attempts at neural-symbolic or differential versions of ILP (see \hyperref[sec:neuralsym]{Section 5.2}). For instance, the $$\partial$$ILP system from researchers at Deepmind used a differentiable ILP system with the aim of integrating inductive search with DL modules \citep{diffilp}. <br> -->


<!-- % http://people.csail.mit.edu/kersting/profile/PROFILE_ilp.html -->


## Background

As the name suggests, ILP is <a href="{{induc}}">inductive logic</a> plus <a href="{{flach}}">logic programming</a>: it constructs logic program generalisations from logic program examples. Both the data and the resulting hypothesis are represented in formal logic, usually of first- or second-order. For computability reasons, systems use subsets of first-order logic, often the <a href="{{cl}}">definite clausal logic</a>. 


<center>
    <img src="/img/ilp/formula.png" /><br><br>
    <i>The parts of a logical formula, which could be input data, a constraint, or the output model.</i>
</center>
<br>

The output of a call to an ILP system (what is induced by the learning algorithm) takes several names: a 'theory', a 'hypothesis', a 'program', a 'concept', or a 'model' (in the machine learning sense, and not the logical sense of a truth-value interpretation). <br>

ILP is a collection of methods, rather than one technique or even family of algorithms, due to the many systems not based on Prolog-like inference, and the many nonsymbolic ILP systems. Our working criterion is just that the output of an 'ILP' system should be a logic program.<br>


<!-- % Great idea for a diagram: lattice of all the fields that border ILP
% top: elements: inductive logic, logic programming, neural stuff
% Theorem proving / SAT solving / program synthesis / ILP / % neural -->

### How it works

In the classic setting, the examples $$E$$ are labelled with a binary class: positive examples $$E^+$$ and negative examples $$E^-$$. An ILP system searches a hypothesis space $$\mathcal{H}$$ until a program $$h$$ is found such that $$B \land h \models E^+$$, and such that  $$\forall e\in E^- \, B\land h \not\models e$$. In practice this is weakened in two ways: firstly by heuristic scoring, so that <i>most</i> positives and few negatives are covered by $$h$$; and secondly by using $$\theta$$-subsumption rather than normal (undecidable) FOL entailment. $$h$$ is then a relational description, in terms of $$B$$, of some concept common to the positive examples and absent from the negatives.<br>

<!-- % TODO: supervised and unsupervised  -->

The normal ILP setting assumes that atoms are either true or false, and that hypotheses have a binary domain. Thus the first ILP designs produced only binary classifiers. But 'upgraded' (relational)\citep{laer} forms of many propositional ML techniques have been developed: multi-class classification \citep{clark}, regression with decision trees \citep{kramer-tree}, clustering \citep{brugh}, and even visual object classification \citep{plane}. This bivalence also entails the inability of early, exact ILP systems to handle ambiguous data.<br>

We can view ILP as a search of the 'subsumption lattice', the graph that results from partially ordering hypotheses in $$\mathcal{H}$$ from most general ($$\mathrm{true} \to E^+$$) to most specific (the bottom clause $$\bot$$, a conjunction of evaluated predicates). 

<br>

<center>
    <img src="/img/ilp/subsumption.jpg" /><br><br>
    <i>A subsumption lattice to search.</i>
</center>
<br>


 
 
<!-- % hypothesis generation = clause discovery -->
The lattice gives us two obvious approaches to hypothesis discovery in ILP: 

* 'bottom-up' search, starting from an initially long clause (i.e. the feature values of individual examples), finds a specific clause to generalise from, and drops or abstracts away literals until a minimally general hypothesis that covers $$E^+$$ is found. This specific-to-general search direction is the default approach in the classic ILP systems Progol and Aleph. 

* 'top-down' search proceeds from a short clause (for instance, the empty implication <code>true</code>, and adds literals to it until the expression becomes too specific to cover the examples. This might involve generating candidate clauses from a template, then testing these clauses against $$E$$, branching through the lattice when violations are found. This general-to-specific approach is used in Metagol and $$\partial$$ILP. <br>

The expressive power of (even subsets of) first-order logic leads to ILP's computational complexity: the resulting combinatorial search over large discrete spaces is a notably difficult problem: it's in $$NP^{NP}$$. As a result, various forms of heuristic scoring are used to guide and prune the search.<br>

<!-- % TODO: 'first order logic typically leads to computationally expensive algorithms because they often involve combinatorial searches in vast discrete spaces.'
%  Its main virtue, the highly expressive representation language, is also the cause of its high computational complexity
 -->
<!-- %\textcolor{purple}{The predicates used in the output program can be handcoded as formulae or (to some extent) invented during learning.}<br> -->

Table 1 relates the various biases of ILP and DL. For instance, we can draw an analogy between the 'program template' that constrains an ILP hypothesis space and the architecture of a neural network; both constrain the hypothesis space and, until recently, both have been entirely handcrafted, though recent results in neural architecture search promise automation of  bias provision. Divide inductive bias into 

* language bias (hypothesis space restriction), 
* procedural bias (how the search is ordered; also called 'search bias'), and 
* simplicity bias (how overfitting is prevented).

I don't know the standard term for the many ways to bias an ILP run (mode declarations, program templates, meta-rules). So I'll call them user-supplied constraints (UCs).

<center>
    {%  include ilp/table1.html     %}
</center>

<!-- Both DL and ILP programs depend on strong inductive bias to find a good model. In DL inductive bias takes the form of architecture search. The architecture of Neural network architecture search has been successful in some cases \citep{elsken}. However, program template search is still an open problem: early experiments in brute-forcing templates gave understandably poor results, and the only other related work to date involves the intelligent <i>selection</i> or distillation of metarules, rather than generation \citep{cropper-reduce}.<br> -->


{%  include ilp/algos.html %}

<!-- % Important distinction between train-time and run-time mechanism
% or is this in fact real? See arch == template point
%Both ILP and DL learn a model based on input data.
%There is however a big difference in how this model is obtained.
%In ILP we use a search algorithm that finds a program (also known as output hypothesis).  This program is the learned model in ILP.
%In DL we start with a neural network with random weights and based on desired input-output pairs the network is updated.
%The trained neural network, i.e. the learned model has the same structure as the the original neural network, it just has different weights.

% Even more important homology between ANN architecture and program template: both handcrafted (aside from Neural architecture search), 
%(To make things more confusing, in some ILP systems, program templates can fully determine the structure of the learned model. See Section [].) -->

{%     include ilp/class.md     %}







## Theoretical bounds 

One of the dirty secrets of computer science is that formal proofs about computability and complexity are often practically useless. (Neural networks trained with RL have dented PSPACE-hard problems like playing Go, and more generally worst-case theories like Rademacher complexity <a href="{{adv}}">overestimate</a> the actual generalisation error of neural networks.) But even then, still interesting.

<div class="accordion">
<h3>How do we beat complexity results?</h3>
<div>
    * Giving up worst-case performance: Worst-case complexity is not the same as average-case complexity \citep{impag}. Since, by definition, the algorithm will be dealing with average instances most of the time, worst-case performance may not be of practical importance. (However, this complacent view may make a system vulnerable to adversarial attack.)
    <br><br>
    * Giving up optimality: If we assume diminishing returns to optimisation, a suboptimal solution within a fixed margin (\(\epsilon\)) of the optimal one may be exponentially (\(\exp{1/\epsilon}\)) faster to find.
    <br><br>
    * Giving up correctness: Randomisation can reduce hardness significantly. Also, the cost of giving up (necessary) correctness can be offset in some cases by several independent runs of the algorithm that make the probability of an error vanish exponentially in the number of runs.
    <br><br>
    * Giving up generalisation: A narrower algorithm may be faster and still useful in most cases.
</div>
</div>
<br>

[Section forthcoming]

### Expressivity

### Computability

### Complexity

### Generalisation

<!-- %  include ilp/express.md %} -->

<!-- %  include ilp/comp.md %}

%  include ilp/complex.md %}

%  include ilp/gen.md %} -->

<br>

{%     include ilp/limits.md    %}




<!-- % \subsection{What would extremely capable ILP be like?}

% This is Very related to the theoretical expressivity of ILP
% TODO: include this property Y/N?
% it is more expressive than 'propositional' ML, since it can represent arbitrary programs [TODO:CITATION NEEDED];

% Random: Take as input positive examples of math theorems and negative examples in the form of incorrect statements, who knows, it might be able to deduce very impressive new theorems

% Random: Find a theorem of everything if you have detailed enough examples and background. (Then computational power could become important.)

% Random: Deep learning can find any function that fits the data. ILP can find any hypothesis that is consistent with the data. -->





<br><br>

## See also

* Our paper looking at ILP from an AI safety perspective.

<br><br>

_Thanks to Javi Prieto, Nandi Schoots, and Joar Skalse for many, many comments._

<br><br>


