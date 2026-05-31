---
title: "Government interventions against Covid"
full_title: "Inferring the effectiveness of government interventions against COVID-19"
authors:
  - Jan M. Brauner
  - Sören Mindermann
  - Mrinank Sharma
  - David Johnston
  - John Salvatier
  - Tomáš Gavenčiak
  - Anna B. Stephenson
  - Gavin Leech
  - George Altman
  - Vladimir Mikulik
  - Alexander John Norman
  - Joshua Teperowski Monrad
  - Tamay Besiroglu
  - Hong Ge
  - Meghan A. Hartwick
  - Yee Whye Teh
  - Leonid Chindelevitch
  - Yarin Gal
  - Jan Kulveit
gleech_role: co-author
year: 2020
venue: Science
type: journal
doi: 10.1126/science.abd9338
url: https://www.science.org/doi/10.1126/science.abd9338
code: https://zenodo.org/record/4268449
links:
  discussion: https://statmodeling.stat.columbia.edu/2020/12/18/inferring-the-effectiveness-of-government-interventions-against-covid-19/
  podcast: https://turing.podbean.com/e/ttps2e6/
  app: http://epidemicforecasting.org/calc
contribution_hours: 130
topics: [covid-19, epidemiology, non-pharmaceutical-interventions, hierarchical-bayesian, policy]
---

## Abstract

We used a hierarchical Bayesian model to see what worked in the first wave of the
pandemic. Until then, people hadn't picked apart the individual effects of
anti-COVID policies, instead using "lockdown" to name all 20 different things
governments tried in spring 2020 (where it should really mean stay-at-home
orders).

We collected a big new dataset (41 countries) and were among the first to spot the
large effect of closing schools and universities, back when people hoped children
were magically not infectious. Our validation was unusually large and rigorous for
epidemiology. Stay-at-home orders did surprisingly little (0–25% reduction) *if*
schools, restaurants, and big events were already closed.

We initially attempted a cost–benefit analysis of each policy (surveying Americans
on how much each interferes with their lives) but it wasn't good enough for the
final paper — and, to our knowledge, still hasn't been done well outside secret
government documents, despite being essential for good decisions.

## Full text

> Extracted from the published Science PDF (Science 371, eabd9338; 19 Feb 2021) and
> reflowed from the two-column layout into logical reading order. Citation numbers
> and figure/table call-outs are dropped; figures and the reference list are
> omitted. The above is the author's plain-language summary; the journal's own
> abstract and text follow.

**Editor's summary — How to hold down transmission.** Early in 2020, SARS-CoV-2
transmission was curbed in many countries by imposing combinations of
nonpharmaceutical interventions. Sufficient data on transmission have now
accumulated to discern the effectiveness of individual interventions. Brauner et
al. amassed and curated data from 41 countries as input to a model to identify the
individual NPIs that were most effective at curtailing transmission during the
early pandemic. Limiting gatherings to fewer than 10 people, closing high-exposure
businesses, and closing schools and universities were each more effective than
stay-at-home orders, which were of modest effect in slowing transmission.

**Abstract.** Governments are attempting to control the COVID-19 pandemic with
nonpharmaceutical interventions (NPIs). However, the effectiveness of different NPIs
at reducing transmission is poorly understood. We gathered chronological data on the
implementation of NPIs for several European and non-European countries between
January and the end of May 2020. We estimated the effectiveness of these NPIs, which
range from limiting gathering sizes and closing businesses or educational
institutions to stay-at-home orders. To do so, we used a Bayesian hierarchical model
that links NPI implementation dates to national case and death counts and supported
the results with extensive empirical validation. Closing all educational
institutions, limiting gatherings to 10 people or less, and closing face-to-face
businesses each reduced transmission considerably. The additional effect of
stay-at-home orders was comparatively small.

### Introduction

Worldwide, governments have mobilized resources to fight the COVID-19 pandemic. A
wide range of NPIs has been deployed, including stay-at-home orders and the closure
of all nonessential businesses. Recent analyses show these large-scale NPIs were
jointly effective at reducing the virus's effective reproduction number $R_t$, but
it is still largely unknown how effective *individual* NPIs were. A promising way to
estimate NPI effectiveness is data-driven, cross-country modeling: inferring
effectiveness by relating the NPIs implemented in different countries to the course
of the epidemic. To disentangle the effects of individual NPIs, we leverage data
from multiple countries with diverse interventions: we evaluated the impact of
several NPIs on the epidemic's growth in 34 European and 7 non-European countries.
Because the COVID-19 response was far less coordinated than a single common
intervention day, countries implemented different sets of NPIs at different times
and orders, making individual effects identifiable.

Estimating NPI effects remains challenging: models rest on uncertain epidemiological
parameters (which we incorporate directly); the data are retrospective and
observational (so unobserved factors could confound); and effectiveness estimates
can be highly sensitive to arbitrary modeling decisions, as shown by two recent
replication studies. We collected a large public dataset on NPI implementation dates
validated by independent double entry, and extensively validated our effectiveness
estimates — a crucial but often absent element of COVID-19 NPI studies. We only
analyzed the effect NPIs had between January and the end of May 2020, and NPI
effectiveness may change over time; lifting an NPI does not imply transmission
returns to its original level, and our window does not include relaxation of NPIs.

### Cross-country NPI effectiveness modeling

We analyzed the effects of seven commonly used NPIs between 22 January and 30 May
2020, all aimed at reducing contacts within the population (Table 1). If a country
lifted an NPI before 30 May, its analysis window terminates on the lifting day. To
ensure quality, all NPI data were independently entered by two authors (independent
double entry) using primary sources, then compared with several public datasets.
Case and death data came from the Johns Hopkins CSSE dataset.

We estimated NPI effectiveness with a Bayesian hierarchical model: case and death
data are used to infer new infections over time, which infer the instantaneous
reproduction number $R_t$; NPI effects are estimated by relating daily reproduction
numbers to the active NPIs across all days and countries. This data-driven approach
sidesteps assumptions about contact patterns, infectiousness by age group, etc., and
directly models many sources of uncertainty (uncertain epidemiological parameters,
between-country differences in NPI effectiveness, unknown changes in testing and
infection-fatality rates, and unobserved influences on $R_t$).

**Table 1 — the seven NPIs:** gatherings limited to 1000 / 100 / 10 people or less;
some (high-risk) businesses closed (e.g. restaurants, bars, nightclubs, cinemas,
gyms); most nonessential businesses closed; schools closed; universities closed; and
stay-at-home order (mandatory; whenever introduced, countries essentially always
already had all other NPIs in place, so we estimate its *additional* effect).

### Effectiveness of individual NPIs

We expressed each NPI's effect as a percentage reduction in $R_t$, with Bayesian
prediction intervals (wider than credible intervals; analogous to the standard
deviation of effectiveness across countries). Under default model settings, the
percentage reduction in $R_t$ (95% prediction interval) was:

- limiting gatherings to 1000 people or less: **23%** (0 to 40%)
- limiting gatherings to 100 people or less: **34%** (12 to 52%)
- limiting gatherings to 10 people or less: **42%** (17 to 60%)
- closing some high-risk face-to-face businesses: **18%** (−8 to 40%)
- closing most nonessential face-to-face businesses: **27%** (−3 to 49%)
- closing both schools and universities in conjunction: **38%** (16 to 54%)
- issuing stay-at-home orders (additional effect on top of all other NPIs): **13%**
  (−5 to 31%)

We could not robustly disentangle closing only schools from only universities
(implemented on the same day or in close succession in most countries), so reported
them as one NPI. Some NPIs co-occurred (partly collinear), but the collinearity was
imperfect and the dataset large enough to isolate individual effects: for every NPI
pair, on average 504 country-days had one without the other (minimum 148). High
sensitivity from excessive collinearity prevented Flaxman et al. (with a smaller
dataset) from disentangling NPI effects; our estimates are substantially less
sensitive, and posterior correlations between estimates are weak.

### Effectiveness of NPI combinations

Although correlations between individual estimates were weak, we accounted for them
when evaluating combined effectiveness. In combination, the NPIs in this study
reduced $R_t$ by **77%** (67 to 85%). Across countries, the mean $R_t$ without any
NPIs (the $R_0$) was 3.3. Starting from this, $R_t$ likely could have been brought
below 1 by closing schools and universities, closing high-risk businesses, and
limiting gatherings to at most 10 people.

### Sensitivity and validation

We performed a range of validation and sensitivity experiments. The model generated
calibrated forecasts for countries that did not contribute fitting data for up to 2
months, with uncertainty increasing over time. We considered NPI effectiveness under
**206 alternative experimental conditions** across 11 sensitivity analyses (varying
priors over epidemiological parameters, excluding countries, using only deaths or
only cases, varying preprocessing, alternative model structures, and possible
confounding by unobserved factors). Categorizing effects as small (<17.5%), moderate
(17.5–35%), or large (>35%): closing schools and universities was large in 96% of
conditions; limiting gatherings to 10 or less was large in 99%; closing most
nonessential businesses was moderate in 98%; stay-at-home orders fell into "small
effect" in 96%. Three NPIs were less clear-cut (gatherings ≤1000: moderate in 81%;
≤100: moderate in 66%; some high-risk businesses: moderate in 58%). Trends were
robust even though precise estimates varied.

### Discussion

We found several NPIs associated with a clear reduction in $R_t$, and that some
outperformed others — broadly robust across the 206 conditions. Business closures and
gathering bans both seem to have been effective; closing most nonessential businesses
was only somewhat more effective than targeted high-risk closures, so targeted
closures can be a promising option. Limiting gatherings to 10 or less was more
effective and more robust than larger limits (estimates derive from Jan–May 2020,
when most gatherings were likely indoors). Issuing a stay-at-home order had only a
small effect *once* educational institutions and nonessential businesses were closed
and gatherings banned — suggesting some countries could have reduced $R_t$ below 1
without a stay-at-home order. In contrast, Flaxman et al. and Hsiang et al. folded
several NPIs into their "lockdown" effect and accordingly found a large effect.

The large, robust effect for closing schools and universities cannot distinguish
direct transmission effects from indirect ones (e.g. the general population behaving
more cautiously after closures signaled the gravity of the pandemic), nor school from
university closures. Evidence on the role of pupils/students in transmission is
mixed.

Limitations: (i) NPI effectiveness depends on implementation context (other NPIs,
demographics, details), so estimates indicate effectiveness in the contexts observed;
expert judgment should adjust to local circumstances. (ii) $R_t$ may have been reduced
by unobserved NPIs or voluntary behavior changes such as mask-wearing; additional
analyses found results stable to a range of unobserved factors, but this cannot
provide certainty. (iii) Results cannot be used without qualification to predict the
effect of *lifting* NPIs (e.g. reopening with safety measures). (iv) We lack data on
some promising interventions (testing, tracing, case isolation), and it is difficult
to estimate mask-wearing effects given limited public life under other NPIs.

### Materials and methods

**Dataset.** Seven NPIs in 41 countries (33 in Europe). We recorded NPIs implemented
nationally or in most regions (affecting ≥3/4 of the population), only mandatory
restrictions. Each country's window starts 22 January and ends at the first NPI
lifting or 30 May 2020, whichever came first (ending after first major reopening
avoids distribution shift).

**Data collection.** Two authors independently researched each country and entered NPI
data into separate spreadsheets (manual internet searches; ~1.5 hours per researcher
per country), then compared against the EFGNPI Database and the Oxford COVID-19
Government Response Tracker, resolving conflicts via primary sources. Each country/NPI
was again independently entered by 1–3 paid contractors with primary sources; a
researcher resolved conflicts; finally the two spreadsheets were combined and all
conflicts resolved. The final dataset contains primary sources for each entry.

**Data preprocessing.** To avoid bias from imported cases and rapidly changing testing
when counts are small, we neglected cases before a country reached 100 confirmed
cases and deaths before 10 deaths (both included in the sensitivity analysis).

**Model.** Building on the semimechanistic Bayesian hierarchical model of Flaxman et
al., with additions: (1) observing both case and death data; (2) placing priors over
uncertain epidemiological parameters; (3) avoiding assuming a specific
infection-fatality or ascertainment rate; (4) allowing all NPI effects to vary across
countries. Each NPI independently affects $R_{t,c}$ as a multiplicative factor:

$$R_{t,c} = R_{0,c} \prod_{i=1}^{I} \exp(-a_{i,c}\, x_{i,t,c})$$

where $x_{i,t,c}=1$ if NPI $i$ is active in country $c$ on day $t$, $I$ is the number
of NPIs, and $a_{i,c}$ is the effect parameter (the multiplicative form encodes that
NPIs have a smaller absolute effect when $R_{t,c}$ is already low). The effect
parameter is $a_{i,c} = a_i + z_{i,c}$ with $z_{i,c} \sim N(0, \sigma_i^2)$; partial
pooling minimizes bias from country-specific sources while reflecting cross-country
variation. Effectiveness (percentage reduction in $R_t$) is $1 - \exp(-(a_i + z_i))$.
An asymmetric Laplace prior on $a_i$ allows positive and negative effects but places
80% of its mass on positive effects.

During early exponential growth there is a one-to-one correspondence between the daily
growth rate and $R_{t,c}$, depending on the generation interval (assumed gamma; prior
mean 5.06 days from meta-analysis). New infections are modeled separately for cases
and deaths (both growing at the same daily rate in expectation), translating into
reported counts after delays (prior mean infection-to-confirmation 10.92 days;
infection-to-death 21.8 days). Reported cases and deaths follow negative binomial
output distributions. Inference uses MCMC (the No-U-Turn Sampler).
