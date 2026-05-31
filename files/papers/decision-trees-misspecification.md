---
title: "Decision trees compensate for model misspecification"
authors:
  - Hugh Panton
  - Gavin Leech
  - Laurence Aitchison
gleech_role: co-author
year: 2023
type: preprint
arxiv: 2302.04081
doi: 10.48550/arXiv.2302.04081
url: https://arxiv.org/abs/2302.04081
code: https://github.com/H-B-P/treedepth
contribution_hours: 50
topics: [decision-trees, gradient-boosting, model-misspecification, statistics, machine-learning]
---

## Abstract

Linear models should arguably match decision-tree ensembles in many settings,
yet very deep trees (e.g. depth-12) are used in industry, which doesn't make
sense on a naive account. We contrive several stochastic processes to test ways
that tree depth could be compensating for realistic data flaws.

Depth partially fixes a range of modelling and training errors: insufficient
training, the wrong link function, assuming single responses when the response
is composite, assuming homogeneity in a heterogeneous population, and missing
variables. We also give simple extensions to Gaussian mixture modelling with
lines. (Our linear fixes did not work that well on randomly selected *real*
data.)

## Full text

> Converted from the arXiv PDF (v1). LaTeX markup removed; figures omitted; data-
> generating processes given in prose; the mathematical appendices (limit behaviour,
> GLM objective derivations, scaling, failed approaches) are summarised at the end.

**Abstract.** The best-performing models in ML are not interpretable. If we can
explain why they outperform, we may be able to replicate these mechanisms and obtain
both interpretability and performance. One example is decision trees and their
descendent gradient boosting machines (GBMs). These perform well in the presence of
complex interactions, with tree depth governing the order of interactions. However,
interactions cannot fully account for the depth of trees found in practice. We
confirm 5 alternative hypotheses about the role of tree depth in performance in the
absence of true interactions, and present results from experiments on a battery of
datasets. Part of the success of tree models is due to their robustness to various
forms of mis-specification. We present two methods for robust generalized linear
models (GLMs) addressing the composite and mixed response scenarios.

### 1. Introduction

Some models are inherently interpretable (e.g. a sparse linear model with few
parameters, each interpretable independently on a single scale). Most are not: a
typical XGBoost GBM (default depth 3, 100 trees) could only be depicted by plotting
all 100 trees — not decomposable, not simulatable. The main obstacle to
interpretability is **interaction**: if the component trees were depth-1 ("decision
stumps"), partial dependence would give a complete feature-wise summary. In complex
systems, variables modify each other's effects, and large datasets increasingly let
us model interactions meaningfully — so models without that capacity underperform.

We give five scenarios in which problems *besides* interaction cause high-depth
models to outperform, and show how each could be addressed without using $d > 1$.
These support the hypotheses that (1) such problems may be present where tree models
are applied and are at least somewhat responsible for the high-vs-low-depth
performance gap; (2) the gap between conventional tree models and the best
interpretable model may be smaller than a grid-search suggests; (3) the maximum
depth required may be lower than a grid-search suggests; (4) models that appear to
benefit from $d > 1$ might be matched by interpretable models once these problems are
addressed.

### 3. Related work

Practitioners find high-depth tree ensembles perform well (depth-10 ensembles are
common and often win Kaggle contests). Most accounts invoke interaction
modelling/data segmentation — but a depth-9 tree claims a ninth-order interaction
estimated with sufficient power, which is implausible: there is a classical
relationship between interaction effect size, power, and sample size (a two-way
interaction with half the main effect's size takes 16× the sample to estimate at
equal precision; standard error compounds with interaction order). So the ubiquity
of high-depth trees, even with small datasets and interaction effects, is a puzzle.
Little past work touches our main topic — trees' robustness to mis-specification
(though some note tree learning handles mis-specification, e.g. CART reducing bias
vs maximum likelihood).

### 4. Methods & Results

Five synthetic scenarios (four with Poisson responses and a canonical log-link, since
three hypotheses involve multiplicative processes). Data have known distributions, so
results are given without error bars.

- **4.1 Insufficient learning.** Sum of ten Bernoulli flips + Gaussian noise — ideal
  for additive ($d=1$) Gaussians, yet higher-depth models score better (MAE 1.967 →
  1.916 → 1.886 for $d=1,2,3$). When other hyperparameters cause undertraining,
  increasing depth makes the model less underfit even with no interactions. Using
  learning rate as a proxy for underfitting: undertrained models tend to be worse
  than overtrained, and high-depth models can represent lower-depth effects but not
  vice versa, so excessive depth is less harmful than insufficient depth. The
  practical point: finding the best depth requires a grid-search over depth *and*
  other hyperparameters.
- **4.2 Incorrect linkages.** A multiplicative gemstone-pricing relation (colour ×
  size). An additive ($d>1$) model needs depth to capture the "interaction" term, but
  a multiplicative (log-linked) model needs only $d=1$. Real datasets have mixed
  additive/multiplicative structure, so conventional linkage is never perfect — but
  using the *least incorrect* linkage minimises the depth required.
- **4.3 Multiple (composite) responses.** When the observed response is the *sum* of
  several log-linked sub-processes (e.g. total zoo-staff injuries that are really
  bites + scratches + headbutts; total insurance claims that are fraudulent + honest),
  $d=3$ beats $d=1$ despite no interactions in any sub-response. The fix is
  **multiresp** (multiple-response modelling): set up $S$ log-linked submodels and
  learn coefficients by gradient descent. $S$ is just a hyperparameter; multiresp
  matches high-depth trees here and stays interpretable.
- **4.4 Multiple populations.** A mixture of multiplicative processes (e.g. injuries
  from lions vs tigers, or honest vs fraudulent customers, with no feature indicating
  the class) again makes $d=3$ beat $d=1$. The fix is a **mixed GLM**: two parameter
  sets with prevalence $f$, learned by gradient descent (≈2× a normal linear model's
  complexity). $f$ is best treated as a hyperparameter; modelling ~10 distinct
  subpopulations may leave no inherently interpretable model. (Mixed GLMs cannot model
  the case $f=0.5$ with equal means.)
- **4.5 Missing features.** A log-linked process where some explanatory variables are
  absent but correlated with present ones: $d=3$ again outperforms $d=1$. Trying to
  infer latent features then predict is impractical and would itself be
  uninterpretable; the practical upshot is to investigate *extra* features when
  modelling a log-linked system — counter-intuitively, adding features (even ones
  correlated with present ones) can make models *less* complex.
- **4.6** We validate the competitiveness of multiresp and mixed GLMs on real data
  (our linear fixes did not transfer as cleanly to randomly-selected real data).

### 5. Conclusions

In all five scenarios $d > 1$ outperforms decision stumps even with no real
interactions: the learner increases depth to compensate for violated assumptions —
insufficient learning, incorrect linkages, composite responses, heterogeneous
populations, and absent columns. All but the first are unsurprising consequences of
the simplification inherent to modelling (all linkages are approximations; all
responses are arbitrarily decomposable; all models elide heterogeneity; all models
have absent columns). Where these mis-specifications occur, there is scope to fix
them and obtain less-complex interpretable models (a stump ensemble or GLM) that
perform comparably to uninterpretable deep trees. (Merely reducing depth may not help
much — there's a discontinuity at $d=1 \to d=2$ where partial-dependence plots stop
being a complete summary.) It is good that learning succeeds even when the model is
wrong, since mis-specification is the norm; but when interpretability is a priority,
the alternative to automatic compensation is to fix the mis-specification and return
to interpretable models.

*(Appendices: **A** — a saturated XGB model with max-depth $d$ on binary features
approaches an unpenalized GLM with every $d$-way interaction (verified identical for
$d<4$); theoretical bounds showing multiresp/mixture GLMs use fewer parameters than a
$d=B$ linear model outside pathological cases. **B** — derivations of the GLM
objectives (simple, composite, and mixture cases). **C** — scaling results where the
best multiresp model outperforms the best XGB model. **D** — two failed approaches
(highly nonlinear effects; outliers/mis-specified errors), the latter explaining the
non-monotonic RMSE effect.)*
