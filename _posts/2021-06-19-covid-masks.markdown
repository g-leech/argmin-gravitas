---
layout:     post
title:      "Do masks work against COVID, at scale?"
baselink:   /masks
permalink:  /masks
date:       2021-06-19
author:     Gavin   
img:        /img/papers/masks.png

visible:    1
published:  true

summary:    Inferring the effect of mass mask-wearing on COVID.
quality:    3
confidence: 
importance: 7
wordcount:  500
warnings: 	
categories: code, science, research
---

{%	assign mxv = "https://www.medrxiv.org/content/10.1101/2021.06.16.21258817v1"	%}
{%	assign tw = "https://twitter.com/g_leech_/status/1406287131534893059"	%}

<a href="{{mxv}}">We have a new preprint!</a>. <a href="{{tw}}">Here's a full explainer thread</a>.

We seem to be first to use the incredible <a href="https://covidmap.umd.edu/">UMD / Facebook</a> survey of COVID behaviour to look at masks.

Short answer: Yup! 25% [6%, 43%] reduction in R the reproduction number, or cases / case.

We also have interesting secondary results

* Voluntary mask wearing started earlier and to a larger extent than previously realised. (64% of the world reported wearing masks by _May_ 2020.)
* We have exactly two examples of noncompliance with mandates.
* Mask wearing has started falling (about 5% over May 2021) _in countries without fast vaccination campaigns_.
* Past work used the timing of government mask mandates. You really can't do this, because of the huge voluntary uptake prior to them.

<br>

---

<br>

## What's the catch?

- We only use data from last summer, our wearing data is still a proxy (self-reported wearing), and our analysis is observational. See Discussion for lots more.

- Our analysis goes further in the quality of wearing data - 100 times the sample size, with random sampling and post-stratification - geographical scope, the sophistication of our infection model, the incorporation of the uncertainty in epidemiological parameters, and the robustness of our results (123 sensitivity experiments).

- Our analysis begins in May 2020, after some of the earliest mandates, as that’s when data first became available. 

- Summer 2020 has distinctive features: many regions began with NPIs already active; public behaviour had already changed following the (in)formal instructions of the first wave; and summer months are thought to have lower transmission

- We don’t break the effect down by the venue of wearing. We don’t look at cultural factors or serious differences in effectiveness of different types of masks. Our analysis is at the national (or US state) level, so we could miss subtler policy effects.

Our definition of ‘mask-wearing' isn’t stringent: it’d apply to a person who wears a cloth mask, only on public transport, 51% of the time; and to a person who always wears an N95 respirator outside home. So there’s scope for more & better wearing, even in regions reporting high levels in our data.

<br>

---

<br>

<a href="{{tw}}">Here's a full explainer thread</a>.

Here's the [code](https://github.com/g-leech/masks_v_mandates) (end-to-end instructions).

<br>