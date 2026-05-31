---
title: "Methods Failing the Data, Data Failing the Methods"
authors:
  - Gavin Leech
gleech_role: sole author
year: 2024
venue: University of Bristol
type: thesis
pdf: https://research-information.bris.ac.uk/ws/portalfiles/portal/437692523/methods_failing_the_data.pdf
links:
  postmortem: https://www.gleech.org/phdone
  hours_breakdown: https://www.gleech.org/phd-numbers
contribution_hours: 3500
topics: [epistemics, metascience, approximate-bayesian-inference, epidemiology, machine-learning, algorithmic-fairness, inductive-logic-programming]
---

## Abstract

> The official thesis abstract (verbatim).

Fields work in very different ways — but fail in similar ways. This thesis covers
some of my work in epidemiology, psychology, and machine learning with the common
thread of shared methodological issues. I identify frameworks which fail to cover
actual practice, practices which fail to live up to normative principles, and
propose practices which are sometimes able to address some failures at some cost.

In epidemiology I take a Bayesian approach to infectious disease modelling and
infer the effect of entire populations wearing face masks during the Covid
pandemic — with the key caveat that this is an observational study. I identify a
ubiquitous methodological mistake (using mandate timings as a proxy for wearing
behaviour, when these are, surprisingly, not strongly correlated).

In psychology I synthesise theories of the replication crisis and report on
initiating a large (n = 1931) dataset of replication studies covering the original
effect sizes, replication effect sizes, and both raw and recalculated statistics.
These nonrandom data still give some insight into post-crisis psychology,
confirming past results showing that even the *sign* of effects is often not
replicable. I note a pattern of considerable 'shrinkage' in effect sizes between
the original study and their replications.

In machine learning I trace recent methodological changes and provide a novel
analysis of roughly forty ways that ML evaluations are often misleading.

Each chapter contains a self-contained background section for its respective field.
I conclude with lessons for each field from the other two.

> "an impressive work of scholarship… The epidemiology has had broad influence,
> including on policy discussion at a national level… the machine learning chapter
> is an admirable feat of investigation and synthesis." — Conor Houghton

*Author's gloss:* Broadly it is about epistemics: why we can't learn that much from
one study, or from many studies. In [Newell's typology](https://www.eecs.harvard.edu/htk/phdadvice/#3)
the thesis "contradicts existing knowledge; thoroughly explores an area; provides
empirical data; and produces a negative result." A Gelmanian work — Gelman is cited
74 times, in every chapter. *Contribution: ~3500 hours on the whole PhD (226 on the
writeup).*

## Full text

> The thesis runs to ~165,000 words across four chapters. Rather than inline it all
> (its chapters are the substance of several other entries in this folder), this
> section reproduces the thesis's unique synthesis material — the structure, the
> per-paper contributions, and the cross-field conclusion — and cross-links each
> chapter to the corresponding paper. Full PDF linked in the frontmatter.

### Table of contents

- **Front matter:** Contributions; Glossary; Software
- **1. Introduction**
- **2. Bayes: How some epidemiologists learn from data** (epidemiology)
  - Background: Bayesian inference and workflow
  - Background: Epidemiology and Bayes
  - Application: Modelling nonpharmaceutical interventions and mask-wearing
  - *Constituent papers:* [inferring-npi-effectiveness](inferring-npi-effectiveness.md),
    [npi-robustness](npi-robustness.md), [mask-wearing](mask-wearing.md),
    [seasonal-covid](seasonal-covid.md), [parallel-reweighted-wake-sleep](parallel-reweighted-wake-sleep.md)
- **3. Crisis: How psychologists learn from data** (psychology)
  - Background: the workflow of frequentist inference
  - Literature review: The replication crisis
  - 'Do more replications'
  - The FORRT Replications and Reversals database
  - Against binary science
  - *Constituent papers:* [tracking-replications](tracking-replications.md),
    [replication-database](replication-database.md),
    [replications-reversals-social-science](replications-reversals-social-science.md)
- **4. Bitter: How ML people learn from data** (machine learning)
  - Background: ML workflow
  - Background: The deep learning revolutions
  - Questionable research practices in ML
  - *Constituent papers:* [questionable-practices-ml](questionable-practices-ml.md),
    [ten-hard-problems](ten-hard-problems.md),
    [decision-trees-misspecification](decision-trees-misspecification.md),
    [ilp-safety](ilp-safety.md)
- **Position paper** (outro)
- **Conclusion**

### Contributions

The thesis draws on the following published work. Bullet points denote Gavin's
contributions; * denotes equal authorship.

**As first or senior (last) author:**

1. *Mask wearing in community settings reduces SARS-CoV-2 transmission* (2022, PNAS)
   — Gavin Leech\*, Charlie Rogers-Smith\*, et al. → [mask-wearing.md](mask-wearing.md)
   - Noted the global pre-mandate voluntary spike in mask wearing
   - Identified a ubiquitous methodological mistake (the mandate timing proxy)
   - First international analysis of masks with a random sample of self-reported
     mask-wearing data; constructed and validated a global dataset from multiple
     sources
   - First Bayesian hierarchical model for mask wearing, adapting an existing NPI
     model
   - Functional-form modelling of mask-wearing effects
   - Lead writer
2. *Ten Hard Problems in Artificial Intelligence We Must Get Right* (2024, in review
   at ACM Computing Surveys) → [ten-hard-problems.md](ten-hard-problems.md)
   - Lead writer; sole author on the capabilities, alignment, opportunities, access,
     and meaning sections
   - Characterised early deep learning and the 'large scale era'
   - 10 diagrams and informal models (AI governance field; two decompositions of the
     alignment problem; the family tree of AlexNet and GPT-2)
3. *Tracking replications in the social, cognitive, and behavioural sciences* (2024,
   in review at Nature Human Behaviour) → [tracking-replications.md](tracking-replications.md)
   - Initiated the project alone with 53 famous effects that fail to replicate
   - Helped organise crowdsourcing of the full dataset of 1932 experiments
   - Designed analyses of the resulting nonrandom sample
   - Sole writer on first draft
4. *Questionable practices in machine learning* (2024, in review at JMLR) →
   [questionable-practices-ml.md](questionable-practices-ml.md)
   - Taxonomy of questionable practices; collated published and unpublished examples
   - Generated possible solutions; literature review relating ML to metascience
   - Lead writer

**As other author:**

5. *Inferring the effectiveness of government interventions against COVID-19* (2020,
   Science) → [inferring-npi-effectiveness.md](inferring-npi-effectiveness.md)
   - Wrote most of the first draft; literature review of semi-mechanistic models;
     helped set the epidemiological priors; one among many authors on NPI data
     collection
6. *How Robust are the Estimated Effects of Nonpharmaceutical Interventions against
   COVID-19?* (2020, NeurIPS spotlight) → [npi-robustness.md](npi-robustness.md)
   - Formalised model assumptions; worked on theorems 1 and 2; writing on final
     draft; diagrams (model variations)
7. *Seasonal variation in SARS-CoV-2 transmission in temperate climates* (2022, PLOS
   Computational Biology) → [seasonal-covid.md](seasonal-covid.md)
   - Insolation analysis; helped formalise the model; diagrams (key figure 2); wrote
     half of the first draft
8. *Massively Parallel Reweighted Wake-Sleep* (2023, UAI) →
   [parallel-reweighted-wake-sleep.md](parallel-reweighted-wake-sleep.md)
   - Sole implementation of an earlier branch of the project; editing on final draft
9. *The Replication Database: Documenting the Replicability of Psychological Science*
   (2024, Journal of Open Psychology Data) → [replication-database.md](replication-database.md)
   - Initiated one of the source datasets; editing on final draft

### Conclusion

The thesis's *de facto* research questions and their answers:

**Epidemiology.**
- *Why do studies of individual mask-wearing conflict with population-level studies
  of mask mandates?* Because a key assumption of the mandate-timing method was
  violated: most of the wearing uptake was pre-mandate in most regions, so the
  binary 'after mandate enforcement' variable is a poor proxy for wearing levels —
  those studies are incoherent and to be deprecated.
- *What did mass mask-wearing achieve during the first year of the Covid pandemic?*
  Some evidence of a substantial ~25% [6%, 43%] reduction in $R_t$ for a
  fully-masked population (slightly lower in practice at 70–85% wearing). The
  estimate is time-bounded (pre-saturation of vaccination/immunity) and reflects
  median mask quality (mostly cloth or surgical masks).
- *How does modelling seasonality change estimates of government-intervention
  effects?* Even a single scalar (the amplitude of the annual sine wave in Europe)
  greatly improved model fit, implying seasonality is a significant but
  easily-estimated confounder. On 2020 data, transmission varies 42% [25%, 53%] from
  peak winter to peak summer.

**Psychology.**
- *What could explain the replication crisis?* Dozens of possible contributors;
  evidence given for each, but no synthesis or structural model.
- *How do effect sizes change under replication?* The nonrandom sample doesn't admit
  a proper answer, but descriptively there was an average 'shrinkage' of $d = 0.34$
  from originals to replications.
- *Does crowdsourcing replications help?* In the weak sense of being a tolerable way
  to collect information (independent double entry gives a <1% error rate); not
  justified as counteracting the replication crisis.
- *Would foregrounding estimation over discovery help?* Some pathologies can be seen
  as optimising a binary 'discover a qualitative effect' ($p<0.05$) objective; but
  little was done to justify that shifting to estimation would be better under equal
  adversarial pressure.

**Machine learning.**
- *How has ML changed in the last decade?* Extensively — deep learning has taken
  over large parts of the field. Besides the belated triumph of neural networks, six
  other huge methodological shifts: the scaling era; end-to-end learning; standard
  training frameworks; pretraining and transfer; prompting as research; and the
  privatisation and turn to secrecy.
- *In what ways can an evaluation metric be misleading?* In at least 43 ways.

**As a whole.** The thesis's strength is that it collates work which *led somewhere*:
the Covid papers were used in (at least) UK and Czech policy decisions; the
psychology replication collection has some chance of being a standard reference with
ongoing crowdsourcing; the QRP work has been used at Ofcom and as part of an
evaluation checklist in at least one industrial lab. Its weakness is that it isn't
theoretically deep: the problems are not common to all three fields (besides
underreporting, the *ur*-problem which hides all other problems). The original plan
was to compare the evidential standards of the three fields and unify Bayesian
inference, hypothesis testing, and statistical learning.
