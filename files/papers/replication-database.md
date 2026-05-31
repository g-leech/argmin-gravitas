---
title: "ReD: The Replication Database"
full_title: "The Replication Database: Documenting the Replicability of Psychological Science"
authors:
  - Lukas Röseler
  - "… (95+ co-authors)"
  - Gavin Leech
gleech_role: co-author
year: 2024
venue: Journal of Open Psychology Data
type: journal
doi: 10.5334/jopd.101
url: https://openpsychologydata.metajnl.com/articles/10.5334/jopd.101
code: https://osf.io/aepcj
links:
  app: https://metaanalyses.shinyapps.io/replicationdatabase/
contribution_hours: 10
topics: [metascience, replication, psychology, open-data]
---

## Abstract

We present the Replication Database, hosting 1,239 original findings paired with
replication findings. Its infrastructure lets researchers submit, access, and
engage with replication findings; makes replications visible and findable via a
GUI; and tracks replication rates across factors such as publication year and
journal. Gavin's replications dataset formed one of several input datasets that
ReD generalises.

## Full text

> Converted from the open-access Journal of Open Psychology Data full text (Europe PMC,
> PMC12270267). Markup removed; the effect-size-conversion table is condensed and the
> reference list omitted. This is a data/database descriptor (~96 co-authors).

**Abstract.** In psychological science, replicability — repeating a study with a new
sample achieving consistent results — is critical for affirming the validity of
scientific findings. Despite its importance, replication efforts are few and far between
in psychological science, with many attempts failing to corroborate past findings. This
scarcity, compounded by the difficulty in accessing replication data, jeopardizes the
efficient allocation of research resources and impedes scientific advancement. Addressing
this gap, we present the Replication Database, a novel platform hosting 1,239 original
findings paired with replication findings. The infrastructure allows researchers to
submit, access, and engage with replication findings. The database makes replications
visible, easily findable via a graphical user interface, and tracks replication rates
across various factors, such as publication year or journal.

### Background

Scientific replication — retesting a hypothesis with new data to determine whether the
original study's conclusions hold — is essential for a robust body of knowledge. From a
theory-driven view, findings that don't replicate force a theory to be discarded or
modified; from a phenomenon-driven view, replication failures reveal confounders; from an
efficiency view, knowing what replicates steers resources. In psychology, replication
attempts have historically been rare, and large-scale projects have found relatively low
rates (<60%), motivating talk of a "replication crisis" and a "credibility revolution".
Yet no comprehensive database tracking which studies have been replicated, and the
outcomes, existed — so we built the **Replication Database**, aggregating, transforming,
and expanding datasets from large-scale replication attempts (e.g. the Open Science
Collaboration), public replication lists (e.g. LeBel et al.; CurateScience), and
individual replications. A public, crowdsourced database reduces wasted resources (results
deemed "unsuccessful" often land in the file drawer), circumvents journal gatekeeping, and
lets researchers monitor meta-scientific factors affecting replicability. This report is a
snapshot of 1,239 replication findings; the database is a **living resource**, searchable
via an interactive Shiny app (the FReD Explorer). Inclusion is not limited to psychology,
but most current entries are from psychology journals.

### Methods

**Inclusion criteria** (chosen liberally a priori): following Hüffmeier et al., any study
testing the same hypothesis as a previous one can be a replication — we require studies to
specify which original study they replicate. Studies from all social, cognitive, and
behavioural sciences plus medicine can be entered. Because "replication" varies in
closeness (reusing the same materials vs merely testing the same hypothesis in another
language/sample), optional variables (mostly from the Replication Recipe) capture the
similarity of instructions, measures, stimuli, etc. as "exact"/"close"/"different"/"does
not apply"/"unknown", plus an open-ended field; contributors make informed assessments and
inter-rater agreement is advised.

**Database structure** is multilevel: each row is one phenomenon/effect, coding the
original finding's reference, the replication's reference, study numbers, standardized
effect sizes, and sample sizes, plus optional metadata. The structure accommodates complex
scenarios — multiple independent replications of one study (e.g. registered replication
reports, one row per lab via a shared `id_sample`), one study replicating many originals
(e.g. Soto's 78 correlations from one sample = 78 rows sharing an `id_sample`), and a
replication mixing two originals (entered twice). Replications that don't specify their
target original cannot be entered.

**Effect-size conversion.** Original effect sizes are kept; where possible they are
converted to Pearson correlation coefficients (via R packages `esc`, `metafor`,
`psychometric`) for commensurability. Cohen's *d*, odds ratios, η², R², and Cohen's *f* are
converted; φ is used directly; standardized regression coefficients, Cramér's V, Bayes
factors, hazard ratios, Cohen's *q*, risk ratios, Spearman's ρ, and Kendall's τ are *not*
converted (so cannot enter effect-size meta-analyses) but are still displayed when
searching.

**Submission & access.** Entries can be submitted via a portal (conversion code on OSF);
the data and the FReD Explorer Shiny app are freely usable to search, filter (e.g. by
statistical power, validation status), and visualize replication rates on a "Replicability
Tracker" tab. A short best-practices guide accompanies it (overall replication rate; what
characterizes successful replications; what attributes predict replicable originals; how
rates vary over time and field) — usable as teaching material.

### Reuse potential

We encourage others to use the database for research or teaching, add their own replication
findings, or merge it with other databases — to increase the findability of replications, to
develop standards for when an effect is "replicable", and to study meta-scientific factors
affecting replicability. It is intended as a continually updated, community-driven resource.
