---
title: "Legally Grounded Fairness"
authors:
  - Dylan Holden-Sim
  - Gavin Leech
  - Laurence Aitchison
gleech_role: co-author
year: 2020
type: preprint
arxiv: 2009.11677
doi: 10.48550/arXiv.2009.11677
url: https://arxiv.org/abs/2009.11677
links:
  blogpost: https://www.gleech.org/lgfo
contribution_hours: 20
topics: [algorithmic-fairness, machine-learning, law, impossibility-results]
---

## Abstract

We try to work around the famous impossibility results in algorithmic fairness
by using legal damages as a signal about all-things-considered unfairness. This
lets us use multiple definitions of fairness at once and set the weight on each
in a non-arbitrary way.

A human picks a set of fairness definitions and gives the algorithm a set of
past cases with the damages awarded in each. LGFO works out how much weight to
give each kind of fairness, producing a classifier that is relatively fair — if
we trust the legal system to know this relatively well.

## Full text

> Converted from the arXiv PDF (v1, "Legally grounded fairness objectives"). LaTeX
> markup removed; citation keys rendered lightly; figures omitted; the
> incompatibility proofs (Appendices A–B) are pointed to, not reproduced.

**Abstract.** Recent work has identified a number of formally incompatible
operational measures for the unfairness of a machine learning (ML) system. As these
measures all capture intuitively desirable aspects of a fair system, choosing "the
one true" measure is not possible, and instead a reasonable approach is to minimize
a weighted combination of measures. However, this simply raises the question of how
to choose the weights. Here, we formulate Legally Grounded Fairness Objectives
(LGFO), which uses signals from the legal system to non-arbitrarily measure the
social cost of a specific degree of unfairness. The LGFO is the expected damages
under a putative lawsuit that might be awarded to those who were wrongly classified,
in the sense that the ML system made a decision different to that which would have
been made under the court's preferred measure. The two quantities necessary to
compute the LGFO — the court's preferences about fairness measures, and the expected
damages — are unknown but well-defined, and can be estimated by legal advice. As the
damages awarded by the legal system are designed to compensate for the harm caused
to an individual by an unfair classification, the LGFO aligns closely with society's
estimate of the social cost.

### 1. Introduction

Automated decision-making systems are increasingly applied in sensitive contexts,
and can discriminate on protected characteristics (race, religion, gender) even when
those attributes are not inputs. The fundamental issue of algorithmic fairness is
that no single definition captures the full phenomenon. The COMPAS recidivism system
is the canonical clash: a ProPublica study found African Americans labelled "high
risk" were 50% less likely to reoffend than white defendants with the same label,
while Flores et al. responded that defendants with the same score had approximately
equal recidivism probability — a direct clash between operationalisations of
fairness. Many reasonable definitions exist (we'd ideally fulfil them simultaneously),
but it can be proven that some sets of commonsensical definitions are incompatible
outside trivial cases.

Since perfect multi-measure fairness is almost always impossible, we aim at systems
that minimise violations — but it is unclear which definitions to relax and to what
extent. This is a question of values: there may be irreconcilable differences between
individuals, and no underlying well-defined "correct answer" even in principle. We
note that society answers such questions in other contexts using the legal system. So
we propose using the legal system to operationalise society's estimate of social
costs: the resulting LGFO measures the damages awarded to those wrongly classified
under a putative lawsuit. This shifts the burden of selecting fairness measures onto
the legal system and away from the programmer.

Setting the objective to minimise legal penalties may look like privileging the
deployer, but the size of legal damages is a reasonable proxy for social good
because: (1) in principle the law is designed to reflect the values of a society,
including the broadest reading of fairness; (2) legal damages are intended to
reimburse an individual for harm caused, as assessed by a judge — so minimising total
damages (e.g. by reducing unfairness and the associated lawsuits) simultaneously
minimises harm. Advantages: it uses the canonical process for balancing values
(civil law, which in well-functioning jurisdictions has public accountability,
adaptiveness, and consensus), and high-quality data (awarded damages in
discrimination cases are often openly available).

**Related work.** CFAθ (Zehlike et al.) maps score distributions towards a
barycenter with a tunable parameter θ, but θ remains a decision rather than something
calculated from concrete values. Dwork et al.'s Individual Fairness ("similar people
should be treated similarly") enforces a Lipschitz condition, but only shifts the
subjectivity onto the distance function. **Contributions:** a new perspective using
legal costs as a proxy for the social cost of a given fairness measure; a method to
combine multiple measures into an overall degree of unfairness; and experiments on a
real-world dataset showing that fairness-measure combinations can correct naive
correlations between the response variable and protected attributes.

### 2. Methods

Our algorithm is a post-processing step for binary classifiers: we find the
cost-minimal decision boundary for each group — the pair $(t_0, t_1)$ of per-group
thresholds. Fairness measures are initially binary (satisfied or violated); we
translate them into measures (functions minimal at 0 when perfectly satisfied,
increasing with deviation). We focus on three mutually-incompatible measures (with
$G$ = group, $Y$ = ground truth, $\hat{Y}$ = prediction):

- **Sufficiency** (Suff): difference in precision between groups,
  $|\text{Prec}_{G=0} - \text{Prec}_{G=1}|$ where $\text{Prec} = P(Y{=}1 \mid \hat{Y}{=}1)$.
- **Equalised Odds** ($\Delta F$): difference in false-positive rate,
  $|\text{FPR}_{G=0} - \text{FPR}_{G=1}|$ where $\text{FPR} = P(\hat{Y}{=}1 \mid Y{=}0)$
  (the ProPublica allegation).
- **Statistical Parity** (SP): difference in positive-prediction rate,
  $|P(\hat{Y}{=}1 \mid G{=}0) - P(\hat{Y}{=}1 \mid G{=}1)|$.

For a set of measures $M$ and example cases $X$, the LGFO is the expected damages from
a hypothetical civil suit for wrongful classification:
$$\text{LGFO} = \sum_{m \in M} P(m) \sum_{x \in X} C(\hat{y}, y_m)$$
where $\hat{y} = c(x)$ is the ML decision, $y_m$ is the decision under measure $m$,
$P(m)$ is the probability the court prefers measure $m$, and $C$ is the
misclassification cost. **Algorithm 1** fixes the classifier and searches per-group
thresholds to minimise summed multi-measure cost, with $\text{P2N}$/$\text{N2P}$ the
expected legal cost of flipping a positive→negative / negative→positive outcome, and
a target number of positives $P^*$ to avoid trivially-fair solutions.

**Cost-sensitivity.** A measure is cost-sensitive if small changes in its value
produce large changes in cost. Cost-insensitive measures can be relaxed far more
without incurring social cost, creating room to improve more cost-sensitive measures.

### 3. Results

We validated LGFO on the COMPAS dataset, training a PyTorch binary classifier (Medium
and High risk merged, mirroring ProPublica). The cost-optimal thresholds were
[0.54, 0.41] (slightly favourable treatment for African Americans), aligning with the
optimum for $\Delta F$. SP and $\Delta F$ roughly agreed on the fairest region (both
penalise discrepancies between group outcomes).

Versus the uncorrected classifier (thresholds [0.5, 0.5]):

| Measure | Uncorrected | LGFO |
|---|---|---|
| Statistical Parity | 0.236 | 0.029 |
| Sufficiency | 0.062 | 0.154 |
| ΔF (Equalised Odds) | 0.162 | 0.033 |
| Accuracy | 67.0% | 65.7% |

The corrected model trades Sufficiency for large gains in Statistical Parity and
Equalised Odds, with a small (~2% relative) accuracy decrease — accuracy is mostly
preserved because LGFO only changes classifications for defendants near the decision
boundary. The uncorrected predictor significantly under-represents white plaintiffs
in positive predictions versus ground truth; LGFO corrects this.

Crucially, Suff is the most cost-sensitive measure (the cost at Suff = 0.2 equals the
cost at ΔF = 0.5), which is why Sufficiency *increases* (worsens) after LGFO — a
principled trade for greatly reduced unfairness elsewhere. The key result: inspecting
measure values alone is insufficient to judge the actual relative fairness of two
classifiers; taking damages into account shows that lower measure-fairness can occur
when damages are reduced. Illustrative counterfactual scenarios (varying P2N/N2P)
show cases where balancing two measures yields a better social outcome than optimising
either alone, and cases where it yields a worse one.

### 4. Discussion

LGFO brings fairness into business operations via an unambiguous, hard-to-game
monetary incentive; lets non-technical stakeholders contribute to system design; and
uses long-standing legal expertise on decision-making in complex social situations.
It corrected erroneous bias in a neural classifier on a real-world dataset while
conserving performance.

**Limitations.** It cannot be applied to arbitrary data without a careful elicitation
step (most legal systems don't use formal fairness measures explicitly); the
experiments use ProPublica's binarised risk groups; only binary classification is
handled (extensions are future work). The legal system is itself an imperfect,
biased estimator of social cost — but plausibly less biased or unaccountable than a
lone technical team with no clear incentives towards fairness. LGFO estimation is
unavoidably local (the distribution over definitions and the damages vary by
jurisdiction) — the converse of its strength, that it uses actual domain knowledge.

*(Appendices A–B give the probabilistic proofs of incompatibility — Statistical
Parity vs Sufficiency, Equalised Odds vs Sufficiency, and Statistical Parity vs
Equalised Odds in the binary case.)*
