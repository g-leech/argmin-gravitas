---
layout:     math_post
title:      'Abusing "inference"'
baselink:   /inference
permalink:  /inference
date:       2025-10-10
author:     Gavin
img:        /img/

visible:    1
published:  true
quality:    4

summary:    Deference in the most profitable science
warnings: 	
confidence: 75%
importance: 4
wordcount:  
categories: AI
where:      "Over the Atlantic"
---

People in ML recently started using "inference" to mean _running_ a model: using it for prediction (see also "forward pass", "[function] evaluation", "forward propagation"). The framework people still call the relevant function `.predict()` or `.eval()` or `.forward()` rather than `.infer()`. This clashes pretty hard with the conventional usage (from statistics), which is much more like training. How did that happen?

### In stats

We _infer_ the values of unobserved $$\theta$$ from observed $$X$$.

"Inference" first got its modern meaning during the great formalisation of the 1920s. Fisher's "Statistical Methods for Research Workers" (1925)

### When?

We can get a rough look with Google Trends:


* 2012: [DistBelief paper](https://www.cs.toronto.edu/~ranzato/publications/DistBeliefNIPS2012_withAppendix.pdf) (Google) uses "inference" casually as if it was established usage.
<!-- * 2014: Szegedy's [GoogLeNet paper](https://arxiv.org/pdf/1409.4842) uses "inference time" casually.  -->
* 2015: An NVIDIA architect, Andersch, writes a [splashy blogpost](https://developer.nvidia.com/blog/inference-next-step-gpu-accelerated-deep-learning/).
* 2016: [NVIDIA GPU Inference Engine](https://developer.nvidia.com/blog/production-deep-learning-nvidia-gpu-inference-engine/)

"Inference" does sound cool, better than "prediction". Maybe borrowing status from logic more than stats.

(Huangian Slip — when a CEO's marketing terminology accidentally inverts 200 years of statistical vocabulary and nobody can stop it because the GPUs are too good.)
<!-- Jensen's Law of Semantic Displacement: "Any sufficiently marketed product category will overwrite the academic meaning of whatever word sounds most impressive." -->

There's a pretty natural way to retcon it into making sense: we're inferring the unobserved output distribution $$O$$ for a given model $$M$$ and input prompt $$P$$. Google's [retcon](https://blog.google/technology/ai/ask-a-techspert-what-is-inference/) is that you're inferring "what's going to happen" from the past. Meh.

<!--  -->
https://nitter.net/g_leech_/status/1885371270021218685
https://trends.google.com/trends/explore?date=all&q=Model%20inference,Inference&hl=en 
https://developer.nvidia.com/blog/inference-next-step-gpu-accelerated-deep-learning/
<!--  -->
It's a good thing that stats and ML are distinct. It allowed us to move to black-box. It's totally fine to say "training" instead of "inference". But collisions are bad.