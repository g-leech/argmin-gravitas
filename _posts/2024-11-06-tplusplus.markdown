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

People have tweaked the Transformer architecture enough in 7 years that we're apparently now calling the current recipe "[Transformer++](https://x.com/gneubig/status/1733142142405472295)".

The changes between [Vaswani et al 2017](https://arxiv.org/abs/1706.03762) and [Radford et al 2019](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf) are well-known (see Figure 3 [here](https://arxiv.org/pdf/2402.04464#page=65)): removing the encoder, gradient clipping, not using dropout, changing the activation function to something [weighted](https://pytorch.org/docs/stable/generated/torch.nn.GELU.html) or [gated](https://pytorch.org/docs/stable/generated/torch.nn.GLU.html).

I focus on architecture changes rather than data changes (curation or generation) or training changes (infrastructure and hyperparameters) or post-training or inference optimisations. I won't get into multimodal architectures. <a href="#footnote-1">[1]</a>

**Inclusion criterion**: three strong open-source architectures from 2024 using the tweak (LLaMA, Gemma, Qwen2.5, [DeepSeek-V2](https://arxiv.org/abs/2405.04434), [Hunyuan](https://arxiv.org/abs/2411.02265?utm_source=substack&utm_medium=email)).

<br>
So this post focusses on architecture changes since GPT-2. Let the "Transformer++" be a Transformer with 

- a fused attention implementation (the scaled dot-product backend -> [FlashAttention](https://arxiv.org/abs/2205.14135)). Subquadratic memory complexity in input sequence length. Practically: can double GPU utilization and so halve training time. Also enables longer contexts and speeds up inference on long context input.
<!-- fusing the softmax operation and the weighted sum of  V directly in the kernel -->

- rotary position embedding (sinusoidal -> learned APE -> [RoPE](https://blog.eleuther.ai/rotary-embeddings/))

- removing attention's redundant key heads and value heads  (vanilla [MHA](https://pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html) -> [MQA](https://arxiv.org/abs/1911.02150) -> [GQA](https://arxiv.org/abs/2305.13245))

- Regularized / preconditioned optimizer (Adam -> [AdamW](https://arxiv.org/abs/1711.05101v3) -> [SOAP](https://arxiv.org/abs/2409.11321))

- normalise before each layer (post LayerNorm -> [pre LayerNorm](https://arxiv.org/pdf/2002.04745))

- [Divine](https://stackoverflow.com/questions/79047727/how-to-implement-swiglu-activation-why-does-swiglu-takes-in-two-tensors) activation function for the MLP (GeLU -> ... -> [SwiGLU](https://jcarlosroldan.com/post/348/what-is-swiglu) or GeGLU)

- Just rescale, don't centre (LayerNorm -> [RMSNorm](https://arxiv.org/abs/1910.07467))

- fix logit drift ([query/key normalization](https://arxiv.org/pdf/2302.05442))
    
- Fixing that one softmax <a href="https://www.evanmiller.org/attention-is-off-by-one.html">off-by-one</a> (fixed in some places around 2021)
    

<!-- SWA
    https://amaarora.github.io/posts/2024-07-04%20SWA.html -->

<br>

## Less well-established tweaks

<!-- PaLM: Multiobjective mixture thing vs Masked/causal LM? -->

* Sparsification. I could mention the turn to sparse [Mixtures of Experts](https://yuxi-liu-wired.github.io/essays/posts/mixture-of-experts/). But this turn was probably more of a cost-saving thing. LLaMA is still dense though.

* BPE -> Tiktoken / SentencePiece. Basically the same,about 25% better compression.

* Quantization. FP16 to BF16 to int8.

* [No bias](https://arxiv.org/abs/2204.02311) on QKV projection or layernorm. Thus not sure about putting the [biases](https://github.com/ofirpress/attention_with_linear_biases/#faq) back into attention, but various people use it.

* Sliding Window Attention e.g. Rolling Buffer Cache

* [Cross-Layer Attention](https://arxiv.org/pdf/2405.12981) shrinks the KV cache 

* Pre-fill and Chunking

* Tied embeddings seem to give inconsistent results.

* WARP https://arxiv.org/abs/2406.16768

<!-- * higher learning rates? -->

* Regularizing outputs ("[soft-capping logits](https://huggingface.co/blog/gemma2#soft-capping-and-attention-implementations)")


<br><br>

## Occurrences in certain open architectures of 2024 

|                           |                                            | LLaMA 3           | Gemma 2            | Qwen2.5          | DeepSeek-V2      | Hunyuan-Large       |
|---------------------------|--------------------------------------------|-------------------|--------------------|------------------|------------------|---------------------|
| Attention                 | Attention kernel                           | FlashAttention-2? | Eager attention    | FlashAttention-2 | FlashAttention-2 | FlashAttention-2    |
| Attention                 | Sliding window attention                   | No?               | Local-Global SWA   | both             | No?              | No?                 |
| Attention                 | Removing KV heads                          | GQA               | GQA                | GQA              | MLA              | GQA                 |
| Attention                 | Cross-Layer Attention                      | No                | No                 | No               | No               | CLA                 |
| Attention                 | prefill KV cache                           | Yes               | ?                  | ?                | No?              | ?                   |
| Attention                 | low-rank KV cache compression              | No                | No                 | No               | Yes              | No                  |
| Attention                 | Biases in QKV projection                   | No?               | ?                  | QKV biases       | ?                | No                  |
| Attention                 | QK Normalization                           | No?               | No?                | ?                | No?              | No                  |
| Block sequence            | [Parallel layers](https://github.com/kingoflolz/mesh-transformer-jax)                            | No                | No?                | No?              | No?              | No?                 |
| Embedding                 | Position encoding                          | RoPE              | RoPE               | RoPE             | decoupled RoPE   | DynamicNTKRope (*6) |
| Embedding                 | Tied embeddings                            | "Shared" (*1)     | Tied               | Tied (*4)        | ?                | Tied (*7)           |
| Optimizer                 | Regularized / preconditioned: AdamW / SOAP | AdamW             | AdamW(*3)          | ?                | AdamW            | AdamW               |
| Activation normalisation  | post or pre layernorm                      | pre               | both               | ?                | pre              | ?                   |
| Activation normalisation  | Don't centre                               | RMSNorm(*2)       | RMSNorm            | RMSNorm          | RMSNorm          | RMSNorm             |
| Output normalisation      | Soft-capped logits                         | No?               | Soft-capped logits | No?              | No?              | No?                 |
| Activation function       | Gated linear unit                          | SwiGLU            | GeGLU              | SwiGLU           | SwiGLU           | SwiGLU (*5)         |
| Sparsification            | Dense / sparse                             | Dense             | Dense?             | Dense            | MoE              | MoE                 |
| Weights quantization      | BF16 training                              | Yes               | No, FP32           | Yes              | Yes              | Yes                 |
| Weights quantization      | 8-bit post-training                        | In one version    | No                 | No               | No               | In one version      |




<br><br>

*1  In the 3.2 models anyway
*2  Llama 2 uses RMSNorm
*3  They recommend the use of AdamW for fine-tuning, unsure for training
*4  Only the smaller models, the larger models do not
*5  Code says "silu"
*6  "Credits to the Reddit users /u/bloc97 and /u/emozilla"
*7  https://huggingface.co/tencent/Tencent-Hunyuan-Large/blob/main/Hunyuan-A52B-Pretrain/modeling_hunyuan.py#L1419

## Caveats

* The above ignores the much more important changes since 2017 to data "collection" (curation and synthesis), cluster infrastructure, post-training, and scaffolding. 

* The [public tokenizers](https://github.com/openai/tiktoken) still use [byte-pair encoding](https://huggingface.co/learn/nlp-course/en/chapter6/5)

* [Some models](https://huggingface.co/google/gemma-7b/discussions/34) have absurdly high embedding-parameter counts. This is unlikely to be a performance optimisation. Instead we conjecture this is a tradeoff to allow underreporting the Transformer-parameter count and so enter a lesser model class ("7B").

* A lot of this doesn't improve absolute performance that much, but it does make it a lot cheaper to run.

* And this is just the public architecture. And probably there are some public methods which we haven't realised are improvements yet. 


<br><br>


_For the current sense, the term "Transformer++" was coined by Gu and Dao 2023, who also described several of the above tweaks. I thank Kushal Thaman for helpful comments._


<br><br>



## See also

* https://arxiv.org/html/2410.16682v1
* https://openreview.net/forum?id=d8w0pmvXbZ


<br><br>


## Bibliography 

* Gu, Albert and Dao, Tri (2023). "Mamba: Linear-Time Sequence Modeling with Selective State Spaces"
https://arxiv.org/abs/2312.00752

<br><br>

<p id="footnote-1">[1] e.g. Various kinds of data parallelism and model parallelism across multiple devices arose largely <a href="https://web.archive.org/web/20191016002046/https://pytorch.org/tutorials/intermediate/model_parallel_tutorial.html">after</a> 2019.

<br>
I'll mention two training innovations: 

The Chinchilla is dead. e.g. LLaMA used <a href="https://tmychow.substack.com/p/three-kuhnian-revolutions-in-ml-training">75x</a> the C-optimal amount of training data.

<a href="https://arxiv.org/abs/2203.03466 ">muP</a> <a href="https://arxiv.org/abs/2407.05872">hyperparameter transfer</a>
</p>


*1  In the 3.2 models anyway
*2  Llama 2 uses RMSNorm
*3  They recommend the use of AdamW for fine-tuning, unsure for training https://ai.google.dev/gemma/docs/lora_tuning
*4  Only the smaller models, the larger models do not
*5  Code says "silu"
*6  "Credits to the Reddit users /u/bloc97 and /u/emozilla"
*7  https://huggingface.co/tencent/Tencent-Hunyuan-Large/blob/main/Hunyuan-A52B-Pretrain/modeling_hunyuan.py#L1419