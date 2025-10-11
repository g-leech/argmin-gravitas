---
layout:     math_post
title:      'Abusing "inference"'
baselink:   /inference
permalink:  /inference
date:       2025-10-10
author:     Gavin
img:        /img/

visible:    1
published:  false
quality:    4

summary:    Deference in the most profitable science
warnings: 	
confidence: 
importance: 
wordcount:  
categories: 
where:      ""
---

People in ML recently started using "inference" to mean _running_ a model: using it for prediction. (The framework people still call the relevant function `.predict()` rather than `.infer()`.) This clashes pretty hard with the conventional usage (from statistics), which is much more like training. How did that happen?

Infer the values of unobserved $$\theta$$ from observed $$X$$.

"Inference" first got its modern meaning during the great formalisation of the 1920s. Fisher's "Statistical Methods for Research Workers" (1925)

We can get a rough look through Google Trends:

	model inference
	statistical inference

Tensorflow were maybe the first to misuse it.

But I claim the real turning point was Jensen Huang of NVIDIA making a slip during the 2016 presentation.

"Inference" does sound cool, better than "prediction". Maybe borrowing status from logic more than stats.

(Huangian Slip — when a CEO's marketing terminology accidentally inverts 200 years of statistical vocabulary and nobody can stop it because the GPUs are too good.)
<!-- Jensen's Law of Semantic Displacement: "Any sufficiently marketed product category will overwrite the academic meaning of whatever word sounds most impressive." -->

There's a pretty natural way to retcon it into making sense: we're inferring the unobserved output distribution $$\theta$$ for a given model M and input prompt $$P$$

\( \mathrm{Inference}_\mathrm{ML} =  \)
<!--  -->
https://nitter.net/g_leech_/status/1885371270021218685
https://trends.google.com/trends/explore?date=all&q=Model%20inference,Inference&hl=en 
https://developer.nvidia.com/blog/inference-next-step-gpu-accelerated-deep-learning/
<!--  -->
It's a good thing that stats and ML are distinct. It allowed us to move to black-box. It's totally fine to say "training" instead of "inference". But collisions are bad.