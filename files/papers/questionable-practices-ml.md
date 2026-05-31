---
title: "How to Lie in Machine Learning"
full_title: "Questionable Practices in Machine Learning"
authors:
  - Gavin Leech
  - Juan Vazquez
  - Misha Yagudin
  - Nic Kupper
  - Laurence Aitchison
gleech_role: first author
year: 2024
type: preprint
status: in review at JMLR
arxiv: 2407.12220
doi: 10.48550/arXiv.2407.12220
url: https://arxiv.org/abs/2407.12220
links:
  explainer: https://x.com/g_leech_/status/1813753309607714992
contribution_hours: 250
topics: [machine-learning, evaluation, metascience, questionable-research-practices, llm, benchmarks]
---

## Abstract

Evaluating modern ML models is hard. The strong incentive to report a
state-of-the-art result on some metric often leads to questionable research
practices (QRPs): bad practices which fall short of outright research fraud. We
describe 44 such practices which can undermine reported results, giving examples
where possible. Our list emphasises the evaluation of large language models
(LLMs) on public benchmarks. We also discuss "irreproducible research
practices" — decisions that make it difficult or impossible for other
researchers to reproduce, build on, or audit previous research.

## Full text

> Converted from the LaTeX source. Prose is reproduced faithfully with LaTeX
> markup removed and citation keys rendered as author–year where named in prose,
> otherwise dropped. Figures omitted; the two summary tables are kept. Canonical
> source: the arXiv PDF.

> Epigraph: *"If, like truth, falsehood had but one face, we'd be better off: we
> could take as true the opposite of what a liar says. But the opposite of the
> truth has a hundred thousand faces and a limitless field."* — Montaigne

### 1. Introduction

To understand the actual capabilities of models like LLMs and develop reliable
systems based on them, it is critical to have trustworthy evaluations comparing
different models and approaches on meaningful benchmarks. However, researchers and
companies have strong incentives to engage in 'questionable research practices'
(QRPs) to inflate their reported results: inflated results help researchers
publish in high-impact venues and help companies attract investment and users.
Not only is there motive: the complexity of the pre-training, post-training and
evaluation procedure also gives ample opportunity. These opportunities fall into
three families. First, **contamination**, in which test-set information is used in
the pre-training, post-training, runtime, or prompt. Second, **cherrypicking**, in
which researchers 'hack' experimental settings (selecting those under which their
model works better, after testing multiple times), or "nerf" (degrade) baselines.
Third are various forms of **misreporting**, such as making broad claims (e.g.
about "reasoning") based on narrow benchmarks. We additionally consider
**irreproducible research practices** (IRPs), which make it more difficult for
others to reproduce, build on, or audit previous research — the most prevalent
example being dataset hiding.

We do not claim that most performance is spurious. Nor do we show the general
prevalence of these problems. This paper answers the limited question: *what could
make a model's reported performance to some extent spurious?*

### 2. Related work

The two lenses we use — QRPs and researcher degrees of freedom (RDOFs) — originate
from psychological science, but similar issues have been studied in ML under other
names (Lipton & Steinhardt on inflated language and superfluous design; Sculley et
al. on the "winner's curse"; the NeurIPS 2019 Reproducibility programme; Liao et
al. on benchmark/real-world mismatch; Biderman et al. on LLM-evaluation problems).
Our work is similar in spirit to Huff's *How to Lie with Statistics*.

**Researcher degrees of freedom.** Scientific analyses have many RDOFs — free
choices in design and analysis that a researcher can manipulate to give themselves
more chances of a (real or spurious) 'significant' result, without increasing the
process's ability to detect genuine effects. This is unavoidable: no science has a
one-to-one mapping between theories, experiments, and analyses. Each degree of
freedom is an opportunity to introduce a QRP, intentionally or not. An ML
evaluation usually has a main method (the researchers' own) and baselines; to
publish usually requires showing the method is statistically significantly better,
giving an incentive to exploit RDOFs. It is essentially never acceptable to
'optimise' any aspect of the evaluation procedure. (By contrast, it is necessary
that researchers optimise the hyperparameters of baselines and of their own method
a "similar amount" — though operationalising this is fraught.)

### Catalog — questionable or fraudulent practices in ML (Table 1)

Stage abbreviations: where in the development path each practice acts (Design →
Collection → Training → Evaluation → Reporting).

| Practice | Description | Stage | Accidental? |
|---|---|---|---|
| **Contamination** | | | |
| Training Contamination | Training on the test set (e.g. in the web corpus) | Training | Plausibly |
| Prompt Contamination | Putting test data into the prompt (few-shot) | Evaluation | Plausibly |
| RAG Contamination | Leaking benchmark data via Retrieval-Augmented Generation | Evaluation | Plausibly |
| Dirty Paraphrases | Rephrasing test data and training on it | Collection | No |
| Contamination Laundering | Contaminated models generating training data | Collection | Plausibly |
| Thieved Test | Obtaining private test labels | Collection | No |
| User Contamination | Post-training on test in user prompts | Training | Plausibly |
| Over-hyping | Tuning hyperparameters further after test | Training | Plausibly |
| Meta-contamination | Reusing contaminated hyperparameters/designs | Training | Plausibly |
| Semantic Duplicates | Train and test set include near-identical points | Collection | Plausibly |
| **Cherrypicking** | | | |
| Baseline Nerfing | Optimising training parameters of baselines less | Evaluation | Plausibly |
| Baseline Hacking | Choosing weak baselines to compare to | Evaluation | No |
| Runtime Nerfing | Optimising baselines' inference parameters less | Evaluation | Plausibly |
| Runtime Hacking | Post-hoc best inference parameters or decoding | Evaluation | No |
| Benchmark Hacking | Choosing easier benchmarks | Evaluation | Plausibly |
| Subset Hacking | Subsetting the benchmark until you win | Evaluation | No |
| Harness Hacking | Choosing post-hoc best evaluation harness | Evaluation | No |
| Golden Seed | Training/tuning with many different seeds | Training | No |
| Prompt Nerfing | Undertuning prompts of baseline models | Evaluation | Plausibly |
| Prompt Hacking | Choosing the best prompt strategy post-hoc | Evaluation | Plausibly |
| **Misreporting** | | | |
| Superfluous Cog | Redundant module added to claim novelty | Design | Plausibly |
| Whack-a-mole | Monitoring for specific failures and fine-tuning them away ad hoc | Training | No |
| Benchmark Decoration | Pretraining on benchmark / instruction data | Training | Plausibly |
| $p$-hacking | Flawed sampling when bolding SOTA results | Reporting | Plausibly |
| Point Scores | Reporting single-run results (no error bars) | Reporting | No |
| Outright Lies | Fabricating results (included for completeness) | Reporting | No |
| Over/Underclaiming | Misleading claims about model capabilities | Reporting | No |
| Reification | General claims from narrow ML benchmarks | Reporting | No |
| Nonzero-shot | Claiming 'zero-shot' while training on examples | Reporting | Plausibly |
| Misarithmetic Mean | Using arithmetic mean on normalised results | Reporting | Plausibly |
| Parameter Smuggling | Under-reporting model size; or substituting in more embedding parameters | Reporting | No |
| File Drawer | Failing to report negative benchmark studies | Reporting | No |
| **Amplifiers** | | | |
| Inductive Smuggling | Handcrafting inductive bias for a task | Design | No |
| Label Noise | Using benchmarks known to be error-ridden | Collection | Plausibly |

### 3. The fundamental tricks

Most ways to mislead others, or delude yourself, in ML evaluation fall into one of
three categories, plus a residual "amplifier" category:

1. **Contamination** — any way the test set influences training: training on data
   semantically identical to test examples, implicitly tailoring model design to
   the test set, reusing hyperparameters from models tested on it, or straight-up
   training on the test set.
2. **Cherrypicking** — choosing among runs and configurations to make your system
   look good or relatively good: picking weak competitors, undertuning a strong
   competitor, or optimising your inference parameters or prompt more.
3. **Misreporting** — any error or misleading presentation of the model's
   specification or evaluation results: reporting only point estimates,
   under-reporting model size, attributing success to inert modules, or claiming
   general task performance from tests without clear external validity.

Plus **Amplifiers**, which have indirect effects by enabling other QRPs. Two key
terms: *nerfing* is intervening to weaken baselines (e.g. tuning their
hyperparameters less); *hacking* is selecting shared experimental settings post-hoc
— after seeing results — then reporting only the favourable ones (and often failing
to correct for multiple comparisons).

### 4. Contamination

Contamination (AKA *leakage*) is any influence of test-set information on model
development, from subtle (reusing hyperparameters) to blatant (training on the test
set). It can totally invalidate a reported benchmark score. Poorly-filtered
web-scale corpora have led to many cases of plausibly accidental contamination;
new versions of existing benchmarks generally show substantial performance drops.

- **Training contamination** — using test-set information at train time. With
  corpora at terabyte scale, full manual inspection is impossible. Contamination
  has been explicitly reported in GPT-4, GLaM, Llama 2 and Gemini. Measuring its
  effect via (a) careful vs careless filtering (Gemini 1.0 Ultra rose from 74.4% to
  89.0% on HumanEval after a single test exposure; removing HumanEval contamination
  dropped Pythia-12B 57.1% and StarCoderBase 25.9%), (b) new test sets mimicking the
  original (GSM1K), and (c) semantics-preserving edits that drop performance.
- **Prompt contamination** — drawing few-shot examples from benchmark data, or
  directly including the answer in the prompt.
- **RAG contamination** — leaking benchmark data via a contaminated
  retrieval-augmented-generation reference database (an agent could use a lookup
  table for perfect accuracy).
- **Dirty paraphrases** — altering test data into a semantically equivalent form
  before training on it, evading string-checking/extraction; can be done at scale
  by LLMs (Yang et al. 2023).
- **Contamination laundering** — a student model trained (via distillation) on a
  teacher that was itself trained on test data; the contamination enters
  "offscreen" without the downstream researcher adding any.
- **Thieved test** — obtaining a private test set: collusion with benchmark makers,
  scraping labels, reverse-engineering via many submissions, or relabelling public
  test inputs by hand.
- **User contamination** — post-training on test data that arrives via user prompts
  (commercial developers are known to train on user data), causing silent behaviour
  change and spurious score boosts.
- **Over-hyping** — tuning hyperparameters on the test set: training, evaluating on
  test, then doing further search or switching metric, iterating many times. Biases
  results even if each iteration uses cross-validation properly.
- **Meta-contamination** — training on test at the field level: a single test set
  (e.g. ImageNet) reused across a descendant lineage of papers effectively becomes
  a multi-team training-on-the-test-set process.
- **Semantic duplicates** — near-identical points in both train and test sets
  (e.g. 3.3% of CIFAR-10 / 10% of CIFAR-100 test images have near-duplicates in
  training; >50% of some molecular benchmarks). Removing them drops accuracy.

### 5. Cherrypicking

Cherrypicking results from running multiple tests and reporting the best; a subtler
form reports the best in the main table and relegates variations to the appendix.

- **Baseline hacking / nerfing** — seeking out and reporting weaker/older
  comparators (*hacking*), or picking competitive methods and under-tuning them
  (*nerfing*). E.g. the Claude 3 release used GPT-4 rather than the available, more
  powerful GPT-4T as baseline; 79% of "ML beats numerical PDE solver" papers compare
  to a weak baseline. A subtle form is comparing your post-trained model to someone
  else's base model. Human baselines can also be nerfed.
- **Runtime nerfing** — not optimising baselines' inference parameters (temperature,
  max tokens, top-$p$) or matching their prompting (majority voting, CoT). E.g. the
  Gemini launch compared Gemini's majority-voted $k=32$ GSM8K score to GPT-4-Turbo's
  5-shot CoT score.
- **Runtime hacking** — post-hoc picking the best inference settings/decoding. Using
  an identical prompt for all models is, counterintuitively, not necessarily fair
  (prompt effectiveness is model-specific), but allowing prompts to vary creates a
  degree of freedom to hack while appearing fair (e.g. Gemini's 'uncertainty-routed
  CoT' MMLU result).
- **Benchmark hacking** — reporting only benchmarks where you win, or using an
  outdated/easy benchmark; also avoiding very hard benchmarks (e.g. ARC-AGI). *"To
  be sure of hitting the target, shoot first, and call whatever you hit the
  target."*
- **Subset hacking** — picking the easy part of a hard task: re-generating subsets
  until easier, oversampling easy difficulty levels, or building a subset from only
  the hard questions your model can solve.
- **Harness hacking** — choosing evaluation code that favours your model. E.g.
  Llama-65b's MMLU scored ~30% lower under the EleutherAI harness than under the
  original/HELM harnesses, due to extremely subtle prompt-formatting differences.
  *Metric hacking* is the post-hoc selection of a scoring metric.
- **Golden seed** — manufacturing luck by training/tuning with many random seeds and
  keeping the best (a spurious 1.82% ImageNet gain from the best of 10,000 seeds).
- **Prompt hacking & prompt nerfing** — choosing the best prompt strategy post-hoc
  (prompting can give ~10% absolute gains on hard tasks), or undertuning baselines'
  prompts.

### 6. Misreporting

Little of the above would be unsalvageable if researchers honestly reported all
their work in sufficient and correct detail.

- **Superfluous cog** — an extra ML module that has no effect on performance but
  enables a novelty claim (often because ablation studies are omitted). Examples:
  Core Vector Machines vs SVMs; recommender-system replication studies where simple
  baselines match complex ones when properly tuned.
- **Whack-a-mole** — ad-hoc post-training: silently fine-tuning a model to fix
  specific embarrassing examples (e.g. a viral failure) without changing the public
  version number. Attested anecdotally (jailbreaks that stop working faster than the
  release cycle) but not definitively demonstrated.
- **Benchmark decoration** — pretraining on (specialised) post-training/benchmark
  data others don't pretrain on, then comparing to their base models, making
  fine-tuning gains look like better pretraining generalisation.
- **$p$-hacking** — claiming SOTA after obtaining many $p$-values without correcting
  for multiplicity; abusing choice of test, one- vs two-tailed, one- vs two-sample.
- **Point scores** — reporting a single run (no error bars), despite nonzero-temp
  sampling nondeterminism (the high–low spread over 10 runs can reach 10% on GPQA).
- **Outright lies** — complete fabrication (beyond our definition of a QRP, included
  for completeness; open code is the only countermeasure). E.g. the withdrawn claim
  that GPT-4 scores perfectly on MIT exam questions.
- **Over/underclaiming** — drawing exaggerated qualitative conclusions from benchmark
  results (e.g. Galactica), or the rarer underclaiming (debunking papers that quietly
  used GPT-3.5 not GPT-4).
- **Reification** — mistaking the proxy for the thing itself: treating performance on
  a narrow benchmark as performance on the general task (overclaiming on external
  validity).
- **Nonzero-shot** — claiming 'zero-shot' while training examples are in the corpus,
  effectively putting evaluations into the few-shot setting (PaLM's "zero-shot"
  translation; VLM zero-shot classification largely recognising class labels).
- **Misarithmetic mean** — using the arithmetic mean on normalised scores (invalid;
  the geometric mean may be appropriate).
- **Parameter smuggling** — under-reporting model size, or substituting large
  embedding matrices for Transformer parameters to hit a size threshold (Gemma-7B is
  actually 8.54B).
- **File drawer** — under-reporting negative results; a field-level misrepresentation
  that could occur even if every individual paper were flawless.

### 7. Amplifiers

Residual practices that amplify the effects of other QRPs.

- **Fishing & half-fishing** — 'confirmatory' research without a pre-stated
  hypothesis (HARKing / data dredging); the LLM-specific version is iterating over
  prompts and reporting only the best completion.
- **Inductive smuggling** — handcrafting inductive bias (language bias, mode
  declarations) or otherwise relying on human input (e.g. a human in the
  chain-of-thought loop, the "Clever Hans" effect) for the model to perform well —
  reporting human judgment as system performance.
- **Label noise** — benchmarking on error-ridden datasets (>9% of MMLU, 57% of its
  virology subset; >60% of some dialogue datasets hallucinated; 36% of HellaSwag has
  syntax errors). The QRP is the community continuing to use a benchmark long after
  its flaws are well known.

### 8. Irreproducible research practices

Practices that prevent third parties from reproducing ML training or evaluation;
not classic QRPs, but they enable QRPs by preventing auditing.

| Practice | Description | Accidental? |
|---|---|---|
| Fishing | Conducting confirmatory research without any hypothesis | Plausibly |
| Half-Fishing | Confirmatory analysis without specifying effect direction | Plausibly |
| Dataset Hiding | Not disclosing the sources of the training data | Plausibly |
| Stochastic Runs | e.g. GPU nondeterminism despite fixed seeds | Plausibly |
| No Access | Providing no way to evaluate your model (or no easy API) | Plausibly |
| Closed Evaluation | Using closed-source evaluation data | Plausibly |
| Hidden CoT | Not providing the full completion of your model | No |
| Runtime Hiding | Failing to disclose inference parameters and methodology | No |
| Dummy Code | Uploading empty placeholder files to foil casual inspection | No |
| API Drift | Not reporting behaviour changes of proprietary LLM services over time | Plausibly |

- **Dataset hiding** — ML is unusually reproducible (public benchmarks, often-required
  open code), but this has reversed for commercial 'frontier' systems; withholding
  training data has legitimate reasons (privacy) but reproducibility costs.
- **Stochastic runs** — intentional (SGD minibatching, Dropout) and unintentional
  (GPU thread scheduling changing float summation order) randomness, the latter
  reintroducing nondeterminism despite a fixed seed.
- **No access** — the simplest way to ensure irreproducibility (Gemini Ultra was
  inaccessible except to select partners while its scores were marketed).
- **Closed evaluation** — not disclosing the evaluation process/data.
- **Hidden CoT** — not disclosing the full output (o1 shows only a summary of its
  chain of thought), blocking comparison and black-box interpretability.
- **Runtime hiding** — not disclosing inference parameters, making independent
  verification infeasible given the large search space.
- **Dummy code** — submitting placeholder or empty repositories to satisfy
  code-availability requirements that reviewers rarely inspect.
- **API drift** — silent updates changing a model's behaviour over time without a
  version-string change (>60% of 63 commercial APIs in the HAPI dataset had
  substantial performance changes).

### 9. Discussion

**Misreporting is all you need?** One could argue the truly fundamental QRP is
inadequate reporting — problems of interpretation and even cheating would *in
theory* evaporate if researchers exhaustively reported what they did. But this
assumes the vast data from a modern training process can be sensibly analysed by
readers (supplementary information can run to hundreds of pages), and dataset
hiding means we usually cannot know the training corpus even at the metadata level
— so, except in rare cases, we cannot separate memorisation from generalisation.

**Defences** (some existing, some underused): standardised evaluation harnesses
(e.g. EleutherAI Harness, UK AISI's *Inspect*); semantic decontamination;
contamination databases; private benchmarks; periodic benchmark refresh; gestalt
human-preference evals (LMSYS Arena, though hackable via style); canary strings
(reusing BIG-Bench's exact string gives a zero-overhead filter); full logging and
log summarisation (Weights & Biases); and preregistration of analyses (evaluation
is more preregistrable than chaotic model design).

**Root causes.** (1) *Researchers must self-certify SOTA*: the publication process
makes algorithm designers evaluate their own performance and usually blocks
negative results, heavily incentivising upward bias. (2) *Industrialization of AI
research*: the industrial era retains the scientific trappings of the academic era,
but AI products also pursue "build the best product" and "get the most attention"
(marketing). Goals diverge on "does training on the test set serve my ends?" — a
scientist says no, a business may say yes. There is nothing inherently wrong with
the business perspective; the problem is using unscientific means (e.g.
contamination) and then making scientific claims. Departures from the default
academic norms should be declared explicitly. The biggest science/business
divergences are often on IRPs such as obscuring the training dataset (withheld for
competitive and copyright-litigation reasons).

**Limitations.** The list is not exhaustive. The paper is dual-use, but bad actors
already have simpler options (fabrication). QRPs are hard to detect systematically,
so the work offers existence proofs, not prevalence or effect-size estimates
(planned for follow-up studies).

### 10. Conclusions

We reviewed 44 QRPs — most involving some form of contamination, cherrypicking, or
misreporting — which affect the internal and external validity of ML (and
especially LLM) evaluations, plus IRPs which prevent reproduction, building-on, and
auditing. We listed possible mitigations and suggested two explanations for QRPs:
researcher incentives and the industrialization of research.
