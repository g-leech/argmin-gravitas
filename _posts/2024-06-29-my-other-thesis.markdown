---
layout:     page
title:      "things I didn't do for my thesis"
baselink:   /theorysis
permalink:  /theorysis
date:       2024-02-29  <!--site.time-->
author:     Gavin

img:        /img/phd/
published:  true
visible:    1
quality:    5
emotion:    5
importance: 6

summary:    what I ran out of time or nerve for
confidence: 
categories: phd, navel-gazing
warnings:   navel-gazing
wordcount:      
---

<br>


I was stoked to do an <a href="https://www.marxists.org/reference/subject/philosophy/works/ge/feyerabe.htm">analytical index</a>: that is, a table of contents which forms a complete logically valid argument. I made a start but did not have the strength.

<br>

---

<br>

I wanted to compare evidential standards across fields. Which is easiest to publish in? Which replicates? What's the average Bayes factor of a paper?

<br>

---

<br>

When is evidence better than nothing? I say that you can't trust observational studies unless they do craploads of really careful data and model checking. But it's not obvious that e.g. my mask paper is nonzero evidence even so.

<br>

---

<br>

We did independent data entry on the psych dataset; FORRT did a round, then my small team (thanks Sam) did one. A typical data entry error rate on a crowdsourced dataset is 2%.

Clearly we can't take the independent product 2x 2= 0.4as the new estimate, it's going to be much higher because the errors are correlated (some cases are much trickier than others). But I could have estimated this! First I would produce like 20 really really really carefully labelled ground truth examples. Then I would get my actual two sets of volunteers to label them and estimate the error correlation.

You could also do some clever work to estimate the size of the label-difficulty `tail'. You could also do Bayesian labelling where the given two labels are just evidence about the actual label.

<br>

---

<br>

I wanted to do vast GPT-enabled exhaustive literature reviews

One notable thing possible now, but not 5 years ago, is an exhaustive automated literature review.
https://www.gleech.org/benchmarks 
https://journals.sagepub.com/doi/full/10.1177/02683962211048201


A recurring theme is getting data on what people are actually doing in new papers, how methods have changed. Large language models enable exhaustive literature reviews including on relatively subtle questions like `does this paper contain a mathematical proof?'

https://journals.sagepub.com/doi/full/10.1177/02683962211048201

    EXPERIMENT: classifying all of ML arxiv by methods
    EXPERIMENT: ablating 10 years of tweaks. What’s the minimal working arch?
    EXPERIMENT: what’s the scaling law for random forests?
    EXPERIMENT: big replication study?

    EXPERIMENT: Rate of test set use in 2019 epi vs 2023 epi
    EXPERIMENT: rate of economic cost estimates in NPI epi

<br>

---

<br>

I wanted to unify all the fields I talked about.

Stamp collecting 
Post hoc theory
competitive testing on fixed benchmarks


Scientific questions: 
* “whether X is”. Discovery. nonzero inference
* “what X is”. Structure 
* “whether X is Y”. Relations, equivalences, clustering, classification
* “why X is Y”. Causal model.

hypothesis testing and confirmation, parameter estimation, sequence prediction, classification, and regression

frameworks that underwrite/retcon different sciences
*    random hypothesis testing, 
*    supervised learning approximates hypothesis testing
*    empirical risk minimization
*    SLT / Bayes / asymptotics / estimator theory
*    list of assumptions, derived constraints, norms, estimators, optima
*    list of practices and QRPs

what are they failing at?


SLT isn't actually practically normative / isn't a complete description of ML practice. everyone knows this but we still use it because we feel bad without a theory

force a pairwise comparison
    contrast two methods
    e.g. “is it a special case of Bayes?”
    build into a Table


<br>

## Why is statistical learning so dominant?
Stats as the science of defaults (automation)
Symbolic AI failure, ILP complexity

Model specification is hard.     Deep learning lets you ~skip it.    NHST lets you sweep it under the rug.     Bayesian nonparametrics?

<br>

## Canned inference and the automation of bad science

## Generalisation and its enemies




## The Pandemic Evidence Failure

We did not learn as much about Covid as we hoped to, in real time.

Incentives. 800,000 papers?
Bad policy
Bad practices, QRPs
EXPERIMENT: No test sets in 2020

https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0248793
https://ebm.bmj.com/content/27/5/253
https://www.ncbi.nlm.nih.gov/pmc/articles/PMC8753111/
https://onlinelibrary.wiley.com/doi/10.1111/ina.13070

Solution: PPLs
TMC, TPP, RWS, …

On the other end of the failure to rapidly gather and analyse evidence was a lack of decision analysis. Cost-benefit analysis



<!-- Here's the kind of thing that one might do if one was able to try harder https://arxiv.org/abs/2403.07949 -->




## Trends, problems

EXPERIMENT: Rate of test set use in 2019 epi vs 2023 epi
EXPERIMENT: rate of economic cost estimates in NPI epi



Failure of reform in psych
Rates in fields. 
No change in 8 years of `reform'.


https://docs.google.com/document/d/1hgAOMoa8DypdwqhXzyDAEchQaB3bnqMKZ_LNFdcXZxs/edit
https://twitter.com/I4Replication/status/1733248064410771546 
https://twitter.com/lakens/status/1735909530205138974?t=Uomd3IVyW8MVsttnNVD_dQ&s=19
SCORE. Random sample, stratified
This is itself QRP
https://www.argmin.net/p/is-the-reproducibility-crisis-reproducible 
https://twitter.com/bethclarke_/status/1735880441083928657?t=xFo9TnZr06Glc95UA4IkbA&s=19 
https://osf.io/preprints/psyarxiv/dm8xn?utm_source=substack&utm_medium=email 
https://osf.io/preprints/psyarxiv/jbu9r 
https://psycnet.apa.org/record/2024-22735-001?utm_source=substack&utm_medium=email
https://pubmed.ncbi.nlm.nih.gov/38327122/ 
https://en.m.wikipedia.org/wiki/GRIM_test#:~:text=The%20granularity%2Drelated%20inconsistency%20of,the%20analysis%20of%20data%20sets 



Privatisation of AI
    Classifier: how open is this paper methodology?
    Joint affiliations and movement into industry
    Classifier: list all AI startups, classify as ex-academic



## The spurious features controversy

https://arxiv.org/abs/2110.04301
https://openreview.net/forum?id=BcbwGQWB-Kd
parrots