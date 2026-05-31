---
title: "A dataset of non-pharmaceutical interventions on SARS-CoV-2 in Europe"
authors:
  - George Altman
  - Janvi Ahuja
  - Joshua Teperowski Monrad
  - Gurpreet Dhaliwal
  - Charlie Rogers-Smith
  - Gavin Leech
  - Benedict Snodin
  - Jonas B. Sandbrink
  - Lukas Finnveden
  - Alexander John Norman
  - Sebastian B. Oehm
  - Julia Fabienne Sandkühler
  - Jan Kulveit
  - Seth Flaxman
  - Yarin Gal
  - Swapnil Mishra
  - Samir Bhatt
  - Mrinank Sharma
  - Sören Mindermann
  - Jan Markus Brauner
gleech_role: co-author
year: 2022
venue: Nature Scientific Data
type: journal
doi: 10.1038/s41597-022-01175-y
url: https://www.nature.com/articles/s41597-022-01175-y
links:
  data: https://figshare.com/articles/dataset/A_dataset_of_non-pharmaceutical_interventions_on_SARS-CoV-2_in_Europe/16777918/1
  explainer: https://twitter.com/jn_ahuja/status/1512010901175558144
topics: [covid-19, dataset, non-pharmaceutical-interventions, europe]
---

## Abstract

The dataset underlying the second-wave paper ("Understanding the effectiveness of
government interventions against the resurgence of COVID-19 in Europe"): a
structured record of non-pharmaceutical interventions against SARS-CoV-2 across
European regions.

See also: [npi-second-wave.md](npi-second-wave.md).

## Full text

> Converted from the open-access Nature Scientific Data descriptor (Europe PMC,
> PMC8975844). Markup removed; the exhaustive data-dictionary and reference list are
> condensed. This is a Data Descriptor, so it documents the dataset rather than
> reporting new findings.

**Abstract.** During the second half of 2020, many European governments responded to the
resurging transmission of SARS-CoV-2 with wide-ranging non-pharmaceutical interventions
(NPIs). These efforts were often highly targeted at the regional level and included
fine-grained NPIs. This paper describes a new dataset designed for the accurate recording
of NPIs in Europe's second wave to allow precise modelling of NPI effectiveness. The
dataset includes interventions from 114 regions in 7 European countries during 1 August
2020 to 9 January 2021, with NPI definitions tailored to the second wave following an
exploratory data collection. Each entry has been extensively validated by semi-independent
double entry, comparison with existing datasets, and, when necessary, discussion with
local epidemiologists.

### Background & summary

First-wave NPIs were generally national and analysed with public datasets; the second
wave saw local, fine-grained NPIs (small gathering-size bans, sector-specific business
closures, curfews) that first-wave definitions were too coarse to capture. This dataset's
distinguishing features: **precise NPI categorisation** (greater differentiation in
gathering size, gathering type, and business-closure types), **regional granularity** (for
a period when NPIs were often subnational), and **manual collection with semi-independent
validation** (higher quality than purely automated collection). It underpins Sharma et al.
(the second-wave effectiveness paper) and has been reused to adjust for NPIs when analysing
seasonality and mask use. Researchers should choose it when they need subnational data for
this period, per-entry context (comments, quotes, sources), second-wave-tailored
definitions, or fully validated data.

### Methods

**Design process.** An open-ended exploratory collection first identified the main
second-wave NPIs, suitable cross-country definitions, and the right administrative level
per country — revealing that first-wave definitions were too coarse (e.g. the strictest
gathering ban elsewhere is "≤10", but many key second-wave NPIs were limits of 6/4/2 or a
full ban). Design decisions were then finalised with team and external-expert input.

**Coverage.** 7 countries, 114 regions of analysis, 1 Aug 2020 – 9 Jan 2021, >5,500
entries. Administrative divisions: whole-country for Austria (9 states), Czechia (14
regions), Italy (21 regions), Netherlands (25 safety regions); stratified random samples of
15 for England (NUTS-3), Germany (districts, from the 4 largest states = 60% of
population), and Switzerland (cantons). Regions of analysis = the largest division with
uniform NPIs; regions with <2,000 cases excluded. Countries were included only where daily
case/death data were public at the NPI implementation resolution (excluding e.g. Spain's
~8,000 municipalities).

**NPIs recorded.** Public/private and indoor/outdoor gathering limits (exact person- and
household-thresholds); business closures (night clubs, gastronomy, leisure/entertainment,
retail + close-contact services, all non-essential — *closures only, not safety measures*);
primary/secondary school closures (incl. holidays and most-teaching-online; for
universities, vacation and student-dispersal periods); the OxCGRT mask-stringency scale
(0–4); and curfews. Stay-at-home orders were *excluded* (too many second-wave exceptions,
often only in legislation), as was shielding (no consistent cross-country definition).

**Data collection & validation.** Nine researchers, ~950 hours (185 on national timelines,
470 on regional data, 290 on validation). Each entry has dates, sources (government/legal/
media), quotes, and comments. Guiding heuristics: record an NPI when it affects "most or
all" (>50%) of the relevant institutions/events/people; record from when behaviour is
expected to change; record only mandatory NPIs (with a few documented exceptions for
private gatherings confirmed via local epidemiologists). Validation: comparison against
OxCGRT (and additional external sources for Italy/England); **semi-independent double
entry** (a second researcher re-entered dates with access to sources/quotes but not the
original dates; differences auto-flagged and resolved); cross-country consistency checks;
and automated checks (dates in range, chronological order, no spurious gaps).

**Data records.** Published on figshare — a human-readable CSV per country, plus a
machine-readable CSV (a row per day 1 Aug 2020 – 6 Jan 2021 with columns per NPI; ~19,000
rows across 144 regions) including daily cases/deaths. Recommended alternatives exist for
researchers needing more countries, greater geographic/income variety, the full pandemic
period, or economic measures (not included here).
