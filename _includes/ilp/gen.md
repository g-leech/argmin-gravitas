### Generalisation

% without recursion, it is often difficult for an ILP system to generalise from small numbers of examples [Cropperet al., 2015].
% Meta-interpretive learning of data transformation programs

% ILP systems have a defence against overgeneral clauses built-in
% Strong Consistency. $B \cup H cup E \not\models \square$
% Empirical question, sure, but also


% DL: generalises despite its range of learning disadvantages: '[many parameters], complexity, possible algorithmic instability, nonrobustness, and sharp minima'

% ILP "can generalise better from few examples". But data is increasingly available for more or less every domain, so this is of decreasing relevance.

% one place we should be able to compare them directly:
% we have generalization PAC-Bayes error bounds for DL
% https://arxiv.org/abs/1805.08522
% should be able to derive them for ILP if we can't find them for others

One way of a machine learning system generalising well is: not overfitting, which means having a low test error relative to the training error.

A common way to avoid overfitting in ILP systems is to prevent too much specialisation (moving down the subsumption lattice toward concrete exemplars), in effect forcing the output hypothesis to become more abstract.<br>
% Cropper makes the reasonable claim that ILP generalises from **few examples** much better with recursive programs
% https://www.doc.ic.ac.uk/~shm/Papers/datacurate.pdf

Overfitting is prevented in DL systems through a bag of tricks intended to overcome the 'laziness' of the training procedure (that is, the network's tendency to only learn what it necessary to represent a particular training set). Dropout, which randomly deletes connections between units, could be interpreted as abstracting over ... 
Another way to encourage abstraction in deep learning is regularization: adding layers to the network which penalize strong weight / (...) <br>
% TO DO gavin

A machine learning algorithm that can perform well with data that is taken out of a different distribution that its learning data is is said to perform well out of distribution. This is a form of generalization as well. <br>

Some deep learning methods are set-up to perform well out of distribution [CITATION needed].
ILP is in general not able to handle data out of its training distribution. For example, if you train a system on traditional family data in which kids can have only one dad, then the output program may not be able to deal with data in which someone has two dads. 
(This is much worse in the case of mistaken background knowledge, for instance $\forall x,  (\,\mathrm{father}(x) \to \mathrm{man}(x)\,)$ - since most systems take B to be absolutely certain.)