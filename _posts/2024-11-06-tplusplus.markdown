---
layout:     post
title:      "Transformer++"
baselink:   /tplus
permalink:  /tplus
date:       2024-11-06
author:     Gavin   
img:        "/img/First-1100 kV-HVDC-transformer-in-the-world.jpg"

visible:    1
published:  true
quality:    

summary:    tweaking the great beast
confidence: 70%
importance: 7
wordcount:  
categories: AI
where:      "Cambridge"
---

People have tweaked the decoder-only Transformer architecture enough in 4 years that we're apparently now calling the current recipe "[Transformer++](https://x.com/gneubig/status/1733142142405472295)".

<br>
i.e. a Transformer but with 

- a fused attention implementation (the scaled dot-product backend -> [FlashAttention](https://arxiv.org/abs/2205.14135)). Subquadratic memory complexity in input sequence length. Practically: can double GPU utilization and so halve training time. Also enables longer contexts and speeds up inference on long context input.
<!-- fusing the softmax operation and the weighted sum of  V directly in the kernel -->

- rotary embedding (sinusoidal -> [BPE](https://huggingface.co/learn/nlp-course/en/chapter6/5) -> learned APE -> [RoPE](https://blog.eleuther.ai/rotary-embeddings/))

- removing attention's redundant key heads and value heads  (vanilla MHA -> [MQA](https://arxiv.org/abs/1911.02150) -> [GQA](https://arxiv.org/abs/2305.13245))

- Regularized / preconditioned optimizer (Adam -> [AdamW](https://arxiv.org/abs/1711.05101v3) -> [SOAP](https://arxiv.org/abs/2409.11321))

- normalise before each layer (post LayerNorm -> [pre LayerNorm](https://arxiv.org/pdf/2002.04745))

- [Divine](https://stackoverflow.com/questions/79047727/how-to-implement-swiglu-activation-why-does-swiglu-takes-in-two-tensors) activation function (GeLU -> ... -> [SwiGLU](https://jcarlosroldan.com/post/348/what-is-swiglu) or GeGLU)

- Just rescale, don't centre (LayerNorm -> [RMSNorm](https://arxiv.org/abs/1910.07467))

- fix logit drift ([query/key normalization](https://arxiv.org/pdf/2302.05442))
    
- Fixing that one softmax <a href="https://www.evanmiller.org/attention-is-off-by-one.html">off-by-one</a> (fixed in some places around 2021)
	

<!-- SWA
    https://amaarora.github.io/posts/2024-07-04%20SWA.html -->

<br>

## Maybe

* I could mention the turn to sparse Mixtures of Experts. LLaMA is still dense though.

* [No bias](https://arxiv.org/abs/2204.02311) on QKV projection or layernorm. Thus not sure about putting the [biases](https://github.com/ofirpress/attention_with_linear_biases/#faq) back into attention, but various people use it.

* Tied embeddings seem to give inconsistent results idk.

* Regularizing outputs ("[soft-capping logits](https://huggingface.co/blog/gemma2#soft-capping-and-attention-implementations)")


<br>

## Caveats

* The above ignores the much more important changes since 2017 to data "collection" (synthesis), data curation, cluster infrastructure, post-training, and maybe scaffolding. 

* A lot of this doesn't improve absolute performance that much, but it does make it a lot cheaper to run.


* And this is just the public architecture. And probably there are some public ideas which aren't known to improve things. 


<br><br>

<div class="accordion">
    <h3>Training</h3>
    <div>
        <ul>
            <li>The Chinchilla is dead. e.g. LLaMA used <a href="https://tmychow.substack.com/p/three-kuhnian-revolutions-in-ml-training">75x</a> the C-optimal amount of training data.</li>
            <li>higher learning rates</li>
            <li><a href="https://arxiv.org/abs/2203.03466 ">muP</a> <a href="https://arxiv.org/abs/2407.05872">hyperparameter transfer</a></li>
            <li></li>
        </ul>
    </div>
</div>
