% Owner Nandi
\subsection{ TODO: Neural-Symbolic Systems: Integrated or Modular? }
\label{sec:neuralsym}

% Muggleton: "replace absolute logical reasoning with a relaxed version that yields continuous values reflecting the confidence of the conclusion. Although this approach limits the expressivity of hypotheses"

% Three types, from Turning 30
% 1) imitating logical reasoning with tensor calculus 
% represent predicates as binary tensors over the domain of constants and perform reasoning by chains of tensor products imitating a clause. 
% [Yang et al., 2017; Dong et al., 2019].

% 2) relaxing the subset selection problem in which the task of a neural network is to select a subset of clauses from a space of pre-defined clauses. 
% [Evans and Grefenstette, 2018; Si et al., 2019] 

% 3) neural theorem provers [Rocktäschel and Riedel, 2017] turn the learning problem towards learning to perform soft unification, which unifies not only the matching symbols but also similar ones, from a fixed set of proofs.

% Do neural approaches to program induction lose guarantees, deductive, readable?
% No, since the output hypothesis is still analytic, symbolic. And since ILP systems working off heuristic search so rarely give you a full explanation of how it was learned.
% 'It is notoriously hard to understand what it has learned, or to what extent it has generalised beyond the training data'


% We can make logic neural or make the neural symbolic \citep{neuralsymb}

% Add diagram illustrating the 2/3 approaches
% |NN| -> |ILP|
% |backprop through ILP|
% ...

% very early (1999) attempt at neural ILP
% https://link.springer.com/article/10.1023/A:1008328630915

In the wider field of AI, an extraordinary amount of recent work has tried to unify connectionist and symbolic methods, mostly in the form of trying to implement relational or logical reasoning on neural networks, chasing breakthrough capabilities a la ImageNet [TODO CITATION] or GPT-2 [TODO CITATION]. 
% TODO: % I suppose many of the capabilities that are chased are things like proving or other logical tasks
From the safety perspective, this unification often loses the properties we originally targeted. <br>

% [rocktaeschel paper] takes a differentiable approach to proving theorems.
% is this relevant? we are not looking at proving per se, more at the method of ILP in particular
% other papers to cite:
% Garnelo et al., 2016, towards deep symbolic reinforcement learning; 
% Hudson and Manning, 2018, compositional attention networks for machine reasoning
% Denil et al., 2017, programmable agents; 


% other names to search for
% Statistical Relational Learning, Probabilistic Logic Learning ...

% Francesca Toni wrote Explanatory predictions with artificial neural networks and argumentation https://spiral.imperial.ac.uk/bitstream/10044/1/62202/2/main.pdf

%"principled hybrids of structure-based methods and deep learning" https://arxiv.org/pdf/1806.01261.pdf#page=25
% In the spirit of numerous recent examples of principled hybrids of structure-based methods and deep learning e.g.:
% Reed and De Freitas, 2016, Neural programmer-interpreters;
% Ritchie et al., 2016, deep amortized inference for probabilistic programs; 
% Wu et al., 2017, learning to see physics via visual de-animation; 
% Artur, Computing First-Order Logic Programs by Fibring Artificial Neural Networks;
% L Serafini, AA Garcez, Logic tensor networks: Deep learning and logical reasoning from data and knowledge

% Bibliography for Neural-symbolic
    % - [RNN implemented without free variables](http://www.staff.city.ac.uk/~aag/papers/flairs2005.pdf)
            
    %         - [Acyclic logic programs from forward chaining](https://www.sciencedirect.com/science/article/pii/S0925231208002051)
            
    %         - [Real Logic abduction](https://arxiv.org/abs/1606.04422)
            
    %         - [HMMs](https://www.aaai.org/Papers/JAIR/Vol25/JAIR-2512.pdf), one atom at a time
            
    %         - 'Covers' as conditional probability. [Naive Bayes](https://people.csail.mit.edu/kersting/ecmlpkdd05_pilp/pilp.pdf) for single nonrecursive clauses
            
    %         - [Single nonrecursive definite clause](https://openaccess.city.ac.uk/id/eprint/297/2/First-order_Logic_Learning_in_Artificial_Neural_Networks.pdf)
            
    %         - [Bottom Clause Propositionalization](https://link.springer.com/article/10.1007/s10994-013-5392-1), very memory efficient 

    %         - [Backward chaining neural theorem prover](https://www.aclweb.org/anthology/W16-1309.pdf), learns clauses and vector embeddings simultaneously.
            
    %         - $\delta$ILP is reportedly fatally memory intensive. Binary predicates or less.


When looking at the safety properties of neural-ILP systems we distinguish between integrated and modularised systems. By integrated we mean an ILP algorithm that uses subsymbolic processing, the whole way from input until output. By modularised we mean a system that has pure ILP modules alternated with neural modules. e.g. preprocessing.<br>

In Section \ref{improv-list}, we discussed potential improvements that can be made to ILP to make it more competitive with other ML methods. Some of these improvements require only attaching a preprocessing module to the main ILP module, which likely won't have much impact on the interpretability of the algorithm as a whole. One way to view this is that preprocessing changes the problem that needs to be solved, but doesn't change the algorithm that is used to solve it.
Other forms of modular neural-ILP systems include student-teacher distillation, where we have a core ILP student module and a neural teacher module \citep{dnn-rules}.<br>
% I'm not thinking super straight and right now I don't know which one should be neural and which one should be ILP

On the other hand, end-to-end differentiable versions of ILP are fundamentally different algorithms in that they are no longer based on a purely logical non-probabilistic core ILP module.
A differentiable version of ILP is introduced in \citep{diffilp}. This approach and similar integrated neural-symbolic versions of ILP may lose some of the interpretability properties, which we think are its main safety advantage, but an integrated neural approach may be necessary to deal with ambiguity.

% TODO: To what extent is the safety of a system the safety of the least safe module?
% when we include DL, what safety properties can be saved? do we need to keep DL out of the core model?

% ILP used as symbolic planner for RL agent
% https://github.com/921kiyo/symbolic-rl/blob/master/report.pdf

