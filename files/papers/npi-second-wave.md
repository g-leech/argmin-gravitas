---
title: "What worked on Covid in the 2nd wave"
full_title: "Understanding the effectiveness of government interventions against the resurgence of COVID-19 in Europe"
authors:
  - Mrinank Sharma
  - Sören Mindermann
  - Charlie Rogers-Smith
  - Gavin Leech
  - Benedict Snodin
  - Janvi Ahuja
  - Jonas B. Sandbrink
  - Joshua Teperowski Monrad
  - George Altman
  - Gurpreet Dhaliwal
  - Lukas Finnveden
  - Alexander John Norman
  - Sebastian B. Oehm
  - Julia Fabienne Sandkühler
  - Thomas Mellan
  - Jan Kulveit
  - Leonid Chindelevitch
  - Seth Flaxman
  - Yarin Gal
  - Swapnil Mishra
  - Jan Markus Brauner
  - Samir Bhatt
gleech_role: co-author
year: 2021
venue: Nature Communications
type: journal
doi: 10.1038/s41467-021-26013-4
url: https://www.nature.com/articles/s41467-021-26013-4
code: https://github.com/MrinankSharma/COVID19NPISecondWave
links:
  explainer: https://twitter.com/MrinankSharma/status/1445760620696006663
contribution_hours: 100
topics: [covid-19, epidemiology, non-pharmaceutical-interventions, bayesian, europe]
---

## Abstract

We examined how policy effects changed in the second wave (late 2020), at a finer
spatial level (units roughly 1/20 of a country rather than whole countries).
Policies got a bit weaker overall (66% combined reduction vs 80% in spring). The
best reading is that safety measures and individual protective behaviour persisted
after the first wave, even when governments said it was OK to stop. School closure
was notably weaker (10% vs 35%), probably because the safety measures enforced
since spring really did make schools safer.

## Full text

> Converted from the open-access Nature Communications full text (Europe PMC,
> PMC8492703). Markup removed; equations summarised; figures/supplement and the
> reference list omitted.

**Abstract.** European governments use non-pharmaceutical interventions (NPIs) to
control resurging waves of COVID-19. However, they only have outdated estimates for how
effective individual NPIs were in the first wave. We estimate the effectiveness of 17
NPIs in Europe's second wave from subnational case and death data by introducing a
flexible hierarchical Bayesian transmission model and collecting the largest dataset of
NPI implementation dates across Europe. Business closures, educational institution
closures, and gathering bans reduced transmission, but reduced it less than they did in
the first wave. This difference is likely due to organisational safety measures and
individual protective behaviours — such as distancing — which made various areas of
public life safer and thereby reduced the effect of closing them. Specifically, we find
smaller effects for closing educational institutions, suggesting that stringent safety
measures made schools safer compared to the first wave. Second-wave estimates outperform
previous estimates at predicting transmission in Europe's third wave.

### Introduction

A second wave followed the reopening of European societies (≈August 2020–January 2021).
First-wave NPI effectiveness was measured relative to baseline contact patterns in the
very early pandemic, when safety measures and protective behaviours were lacking — so
first-wave estimates proxy how much transmission is associated with various areas of
public life *operated without safety measures*, and are likely inadequate for an ongoing
pandemic. After the first wave, contact patterns did not return to pre-pandemic normal
(distancing, testing, ventilation), likely making areas of public life safer and
reducing the additional effect of closures. Governments also need estimates for the
finer-grained NPIs used later (specific business-sector closures, small gathering-size
bans, nighttime curfews). Identifying individual effects requires a *multinational,
subnational* dataset (national modelling would obscure local heterogeneity and risk
ecological fallacies). Since existing trackers lack granular subnational data, we built a
novel dataset across 114 regions in 7 countries (Austria, Czech Republic, England,
Germany, Italy, Netherlands, Switzerland) and a semi-mechanistic hierarchical Bayesian
model with a latent random walk and stochasticity for low case counts.

### Results and discussion

**The combined effect of all NPIs was smaller in the second wave.** All NPIs together
reduced $R_t$ by **66% [61–69%]**, vs first-wave medians of 77–82%. The most stringent
set actually implemented in each region reduced $R_t$ by **56% [40–64%]** (vs 76–82%),
even though second-wave NPIs were often similarly strict or stricter. These differences
are likely explained by pre-intervention contact patterns, safety measures, and
protective behaviours — population immunity, reduced adherence, and ascertainment changes
seem less important.

**A detailed assessment.** Second-wave NPIs were more spaced out (only 23% of
interventions starting in the same 10-day window vs 83% in the first wave), enabling
finer-grained identification given the larger subnational dataset (9.2× more NPI
implementations than the largest Europe-focused study). Key estimates (reduction in $R_t$):

- Business closures combined: **35% [29–41%]**; gastronomy **12% [8–17%]** (in line with
  the UK "eat out to help out" increases); nightclubs **12% [8–17%]**; retail + close-
  contact services **12% [7–18%]**; leisure/entertainment venues only **3% [−1 to 10%]**.
- Banning all gatherings (incl. 1-on-1): **26% [18–32%]**. No diminishing returns — the
  *strictest* thresholds (≤2 people) had considerably larger effects; lenient limits (≥10)
  had small effects (contrasting the first wave). A "lockdown" (all gatherings banned + all
  nonessential businesses closed): **52% [47–56%]**. Both public gatherings and private
  household mixing contributed (private mixing only effective at the strict 2-person
  threshold).
- Closing all educational institutions: only **7% [4–10%]** — in *strong contrast* to the
  first wave, where it was among the most effective NPIs. Conjectured cause: safety
  measures, behaviour changes, and a reduced "signalling" effect (school closures were not
  among the early second-wave NPIs). The school-only vs university-only split was not
  robust, so the combined effect is reported.
- Stricter mask-wearing policy (most/all public spaces, vs prior select-spaces policies):
  **12% [7–17%]** additional benefit. Nighttime curfews: **13% [6–20%]** (additional
  effect on top of other NPIs).

**Robustness.** 17 sensitivity analyses across 86 experimental conditions (priors,
structure, epidemiological delays, region resampling, unobserved confounders): some NPIs
robustly outperform others, so high-level policy conclusions hold; but as an observational
study, the true strength of unobserved confounding is unknown.

**Generalisation across time.** Second-wave estimates should predict current effects if
safety measures/behaviours are similar (and they have been relatively stable through the
second and third waves). Testing on third-wave national data (Jan–May 2021, influenced by
B.1.1.7 and vaccination): first-wave estimates overestimate observed $R_t$ changes by ~18
percentage points on average, while **second-wave estimates overestimate by only ~2 pp** —
though this experiment has limitations (confounding by VOC arrival or unrecorded NPIs).

**Implications.** Closures and bans still considerably reduced transmission in the second
wave, but less than the first. First-wave estimates overestimate effectiveness in an
ongoing pandemic. Educational institutions with appropriate safety measures can be made
considerably safer; only the strictest gathering-size limits remain effective; face-to-face
businesses still carry considerable transmission; and stricter mask policies and nighttime
curfews can help. NPI effectiveness being *dynamic in time* is an important,
under-discussed policy consideration; real-time modelling of evolving NPI effects should
be a priority.

### Methods (summary)

**Data.** A custom NPI dataset: 7 countries, 114 regions of analysis, 1 Aug 2020 – 9 Jan
2021, ~19,000 region-days, >5,500 NPI entries (each with start/end dates, quotes, and
government/university/legal/media sources). Regions of analysis are the highest
administrative division with uniform NPIs; whole countries for Austria/Czechia/Italy/
Netherlands, stratified random samples of 15 regions elsewhere (stratified by first-wave
COVID deaths); regions with <2,000 cases excluded. **Data collection** used
semi-independent double entry (each country by two authors; second-round entry with access
to sources/quotes but not the first round's NPI data), interviews with local
epidemiologists, external-source validation, and automated checks — ~950 hours of manual
collection. B.1.1.7-contaminated observations (>10% VOC) were excluded (mostly English
regions).

**Model.** A semi-mechanistic hierarchical Bayesian model (adapting Brauner et al.):
$R_{t,l} = \tilde{R}_{0,l} \cdot \prod_{i=1}^{I} \exp(-\beta_i x_{i,t,l}) \cdot
\exp(z_{t,l})$, where $\tilde{R}_{0,l}$ is the no-NPI reproduction number on 1 Aug 2020
(TruncatedNormal(1.35, 0.3²)), NPI effects $\beta_i$ are time-invariant and shared across
locations with an Asymmetric Laplace shrinkage prior (80% mass on positive effects), and
$z_{t,l}$ is a per-location latent **weekly random walk** flexibly absorbing unobserved
factors (e.g. unrecorded NPIs, low adherence). $R_{t,l}$ drives a discrete-renewal
infection process with *additive infection noise* (important at low incidence); infections
are convolved with infection-to-confirmation / infection-to-death delays (re-estimated from
second-wave linelist data: onset-to-confirmation gamma mean 5.28 d; onset-to-death gamma
mean 18.61 d; generation interval mean 4.83 d) and observed cases/deaths follow
country-specific negative binomials. The model is invariant to time-constant IFR/IAR.
Implemented in NumPyro with NUTS (4 chains, 250 warmup + 1250 draws = 5000 samples; no
divergences). Data and code: github.com/MrinankSharma/COVID19NPISecondWave.
