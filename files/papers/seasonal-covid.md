---
title: "Seasonal variation in SARS-CoV-2 transmission"
full_title: "Seasonal variation in SARS-CoV-2 transmission in temperate climates: a Bayesian modelling study in 143 European regions"
authors:
  - Tomáš Gavenčiak
  - Joshua Teperowski Monrad
  - Gavin Leech
  - Mrinank Sharma
  - Sören Mindermann
  - Jan Markus Brauner
  - Samir Bhatt
  - Jan Kulveit
gleech_role: co-author
year: 2022
venue: PLOS Computational Biology
type: journal
doi: 10.1371/journal.pcbi.1010435
url: https://journals.plos.org/ploscompbiol/article?id=10.1371/journal.pcbi.1010435
code: https://github.com/gavento/covid_seasonal_Brauner
links:
  explainer: https://twitter.com/jankulveit/status/1404369971334070273
contribution_hours: 110
topics: [covid-19, epidemiology, seasonality, bayesian, transmission]
---

## Abstract

We reconstruct the complex causal web that makes COVID less severe in summer, then
estimate a single scalar instead. It turns out to be large but not enormous: about
40% reduced transmission in summer in temperate climates. This provides an
important adjuster for observational studies and updates unadjusted estimates from
the prior year.

## Full text

> Converted from the open-access PLOS Computational Biology PDF. Markup removed;
> equations rendered inline or summarised; figures and the reference list omitted.

**Abstract.** Although seasonal variation has a known influence on the transmission of
several respiratory viral infections, its role in SARS-CoV-2 transmission remains
unclear. While there is a sizable and growing literature on environmental drivers of
COVID-19 transmission, recent reviews have highlighted conflicting and inconclusive
findings. This indeterminacy partly owes to the fact that seasonal variation relates to
viral transmission by a complicated web of causal pathways, including many interacting
biological and behavioural factors. Since analyses of specific factors cannot determine
the aggregate strength of seasonal forcing, we sidestep the challenge of disentangling
various possible causal paths in favour of a holistic approach. We model seasonality as
a sinusoidal variation in transmission and infer a single Bayesian estimate of the
overall seasonal effect. By extending two state-of-the-art models of NPI effects and
their datasets covering 143 regions in temperate Europe, we adjust our estimates for the
role of both NPIs and mobility patterns in reducing transmission. We find strong
seasonal patterns, consistent with a reduction in the time-varying reproduction number
R(t) of **42.1% (95% CI: 24.7%–53.4%) from the peak of winter to the peak of summer**.
These results imply that the seasonality of SARS-CoV-2 transmission is comparable in
magnitude to the most effective individual NPIs but less than the combined effect of
multiple interventions.

**Author summary.** Building on two state-of-the-art observational models and datasets,
we adapt a fully Bayesian method for estimating the association between seasonality and
SARS-CoV-2 transmission in 143 temperate European regions. This overcomes limitations of
previous analyses that do not account for NPIs or mobility during the first year of the
pandemic and hence may yield biased seasonal estimates. As seasons change, policymakers
need accurate seasonal estimates: in summer, reductions due to seasonality should not be
misattributed to population immunity; in winter, policymakers must avoid anticipating a
greater seasonal reduction than will actually occur.

### Introduction

The role of seasonal variation in SARS-CoV-2 transmission has received significant
scientific and political attention. While seasonal variation is well-established for many
respiratory viral infections, and some studies suggest associations between temperature,
humidity, and COVID-19 incidence, other analyses fail to show a robust role of climate
and weather. Understanding seasonality matters because, e.g., it helps policymakers avoid
attributing declining summer incidence to population immunity alone when seasonality may
be playing a meaningful role.

### Methods

We build on two previously published, state-of-the-art NPI effectiveness models —
**Brauner et al.** and **Sharma et al.** — extending each "base model" to include a
seasonality effect, fitting the two resulting "seasonal models" separately on separate
data to obtain two distinct seasonality estimates.

**The base models.** Both use a data-driven, cross-region Bayesian hierarchical approach:
case and death data are used to "backward" infer new infections over time, which infer
reproduction numbers; NPI effects are estimated by relating daily reproduction numbers to
active NPIs across all days and regions. The central equation is
$R_l(t) = R_{0,l} \cdot N_l(t) \cdot \prod_{i=1}^{I} \exp(-a_i x_{i,l,t})$, where each
NPI's effectiveness is $1 - \exp(-a_i)$, $N_l(t)$ is a noise term (log-normal
multiplicative in Brauner et al.; random-walk-based in Sharma et al.), and (in Brauner et
al. only) NPI effects vary slightly between countries via partial pooling. $R_l(t)$ drives
daily new infections (via a one-to-one early-pandemic growth-rate correspondence in
Brauner et al.; a renewal process in Sharma et al.), moderated by the generation interval;
infections are observed as cases/deaths after delays, following negative-binomial output
distributions with separate dispersion parameters. Priors follow the originals.

**Estimating seasonality.** We model seasonality as a sinusoidal multiplicative factor on
R(t), $\Gamma(t) = 1 + \gamma \sin(\cdot)$, inferring a single global amplitude parameter
$\gamma$ shared across all locations (so it captures common dynamics not explained by
location-specific noise or NPIs). The sinusoid is motivated by its fit to upstream causal
factors (temperature, humidity) in temperate Europe; we assume a single common seasonal
effect for countries in similar climates and proximity (the *amplitude* is assumed similar
even though average temperatures differ by latitude). The seasonal multiplier is normalised
to 1 at the start of the window. The amplitude converts to a peak-of-winter-to-peak-of-
summer reduction via $R(\text{trough})/R(\text{peak}) = (1-\gamma)/(1+\gamma)$. We use
January 1 as the seasonal peak day (close to the median peak date, January 3, inferred by a
variable-peak model, and robust in sensitivity analysis). Prior $\gamma \sim U(-0.95,
0.95)$. Inference via NUTS (4 chains × 1250 samples; $\hat{R} < 1.02$, no divergences).

**Data.** Each model is fitted on its original data/period: Brauner et al. (NPIs in 41
countries, 22 Jan – 30 May 2020), restricted to the 29 temperate-European countries; Sharma
et al. (17 NPIs in 114 subnational regions across 7 countries, 1 Aug 2020 – 9 Jan 2021),
used unmodified — a total of **143 regions**. Case/death data from Johns Hopkins CSSE; for
Sharma et al. we exclude data points where variants-of-concern prevalence exceeded 10%.

### Results

Across two model structures and datasets covering 72% of the 2020–2021 period in Europe,
our combined estimate (an equal-weight mixture of the two models' posteriors) is consistent
with a reduction in R of **24.7% to 53.4% (95% CI), median 42.1%**, from 1 January (peak
winter) to 1 July (peak summer). The winter→summer transition is associated with a
reduction comparable to or greater than individual NPIs, but less than the total effect of
combined interventions. Restricting the comparison NPI estimates to temperate regions has
little effect on the inferred total NPI effect.

**Robustness.** Adjusting for country-level mobility changes $\gamma$ by less than 1.64%
(so the seasonal pattern is unlikely to be explained by unrelated behavioural change —
though mobility may not capture every relevant trend). Including a seasonality×NPI
interaction term leaves both seasonality and NPI estimates little changed. Adding the
seasonality term reduces the magnitude and asymmetry of Sharma et al.'s random walk
considerably (log-space mean-square displacement falls from 0.131 to 0.072), so a
considerable amount of residual variation is explained by a common seasonality profile. The
inferred peak-to-trough reduction varies by less than 5% across peak-seasonality dates in
December and January, and results are robust to the initial-$R_0$ prior.

### Discussion

We sidestep the intractable challenge of evaluating many interrelated seasonal factors and
instead give a precise estimate for the overall seasonal variation in temperate Europe,
adjusting for NPIs and mobility. The strong associations match the seasonal patterns of
other respiratory viruses and are in line with prior assumptions (e.g. Kissler et al.'s
10–40% winter→summer R₀ reduction; Neher et al.'s $\gamma$ of 0.3–0.7). Importantly, the
results are **not** inconsistent with warm-region outbreaks or summer surges in temperate
regions: despite moderate seasonal forcing, R can remain well above 1 in summer, especially
with more transmissible variants — so vaccination, NPIs, and variant prevalence remain
important year-round.

The study uses variation across time while holding the climate zone constant, so results
may not translate to cross-region comparisons at a fixed season (latitude correlates with
many epidemiological, demographic, and societal factors). Main limitations: it relies on
data from only one complete period of seasonality; it focuses on temperate Europe (may not
generalise to other climates); and, being observational, it demands causal caution — we did
not attempt to disentangle the various plausible causal pathways.
