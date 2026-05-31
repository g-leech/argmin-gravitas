---
title: "Mask wearing reduces SARS-CoV-2 transmission"
full_title: "Mask wearing in community settings reduces SARS-CoV-2 transmission"
authors:
  - Gavin Leech
  - Charlie Rogers-Smith
  - Jonas B. Sandbrink
  - Ben Snodin
  - Rob Zinkov
  - Benjamin Rader
  - John S. Brownstein
  - Yarin Gal
  - Samir Bhatt
  - Mrinank Sharma
  - Sören Mindermann
  - Jan Markus Brauner
  - Laurence Aitchison
gleech_role: first author
year: 2022
venue: PNAS
type: journal
doi: 10.1073/pnas.2119266119
url: https://doi.org/10.1073/pnas.2119266119
code: https://zenodo.org/record/6385347
links:
  blogpost: https://www.gleech.org/masks
  explainer: https://twitter.com/g_leech_/status/1531691152327458819
  rebuttal: https://www.lesswrong.com/posts/J7RnKXcyCNdrdAus4/we-have-some-evidence-that-masks-work
contribution_hours: 565
topics: [covid-19, epidemiology, masks, non-pharmaceutical-interventions, bayesian]
notes: "16th most-discussed paper in the history of PNAS."
---

## Abstract

A puzzle: studies measuring individuals found large reductions in COVID
transmission from masks (~50%), but society-level studies found scattered results
in [−2%, 40%]. The proxy people used was weak. We built a much better proxy, using
Facebook's reach to obtain ~20 million data points on where and when people
actually wore masks.

We ran a regression model on 56 regions (including US states treated separately)
and checked it in 22 ways to guard against cherry-picking or pure correlation. We
find masks can be confidently linked to a 6–43% reduction in transmission, while
we cannot really say what the effect of *mandates* was. For comparison: the
summer–winter difference is ~42%, and all government interventions in the first
wave were ~80%.

## Full text

> Converted from the open-access PNAS full text (Europe PMC, PMC9191667). Markup
> removed; equations rendered inline or summarised; figures and the reference list
> omitted. The above is the author's plain-language summary; the journal's own text
> follows.

**Significance.** We resolve conflicting results regarding mask wearing against
COVID-19. Most previous work focused on mask mandates; we study the effect of mask
wearing directly. We find that population mask wearing notably reduced SARS-CoV-2
transmission (mean mask-wearing levels corresponding to a 19% decrease in R). We use
the largest wearing survey (n = 20 million) and obtain our estimates from regions
across six continents. We account for nonpharmaceutical interventions and time spent
in public, and quantify our uncertainty. Factors additional to mask mandates
influenced the worldwide early uptake of mask wearing.

**Abstract.** The effectiveness of mask wearing at controlling SARS-CoV-2 transmission
has been unclear. While masks are known to substantially reduce disease transmission in
healthcare settings, studies in community settings report inconsistent results. Most
such studies focus on how masks impact transmission by analyzing how effective
government mask mandates are. However, we find that widespread voluntary mask wearing,
and other data limitations, make mandate effectiveness a poor proxy for mask-wearing
effectiveness. We directly analyze the effect of mask wearing on SARS-CoV-2
transmission, drawing on several datasets covering 92 regions on six continents,
including the largest survey of wearing behavior (n = 20 million). Using a Bayesian
hierarchical model, we estimate the effect of mask wearing on transmission by linking
reported wearing levels to reported cases in each region, while adjusting for mobility
and NPIs such as bans on large gatherings. Our estimates imply that the mean observed
level of mask wearing corresponds to a 19% decrease in the reproduction number R. We
also assess the robustness of our results in 60 tests spanning 20 sensitivity analyses.
In light of these results, policy makers can effectively reduce transmission by
intervening to increase mask wearing.

### Introduction

Face masks are one of the most prominent interventions against COVID-19, with very high
uptake in most countries — though global wearing fell substantially in 2021 even where
vaccination was low. In healthcare, N95 masks reduce coronavirus transmission by at
least half, and ideal surgical masking corresponds to a 65–75% reduction in a
non-infected person's risk. But the effect in *small-scale community settings* is harder
to detect: four meta-analyses estimate mean decreases in infection risk of 4–15% for
surgical masks, but with large uncertainty (individual results from a 7% increase to a
61% decrease), and the few RCTs have issues (Bundgaard found a small non-significant
reduction; Abaluck found a significant 8.6% decrease in symptomatic seropositivity, but
with limitations).

Many studies use the *timing of mask mandates* as a proxy for sharp changes in wearing —
and find limited or inconsistent effects. We argue mandate timing is a poor proxy:
existing literature shows surprisingly weak mandate→wearing effects (one US-states study
found no significant relationship; others found post-mandate increases of just 13% and
23%). Three further factors make a mandate→transmission link hard to detect: mandates are
coarse one-off events that lose day-to-day signal (and fewer than half our regions
enforced any mandate); they're treated as binary instantaneous changes (missing
pre-enforcement and gradual behaviour change); and their circumstances are highly
heterogeneous. These arguments point to the mandate→transmission link being *difficult to
detect*, not absent. So we instead estimate the effect of mask *wearing* directly, using
a large (n = 19.97 million) global self-report survey. Our analysis goes further than
past work in wearing-data quality (~100× the sample size, random sampling and
poststratification), geographical scope (56 countries on six continents, May–Sept 2020),
a semimechanistic infection model, uncertainty quantification, and robustness (59
sensitivity tests).

### Results

**The effect of mask wearing on transmission.** Using data from May–September 2020, we
estimate effects in 92 regions. The covariate is the weighted percentage who said they
wore masks in public most/all of the time over the past 7 days (a proxy). A Bayesian
hierarchical model links wearing to reported cases via the instantaneous reproduction
number $R_t$, adjusting for other NPIs and mobility, and modelling many sources of
uncertainty. **The difference between zero wearing and 100% self-reported wearing
corresponds to a 25% [6%, 43%] reduction in transmission.** Since 100% wearing is not
achievable, multiplying the median effect by each region's time-averaged wearing gives a
mean per-region reduction of **19%**.

**The effect of mandates.** Running the same model with mandate data in place of wearing
data — modelling mandates as instantaneous, gradually increasing, or starting at
announcement — *fails to discover a mandate-driven effect on R* in every case.
Presumably because mandates are coarse, heterogeneous, and increased wearing by an
average of only 8.6% in our data.

**The mandate–wearing correlation.** Most of the uptake in wearing occurred *pre*-mandate.
Examples of decoupling: South Korea's mandate came after voluntary wearing plateaued at
94%; the Netherlands and Switzerland had limited transport mandates with low reported
wearing even three weeks in; the Czech Republic's wearing rose only long after its
mandate. (Strong mandate–wearing correlation was seen in Ireland and in Germany's local
mandates.) Using an earlier YouGov survey for the first wave, most of the increase
occurred before the earliest national mandates (64% average wearing on the mandate day,
75% three weeks later).

**Robustness.** 60 tests across 20 sensitivity analyses (varying epidemiological priors,
delay distributions, covariate-effect priors, model structure, and data): **95% of the
median reductions fall between 22.7% and 31.3%.** As the study is observational, causal
caution is needed. We probe confounding in four analyses: excluding each NPI in turn, all
NPIs at once, and mobility produces small changes (so unless unobserved confounding
greatly exceeds observed, results are unlikely to be meaningfully affected). A
fake-wearing variable (same start/end, linear interpolation) yields a small uncertain
effect (7.6% [−20.2%, 30.0%]), so the inferred effect doesn't rely solely on the overall
wearing/transmission trend. Endogeneity is limited (Spearman's ρ = 0.05 between new cases
and wearing).

### Discussion

We find mask wearing is associated with a notable reduction in SARS-CoV-2 transmission,
robust to extensive sensitivity analyses. The mandate–wearing analysis shows factors
beyond mandates strongly affect wearing — but does not imply mandates have no role;
rather, mandates (and other mask-promotion policies) may be effective *when* they
increase the use of masks.

- **Heterogeneity.** The inconclusive prior literature partly reflects not accounting for
  mask quality, fit, venue, reuse, risk compensation, and cultural norms. We estimate the
  effect of *mass* wearing, aggregating over these. Since most masks in this window were
  the least-effective types (cloth/unrated), the actual effectiveness of mass wearing
  today is likely *stronger* than we estimate. Masks have wearer-protection and
  source-control effects; most prior studies estimate individual wearer protection, not
  the policy-relevant mass-wearing effect.
- **Window of analysis.** Based on May–September 2020 (similar results for shorter
  windows). A short window aids internal validity (less distribution shift / unobserved
  confounding) but our period has features — e.g. summer's lower transmission, no tiered
  regional containment yet — that may not generalise.
- **Operationalizing wearing.** Estimates rely on self-reports (a proxy). Social
  desirability bias may inflate them (one Kenyan study found a 77% self-report vs observed
  disparity), which would mean the true effect of 100% wearing is *larger* than estimated.
  The survey's "wearing" definition is non-stringent, so there is probably scope for more
  and better wearing even where reported levels are very high.

**Conclusion.** Mask wearing is associated with a notable reduction in transmission, and
factors other than mandates must have contributed to the 2020 worldwide uptake. Where
mandates are unlikely to greatly increase uptake (e.g. voluntary wearing already high),
policymakers can use other levers — education on mask fit and quality, and mandates
focused on the highest-risk venues.

### Materials and Methods

**Data** (national or US-state level): 92 regions (55 countries + 37 US states), 1 May – 1
September 2020, 13,248 region-days. Wearing from the UMD/Facebook COVID-19 World Symptoms
Survey (19.97M responses, randomly sampled from active Facebook users, poststratified for
nonresponse/demographic bias; mean 1,131 responses per region-day) plus US data from
COVIDNearYou/SurveyMonkey (558,670 responses). Cases from the Johns Hopkins CSSE
repository (by report date). "Mobility" = average of commercial and workplace Google
Community Mobility indices (relative to 2019). NPIs from OxCGRT.

**Model.** A hierarchical Bayesian model: reported cases → later-ascertained infections →
instantaneous $R_t$ → covariate effects, via MCMC. $R_{t,c} = R_{\text{init},c} \cdot
X_{t,c} \cdot W_{t,c} \cdot M^-_{t,c} \cdot \exp(z_{t,c})$, where $X$ aggregates
multiplicative NPI (and reopening-NPI) effects $\exp(-\alpha_i \bar{x}_{i,t,c})$, mask
wearing $W_{t,c} = \exp(-\alpha_w w_{t,c})$, $M^-$ is normalised mobility, and $z_{t,c}$ is
a per-region weekly latent random walk (starting after two weeks to avoid
unidentifiability). $R_t$ is converted to daily growth via the generation interval;
infections are convolved with an infection-to-confirmation delay; observed cases follow a
negative binomial. Priors: $R_{\text{init}}$ hyperpriors from Epidemic Forecasting
estimates ($\mu = 1.07$); NPI effects an asymmetric Laplace placing 80% mass on positive
effects; the **wearing effect prior is symmetric** Normal(0, 0.4) — an uninformative choice
reflecting prior uncertainty about mask efficacy; generation-interval mean prior ~5.06
days; infection-to-confirmation delay NegBin mean ~10.92 days. Data and code: Zenodo
record 6385347.
