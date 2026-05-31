---
title: "Steering Language Models Without Optimisation"
full_title: "Activation Addition: Steering Language Models Without Optimization"
authors:
  - Alex Turner
  - Lisa Thiergart
  - David Udell
  - Gavin Leech
  - Ulisse Mini
  - Monte MacDiarmid
gleech_role: co-author
year: 2023
type: preprint
arxiv: 2308.10248
doi: 10.48550/arXiv.2308.10248
url: https://arxiv.org/abs/2308.10248
code: https://github.com/montemac/activation_additions
links:
  iclr: https://openreview.net/forum?id=2XBPdPIcFK
  summary: https://www.lesswrong.com/posts/HWxLQvzJGeXoLPJWd/actadd-steering-language-models-without-optimization
  blogpost: https://www.lesswrong.com/posts/5spBue2z2tw4JuDCx/steering-gpt-2-xl-by-adding-an-activation-vector
contribution_hours: 190
topics: [interpretability, ai-safety, activation-engineering, steering, llm]
---

## Abstract

We investigate activation engineering: modifying the activations of a language
model *at inference time* to predictably alter its behavior. It works by adding
a bias to the forward pass, a "steering vector" implicitly specified through
normal prompts.

Activation Addition (ActAdd) computes these by taking the activation differences
of pairs of prompts. We get control over high-level properties of the output
without damaging the model's performance. ActAdd takes far less compute and
implementation effort than finetuning or RLHF, allows nontechnical users to
provide natural-language specifications, and scales naturally with model size.

This is the first? alignment method which doesn't need training data or gradient
descent.

## Full text

> Converted from the LaTeX source of the **ICLR 2025 version**, which is retitled
> *Steering Language Models With Activation Engineering* and expanded (it now
> primarily steers LLaMA-3.1-8B; the author list adds Juan J. Vazquez). LaTeX
> markup removed; citation keys rendered as author–year where named in prose,
> otherwise dropped. Figures omitted; the algorithm and key tables are kept.

**Abstract (ICLR version).** Prompt engineering and finetuning aim to maximize
language model performance on a given metric (like toxicity reduction). However,
these methods do not optimally elicit a model's capabilities. To reduce this gap,
we introduce a form of *activation engineering*: the inference-time modification
of activations in order to control (or *steer*) model outputs. Specifically, we
introduce the *Activation Addition* (ActAdd) technique, which contrasts the
intermediate activations on prompt pairs (such as "Love" versus "Hate") to compute
a *steering vector* (Subramani 2022). By tactically adding in e.g. the
"Love"−"Hate" steering vector during the forward pass, ActAdd can perform many
tasks like topic steering, sentiment steering, and detoxification. ActAdd yields
inference-time control over high-level output properties (like topic and
sentiment) while sometimes preserving performance on off-target tasks. ActAdd is
lightweight: it does not require any machine optimization and works with a single
pair of data points, which enables rapid iteration over steering. While ActAdd
does not achieve SOTA, we explore its benefits and drawbacks.

### 1. Introduction

LLMs contain hidden capabilities we do not know how to fully elicit (Korinek
2023). Naively prompting a model with a question does not maximize the probability
of the correct response. For example, consider how prompting a model to think
"step-by-step" (Wei 2022) massively improves performance on a range of benchmarks.
Similarly, "few-shot" prompting a model with correct answers to unrelated
in-distribution questions allows "in-context learning" for e.g. stronger
performance on NLP tasks (Brown 2020). Importantly, these interventions do not
supply the LLM with extra task-relevant information or update the algorithm
implemented by the LLM's computational graph. Even though the model is initially
*able* to score higher on these benchmarks, those capabilities do not emerge
without a specific intervention. We therefore hypothesize an *elicitation
overhang*: we do not know how to elicit all relevant abilities and information from
models.

Prompt engineering is the most obvious way to steer a model, but prompting has
limited reliability. Therefore, to reduce the elicitation overhang, we explore a
new modality for steering language model outputs. By strategically perturbing
activations during the forward pass, we hope to more reliably and effectively
steer models compared to prompt engineering. We call this methodology *activation
engineering*.

We suspect that compared to prompt engineering, activation engineering can elicit
a wider range of model capabilities. Consider, for example, a model optimized to
imitate the text outputs of eloquent poets and awkward mathematicians. The model
may contain the internal mechanisms required to output text which is *both*
eloquent and mathematical. However, if the model is an accurate estimator of the
training distribution, it will (correctly) assign low probability to eloquent
mathematical prose. Because nothing in the training data was both eloquent and
mathematical, there may exist no prompt which elicits mathematical prose. In
contrast, activation engineering might be able to simultaneously activate the
circuitry for eloquent speech and for mathematical content.

To demonstrate the power of activation engineering, we introduce *Activation
Addition* (ActAdd). Suppose we want to achieve negative-to-positive sentiment
control. To achieve this, ActAdd first compares the model's activations on a
contrast pair of prompts, such as the prompts "Love" and "Hate." The
otherwise-similar prompts differ along the target dimension of sentiment. ActAdd
then computes the difference of these activations in order to compute *steering
vectors*. These vectors act like "virtual bias terms" because ActAdd *directly
adds* the steering vectors to the forward pass at inference time. By shifting the
inference-time activations along the direction of the steering vector, ActAdd
steers the model to generate positive sentiment completions.

**Example impact of ActAdd** (steering vectors from "Love"−"Hate" and "I talk
about weddings constantly"−"I do not talk about weddings constantly"):

| Prompt | Steering | Completion |
|---|---|---|
| I hate you because... | [None] | ...you are the most disgusting thing I have ever seen. |
| I hate you because... | ActAdd (love) | ...you are so beautiful and I want to be with you forever. |
| I went up to my friend and said... | [None] | ..."I'm sorry, I can't help you." "No," he said. "You're not." |
| I went up to my friend and said... | ActAdd (weddings) | ..."I'm going to talk about the wedding in this episode of Wedding Season. I think it's a really good episode. It's about how you're supposed to talk about weddings." |

**Contributions.** We unify past literature on related topics to introduce
*activation engineering*. To better elicit the full capabilities of models, we
introduce the ActAdd steering method. ActAdd achieves substantial (but not SOTA)
control on toxicity reduction and sentiment control. We thoroughly test ActAdd's
generality and effects on general capabilities. We therefore show the promise of
ActAdd as an effective and cheap method for steering LLM outputs.

### 2. Related Work

**Latent space arithmetic.** Computer vision researchers have long demonstrated
the ability to steer image generation using derived vectors, including steering
latent variables — most famously, shifting activations along a direction that
corresponds to smiling in images (Larsen 2016; White 2016). Similarly, in the text
domain, classic results on the word2vec embedding show that arithmetic on word
vectors can capture some parts of semantic reasoning (for instance, analogies:
Mikolov 2013). Our work focuses on steering generative language models.

**LLM steering.** Many approaches attempt to affect the output of a pretrained
LLM, whether:

- *Intervening on weights*, as with supervised finetuning, RLHF, steerable layers,
  and weight editing (targeted fine-tuning). However, naive RLHF, finetuning, and
  weight editing have known side-effects on overall model performance;
- *Intervening at decoding*, as with guided or trainable decoding;
- *Intervening on the prompt*, as with automated prompt engineering;
- *Intervening on token embeddings*, as with 'soft prompting';
- *Intervening on activations*, for instance by freezing the weights of the LLM
  and searching for a 'steering vector' of activations, e.g. using gradient descent
  (Subramani 2022; Hernandez 2023). These optimized extraction methods, which
  search for a steering vector, differ from extraction methods which directly
  compute it (present work and Li 2023, ITI). In our work, we do not use gradient
  descent or other optimization methods.

**Table — locating our work in the steering literature.**

| Intervention vectors obtained via | ... intervenes on *weights* | ... intervenes on *activations* |
|---|---|---|
| Differences after fine-tuning | Ilharco 2023 | N/A |
| Per-query gradient-based search | Meng 2022, Orgad 2023 | Dathathri 2020, Subramani 2022, Hernandez 2023 |
| Differences between prompt pairs | N/A | **ActAdd** (present work), Li 2023 (ITI) |

**Activation engineering.** Activation engineering involves creating vectors of
activations which cause desired changes to output text when added to the forward
passes of a frozen LLM (Dathathri 2020). An early antecedent is the Plug-and-Play
Language Model of Dathathri 2020, which uses a separate classifier (one per
attribute) to perturb the model's activations. Subramani 2022 extract latent
steering vectors from a frozen LLM, discovering sentence-specific vectors which
steer completions to near-perfect BLEU scores and unsupervised style transfer;
however, the method requires running gradient descent for each new steering
vector. Hernandez 2023 locate and edit an LLM's knowledge through learning an
encoding of facts in its activation space. Ablating attention heads can also be
seen as activation engineering, though mostly used for interpretation rather than
steering.

Independently of our work, Li 2023 developed a similar method called ITI which
computes steering vectors applied selectively according to trained linear probes;
ITI adds the same vector at all sequence positions and requires dozens of samples,
whereas ActAdd adds steering vectors to a subset of sequence positions and
requires as few as 2 samples. Similar work on 'in-context vectors' also followed
ours (Liu 2023). Zou 2023's "representation engineering" also followed our work,
developing a range of techniques for deriving steering vectors; in comparison, we
steer different models (primarily LLaMA-3.1-8B, but also LLaMA-3, OPT, GPT-2, and
GPT-J) on different tasks. Dekoninck 2024's Language Model Arithmetic (LMA)
combines multiple models' output characteristics by solving an optimization
problem involving KL-divergences, though it requires having trained multiple
models. Not all activation-focused works aim to control outputs: some
interpretability techniques, like *activation patching*, simply resample
activations instead of adding a vector.

### 3. How Activation Addition works

We use decoder-only Transformer neural networks. The LLMs in this work contain a
stack of Transformer layers, each consisting of multi-head attention (MHA) and a
feedforward network (FFN). We focus on its "residual streams", the sequences
$(\mathbf{x}_0, ..., \mathbf{x}_n)$ of intermediate activation vectors processed by
each layer. ActAdd manipulates the residual stream values $\mathbf{h}^l$ input to
layer $l$. At inference time, the residual stream is initialized $\mathbf{h}^1$
with the embedding of the tokenized prompt.

**Activation addition.** Our method takes a pair of natural-language prompts
$(p_+, p_-)$, where $p_+$ represents the property we wish output text to emphasise
(e.g. love) and $p_-$ represents its opposite (e.g. hate or indifference).
$\mathbf{h}_+^l$ is the activation vector for the prompt $p_+$ at layer $l$. The
difference $\mathbf{h}_+^l - \mathbf{h}_-^l$ is a new activation vector which
(intuitively) captures the difference between a prompt with the target property
and a prompt without it. The steering vector is computed before inference time.

**Algorithm (ActAdd, optimization-free activation addition).** Input: steering
prompt pair $(p_+, p_-)$ tokenized; user prompt $p^*$; target layer $l$; injection
coefficient $c$; sequence position $a$ to align $\mathbf{h}_A$ and
$\mathbf{h}_{p^*}$; pretrained model $M$. Output: steered output $S$.

1. $(p_+', p_-') \leftarrow$ `pad_right_same_token_len`$(p_+, p_-)$
2. $\mathbf{h}_+^l \leftarrow M.\texttt{forward}(p_+').\texttt{activations}[l]$
3. $\mathbf{h}_-^l \leftarrow M.\texttt{forward}(p_-').\texttt{activations}[l]$
4. $\mathbf{h}_A^l \leftarrow \mathbf{h}_+^l - \mathbf{h}_-^l$
5. $\mathbf{h}^l \leftarrow M.\texttt{forward}(p^*).\texttt{activations}[l]$
6. $S \leftarrow M.\texttt{continue\_forward}(c\,\mathbf{h}_A^l + \mathbf{h}^l[a])$

To obtain a steering vector, we perform a forward pass on each prompt, record the
activations at the given location in each pass, take the difference
$\mathbf{h}_+^l - \mathbf{h}_-^l$, and finally rescale this difference by an
'injection coefficient' $c$. To steer, we add the resulting activation vector to
the input of layer $l$ and allow the forward pass to continue. $c$ represents the
intervention strength (typically magnitude < 15). We perform hyperparameter tuning
to select $c$ and the injection layer $l$; as expected from past work, intervening
at the middle layers is most effective. We test whether 1) steering vectors are
effective at eliciting the desired behavioral shift, and 2) whether they preserve
the general capabilities of the model. We run perplexity-based experiments on
GPT-2-XL (1.5B parameters), then toxicity and sentiment experiments on
LLaMA-3.1-8B.

### 4. Results: Activation Addition works

**4.1. ActAdd intuitively modifies next-token probabilities.** We consider the
OpenWebText corpus. Our running example is the "wedding" topic vector produced by
setting $p_+ =$ `weddings`, $p_- =$ ` ` (a space), $l=16$, $c=1$.

*ActAdd reduces perplexity on a target topic.* For each document in OpenWebText we
calculate the frequency of wedding-related words; a document containing one is
considered wedding-related. We randomly sample 300k documents, half wedding-
related, split into sentences, and measure GPT-2-XL's perplexity on wedding-related
and unrelated sentences. In sentences where the injected topic (weddings) is more
relevant, ActAdd's perplexity is lower and predictive performance increases.

*ActAdd's impact on token probabilities.* Sampling 500 documents and recording
log-probabilities from baseline and steered models (~500k tokens, 29k unique), we
group by token, filter for tokens with >20 instances, and compute the mean
perplexity difference. The mean log-probability difference distribution is
approximately normal for the bulk of tokens, with clearly heavy tails; the
positive tail is generally wedding-related and significantly heavier than the
negative tail. The probabilities most increased on average are primarily
wedding-related.

*ActAdd steers the model to discuss weddings.* Sweeping over GPT-2-XL injection
layers for the wedding vector, the intervention is already effective at the very
first layer, rises in effectiveness until layer 6, and then declines. For the
optimal injection site, we see >90% success in topic steering (compared to a ~2%
baseline).

**4.2. ActAdd can control what the model talks about.** Steering vectors can elicit
generations on a range of topics — not just weddings. We generate 1000 completions
from the unsteered model and 1000 for each target single-token ActAdd intervention,
using GPT-4o-mini to score whether generations are about a target topic. We record
a large boost in relevance (5–25%) on all topics at injection coefficient $c=2$.

**4.3. ActAdd can reduce toxicity.** We benchmark on the /pol/ dataset and
RealToxicityPrompts (random subset $n=2000$, $T=1$, nucleus $p=1.0$, 5 repeats for
$p$-values), using the "love"−"hate" ActAdd vector, $l=6, c=3$, the Perspective API
for toxicity and LLaMA-3.1-8B logprobs for disfluency. On RealToxicityPrompts,
ActAdd makes a 20% improvement over an unsteered baseline — but the best method
(LMA+Classifier) sees 29%. On /pol/, ActAdd improves 6% over baseline where the
best method improves 37%. ActAdd's disfluency is much worse than other methods on
/pol/.

**Table — detoxification** (steering LLaMA-3-8B; **bold** = $p<0.05$ vs second-best;
toxicity = Perspective API; disfluency = LLaMA-3.1-8B perplexity; lower better):

| Method | RealToxPrompt ↓ | Disfluency ↓ | /pol/ ↓ | Disfluency ↓ |
|---|---|---|---|---|
| Unsteered | .127 | 16.0 | .323 | 19.3 |
| ActAdd (*ours*) | .101 | 20.4 | .305 | 48.0 |
| FUDGE | .103 | 16.2 | .269 | 20.5 |
| LMA | .104 | 15.8 | .232 | **17.9** |
| LMA + Classifier | **.090** | 16.1 | **.205** | 18.7 |
| SelfDebias | .123 | 18.2 | .299 | 22.8 |
| PreADD | .099 | 16.7 | .234 | 19.3 |

**4.4. ActAdd can control sentiment.** Using the Stanford IMDb dataset, our goal is
for the model to continue each review with the opposite sentiment; we compute the
proportion of outputs with the desired sentiment, classified by Twitter-roBERTa
(random subset $n=1000$, $l=6$, $c=3$). ActAdd can control sentiment on one
conventional measure, though it falls short of SOTA.

**Table — sentiment steering** (Stanford IMDb; "Success" = probability of changing
the classified sentiment, higher better; **bold** = $p<0.05$ vs second-best):

| Method | Pos-to-neg ↑ | Disfluency ↓ | Neg-to-pos ↑ | Disfluency ↓ |
|---|---|---|---|---|
| Unsteered | 0.207 | 17.23 | 0.200 | 18.49 |
| ActAdd (*ours*) | 0.395 | 29.18 | 0.349 | 29.30 |
| Prompted | 0.265 | 17.94 | 0.246 | 18.36 |
| LMA | 0.423 | **16.74** | 0.378 | **16.69** |
| LMA + Classifier | **0.471** | 17.01 | **0.459** | 17.51 |
| SelfDebias | 0.275 | 18.46 | 0.236 | 20.35 |
| FUDGE | 0.367 | 17.93 | 0.302 | 19.75 |
| PreADD | 0.420 | 19.30 | 0.339 | 19.05 |

**4.5. ActAdd preserves the model's general knowledge.** We use ConceptNet from the
LAMA benchmark ($n=29{,}774$ sentences). $P@K$ is the probability that the expected
label is among the model's top-$K$ predicted tokens. On this benchmark of factual
questions, our method has a negligible impact on off-target answer probabilities
(i.e. where the domain is unrelated to the steering vector).

### 5. Discussion

**Limitations.** Initially, ActAdd seemed to achieve SOTA on detoxification and on
one kind of sentiment steering. However, stronger methods have since been released,
and our standardized tests on /pol/ show ActAdd does not robustly outperform across
datasets; ActAdd substantially increases perplexity. To steer the model, the user
supplies the injection coefficient $c$ and the intervention layer $l$ (we fix
sequence alignment $a=1$); these free hyperparameters make ActAdd less
user-friendly than simple prompt engineering, though in practice intervention
hyperparameters are stable. We have not examined ActAdd's potential impact on
reasoning. ActAdd is not immediately applicable given only API access, since the
model must cache and expose intermediate activations.

**Activation engineering vs finetuning.** Finetuning is better understood and more
flexible — we doubt activation engineering can e.g. teach a model a new skill.
However, finetuning is significantly more costly and may not elicit the same kinds
of capabilities. ActAdd's first advantage is efficiency: it requires no backward
passes and minimal labeled data (just the steering prompt pair); even nontechnical
users can benefit from rapid feedback and easy iteration.

**Activation engineering vs prompt engineering.** Activation additions can be
continuously weighted, while prompts are discrete. To more intensely steer, our
method does not require any edit to the prompt, just increasing the injection
coefficient. Activation additions do not take up token space in the context
window. While prompting is more flexible and even cheaper than ActAdd, activation
additions may elicit capabilities which prompting cannot.

**Algebraic combination of forward passes.** ActAdd can be viewed as composition of
separate forward passes. We were surprised that forward passes can "compose" in
this way despite the model not being trained to allow it; this composability is
itself evidence for compositional representations.

**Interpretability.** In most programs, adding values to imprecisely targeted
intermediate memory would not yield sensible results — why expect this from
Transformers? We think neural networks represent features of the input as
directions in activation space, and that the direction corresponding to (say) a
love–hate latent variable stays approximately the *same* across a broad class of
inputs. The success of activation addition gives experimental evidence of feature
linearity, demonstrating that models *use* feature-related information; steering
vectors establish causality, at least in the limited set of contexts examined.

**Value alignment of LLMs.** Activation engineering is a promising way to control
LLMs. Successor methods may provide general steering (e.g. through some analogue of
a `Be helpful` vector). Our experiments suggest activation engineering can flexibly
retarget LLM behavior without damaging general performance; suitably developed, the
approach could enable safety progress while preserving overall capabilities.

### 6. Conclusion

While methods like prompt engineering, controlled decoding, and finetuning have
benefits, they fail to elicit full capabilities from language models. To more
reliably elicit these abilities, *activation engineering* strategically perturbs
activations at inference time. In particular, we introduced *Activation Addition*
to steer models by shifting their inference-time activations along a certain
direction (like the "Love"−"Hate" vector). ActAdd is lightweight and sometimes
effective; we achieve good results on topic steering and mixed results on toxicity
reduction and sentiment shift. ActAdd demonstrates the potential promise of
activation engineering.

**Reproducibility.** Code: https://zenodo.org/records/14177088. Data processing:
wedding-related subset of OpenWebText retained via wedding-related words;
preprocessing removes null-character sequences; documents split into sentences via
the Punkt tokenizer. Sampling: nucleus $p=1.0$, temperature $T=1.0$, no top-$k$,
frequency penalty 1.0. Earlier versions used Llama-1-13B, GPT-J-6B, OPT, and
LLaMA-3-8B. Toxicity scored via Perspective API; fluency via LLaMA-3.1-8B. All
generations on seed 0 (validated on seeds 1, 2). ActAdd hyperparameters via simple
grid search, usually $c \in [3,20]$ and $l \in [6,24]$.
