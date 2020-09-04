## Limitations 

#### <i>Naivety about noise and ambiguity</i>. 

In simple concept learners, a single mislabelled example can prevent learning entirely. However, progress has been made in handling noise and ambiguity: 
    first, the low-hanging fruit of detecting mutual inconsistency in data (by deriving contradictions); and second limiting how far a top-down specialisation should go, when noise is assumed to be present.
    <!-- % Nandi: after this one sentence "explaining how mutual inconsistency in data is found" would be good -->
    <!-- % Nandi: after this one sentence clarifying what it means to "limit how far down a specialization should go" -->
    Further, the use of learned hypotheses for transfer learning between runs of ILP is limited by the noise a given learned program is likely to have picked up: current systems assume that background knowledge (like a transferred program) is certain.
    
#### <i>Large resource requirements</i>. 

We discussed the terseness of ILP outputs as a virtue, but it's equally true that present systems cannot learn large theories given practical compute. 
<!-- % Nandi: imo terseness and large theories are not polar opposites: terseness means that the info that needs to be expressed is expressed concisely; a large theory here means a theory that contains a lot of info. But maybe it's because I'm not a native speaker and in any case it's not extremely important. I would consider to just remove the "Above, terseness" part if you don't feel like being more precise -->
The space complexity of admissible search (that is, algorithms guaranteed to yield an optimum) is exponential in hypothesis length for some systems like Progol. For this reason, predicate invention is limited even in state-of-the-art systems to, at most, dyadic predicates. This appears related to the expressivity of FOL. 
<!-- % Forward-chaining (as in $$\partial$$ILP, NLM) doesn't scale. -->
    

#### <i>Handcrafting task-specific inductive biases</i>. 

Almost all ILP systems use user-supplied constraints to generate the candidate clauses that form predicates (for instance 'mode declarations' in Progol and Aleph, meta-rules in Metagol, or rule templates in $$\partial$$ILP). In some sense these are just hyperparameters, as found in most ML systems. But templates can be enormously informative, up to and including specifying which predicates to use in the head or body of $$h$$, the quantifier of each argument in the predicate, which arguments are to be treated inputs and outputs, and and so on. Often unavoidable for performance reasons, templates risk pruning unexpected solutions, involve a good deal of expert human labour, and lead to brittle systems which may not learn the problem structure, so much as they are passed it to begin with.<br><br>This is an open problem, though recent work has looked at <a href="{{min}}">selecting</a> or <a href="{{comp}}">compressing</a> given templates. The $$\partial$$ILP authors also report an experiment with generating templates, but the authors note that at least their brute-force approach is straightforwardly and permanently infeasible\citep{diffilp}.
    <!-- % Rayani  https://arxiv.org/pdf/1906.03523.pdf -->

<!-- * <i>Negative learnability results</i>. A number of proofs find that various ILP settings and algorithms are not learnable in the limit; only fragments have thus far been shown to be PAC-learnable for instance (see \hyperref[sec:sample]{Section 3.4} for a detailed treatment). -->
    
 <!-- <i>Single answers with no traces</i>. Most ILP systems yield just one output program, and the heuristics used in high-performance systems must discard the counterfactual clauses (and paths of clauses), to constrain the search. However, nontrivial learning problems tend to have many aspects (i.e. ultimately unused clauses) of independent interest to the end-user. Consider the "top-5" outputs of modern computer vision classifiers, which give several calibrated answers for a given input. By default, ILP would give only the 'top-1' predicted label and offer only a sketch of the search process. -->
    
#### <i>Usability</i>. 

ILP systems remain a tool for researchers, and specialists at that: to our knowledge, no user-friendly system has so far been developed. (The <a href="{{rdm}}">RDM Python wrapper</a> is a partial exception. To some extent this is due to the data representation: somewhat more than a working knowledge of first-order logic and (usually) Prolog are required to input one's own data. Whether this is a higher barrier than the basic linear algebra required for modern deep learning libraries, or merely a rarer skill in the data science community, would require empirical study. But the effect is the same: ILP seems more difficult to use.
    
#### <i>The deep threat to knowledge representation</i>. 

The "knowledge representation" programme is challenged by rapid progress in learned representations and end-to-end deep learning in computer vision, natural language processing, and many other fields. Rich Sutton summarises this challenge as "<a href="{{sutt}}">the bitter lesson</a>": that massively scaling dataset size and model size tends to outperform hand-crafting of features and heuristics by domain experts. This is a 'limitation' of ILP insofar as it cannot itself follow suit and take advantage of learned representations to the same degree.


An even more contentious claim: perhaps we shouldn't expect human-sized steps in advanced machine reasoning. If the bitter lesson holds in general, then expert elicitation is dead.

<br>