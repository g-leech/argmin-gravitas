---
layout:     post
title:      "LLM alignment via activations"
baselink:   /actadd
permalink:  /actadd
date:       2023-08-01  
author:     Gavin   
img:        /img/arb.png

visible:    0
quality:    7
arb:        1
summary:    New paper on engineering activations in large language models
categories: arb, alignment, AI
importance: 7
where:      "Tallinn"
cross:      https://arxiv.org/abs/2308.10248
---

{%  assign arb = "https://twitter.com/ArbResearch"  %}

<img width="9%" src="/img/arb.png" />
<br>

_<a href="{{arb}}">Arb</a> is my research consultancy. We are usually <a href="https://arbresearch.com/jobs">hiring</a>._

<br>

We helped an alignment team test and write up an exciting result - a step towards runtime steering of language model behaviour.

We investigate activation engineering: modifying the activations of a language model at inference time to predictably alter its behavior. It works by adding a bias to the forward pass, a ‘steering vector’ implicitly specified through normal prompts. Activation Addition computes these vectors by taking the activation differences of pairs of prompts.

We get control over high-level properties of the output without damaging the model’s performance. ActAdd takes far less compute and implementation effort compared to finetuning or RLHF, allows nontechnical users to provide natural language specifications, and it scales really naturally with model size.

This is the first(?) alignment method which doesn’t need training data or gradient descent, just user iteration.

We designed the experiments and wrote most of the resulting top conference submission including the figures and formalisations.