---
title: "Massively Parallel Reweighted Wake-Sleep"
authors:
  - Thomas Heap
  - Gavin Leech
  - Laurence Aitchison
gleech_role: co-author
year: 2023
venue: UAI (oral)
type: conference
arxiv: 2305.11022
doi: 10.48550/arXiv.2305.11022
url: https://proceedings.mlr.press/v216/heap23a.html
code: https://github.com/ThomasHeap/MPRW-S
links:
  poster: https://www.auai.org/uai2023/posters/216.pdf
  video: https://www.youtube.com/watch?v=LypuoCIX6xA
contribution_hours: 300
topics: [bayesian-inference, probabilistic-ml, wake-sleep, importance-weighting]
---

## Abstract

Reweighted wake-sleep (RWS) can do Bayesian inference in a very general class of
models. It samples from an approximate posterior Q, then uses importance
weighting to estimate the true posterior P, then updates Q towards the
importance-weighted estimate of P. But the sheer number of samples needed in
importance weighting rules out any realistic-size model.

We develop "massively parallel RWS", which samples all latent variables and
reasons about all possible combinations of samples. This is doable in polynomial
time by exploiting conditional independencies, giving a dramatic speedup over
standard RWS (which samples the full joint).

## Full text

> Converted from the arXiv PDF (v1, UAI 2023). LaTeX markup removed; the detailed
> derivations (Section 4 and Appendices) are heavy with equations and are summarised
> rather than transcribed; figures omitted.

**Abstract.** Reweighted wake-sleep (RWS) is a machine learning method for performing
Bayesian inference in a very general class of models. RWS draws $K$ samples from an
underlying approximate posterior, then uses importance weighting to provide a better
estimate of the true posterior, and updates its approximate posterior towards the
importance-weighted estimate. However, recent work (Chatterjee and Diaconis, 2018)
indicates that the number of samples required for effective importance weighting is
exponential in the number of latent variables — intractable in all but the smallest
models. Here, we develop massively parallel RWS, which circumvents this by drawing $K$
samples of all $n$ latent variables and individually reasoning about all $K^n$ possible
combinations of samples. While reasoning about $K^n$ combinations might seem
intractable, the required computations can be performed in polynomial time by
exploiting conditional independencies in the generative model. We show considerable
improvements over standard "global" RWS, which draws $K$ samples from the full joint.

### 1. Introduction

Many ML tasks involve inferring latent variables from observations. Bayesian inference
computes the posterior $P(\text{latents} \mid \text{data}) \propto P(\text{data} \mid
\text{latents})\,P(\text{latents})$, but this is typically intractable. Modern
approaches — variational inference (VI) and RWS — instead learn the parameters $\phi$
of an approximate posterior $Q_\phi(\text{latents} \mid \text{data})$. VI optimizes the
ELBO, which often has considerable slack that biases inferences; importance-weighted
autoencoders (IWAEs) draw multiple samples and use importance weighting for a tighter
bound. In RWS, we draw multiple samples, reweight them to approximate the true
posterior, then update the approximate posterior towards that reweighted estimate (the
wake-phase Q update).

However, Chatterjee and Diaconis (2018) showed the number of samples needed for
accurate importance-weighted estimates scales as $e^{D_{KL}(P(z|x)\,\|\,Q(z|x))}$, and
we expect the KL divergence to scale linearly in the number of latent variables $n$ —
so the required samples are *exponential* in $n$, infeasible for larger models. This
was addressed in the IWAE context by TMC (Aitchison, 2019), which draws $K$ samples per
latent and reasons about all $K^n$ combinations. We develop the analogous approach for
RWS — **massively parallel (MP) RWS**. This is not a simple extension: TMC's derivations
are restricted to factorised approximate posteriors or use an augmented-state-space
viewpoint that cannot apply to RWS. Our considerably more general derivations even
allow a more general class of approximate posteriors in the original VI setting.

### 2. Related work

Our methods build on VI, IWAE, and RWS. The most obvious related work is **TMC**
(Aitchison, 2019), which also draws $K$ samples per latent and considers all $K^n$
combinations — but TMC applies only to VI, while ours applies to RWS; and our more
general derivations improve on TMC itself (TMC forces the $K$ particles for a latent to
be IID, whereas we can *couple* them, enabling variance-reduction strategies).
**Local importance weighting** (Geffner and Domke, 2022) applies to single-level
hierarchical models and draws only a single sample of the Bayesian parameter; it (like
TMC) ultimately performs VI and is restricted to that model class. Massively parallel
methods in timeseries resemble particle filtering / sequential Monte Carlo, but SMC
work that learns a proposal focuses on VI not RWS, and usually a restrictive timeseries
class.

### 3. Background

Both IWAE and RWS work with a collection of $K$ samples of the latent variables. For
global VI and RWS, the $K$ samples are drawn from the single-sample approximate
posterior, $Q_{\text{global}}(z \mid x) = \prod_{k} Q_\phi(z^k \mid x)$. Both can be
written via an unbiased marginal-likelihood estimator $P_{\text{global}}(z) = \frac{1}{K}
\sum_k r_k(z)$, where $r_k(z) = P_\theta(x, z^k) / Q_\phi(z^k \mid x)$. IWAE optimizes a
lower bound on $\log P_\theta(x)$; RWS instead uses this estimator in its wake-phase Q
update.

### 4. Massively parallel RWS

The core idea: draw $K$ samples for *each* of the $n$ latent variables, and reason about
all $K^n$ combinations — made tractable in polynomial time by exploiting conditional
independencies in the generative model (a tensor-contraction / `opt_einsum`-style
computation). The paper gives new, more general derivations of the massively parallel
marginal-likelihood estimator and the corresponding RWS wake-phase update, which (unlike
TMC) permit coupling the distribution over the $K$ particles for a single latent — the
key to later variance reduction.

### 5. Experiments

MP RWS tested with $K \in \{3,10,30\}$ vs global RWS with $K$ up to 30,000; variational
posterior factorised over latents in the same families as the generative model;
optimised with Adam (lr 0.001, decayed 10× every 10k iters); 5 seeds (mean ± standard
error); single Nvidia A100.

- **MovieLens-100K** (943 users × 1682 films, ratings binarised): a hierarchical model
  with a global mean, a *discrete* variance latent $\psi$, and per-user latent vectors;
  high-rating probability from the dot product of user vector and film features. RWS
  handles the discrete latent straightforwardly (no reparameterisation, REINFORCE, or
  continuous relaxation needed). Evaluated on subsets (5 or 10 films/user; 50/150/300
  users) by test predictive log-likelihood. **MP RWS gives considerably higher
  predictive log-likelihoods for all $K$**; even accounting for its more complex (slower)
  updates by plotting performance vs wall-clock per iteration, improvements remain
  considerable.
- **NYC Bus Breakdown** dataset: delay length modelled with a three-level hierarchical
  regression (Year → Borough → ID) plus weight vectors for bus company and journey type,
  feeding a negative-binomial likelihood. **MP RWS again outperforms global RWS for all
  $K$.**
- **MP VI vs TMC** (two toy timeseries, single- and multi-observation): our derivations
  also yield a more general MP VI that *couples* the $K$ samples per latent, enabling
  variance reduction inspired by anti-degeneracy methods in particle filters. TMC performs
  considerably worse than MP VI and IWAE due to particle degeneracy (a parent particle can
  have zero, one, or many children, reducing diversity); MP methods ensure each parent has
  exactly one child. With multiple observations, MP VI beats both alternatives for lower
  $K$.

### 6. Conclusion

We introduced massively parallel RWS, drawing $K$ samples for $n$ latent variables and
efficiently considering all $K^n$ combinations by exploiting conditional independencies in
the generative model. It represents a considerable improvement over previous RWS methods
that draw $K$ samples from the full joint latent space.
