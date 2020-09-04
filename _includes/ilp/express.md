### Expressivity

<!-- % 
Nandi: I actually think this section may be the most important section in the paper
% It deals with whether we are comparing apples to apples, or apples to pears
% For example: are we comparing "ILP to differentiating" or are we comparing "integrating with differentiating" 
% How interchangeable are the tools (ILP and DL)? 
% Can ILP and DL be applied to the same problems?
 -->

"The class of dyadic logic programs with one function symbol has Universal Turing Machine (UTM) expressivity" \citep{horn-comput}
<!-- https://arxiv.org/abs/1606.05336 -->


The expressiveness of a formal language is a measure of its ability to distinguish structures. We can only compare the expressiveness of two languages if we are interpreting the same class of structures with them. <br>

% It suffices to show that ILP can distinguish two models indistinguishable by DL.

Is there a function which one can represent which the other cannot?<br>

Which system is more expressive? There are at least two contradictory directions:

1) The logical hierarchy
$$ILP \to \mathrm{FOL}$$
$$DL \to \mathrm{propositional\, logic}$$
$$ \mathrm{Expressivity(FOL)} > \mathrm{Expressivity(prop)}$$
$$\therefore  \mathrm{Expressivity(ILP)} >  \mathrm{Expressivity(DL)}$$ 

However, natural neural networks have proven themselves to be very powerful in the past. Humans seem an existence proof that there are neural networks that can learn logic. <br>
% humans, a neural network, came up with logic. existence proof in one direction but not the other

2) The numerical hierarchy

ILP deals with discrete data; <br>
DL deals with continuous data
Continuous data $>$ discrete data
$$\therefore Expressivity(ILP) > Expressivity(DL)$$ 

3) Claim: logic $\subset$ probability <br>
Dubious: inference rules aren't the same.<br>


Deep nets are a universal _continuous_ function approximator.

### Theoretical Expressivity

We first compare the two paradigms on what the algorithms could in theory learn, regardless of how inefficient it would be to learn this.<br>

You could -- in theory -- encode tensors and tensor operations (such as activation functions) as logical sentences. <br>

Likewise, you could in theory enumerate all logical sentences up to length $n$ and then encode them as a gigantic one-hot vector. (You could also come up with a more elegant and shorter description.) A 2-layer neural network that is wide enough can approximate all possible functions from this vector to $\{0,1\}$ arbitrarily closely. % why not just to R^m?
Neural networks are functions from $\mathbb{R}^n$ to $\mathbb{R}^m$. 
Neural networks can approximate any continuous function. % The linear transformations and activation functions don't introduce any discontinuity, so probably only continuous functions? Continuous functions can approximate discontinuous functions pretty well
<br>

### Practical Expressivity


We now compare ILP and DL on what they can be used for in practice.<br>

% ILP can learn "complex relational theories, such as cellular automata[Inoue et al., 2014],event calculus theories[Katzouris et al., 2016], and Petri nets[Bain and Srinivasan, 2018]"

% Base case: FOL with function-free ("example setting")

In ILP it is generally awkward to work with data that is non-categorical. <br>

% Can ILP learn "approximations"? Can ILP search among numbers in R? Can it find/learn new constants to use in the output program? Or does it basically only use variables and known constants
% The pi approximation was an idea by Joar, but we are not yet sure if it works. It's just his intuition that it does
% Joar: I can't see why it wouldn't work, if you give the system a few known numbers as constants (eg a few integers) + functions to make new numbers from old numbers (eg addition, division, etc). Of course, this would be really clunky and slow, but it should be within the "reach" of ILP.

%Suppose you wanted an ILP system to learn an approximation of pi. The background knowledge could include a formula for calculating the surface area of circle based on the radius. The examples could include radiuses and an interval for their corresponding surface area. ILP should be able to output a program that calculates pi. (The background would also need some ingredients to write this program.)<br>

On the other hand it is difficult to express "self-referential" functions using a neural network. 
For example, the following are all hard to learn for a neural network: an identity function; recognizing palindromes; and Fizzbuzz. 
Neural networks are a propositional method, it has no quantifiers and no equality constraints. This makes it hard to make recursion work.

% no logical system satisfying both compactness and the Löwenheim-Skolem property can possess greater expressive power than first-order logic: so in that sense, first-order logic is indeed a “natural” entity.
%https://en.wikipedia.org/wiki/Principles_of_Mathematical_Logic
% HOL...

% recursive policies
        
% "We show that with an infinite signature the higher-order dyadic datalog class $H^2_2$ has universal Turing expressivity though $H_2^2$ is decidable given a finite signature." \citep{metagolPaper}