---
title: "The rise of the Delta Covid variant"
full_title: "Changing composition of SARS-CoV-2 lineages and rise of Delta variant in England"
authors:
  - Swapnil Mishra
  - Sören Mindermann
  - Mrinank Sharma
  - Charles Whittaker
  - Thomas Mellan
  - Thomas Wilton
  - Dimitra Klapsa
  - Ryan Mate
  - Martin Fritzsche
  - Maria Zambon
  - Janvi Ahuja
  - Adam Howes
  - Xenia Miscouridou
  - Guy P. Nason
  - Oliver Ratmann
  - Gavin Leech
  - Julia Fabienne Sandkühler
  - Charlie Rogers-Smith
  - Michaela Vollmer
  - Juliette T. Unwin
  - Yarin Gal
  - Meera Chand
  - Axel Gandy
  - Javier Martin
  - Erik Volz
  - Neil M. Ferguson
  - Samir Bhatt
  - Jan M. Brauner
  - Seth Flaxman
gleech_role: co-author
year: 2021
venue: "The Lancet — eClinicalMedicine"
type: journal
doi: 10.1016/j.eclinm.2021.101064
url: https://authors.elsevier.com/sd/article/S2589537021003448
code: https://github.com/ImperialCollegeLondon/SARS_CoV_2_variants_uk
links:
  explainer: https://twitter.com/JanMBrauner/status/1421549366926614534
  discussion: https://www.imperial.ac.uk/news/221873/uk-covid-19-transmission-shifted-towards-emerging/
contribution_hours: 10
topics: [covid-19, genomics, variants, delta, surveillance]
---

## Abstract

Using tests and sewage data from early 2021, we saw that "the English variant" of
COVID (B.1.1.7), which took over England in December, was itself being displaced
by other variants. The main worry was that one of the new variants would be
resistant to the vaccines.

## Full text

> Converted from the open-access Lancet eClinicalMedicine full text (Europe PMC,
> PMC8349999). Markup removed; figures/supplement and the reference list omitted.

**Background.** Since its emergence in Autumn 2020, the SARS-CoV-2 Variant of Concern
(VOC) B.1.1.7 (WHO label Alpha) rapidly became the dominant lineage across much of
Europe. Simultaneously, several other VOCs were identified globally. Unlike B.1.1.7, some
of these VOCs possess mutations thought to confer partial immune escape. Understanding
when and how these additional VOCs pose a threat in settings where B.1.1.7 is currently
dominant is vital.

**Methods.** We examine trends in the prevalence of non-B.1.1.7 lineages in London and
other English regions using passive-case-detection PCR data, cross-sectional community
infection surveys, genomic surveillance, and wastewater monitoring. The study period
spans 31 January 2021 to 15 May 2021.

**Findings.** Across data sources, the percentage of non-B.1.1.7 variants increased since
late March 2021, initially driven by a variety of lineages with immune escape. From
mid-April, B.1.617.2 (Delta) spread rapidly, becoming the dominant variant in England by
late May.

**Interpretation.** The outcome of competition between variants depends on intrinsic
transmissibility, evasion of prior immunity, demographics, and interactions with NPIs.
The presence and rise of non-B.1.1.7 variants in March was likely driven by importations
and some community transmission; competition between them resulted in B.1.617.2 becoming
dominant in April–May with considerable community transmission. Early detection of new
variants requires a diverse array of community-surveillance data sources.

### Introduction

B.1.1.7 (epidemiologically 50–80% more transmissible, more severe) rose from ~0% to >98%
of sequenced English samples, necessitating a third national lockdown in January 2021.
Its 69–70 Spike deletion causes S-gene target failure (SGTF) in PCR, a fast proxy for
genomic surveillance (SGTF frequency rose from ~0% in October 2020 to 98.8% in March
2021). Other VOCs — B.1.351 (South Africa), P.1 (Brazil), B.1.617.2 (India) — lack the
69–70 deletion (so are S-gene positive, "S+") and carry immune-escape mutations (E484K or
T478K). By early April 2021 ~55% of the English population were seropositive (infection or
vaccination); such immunity is a selection pressure that could give partially
immune-escaping VOCs a fitness advantage, especially as control measures relaxed and the
rollout relied heavily on AstraZeneca (highly protective against B.1.1.7, possibly reduced
against other VOCs).

### Methods (four data sources)

- **Pillar 2 symptomatic community testing.** ~30% of Lighthouse-lab samples use the
  ThermoFisher TaqPath assay including Spike; SGTF is an accurate B.1.1.7 proxy at non-Spike
  Ct ≤ 30. Over 31 Jan – 15 May 2021 (symptomatic, non-lateral-flow): 72,881 S+ and 586,854
  S− cases in England (4,246 S+ / 79,207 S− in London).
- **ONS infection survey.** A fortnightly random-household PCR survey (139,948 participants
  in the fortnight before 16 April 2021); we use the "UK variant compatible" vs "not
  compatible" gene patterns.
- **Sewage-water monitoring.** Fortnightly samples from Beckton Sewage Treatment Works (~4
  million people, North London) — low sampling bias, capturing everyone in the catchment.
- **COG-UK genomic sequencing.** 10,324 sequences from Greater London Pillar 2 testing
  after 1 March 2021; 2,957 non-B.1.1.7 (including B.1.617.2 n = 2,225, B.1.351 n = 254,
  P.1 n = 81). Lineage frequencies over time via a Gaussian-process multinomial GAM; analysis
  repeated excluding travel-linked cases.

### Results

Since early March 2021, S+ case incidence (Pillar 2) increased against falling-then-stable
overall case numbers, earliest in London but in every region. ONS S+ prevalence rose
slightly in March, dipped, then rose strongly in early May (lagging Pillar 2, with
sampling variability at low incidence). **Ct values:** until March, S− (mostly B.1.1.7)
samples had lower Ct (higher viral load) than S+; since end-March, mean S+ Ct decreased to
comparable levels, suggesting increased S+ transmission. **Sewage:** B.1.1.7-defining
mutations (HV69–70del, Y144del, A570D), stable >95% from January to mid-March, fell to
67–75% by 13 April and 28–49% by 11 May; E484K rose above 30% by mid-April; B.1.617.2
mutations rose to 41–62% by mid-May (≈half the viral population). **COG-UK sequencing:**
non-B.1.1.7 in March was chiefly B.1.351 and B.1.525, then B.1.617.2 rose rapidly over
April to >75% of London sequences by late May — a similar pattern when excluding
travel/surge-testing cases, indicating community transmission.

### Discussion

Using four independent data sources, we show recent increases in the proportion of S+
infections — first a variety of immune-evading lineages, then overwhelmingly B.1.617.2. A
key question when a new variant's proportion rises is whether it reflects local
transmission or imported infections against low overall incidence; by the time of writing,
B.1.617.2 was dominant (>75% of London sequences even excluding travel), indicating
sustained community transmission. A considerable rise in non-B.1.1.7 was apparent across
sources by early/mid-April, giving an early warning — though the community-transmission
signal was then ambiguous (only ~10% of non-travel-linked cases were non-B.1.1.7 in
mid-April; Ct and sewage signals could partly reflect importation given London's airports).
Variant competition depends on intrinsic transmissibility, immune escape, and targeted
NPIs; even if less intrinsically transmissible than B.1.1.7, immune-escaping VOCs may gain
an advantage in a highly immunised population (cf. rapid resurgences in Manaus with P.1 and
Delhi with B.1.617.2 despite high prior immunity). The results underscore the value of a
diverse array of community-surveillance data sources and timely genomic surveillance for
real-time lineage tracking — critical to immediate control and future vaccine development.
Data/code: github.com/ImperialCollegeLondon/SARS_CoV_2_variants_uk.
