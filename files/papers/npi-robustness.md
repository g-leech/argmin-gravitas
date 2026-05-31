---
title: "How robust are estimates of Covid policies?"
full_title: "How Robust are the Estimated Effects of Nonpharmaceutical Interventions against COVID-19?"
authors:
  - Mrinank Sharma
  - Sören Mindermann
  - Jan M. Brauner
  - Gavin Leech
  - Anna B. Stephenson
  - Tomáš Gavenčiak
  - Jan Kulveit
  - Yee Whye Teh
  - Leonid Chindelevitch
  - Yarin Gal
gleech_role: co-author
year: 2020
venue: NeurIPS (spotlight)
type: conference
arxiv: 2007.13454
doi: 10.48550/arXiv.2007.13454
url: https://papers.nips.cc/paper/2020/file/8e3308c853e47411c761429193511819-Paper.pdf
code: https://github.com/epidemics/COVIDNPIs/tree/neurips
links:
  video: https://nips.cc/virtual/2020/public/poster_8e3308c853e47411c761429193511819.html
  explainer: https://twitter.com/MrinankSharma/status/1346891304932880386
contribution_hours: 120
topics: [covid-19, epidemiology, npi, robustness, bayesian, sensitivity-analysis]
---

## Abstract

COVID-19 policy studies mostly don't do proper validation — very few papers
check their performance on holdout data, and their sensitivity checks are
usually limited. We re-ran one of the famous models, and several variations of
our own, and found the famous model's results depend quite a lot on analysis
decisions (ours is a bit more robust).

We also prove a couple of theorems about how to interpret the effects: it is not
the unconditional effect of doing policy *p*, but the average *additional* effect
of *p* when implemented alongside average existing policies (the average in your
dataset).

## Full text

> Converted from the arXiv PDF (v3, NeurIPS 2020). LaTeX markup removed; equations
> rendered inline or summarised; figures omitted; the supplement (proofs, full
> sensitivity analyses) is pointed to, not reproduced.

**Abstract.** To what extent are effectiveness estimates of nonpharmaceutical
interventions (NPIs) against COVID-19 influenced by the assumptions our models make?
To answer this, we investigate 2 state-of-the-art NPI effectiveness models and
propose 6 variants that make different structural assumptions. In particular, we
investigate how well NPI effectiveness estimates generalise to unseen countries, and
their sensitivity to unobserved factors. Models that account for noise in disease
transmission compare favourably. We further evaluate how robust estimates are to
different choices of epidemiological parameters and data. Focusing on models that
assume transmission noise, we find that previously published results are remarkably
robust across these variables. Finally, we mathematically ground the interpretation
of NPI effectiveness estimates when certain common assumptions do not hold.

### 1. Introduction

NPIs (business closures, gathering bans, stay-at-home orders) are central to the
fight against COVID-19, yet how effective different NPIs are at reducing transmission
is largely unknown. Data-driven NPI modelling — relating publicly available incidence
and fatality data plus NPI implementation dates to NPI effect sizes — is one of the
best approaches, but it is impossible to construct a model without making
assumptions. Given the policy relevance, we ask: to what extent are our estimates
influenced by the assumptions our models make? If estimates fluctuate widely under
plausible assumptions, they cannot inform policy. Analyses are also limited to a
subset of countries, and epidemiological parameters are only known with uncertainty,
so robustness to these must also be assessed.

We build on previous SOTA models and construct 6 variants with different structural
assumptions. Without ground truth, we evaluate models by how well their estimates
generalise to unseen countries and how much they are influenced by unobserved
factors; assuming transmission noise yields more robust, better-generalising
estimates. We systematically validate all models against variations in data and
epidemiological parameters and find consistent trends: closing schools and
universities in conjunction was consistently highly effective; the effect of
stay-at-home orders is modest; the additional benefit of closing most nonessential
businesses was smaller than targeted closures of high-exposure businesses; and
gathering-ban effectiveness increased as the maximum size decreased. Finally, we
mathematically ground the interpretation: estimates should be read as *average,
marginal* effectiveness, averaged over the situations in which each NPI was active —
e.g. mask-wearing mandates were only active in our data alongside several other NPIs,
so we can only reason about their effectiveness in the presence of those others.

### 2. Common assumptions in NPI modelling

These models assume implementing an effective NPI immediately reduces transmission,
measured via the reproduction number $R$. Given NPIs, their effectiveness, and the
basic reproduction number $R_0$, one can compute $R$ on a given day — but $R$ alone is
insufficient to compute infections; one also needs the **Generation Interval** (time
between successive infections) and infection-to-report delays, all known only with
uncertainty. The default model rests on:

- **A1.** Epidemiological parameters are constant across countries and time.
- **A2.** Constant NPI effectiveness — (a) independent of country; (b) independent of
  time.
- **A3.** NPIs have multiplicative effects on $R_{t,c}$.
- **A4.** No NPI interactions (effectiveness independent of which other NPIs are
  active).
- **A5.** No unobserved factors ($R_{t,c}$ depends only on $R_{0,c}$ and active NPIs;
  each NPI has its full effect immediately).
- **A6.** $R_{t,c}$ is converted to a daily growth rate by assuming constant
  exponential growth (via the inverse moment-generating function of the generation
  interval).

Under A2–A5, $R_{t,c} = R_{0,c} \prod_{i \in I} \exp(-\alpha_i x_{i,t,c})$. Reported
cases and deaths follow negative-binomial output distributions (over-dispersed; the
dispersion parameters, larger = less noise, are inferred).

**Alternative assumptions** used to build variant models: **Additive effects** (A9:
each NPI eliminates a non-overlapping constant fraction of $R_0$); **Different
effects** (A10: country-specific effectiveness drawn i.i.d. from a Normal, relaxing
A2a and A4); **Noisy-R** (transmission noise applied to $R_{t,c}$ rather than the
growth rate); **Discrete Renewal** infection process (no constant-exponential-growth
assumption); and **No transmission noise** (A11).

### 3. Experiments & methodology

Eight models comparing assumptions: **Default** (prior work), **Additive Effects**,
**Different Effects**, **Noisy-R**, **Discrete Renewal (DR)**, **Deaths-Only DR**,
**Flaxman et al.** (= Deaths-Only DR with no transmission noise), and **Default (No
Transmission Noise)**.

- **Model evaluation:** holdout predictive likelihood on a test set of 6 countries
  (hyperparameters tuned by cross-validation) — used to rule out models that fail to
  predict held-out data.
- **Sensitivity to unobserved factors:** how inferred effectiveness changes when
  observed NPIs become unobserved (NPI leave-outs) and when previously unobserved
  NPIs (from OxCGRT) are added. We favour models with stable estimates (assigning
  unobserved effects to noise, not to our NPIs), via a loss $L_c$ = mean over NPIs of
  the standard deviation of posterior-median effectiveness across test conditions.
- **Model robustness:** extensive sensitivity across 6 further categories (varying
  included countries, case/death masking thresholds, generation interval,
  infection-to-case/death delays, $R_0$ prior, NPI-effectiveness prior) — **over 600
  experimental conditions** in total.
- **Data/implementation:** the prior NPI dataset (9 NPIs, 41 countries, Jan–May 2020,
  independent double entry); Johns Hopkins CSSE case/death data; PyMC3 with NUTS HMC
  (4 chains × 1250 samples; $\hat{R} < 1.05$, no divergences). Most prior NPI studies
  report no holdout performance and limited structural-sensitivity analysis.

### 4. Results & discussion

Holdout performance is similar across models (consistently better for deaths than
cases, as deaths are predicted further into the future). Sensitivity to unobserved
factors varies significantly: the discrete-renewal model is more sensitive than the
default; the Additive Effects model has the lowest sensitivity (its NPI sum is
constrained). **Including transmission noise both improves holdout performance and
increases robustness to unobserved factors** — so subsequent analyses exclude models
without transmission noise.

**Structural sensitivity** (the 6 transmission-noise models): systematic trends in
median effectiveness hold across model structure, data, and epidemiological
parameters. Stay-at-home orders and mask-wearing mandates are consistently among the
*least* effective; closing schools and universities in conjunction (inseparable —
highly collinear) tends to be among the *most* effective; the marginal benefit of
closing most nonessential businesses is modest. The DR and Deaths-Only DR models find
lower effectiveness for gatherings ≤1000 and higher for ≤10.

### 5. Effectiveness depends on context

If A2 and A4 are not assumed away (and in reality they don't hold — mask mandates may
matter more without social distancing; implementation and adherence vary), how should
estimates be interpreted? Assuming ground-truth $g_{t,c}, R_{t,c}, R_{0,c}$ are given,
consider simplified Default and Noisy-R models and derive the maximum-likelihood
estimate of $\alpha_i$:

- **Theorem 1 (Simplified Noisy-R).** The ML solution for $\exp(-\alpha_i)$ is the
  ratio of two **geometric means** over all country-days when NPI $i$ is active: the
  mean ground-truth $R_{t,c}$ over the mean predicted $R_{t,c}$ with NPI $i$
  deactivated.
- **Theorem 2 (Simplified Default).** The ML solution is instead a ratio of
  **generalized weighted means** (exponent $1/\nu$, where $\nu$ is the GI shape),
  weighting observations more when the predicted $R$ (excluding NPI $i$) is larger.

A minor variation in model structure thus gives a significantly different ML
solution. But in both, when A2/A4 fail, $\alpha_i$ is an **average additional
effectiveness**, produced by averaging over the data distribution. So care is needed:
e.g. our earlier "small reduction from stay-at-home orders" should be read as
"implementing a stay-at-home order is associated with a modest reduction in $R$ *when
other effective NPIs are already active*", since stay-at-home orders almost always
co-occurred with several other NPIs in our data.

### 6. Conclusions

Our previously reported NPI effectiveness results are robust across several
alternative model structures with transmission noise. Still, the numerous assumptions
and limitations of data-driven NPI modelling mean these should be neither the last
word nor treated as causal; policymakers should draw on diverse evidence (other
retrospective studies, experimental methods, clinical experience). We release our
validation suite and model implementations and urge others to systematically validate
their models.

**Broader impact.** The rapid COVID-19 research cycle has increased erroneous,
misreported findings reaching popular attention; sensitivity analyses like ours can
uncover faulty assumptions and prevent overconfidence. One risk is miscommunication —
high robustness must not be mistaken for excessive certainty, and the subtle
interpretation issues (Section 5) could be misread as unconditional effects.
