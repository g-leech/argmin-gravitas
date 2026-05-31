---
title: "Soft Contamination and Local Generalization"
authors:
  - Ari Spiesberger
  - Juan J. Vazquez
  - Nicky Pochinkov
  - Tomáš Gavenčiak
  - Peli Grietzer
  - Gavin Leech
  - Nandi Schoots
gleech_role: co-senior author
year: 2026
type: preprint
status: in review at NeurIPS
arxiv: 2602.12413
doi: 10.48550/arXiv.2602.12413
url: https://www.arxiv.org/pdf/2602.12413
pdf: https://www.arxiv.org/pdf/2602.12413
code: https://github.com/AriSpiesberger/Soft-Contamination-Prevelance
links:
  explainer: https://x.com/g_leech_/status/2023384075537432662
contribution_hours: 100
topics: [benchmarks, data-contamination, evaluation, llm, generalization, semantic-duplicates]
full_text: true
full_text_source: "LaTeX source (main body, verbatim; figures and the related-work comparison table omitted; appendices pointed to, not reproduced)"
---

## Abstract

If LLM training data is polluted with benchmark test data, then benchmark
performance is only a biased estimate of out-of-distribution (OOD)
generalization. Typical "decontamination" filters use $n$-gram matching which
fail to detect "semantic" duplicates: sentences with equivalent (or
near-equivalent) content that are not close in string space. We study this "soft"
contamination of training data by semantic duplicates. We embed the Olmo 3
training corpus and find that:

1. contamination remains widespread: we observe semantic duplicates for 76% of
   the CodeForces test set and 100% of MBPP's;
2. training on semantic duplicates of benchmark data improves benchmark
   performance by up to 22pp in controlled experiments and 18–22pp in ecological
   finetuning, depending on training regime and type; and
3. finetuning on duplicates of benchmark datapoints improves performance by up to
   22pp (controlled) and 13–19pp (ecological) on truly-held-out datapoints from
   the same benchmark.

The generalization we find is "shallow": it is limited to (2) and (3), and does
not typically extend to related benchmarks. We replicate on Olmo 3, Qwen3, and
Qwen3.5. We thus argue that recent benchmark gains are confounded: the prevalence
of soft contamination means gains reflect both genuine capability improvements
and the accumulation of *effective* test data in growing training corpora.

## Full text

> Converted from the LaTeX source. The prose is reproduced verbatim, with LaTeX
> markup removed and citation keys rendered as author–year where named in prose
> (otherwise dropped). Figures and the related-work comparison table are omitted;
> cross-references to them are dropped. The appendices are pointed to at the end,
> not reproduced. Canonical source: the PDF linked above.

### 1. Introduction

LLM scores on hard reasoning benchmarks have grown rapidly, with many benchmarks
nearing saturation even for smaller models. But LLM training corpora have also
grown by a factor of at least 10,000x since 2020. Thus, there is reason to expect
a proportional increase in benchmark test examples being included in the training
corpora of recent LLMs. While AI labs make good-faith efforts to remove syntactic
duplicates of benchmark items from their corpora, 'softer' forms of contamination
(i.e. by 'semantic duplicate' examples equivalent to test data but with a high
string distance from test data) are hard to detect, and can result from
independent reinvention of similar examples, rather than from leaking existing
test data into the training corpus. This paper asks: how much does this confound
current LLM benchmark scores?[^1]

[^1]: The question is sharpened by the "approximate retrieval" view of LLMs, on
which outputs are similarity-weighted combinations of nearby training
documents—making soft contamination exactly the regime where benchmark scores
should inflate.

We address this question by combining existing contamination detection methods
with novel finetuning experiments to diagnose *shallow generalization* across
examples: narrow performance gains from training on datapoints that are
qualitatively typical of an existing benchmark. Using the open-data model Olmo 3,
we show that modern LLM training corpora include data that are *effectively*
samples from major reasoning benchmark test sets, leading to benchmark scores
that (to some extent) demonstrate shallow generalization rather than general
reasoning capability. We thus hypothesize that the rapid increase in LLMs'
performance on reasoning benchmarks partly reflects that the rapid growth of LLM
corpora has enabled increasing levels of a hidden and internally hard-to-detect
kind of contamination. If true, then recent progress on major reasoning
benchmarks is weaker evidence for AI progress (conceived as OOD generalization).

**Example semantic duplicate of test data, found by our pipeline.** Original MBPP
sample 297 (*Write a function to flatten a given nested list structure.*):

```python
def flatten_list(list1):
    result_list = []
    if not list1: return result_list
    stack = [list(list1)]
    while stack:
        c_num = stack.pop()
        next = c_num.pop()
        if c_num: stack.append(c_num)
        if isinstance(next, list):
            if next: stack.append(list(next))
        else: result_list.append(next)
    result_list.reverse()
    return result_list
```

Semantic duplicate from `dolmino`:

```python
    :param nested_list: A list that
        may contain other lists
    :return: A flattened list of
        all elements
    """
flattened_list = []
for element in nested_list:
    if isinstance(element, list):
        # Recursively flatten the sublist
        flattened_list.extend(
            flatten_nested_list(element))
    else:
        # Append non-list elements
        flattened_list.append(element)
return flattened_list
```

Let an *exact* duplicate of test data be an example in the training corpus which
is syntactically identical (up to some $n$-gram) to some item in a relevant test
set. A *semantic* duplicate of test data is an example in the training corpus
which has the same meaning (in some sense) as some item in a relevant test set.
We call contamination *soft* when it involves semantic duplicates. We call
generalization *shallow* when it is limited to 1) generalizing
within-distribution and 2) generalizing across semantic duplicates.

To test the hypothesis that soft contamination confounds benchmark results, we
gauge the prevalence of semantic duplicates, of items from major reasoning
benchmarks, in the training corpus of Olmo 3. We finetune on exact duplicates,
semantic duplicates, and (mere) close embedding neighbors to test their capacity
to induce shallow generalization.

**Our contributions:**

- **High rates of soft contamination in the wild:** We screen 1% of the
  pretraining data and all of the finetuning data of Olmo 3, around 260 GBs of
  text, for natural semantic duplicates by using their embedding distance to
  benchmark data, far more than previous studies. Despite the standard levels of
  decontamination by the Olmo team, we find more contamination than previous
  studies;
- **Shallow generalization:** We finetune Olmo 3 on duplicates of the MuSR,
  ZebraLogic and MBPP benchmarks and find that benchmark performance also improves
  on unseen benchmark data. The relative utility of semantic vs. exact duplicates
  varies by benchmark: on MuSR, both yield comparable gains of approximately 20pp
  on both seen and unseen items; on MBPP, exact duplicates produce larger gains on
  seen items (17pp vs. 9pp), but on unseen items semantic duplicates produce
  larger gains than exact duplicates (7pp vs. 2pp). Finetuning on one benchmark
  does not tend to improve performance on related benchmarks, suggesting that the
  generalization in our finetuning experiments is strictly 'shallow'.
- **Natural levels of soft contamination have a substantial effect:** To our
  knowledge, we design the first ecologically valid finetuning study of exposure
  to semantic duplicates, demonstrating that even truly held-out benchmark items
  may be significantly inflated by shallow contamination. In the case of MuSR we
  demonstrate inflation on held-out duplicates by 10–11pp ($p < 0.05$) in Olmo 3,
  Qwen3 and Qwen3.5. We conclude that the held-out evaluation may be confounded by
  this effect.
- **True hold-out has never been tried:** We view this work as a step towards
  putting LLMs into the original scientific setting of empirical risk
  minimization: imposing a strict separation between training and test sets,
  validated on open training data.

### 2. Related Work

Contamination of LLM training corpora by benchmark test items is a large field of
study. (A comparison table in the paper covers key differences between our work
and prior studies of semantic duplicates.)

The first wave of contamination studies focused on the prevalence and impact of
exact syntactic duplicates (i.e. word-for-word matches in string space), with
later work extending the analysis to partial syntactic duplicates. More recently,
researchers have begun to study indirect or 'semantic' contamination, where data
in the training corpora is equivalent to benchmark-items in substantive content
without sharing $n$-gram sequences or other syntactical properties. The literature
on semantic contamination focuses on performance on a benchmark item and
training-exposure to that same item's near-duplicates as a variant on
memorization. Our work instead studies semantic duplicates as a source of
*shallow generalization*, which includes generalization from semantic duplicates
to their equivalents in the benchmark, but also -- more importantly, and why it
still warrants the name 'generalization' -- generalization to *unduplicated*
examples of the task. Domínguez-Olmedo et al. (2024) study a related phenomenon
they call 'training on the test *task*'.

Studies of the prevalence of corpora-contamination by exact duplicates of
benchmark data typically deploy two kinds of methods: searching for benchmark-item
duplicates in open datasets, and memorization testing using 'membership inference'
style techniques. When studying semantic-duplicates contamination, by contrast,
memorization diagnostics are unlikely to capture the right contamination effects.
Search in open datasets is therefore preferred in the (small) literature, using a
heuristic semantic distance to guide search and human judgment, AI-assistant
judgment, or plagiarism-detection software to assign 'semantic duplicate' status.
Previous work by Yang et al. (2023) and Riddell et al. (2024) has provided
high-quality estimates of the prevalence of semantic duplicates of items from
major reasoning benchmarks including HumanEval, MMLU, and GSMK8k, in widely-used
training corpora such as The Pile, StarCoderData, and RedPajama.

We use a method convergent with that of Yang et al. (2023) to estimate the
prevalence of semantic duplicates in Dolma, Olmo's custom training corpus. While
our search covers a much larger dataset and finds many more semantic duplicates
per benchmark item, our results are consistent with their findings of the rate of
semantic duplicates in corpora.

Xu et al. (2025) study fake-news detection with a domain-specific concept of
'entity-exposure'. Two central methods in the literature are 1) finetuning on
contaminated data to simulate training-exposure (assuming or verifying that the
model had no or limited prior exposure), as applied by Yang et al. (2023) to
semantic duplicates, and 2) using duplicate-prevalence data to test for
correlations between a benchmark-item's rate of duplication in a model's training
corpus and the model's performance on the item, which Riddell et al. (2024) apply
to semantic duplicates. Our work uses a finetuning approach, but tests not only
gains on a benchmark item from finetuning on its own semantic duplicates, but also
gains on benchmark items from finetuning on semantic and exact duplicates of
*other* items in the benchmark. Inspired by Koçyiğit et al.'s study of the
memorization-effects caused by injecting realistic dosages of exact duplicates
into a clean finetuning corpus, we also design (to our knowledge) the first
ecologically valid finetuning study of the effect of exposure to semantic
duplicates.

### 3. Methodology

Our full pipeline is described in the paper's pipeline figure. To know what the
model has seen (and so conduct a controlled analysis of OOD), our experiments use
`Olmo-3-7b`, an open-*data* model. (Some closed models also allow finetuning, but
with their closed corpora we cannot rule out prior training, which would confound
our effect estimates.) The appendix describes the four benchmarks we used to
measure the effect of finetuning: MBPP, CodeForces, MuSR, and ZebraLogic. All code
available on GitHub.

#### 3.1. Finding semantic duplicates in the wild

**Olmo 3 corpus.** We embed 1% of the Olmo 3 Base training data (Dolma3 and
Dolmino) and all of the Olmo 3 Instruct finetuning data (Dolci SFT, Dolci Instruct
DPO and Dolci Instruct RL). To sample from the base training data, we employ a
stratified reservoir sampling strategy that preserves the corpus's hierarchical
structure. See the appendix for more dataset details.

**Embeddings.** We used `llama-embed-nemotron-8b` to embed the above datasets. At
time of writing this model is #2 on the Massive Text Embedding Benchmark (MTEB)
leaderboard. All embeddings were done in FP16 precision. Embeddings were performed
on around 260GB of raw text and saved to an AWS storage system. Embeddings were
performed over the course of approximately 2 days using a cluster of 8 H100 gpus.

**Cosine similarity.** To find candidate duplicates, we embed both Olmo 3 corpus
data and benchmark data and calculate the cosine similarity between benchmark data
points (MBPP and CodeForces) and our 1% sample of the whole corpus. The appendix
covers this process in detail.

**Semantic and exact duplicates among high cosine similarity matches.** For MuSR,
the highest cosine similarity matches are around $0.4$, and upon manual inspection
it appears there are no duplicates, or MuSR-like problems in the training corpora.
We conclude that the Olmo 3 datasets have no semantic duplicates of MuSR. For
ZebraLogic we found many exact duplicates and a few semantic duplicates, which we
take as a demonstration of the sensitivity of our cosine similarity pipeline.
Besides these semantic duplicates, most other high cosine similarity matches were
easy 'Zebra puzzles' not comparable in complexity to the ZebraLogic examples.
After manual inspection of the top matches for MBPP and CodeForces we decide to
sample and annotate using an LLM.

*Exact duplicates: sampling and annotating ZebraLogic.* We observed several
ZebraLogic tasks in the training corpora verbatim. We obtain the rate of exact
duplicates as in the appendix.

*Semantic duplicates: sampling and annotating MBPP and CodeForces.* We investigate
matches between MBPP and CodeForces across the five training datasets. For each
dataset and benchmark, we select candidates in the top 0.1% of cosine similarity.
From this subset, we either take the top 100 or randomly sample 100 points, and
prompt `gemini-3-flash-preview` to classify them. The model outputs a boolean
label for semantic duplication, a categorical relationship type (exact,
equivalent, subset, superset, or unrelated) which serves as a guiding heuristic,
the reasoning for its choice, and a confidence score.

To validate this automated pipeline, we manually annotated a stratified sample of
190 items across CodeForces and MBPP (available in our GitHub repository). Our
human annotations demonstrated general consistency and correctness with the LLM
judge, though we noted occasional LLM misclassifications regarding whether
CodeForces superset problems constituted true semantic duplicates, and can miss
related problems as being not semantic duplicates when they are. To be
conservative against false positives, we ran a secondary verification pass using
`gemini-3-pro` on the potential duplicates initially flagged by the preview model,
ensuring there was sufficient algorithmic uplift from the corpus text to the test
problem. Examples of semantic duplicates found by our pipeline are provided in the
appendix, and further details on the annotation process can be found in the
appendix.

*Generating synthetic semantic duplicates.* The appendix gives a detailed
description of our generation of synthetic semantic duplicates for each benchmark.

#### 3.2. Finetuning on Duplicates

Our experiments distinguish between different kinds of 'shallow generalization'
gains: 1) gains on one benchmark item from training on semantic duplicates; 2)
gains on one benchmark item from training on exact duplicates of *other* items in
the benchmark; and 3) gains on a benchmark item from training on semantic
duplicates of other items in the benchmark.

We finetune Olmo 3 Instruct on duplicates of the following benchmark datasets:
MuSR, ZebraLogic, and MBPP. We use either exact duplicates or semantic duplicates
generated as in Section 3.1. To finetune Olmo 3 Instruct (a non-reasoning model)
on these datapoints we first get a teacher model to generate Chain-of-Thought
(CoT) reasoning traces. We use Opus 4.5 as teacher model, and for MuSR we also
experiment with using GPT 4.1-mini.

We take the formatted questions and corresponding CoT answers and use LoRA to
finetune Olmo 3 Instruct. The appendix explains our choice of finetuning approach.
To get propensities, we use a temperature of 0.7 and do 8 parallel generations
(for each of the unfinetuned model, the CoT generations of the teacher model, and
for the finetuned model).

We split a finetuning dataset of duplicates in two and only finetune on half of
it, while evaluating the finetuned model on both seen and unseen duplicates. To
assess whether performance goes up on related benchmarks we use TrueDetective as a
mirror for MuSR, Arc Challenge as a mirror for ZebraLogic, and HumanEval as a
mirror for MBPP. We also evaluate performance on Arc Easy, BoolQ, HellaSwag, PIQA
and Winogrande.

### 4. Results

#### 4.1. Exact duplicates in training corpora

Surprisingly, we found that the Olmo Instruct RL dataset contains exact duplicates
of 70% of one of their reported benchmarks' (ZebraLogic) test data. The appendix
describes this issue.

#### 4.2. Natural semantic duplicates in training corpora

**Relationship of cosine similarity to semantic duplicate status.** We plot cosine
similarity against semantic duplicate status. For both MBPP and CodeForces we find
that even within the top 0.1% highest cosine similarity matches, semantic
duplicates are far more common among the highest cosine similarity matches.

**Benchmark examples with at least one semantic duplicate.** 100% of MBPP problems
have at least one semantic duplicate in the top-100 training data points by cosine
similarity. We also find $\geq 1$ semantic duplicate per benchmark datapoint when
we randomly sample 100 matches out of the top 0.1% cosine similarity matches. For
CodeForces we find that 75.9% of problems have at least one semantic duplicate in
the top 100 cosine similarity matches. Based on these findings we conclude
contamination by semantic duplicates is widespread. (The above numbers are a lower
bound: if we checked all the data and not just the top 100, we would very likely
find more semantic duplicates for CodeForces.) We also consider the likelihood of
a test point having a single semantic duplicate based on its elo, and find that
easier problems (lower elo score) are statistically more likely to have semantic
duplicates. See the appendix for more details.

**Semantic duplicate occurrence stratified by training dataset.** We investigate
the relationship between where in the training scheme a training dataset is used,
and the extent of semantic duplicate contamination. All training datasets have
semantic duplicates for MBPP. For CodeForces we observe that again, semantic
duplicates of problems exist in each dataset, particularly in Dolma, Dolci SFT and
Dolci DPO.

**Semantic duplicates are sparse and investigating more data leads to more
matches.** Previous work has found that $n$-grams typically miss many semantic
duplicates; we instead work with cosine similarity. See the appendix for details.
In the appendix we also show the proportion of benchmark problems that have at
least one semantic duplicate in the training data scaling with the number of
datapoints we sample. For CodeForces, this statistic drops to 28.4% when sampling
only the top-1 cosine similarity match. We suggest that the reason we found more
semantic duplicates than previous work is that we investigated far more data.

#### 4.3. Finetuning on Semantic Duplicates

**MuSR.** We experimented with three levels of synthetic semantic duplication,
with increasing dissimilarity, generated as detailed in the appendix. The table
below shows that when we finetune on duplicates of half of the benchmark
$(n = 125)$, performance goes up equally on the unseen half of the benchmark.
Finetuning on exact duplicates of MuSR benchmark data leads to similar gains as
finetuning on a variety of semantic duplicates: in both cases performance goes up
by c. 20%. When instead of finetuning on duplicates (exact or semantic), we
finetune on datapoints that have been selected for high cosine similarity to
benchmark datapoints, the performance hardly goes up from baseline. We also check
the performance change on a different benchmark from the same domain,
TrueDetective, and find that performance remains stable. The appendix also shows
that using a better teacher model to generate CoT reasoning traces results in
further gains for finetuning.

**ZebraLogic.** The table below shows that, when finetuning on exact duplicates of
half of the benchmark data $(n = 500)$, performance also goes up on the other
half. Finetuning on exact duplicates leads to a much larger jump in performance
(for both seen and unseen benchmark items) than semantic duplicates, in contrast
to our finding for MuSR. In fact, we find that finetuning on semantic duplicates
hardly increases performance, and in one case (combining shuffling, substituting
and paraphrasing), that the performance on ZebraLogic substantially degrades, even
though performance on the control same-domain benchmark (Arc Challenge) remains
stable. We tentatively attribute this to compounding distribution shift in both
the puzzles and their CoT traces: ZebraLogic's substitution operates on domains
(e.g., color→shape) in a way MuSR's tree-based regeneration does not, so the
teacher's reasoning traces shift accordingly, and stacking all three
transformations likely pushes training too far from the test distribution to
support transfer.

**MBPP.** The table below shows that, when finetuning on exact duplicates of half
of the benchmark data (n=210), we see a substantial jump on that half of the data,
while performance on the unseen half hardly changes. Finetuning on semantic
duplicates has a moderate effect, but it affects both seen and unseen performance.
Again, finetuning on nonduplicate but cosine-similar data does not improve
performance substantially. We also evaluate the finetune on a related benchmark,
HumanEval, and find a surprising jump after tuning on semantic duplicates. We
hypothesize that this jump happened because our semantic duplicate dataset is
fairly rich, and so demands some generalization.

**Key results from our experiments finetuning Olmo 3 on semantic duplicates.** We
see large gains from training on semantic duplicates, in MuSR's case statistically
equivalent to training on exact duplicates. The 'Control' column denotes a second
similar-domain benchmark: for MuSR this is 'True Detective'; for ZebraLogic this
is the ARC Challenge and for MBPP this is HumanEval. We see no significant
improvement to the control scores when finetuning on any setting of MuSR or
ZebraLogic duplicates; we do see a surprising leap in HumanEval (MBPP's control)
for semantic duplicates. Accuracy (%) ± stderr. Abbreviations: P=Paraphrase,
SH=Shuffle, SU=Substitution, cos similarity=high cosine similarity corpus matches.
Experiments were performed on 8 A100 GPUs, each experiment taking around 2-3 hours.

| Target | Degree of similarity | Seen accuracy | Unseen accuracy | Δ unseen | Accuracy on control benchmark |
|---|---|---|---|---|---|
| **MuSR** | No finetuning | — | 66.0 ± 3.00 | — | 28.3 ± 3.26 |
| | Exact dupes | 87.9 ± 2.06 | 87.3 ± 2.11 | −0.6 | 27.7 ± 3.24 |
| | Sem. dupes (regen) | 85.8 ± 2.21 | 86.2 ± 2.18 | +0.4 | 29.3 ± 3.29 |
| | Sem. dupes (1-branch) | 85.7 ± 2.21 | 86.0 ± 2.20 | +0.3 | 28.3 ± 3.26 |
| | Sem. dupes (n-branches) | 87.5 ± 2.09 | 87.9 ± 2.06 | +0.4 | 29.8 ± 3.31 |
| | cos similarity (SFT) | — | 68.6 ± 2.08 | — | 25.1 ± 3.14 |
| | cos similarity (DPO) | — | 67.9 ± 2.09 | — | 26.7 ± 3.20 |
| | cos similarity (RL) | — | 65.3 ± 2.13 | — | 26.7 ± 3.20 |
| **ZebraLogic** | No finetuning | — | 36.9 ± 1.53 | — | 50.1 ± 1.46 |
| | Exact dupes | 48.4 ± 1.58 | 43.4 ± 1.57 | −5.0 | 49.5 ± 1.46 |
| | Sem. dupes (P) | 39.2 ± 1.54 | 36.2 ± 1.52 | −3.0 | 49.3 ± 1.46 |
| | Sem. dupes (SH,SU) | 36.0 ± 1.52 | 36.8 ± 1.53 | +0.8 | 50.7 ± 1.46 |
| | Sem. dupes (SH,P) | 38.0 ± 1.53 | 36.0 ± 1.52 | −2.0 | 49.4 ± 1.46 |
| | Sem. dupes (SH,SU,P) | 28.0 ± 1.42 | 28.4 ± 1.43 | +0.4 | 50.4 ± 1.46 |
| | cos similarity (SFT) | — | 22.9 ± 1.33 | — | — |
| **MBPP** | No finetuning | — | 46.4 ± 3.44 | — | 55.3 ± 3.88 |
| | Exact dupes | 63.0 ± 3.32 | 48.8 ± 3.45 | −15.2 | 49.2 ± 3.90 |
| | Sem. dupes (Python) | 55.1 ± 3.42 | 53.6 ± 3.44 | −1.5 | 67.0 ± 3.67 |
| | cos similarity (SFT) | — | 48.8 ± 3.45 | — | 53.1 ± 3.90 |

**We observe a pattern of shallow generalization.** We repeatedly find that
performance also improves on benchmark data that was unseen during finetuning:
i.e. the delta between seen and unseen performance is small (apart from two cases
of finetuning on exact duplicates). This suggests within-benchmark-distribution
generalization. We tested benchmark improvement on different, but same domain
benchmarks, and typically did not find substantial improvement, confirming shallow
generalization. We also find that improvement or finetuning on high cosine similar
datapoints does not by itself improve benchmark performance.

#### 4.4. Ecologically Valid Finetuning

**Realistic Contamination Dosage.** We simulate SFT-stage exposure using our
observed contamination rates. For MBPP, 40% of the top 0.1% cosine-similarity
matches were true semantic duplicates. This establishes that roughly 4 in 10,000
training datapoints act as a semantic duplicate for any given test item—a
conservative lower bound given our sampling density and false negative rate.

**Finetuning contaminated and clean models.** We finetune Olmo-3-7b-base,
Qwen3-8b-base and Qwen3.5-9b-base, on Olmo-SFT data interspersed with MuSR semantic
duplicates, for which our annotation experiments verify that no duplicates exist in
the model's training data. We then split the MuSR data into two halves of 125
datapoints each, and generate 4 semantic duplicates for them, two duplicates of
level 2 and two of level 3, so 500 duplicates in total. We perform three finetuning
runs with 1) a clean dataset of 10,000 SFT datapoints verified to be
decontaminated, 2) a contaminated version of the same dataset where 5% of clean
samples are swapped with 500 semantic duplicates corresponding to the 'seen'
subset, and 3) an exact contamination version of the same dataset where 5% of
clean samples are swapped with 500 exact duplicates corresponding to the seen
subset. We evaluate each finetuned model at each epoch by prompting each model 20
times per question on all three finetuned models. See the appendix for details. We
plot the accuracy curves during finetuning, and a table of the mean values and
p-values.

**Results on Olmo-3-7b.** Despite a very low contamination rate, the contaminated
model scores 16.9pp higher than the clean model on the seen subset ($p = 0.001$)
and 11.4pp higher on the unseen subset ($p = 0.037$). Notably, on the unseen subset
semantic contamination produces larger gains than exact contamination: training on
exact duplicates yields essentially no held-out gain (51.2 vs. 51.0 clean
baseline), suggesting that paraphrased duplicates leak to held-out items more
effectively than verbatim ones. To verify that these performance gains reflect
'shallow' generalization rather than robust capability gains, we evaluate on
TrueDetective, a benchmark in the same domain as MuSR. We find that performance on
TrueDetective remains stable across finetuning conditions.

**Results on Qwen3-8b and Qwen3.5-9B.** We replicate the identical experiment on
Qwen3-8B-base and find that contamination significantly inflates benchmark
performance, comparable in magnitude to Olmo 3. The contaminated model scores
13.2pp higher than the clean model on the seen subset ($p$ < 0.001) and 9.9pp
higher on the unseen subset ($p <$ 0.001). As with Olmo 3, semantic contamination
is at least as effective as exact contamination on held-out items (68.9 vs. 65.6
for exact). Unlike with Olmo 3, however, the clean model itself shows substantial
benchmark gains from finetuning relative to baseline (+5.0pp seen, +7.8pp unseen),
concentrated on the random benchmark-subset we use as the 'unseen' subset for the
contaminated model. We observe modest improvement on the control benchmark
TrueDetective for the finetuned contaminated model. We further replicate the
experiment on Qwen3.5-9B-base, finding the largest contamination effect of the
three models. The contaminated model scores 15.3pp higher than the clean model on
the seen subset ($p < 0.001$) and 11.4pp higher on the unseen subset
($p = 0.009$). As with Olmo 3 and Qwen3, semantic contamination is statistically
indistinguishable from exact contamination on held-out items (74.7 vs. 75.0 exact,
$p = 0.754$). As with Qwen3, the clean model also shows gains relative to baseline
(+6.8pp seen, +8.2pp unseen), concentrated on the random unseen subset.

**Natural contamination leads to benchmark gains.** Our results show that the
presence of semantic duplicates in training corpora, even at low rates, can lead
to substantial gains in evaluation results. When finetuning on semantic duplicates
instead of finetuning on clean data, we find a statistically significant increase
on truly-held-out benchmark test data performance, that is, we observe
within-benchmark generalization when finetuning on semantic duplicates.

### 5. Conclusion

We found widespread soft contamination of Olmo 3 training data with semantic
duplicates, despite strong efforts from the Olmo team to filter out duplicates
(Section 4.2); we also found that finetuning models with an ecologically valid
amount of semantic benchmark duplicates leads to a statistically significant
increase in benchmark performance (Section 4.4). We conclude that benchmark
performance is indeed confounded by soft contamination. *Shallow generalization*
is observed when the controlled finetuning on test data semantic duplicates
improves performance by up to 22 percentage points while same-domain benchmark
performance remains unchanged (Section 4.3).

#### 5.1. Limitations

We likely underestimate the prevalence of semantic duplicates in frontier training
corpora, because for cost reasons our detection methods did not embed the full
pretraining corpus. Another downward bias is the relative absence of rephrasings
in Dolma: synthetic data pipelines now often involve intentionally creating
semantic duplicates at scale, and so our estimates of their prevalence are likely
lower than the true rate in closed corpora using such methods. Similarly, we do
not cover SOTA synthetic data provided via RL environments. Our experiment design
is limited to models with an open-source training corpus - a small and potentially
unrepresentative set of systems. While our comparative claims remain robust,
absolute performance effects might differ under full finetuning. Furthermore,
contamination dynamics during our SFT-stage simulation may differ from massive
pretraining. The training corpora used in frontier models are much larger than
Dolma -- see e.g. the 30T tokens used in some Llama 4 runs. These larger corpora
will have more semantic duplicates, but also a different rate of natural semantic
duplicates than Dolma.

A fundamental objection to our project is that out-of-distribution (OOD)
generalization is no longer the (only) goal of AI development: an alternative is to
instead bring all common tasks in-distribution (Chollet 2024, Patel 2025, Leech
2024 §5.3.2). Perhaps the real-world utility of LLMs shows that our concerns about
OOD generalization are not practically important, even if generalization is largely
shallow. This is valid, but 1) then the deviation from the assumptions of empirical
risk minimization should be explicitly noted, and secondly 2) (even perfect) hidden
interpolation may not be practically equivalent to OOD generalization, and so being
explicit about which is being instantiated is crucial.

### Impact Statement

We aim to advance understanding of what LLM benchmark scores say about general
capabilities on new inputs. Our findings have implications for how the AI research
community, policymakers, and the public interpret reported progress on reasoning
benchmarks.

More-accurately measuring AI capabilities helps to calibrate decisions about AI
deployment, regulation, and research. If benchmark gains partly reflect
interpolation from a growing corpus rather than more general capability
improvements, then recognizing this could help prevent overconfidence in model
generalization to novel tasks.

We do not believe this work poses significant risks. While our methods could
inform more sophisticated benchmark gaming, the contamination we study is likely
accidental rather than adversarial, and our detection methods are likely more
useful for auditing than for evasion.

Our finding of contamination in Olmo 3 is only possible because of the unusual
level of transparency of its model development process. It would be unfair for
readers to thereby assume that the degree of contamination in Olmo is unusual, and
worse, a perverse incentive against transparency.

Our work could easily be misread as 'debunking' LLM capabilities and so spur
complacency about near-term AI impacts. Our results suggest that benchmark gains
are confounded (and so partially shallow), not that they are illusory.

### Appendices

The paper continues with appendices (detailed methodology, synthetic data
generation, annotation schemes, and further results, including degradation
analysis and the ecological-finetuning tables). These are not reproduced here; see
the PDF.
