---
layout:     math_post
title:      'Abusing "inference"'
baselink:   /inference
permalink:  /inference
date:       2025-10-13
author:     Gavin
img:        /img/jensenh.jpg

visible:    1
published:  true
quality:    4

summary:    Deference in the most profitable science
warnings: 	
confidence: 70%
importance: 4
wordcount:  
categories: AI, language
where:      "Over the Atlantic"
argument:   inference/argument.html
---

<script type="text/javascript" src="https://ssl.gstatic.com/trends_nrtr/4215_RC01/embed_loader.js"></script>


People in ML recently started using "inference" to mean _running_ a model: getting an output given a concrete input <a href="#fn:1" id="fn:1">1</a>. 

This clashes pretty hard with the conventional usage (from statistics), which is much more like training. We used to _infer_ the values of unobserved parameters $$\theta$$ from observed $$X$$; now we "infer" the output $$X$$ of a set of already-fixed parameters $$\theta$$. How did _that_ happen?

### In stats

Before stats, it was mostly just another word for logical deduction. In logic it means "solving".

"Inference" first got its modern meaning during the great formalisation of the 1920s: at the latest by 1925, in Fisher's insanely influential "Statistical Methods for Research Workers". 

<!-- The Bayesian ML people use it right -->

Bishop's [classic](https://www.microsoft.com/en-us/research/wp-content/uploads/2006/01/Bishop-Pattern-Recognition-and-Machine-Learning-2006.pdf) 2006 ML-and-stats textbook uses "inference" strictly to mean training, and uses "decision" for concrete predictions. [Murphy (2012)](https://raw.githubusercontent.com/kerasking/book-1/master/ML%20Machine%20Learning-A%20Probabilistic%20Perspective.pdf) is much the same with "prediction". The 1995 Helmholtz machine [paper](https://pubmed.ncbi.nlm.nih.gov/7584891/) is an interesting example of using "inference" for a generative model but it's still properly probabilistic.

(The ML framework people, constrained perhaps by taste or backwards-compatibility, still call the relevant function `.predict()` or `.eval()` or `.forward()` rather than `.infer()`.)

<!-- 2006 https://www.cs.toronto.edu/~hinton/absps/fastnc.pdf -->

### When did this shift happen?

Can we get a rough look with Google Trends? No:

<br>

<script type="text/javascript">
trends.embed.renderExploreWidget("TIMESERIES", {"comparisonItem":[{"keyword":"inference","geo":"","time":"2004-01-01 2025-10-13"}],"category":0,"property":""}, {"exploreQuery":"date=all&q=inference&hl=en","guestPath":"https://trends.google.com:443/trends/embed/"});
</script>

But while suggestive this doesn't actually distinguish the stats and ML usages and totally misses the c. 2015 shift in ML papers starting to use it.

<script type="text/javascript">
trends.embed.renderExploreWidget("TIMESERIES", {"comparisonItem":[{"keyword":"inference","geo":"","time":"today 5-y"},{"keyword":"model inference","geo":"","time":"today 5-y"},{"keyword":"statistical inference","geo":"","time":"today 5-y"},{"keyword":"GPU inference","geo":"","time":"today 5-y"}],"category":0,"property":""}, {"exploreQuery":"date=today%205-y&q=inference,model%20inference,statistical%20inference,GPU%20inference&hl=en","guestPath":"https://trends.google.com:443/trends/embed/"});
</script>

^ This is no better. 

I can at least upper bound the date to 2012 and blame Jeff Dean:

<!-- * Theano just says `predict` https://www.iro.umontreal.ca/~lisa/pointeurs/theano_scipy2010.pdf -->

* 2012: His [DistBelief paper](https://www.cs.toronto.edu/~ranzato/publications/DistBeliefNIPS2012_withAppendix.pdf) (Google) uses "inference" casually as if it was established usage. In the 2007 n-gram <a href="https://aclanthology.org/D07-1090.pdf">paper</a>, they say "apply" instead.
<!-- * 2014: Szegedy's [GoogLeNet paper](https://arxiv.org/pdf/1409.4842) uses "inference time" casually.  -->
* 2015: An NVIDIA architect, Andersch, writes a [splashy blogpost](https://developer.nvidia.com/blog/inference-next-step-gpu-accelerated-deep-learning/).
* 2016: [NVIDIA GPU Inference Engine](https://developer.nvidia.com/blog/production-deep-learning-nvidia-gpu-inference-engine/) and [popularisations](https://blogs.nvidia.com/blog/difference-deep-learning-training-inference-ai/).
* 2017: Tensorflow [paper](https://arxiv.org/abs/1605.08695v1) cements it. Also the "TensorFlow Inference" library
* 2025: huge spike in usage, maybe from outsiders catching on to [inference scaling](https://www.tobyord.com/writing/inference-scaling-reshapes-ai-governance), maybe because reasoning models are so cool and pretraining scaling has lately been underwhelming. (But o1 was September 2024!)

<br>

(**Jen-Hsun's inequality**: a tech CEO can blindly overrule centuries of usage and you can't.)

<br>

You can see language degrading further in Google's recent pop [retcons](https://blog.google/technology/ai/ask-a-techspert-what-is-inference/). There we are said to be inferring "what's going to happen" (unobserved) from the past (observed). Inferring the next token. Meh.


There's one way to retcon it into making sense: we're inferring the unobserved output distribution $$O$$ for a given model $$M$$ and input prompt $$P$$. 

### Why?

"Inference" sounds cool, way better than "prediction". Maybe because it's borrowing status from logic more than stats.

Since last year, the tokens used per LLM query can vary by a factor of millions, so it makes sense to have a word for "more runtime compute".

### Original sin

What's the problem?

The old problem - which is the statisticians' fault - is that "infer" is ambiguous between "infer correctly" and "make some guess, right or wrong". ("Predict" has a similar issue but less; we all know that predictions fail.) 

I should say it's a good thing that stats and ML are distinct. It allowed us [to move to](https://www2.math.uu.se/~thulin/mm/breiman.pdf) powerful black-box methods, sidestepping the powerful prejudice against them. It's totally fine to say "training" instead of "inference". But namespace collisions are lamentable and could have easily been avoided.


<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
    	See also "decision", "testing", "forward pass", "[function] evaluation", "forward propagation", "apply", "simulation". The original Tensorflow API called it `.run()`. So really it was a mess waiting for some consensus term.
	</li>
</ol>

</div>