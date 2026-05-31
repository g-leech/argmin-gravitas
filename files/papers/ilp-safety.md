---
title: "Safety Properties of Inductive Logic Programming"
authors:
  - Gavin Leech
  - Nandi Schoots
  - Joar Skalse
gleech_role: first author
year: 2020
venue: "AAAI 2021 workshop (SafeAI), CEUR-WS Vol-2808"
type: workshop
url: http://ceur-ws.org/Vol-2808/Paper_14.pdf
links:
  video: https://youtu.be/leQ56mahNMs?t=605
  outtakes: https://www.gleech.org/ilp
contribution_hours: 100
topics: [ai-safety, inductive-logic-programming, interpretability, formal-verification]
---

## Abstract

We examine an obscure kind of machine learning — inductive logic programming
(ILP) — to see whether it is (or could be) safer than neural networks, using an
existing AI-safety framework formalised to allow the comparison.

Upsides: ILP is convenient for specification, robust to some syntactic input
changes, gives greater control over inductive bias, can actually be formally
verified, and produces interpretable results (you can read the model). But ILP
is so far limited to domains with neat symbolic data, can't do architecture
search, and lags far behind NNs on almost all tasks. Hybrid ILP/NN systems look
like they would lose most of what we like about ILP in the first place.

## Full text

> Converted from the CEUR-WS workshop PDF (Vol-2808, Paper 14). PDF markup removed;
> the reference list omitted. Definitions and the comparison table are kept.

**Abstract.** This paper investigates the safety properties of inductive logic
programming (ILP), particularly as compared to deep learning systems. We consider the
following properties: ease of model specification; robustness to input change; control
over inductive bias; verification of specifications; post-hoc model editing; and
interpretability. We find that ILP could satisfy many of these properties in its home
domains. Lastly, we propose a hybrid system using ILP as a preprocessor to generate
specifications for other ML systems.

### Introduction

Symbolic approaches to AI are sometimes considered safer than neural approaches; we
investigate this for one symbolic approach, inductive logic programming (ILP) — a
declarative subfield of ML for learning from examples and encoded "background
knowledge" (predicates and constraints), using logic programs to represent both inputs
and the output model. We survey existing ILP and deep-learning (DL) work in light of
the safety framework of Ortega and Maini (specification, robustness, assurance), and
formalise robustness to input change and model editing. Consider an ML system "safe"
when its goals are specified correctly, it acts robustly according to those goals, and
we are assured of these two properties. ILP may be a natural fit for the *assurance*
side, since often not just the output model but the learning process takes place at a
relatively high (symbolic) level. (We refer to "ILP" as monolithic, but ILP systems
differ widely in search strategy, exactness, completeness, target logic, noise handling,
predicate invention, and theory order — which limits general statements.)

### Safety properties of ILP

**Model specification.** A defining feature of ILP is user-specified background
knowledge, a natural way to impose specifications (a problem = positive examples,
negative examples, and background). Given background $B$ and model $M$: $M$ is *weakly
consistent* if $B \wedge M \not\models \text{False}$, and *strongly consistent* if $B
\wedge M \wedge E^+ \not\models \text{False}$. To guarantee $M$ satisfies a
specification $s$, encode $s$ in first-order logic (FOL) and add it to $B$. Caveats:
noise-handling systems only *nudge* toward example-encoded specifications; probabilistic
ILP can violate even weak consistency; incomplete algorithms may miss an existing
solution; some specifications are hard to encode as FOL (as in computer vision, where
hand-coded concepts were outperformed by learned ones); and *human values are hard to
encode* as FOL (deontic logic partially formalises norms, but a complete encoding seems
unlikely given inconsistency across people and the contextual nature of value). In DL,
most methods impose only *soft* constraints (loss modifications), since hard constraints
are computationally impractical for million-parameter networks.

**Robustness to input change.** Does a slight input change produce only a slight output
change? DL is often insensitive to small input changes but vulnerable to adversarial
ones. The paper defines similarity of datasets (overlap of compatible hypothesis sets,
complexity-weighted), similarity of hypotheses (high agreement probability on sampled
instances), and robustness of a learner $L$ (similar inputs → similar outputs, with
parameters $r_D, r_M$). **ILP is robust to syntactic input change** (renaming
atoms/predicates, variable substitution, duplicate examples don't change the output;
though example *order* can change which consistent hypothesis is found first, e.g. in
Metagol). Semantic changes (e.g. negating one example) can completely change the output.
Robustness can be assessed empirically by generating ILP problems at known distances and
measuring output-hypothesis distances.

**Control over inductive bias.** Inductive bias = the assumptions used to generalise
finite inputs to a model. Its two components are limiting the hypothesis space and
guiding the search. In DL, the hypothesis space is largely the network architecture
(controllable), but the simplicity/search bias is poorly understood (little explicit
control). In ILP, the hypothesis space is restricted by the FOL fragment (classically
definite Horn clauses) and a strong, informative language bias from user constraints
(mode declarations, meta-rules, program templates — specifying predicates, argument
quantifiers/types, input/output, length bounds); search is top-down or bottom-up with
information-gain or probabilistic scoring. ILP's strong bias lets it perform well on
small datasets. *Caveat:* heavily customised inductive biases shade into hand-coding the
solution — risking pruned solutions, expert labour, and brittleness — which automating
bias choice could mitigate.

| Bias | ILP | DL |
|---|---|---|
| Simplicity | Bound on program length | Not well understood (besides regularisers, e.g. dropout, LR decay) |
| Language | User constraints, target logic | NN architecture |
| Search | Search order, hypothesis scoring | Local gradient search |

**Verification of specifications.** Determining whether a model satisfies a
specification is NP-hard for both neural networks and logic programs. In practice,
verifying an ILP output is often easy: check whether $M$ satisfies an arbitrary Datalog
specification $s$ by running resolution on $M \cup \{\neg s\}$ to see if it derives
False (possible even for some non-Datalog $s$, e.g. the Bernays–Schönfinkel fragment, in
double-exponential time). In DL, verifying even simple properties is NP-complete;
complete solvers handle thousands of nodes, incomplete methods ~100k ReLU nodes.

**Post-hoc model editing.** Find a model $M'$ with property $s$ without re-running the
learner (ideally minimal distance from $M$). ILP's symbolic representation makes editing
relatively easy — an output is a conjunction of clauses, edited by adding/removing
clauses (adding $s$ gives minimal rewrite distance, if consistency is preserved). DL
yields a black-box that's hard to manipulate (active/incremental learning gives little
control over exactly how it changes).

**Transparency.** ILP outputs are in an explicit high-level language, so more
transparent than DL. *Decomposability*: each human-specified predicate admits an
intuitive explanation (invented predicates may be counter-intuitive but are still
decomposable clauses). *Simulatability*: a small study ($n=16$) found access to an ILP
program let users simulate a concept they couldn't infer themselves (complexity reduced
simulatability). ILP explanations are usually redundant (the model is transparent),
whereas DL explainability is a large post-hoc research field. The *learning algorithm*
is also more transparent in ILP — each step is symbolic (e.g. dropping a literal), so a
user can in principle follow the concept at each step and attribute the solution to
particular inputs (impractical over thousands of steps); DL backpropagation updates are
not interpretable.

### Discussion

ILP has safety properties attractive vs DL: (1) convenient for specification; (2) robust
to most syntactic input changes; (3) program templates and length bounds give control
over inductive bias; (4) verifiable against arbitrary Datalog specifications via
resolution; (5) editable by adding/removing clauses; (6) interpretable.

**Competitiveness.** The community is unlikely to adopt ILP unless its performance is
competitive — and DL efforts now dominate domains where ILP once succeeded (e.g.
chemistry, protein-structure prediction). But far less is invested in ILP (≈130 listed
ILP researchers worldwide vs ~420 at a single DL lab, or 13,000 NeurIPS attendees), so
this may understate its potential. A deeper concern is ILP's limited domains: its
transparency benefits only apply where data is already symbolic, and rule-based
approaches may be insufficient for most human-level concepts (which seem to need
continuous features and exemplar similarity). Unifying connectionist and symbolic
methods is attractive, but current unifications tend to lose ILP's desirable safety
properties.

**ILP as a specification module.** A fruitful role: we may not directly specify safety
properties, but can give positive/negative examples of safe behaviour; ILP can generate
interpretable hypotheses from these partial specifications, which humans can verify (and
in some cases formally verify, e.g. Datalog) and edit, then transfer losslessly to
another learning system that handles logical expressions (e.g. graph neural networks).
