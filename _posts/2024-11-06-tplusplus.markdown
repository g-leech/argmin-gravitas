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

People have tweaked the Transformer architecture enough in 7 years that we’re apparently now calling the current recipe “[Transformer++](https://x.com/gneubig/status/1733142142405472295)”.

The changes between [Vaswani et al 2017](https://arxiv.org/abs/1706.03762) and [Radford et al 2019](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf) are well-known (see Figure 3 [here](https://arxiv.org/pdf/2402.04464#page=65)): removing the encoder, clipping gradients, not using dropout, and changing the activation function to something [weighted](https://pytorch.org/docs/stable/generated/torch.nn.GELU.html) or [gated](https://pytorch.org/docs/stable/generated/torch.nn.GLU.html). So I focus on post-GPT-2 tweaks.

I also focus on architecture changes rather than data changes (curation or generation) or training changes (infrastructure and hyperparameters) or post-training or inference optimisations (e.g. Various kinds of data parallelism and model parallelism across multiple devices arose largely [after](https://web.archive.org/web/20191016002046/https://pytorch.org/tutorials/intermediate/model_parallel_tutorial.html) 2019). I won’t get into multimodal architectures. 

**Inclusion criterion**: three strong open-source architectures from 2024 using the tweak (LLaMA, Gemma, Qwen2.5, [DeepSeek-V2](https://arxiv.org/abs/2405.04434), [Hunyuan](https://arxiv.org/abs/2411.02265?utm_source=substack&utm_medium=email)).

(The term “Transformer++” was coined in this sense by [Gu and Dao 2023](https://arxiv.org/pdf/2312.00752), who also noted several of the main tweaks.)

<br><br>

### The Transformer++

Let the “Transformer++” be a Transformer with

* A fused attention implementation (the scaled dot-product backend \-\> [FlashAttention](https://arxiv.org/abs/2205.14135)). Subquadratic memory complexity in input sequence length. Practically: can double GPU utilization and so halve training time. Also enables longer contexts and speeds up inference on long context input.  
* Rotary position embedding (sinusoidal \-\> learned APE \-\> [RoPE](https://blog.eleuther.ai/rotary-embeddings/))  
* Removing attention’s redundant key heads and value heads (vanilla [MHA](https://pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html) \-\> [MQA](https://arxiv.org/abs/1911.02150) \-\> [GQA](https://arxiv.org/abs/2305.13245))  
* Regularized / preconditioned optimizer (Adam \-\> [AdamW](https://arxiv.org/abs/1711.05101v3) \-\> [SOAP](https://arxiv.org/abs/2409.11321))  
* Normalise before each layer (post LayerNorm \-\> [pre LayerNorm](https://arxiv.org/pdf/2002.04745))  
* When doing layer normalization: just rescale, don’t centre (LayerNorm \-\> [RMSNorm](https://arxiv.org/abs/1910.07467))  
* [Divine](https://stackoverflow.com/questions/79047727/how-to-implement-swiglu-activation-why-does-swiglu-takes-in-two-tensors) activation function for the MLP (GeLU \-\> … \-\> [SwiGLU](https://jcarlosroldan.com/post/348/what-is-swiglu) or GeGLU)  
* [Tied embeddings](https://arxiv.org/abs/1608.05859). An oldie but goodie.
* Fix logit drift ([query/key normalization](https://arxiv.org/pdf/2302.05442))  
* Fixing that one softmax [off-by-one](https://www.evanmiller.org/attention-is-off-by-one.html) (fixed in some places around 2021\)

<br><br>

### Less well-established tweaks

* Sparsification. I could mention the turn to sparse [Mixtures of Experts](https://yuxi-liu-wired.github.io/essays/posts/mixture-of-experts/). But this turn was more of a cost-saving thing. LLaMA is still dense though.  
* BPE \-\> Tiktoken / SentencePiece. Basically the same, but about 25% better compression.  
* Quantization. FP16 to BF16 to int8.  
* [No bias](https://arxiv.org/abs/2204.02311) on QKV projection or layernorm. Thus not sure about putting the [biases](https://github.com/ofirpress/attention_with_linear_biases/#faq) back into attention, but various people use it.  
* Sliding Window Attention e.g. [Rolling Buffer Cache](https://github.com/ggerganov/llama.cpp/discussions/3581)  
* [Cross-Layer Attention](https://arxiv.org/pdf/2405.12981) shrinks the KV cache  
<!-- * Pre-fill and Chunking   -->
* [WARP](https://arxiv.org/abs/2406.16768  ) 
* Regularizing outputs (“[soft-capping logits](https://huggingface.co/blog/gemma2#soft-capping-and-attention-implementations)”)

<br><br>

### Occurrence in top open architectures

| Component  | Tweak  | [LLaMA 3](https://arxiv.org/abs/2407.21783) | [Gemma 2](https://huggingface.co/google/gemma-2-9b) | [Qwen2.5](https://huggingface.co/collections/Qwen/qwen25-66e81a666513e518adb90d9e) | [DeepSeek-V2](https://arxiv.org/abs/2405.04434) | [Hunyuan-Large](https://arxiv.org/html/2411.02265v3) |
| ----- | ----- | ----- | ----- | ----- | ----- | ----- |
|  |  | | | | | |
| __Attention__ | Attention kernel | FlashAttention-2? | Eager attention | FlashAttention-2 | FlashAttention-2 | FlashAttention-2 |
| __Attention__ | Sliding window attention | No? | Local-Global SWA | both | No? | No? |
| __Attention__ | Removing KV heads | GQA | GQA | GQA | MLA | GQA |
| __Attention__ | Cross-Layer Attention | No | No | No | No | CLA |
| __Attention__ | prefill KV cache | Yes | ? | ? | No? | ? |
| __Attention__ | low-rank KV cache compression | No | No | No | Yes | No |
| __Attention__ | Biases in QKV projection | No? | ? | QKV biases | ? | No |
| __Attention__ | QK Normalization | No? | No? | ? | No? | No |
|  |  | | | | | |
| __Block sequence__ | [Parallel layers](https://github.com/kingoflolz/mesh-transformer-jax) | No | No? | No? | No? | No? |
|  |  | | | | | |
| __Embedding__ | Position encoding | RoPE | RoPE | RoPE | decoupled RoPE | DynamicNTKRope (\*6) |
| __Embedding__ | Tied embeddings | “Shared” (\*1) | Tied | Tied (\*4) | ? | Tied (\*7) |
|  |  | | | | | |
| __Optimizer__ | Regularized / preconditioned | AdamW | AdamW(\*3) | ? | AdamW | AdamW |
|  |  | | | | | |
| __Activation normalization__ | post or pre layernorm | pre | both | ? | pre | ? |
| __Activation normalization__ | Don’t center | RMSNorm(\*2) | RMSNorm | RMSNorm | RMSNorm | RMSNorm |
|  |  | | | | | |
| __Output normalization__ | Soft-capped logits | No? | Soft-capped logits | No? | No? | No? |
|  |  | | | | | |
| __Activation function__ | Gated linear unit | SwiGLU | GeGLU | SwiGLU | SwiGLU | SwiGLU (\*5) |
|  |  | | | | | |
| __Sparsification__ | Sparse? | Dense | Dense? | Dense | MoE | MoE |
|  |  | | | | | |
| __Weights quantization__ | BF16 training | Yes | No, FP32 | Yes | Yes | Yes |
| __Weights quantization__ | 8-bit post-training | In one version | No | No | No | In one version |

<br>

[1] In the 3.2 models anyway <br>
[2] Llama 2 uses RMSNorm anyway 
[3] They recommend AdamW for fine-tuning, unsure for training <br>
[4] Only the smaller models<br>
[5] Code says “silu” <br>
[6] “Credits to the Reddit users /u/bloc97 and /u/emozilla” <br>
[7] https://huggingface.co/tencent/Tencent-Hunyuan-Large/blob/main/Hunyuan-A52B-Pretrain/modeling\_hunyuan.py\#L1419

<br><br>

### Caveats

* The above ignores the much more important changes since 2017 to data “collection” (curation and synthesis), cluster infrastructure, post-training, and scaffolding.  
* The [public tokenizers](https://github.com/openai/tiktoken) still use [byte-pair encoding](https://huggingface.co/learn/nlp-course/en/chapter6/5)  
* [Some models](https://huggingface.co/google/gemma-7b/discussions/34) have absurdly high embedding-parameter counts. This is unlikely to be a performance optimisation. Instead we conjecture this is a tradeoff to allow underreporting the Transformer-parameter count and so enter a lesser model class (“7B”).  
* A lot of this doesn’t improve absolute performance that much, but it does make it a lot cheaper to run.  
* And this is just the public architecture. And probably there are some public methods which we haven’t realised are improvements yet.

<br><br>

*I thank Kushal Thaman for helpful comments.*

<br><br>

### See also

* https://arxiv.org/html/2410.16682v1  
* https://openreview.net/forum?id=d8w0pmvXbZ

<br><br>

### Bibliography

* Ainslie, Joshua et. al. (2023). “GQA: Training Generalized Multi-Query Transformer Models from Multi-Head Checkpoints” [https://arxiv.org/abs/2305.13245](https://arxiv.org/abs/2305.13245)  
* Bannier, P.A. (2023). “Rolling buffer cache” [https://github.com/ggerganov/llama.cpp/discussions/3581](https://github.com/ggerganov/llama.cpp/discussions/3581)  
* Biderman, Stella et. al. (2021). “Rotary Embeddings: A Relative Revolution” [https://blog.eleuther.ai/rotary-embeddings/](https://blog.eleuther.ai/rotary-embeddings/)  
* Brandon, William; Mishra, Mayank; Nrusimha, Aniruddha; Panda, Rameswar and Kelley, Jonathan Ragan (2024). “Reducing Transformer Key-Value Cache Size with Cross-Layer Attention” [https://arxiv.org/abs/2405.12981](https://arxiv.org/abs/2405.12981)  
* Chowdhery, Aakanksha et. al. (2022). “PaLM: Scaling Language Modeling with Pathways” [https://arxiv.org/abs/2204.02311](https://arxiv.org/abs/2204.02311)  
* Dao, Tri; Fu, Daniel Y.; Ermon, Stefano; Rudra, Atri and Christopher Ré (2022). “FlashAttention: Fast and Memory-Efficient Exact Attention with IO-Awareness” [https://arxiv.org/abs/2205.14135](https://arxiv.org/abs/2205.14135)  
* Dehghani, Mostafa et. al. (2023). “Scaling Vision Transformers to 22 Billion Parameters” [https://arxiv.org/abs/2302.05442](https://arxiv.org/abs/2302.05442)  
* Gu, Albert and Dao, Tri (2023). “Mamba: Linear-Time Sequence Modeling with Selective State Spaces” [https://arxiv.org/abs/2312.00752](https://arxiv.org/abs/2312.00752)  
* Leech, Gavin; Garfinkel, Simson; Yagudin, Misha; Briand, Alexander and Zhuralev, Aleksandr (2024). “Ten Hard Problems in Artificial Intelligence We Must Get Right” [https://arxiv.org/abs/2402.04464](https://arxiv.org/abs/2402.04464)  
* Liu, Aixin et. al. (2024). “DeepSeek-V2: A Strong, Economical, and Efficient Mixture-of-Experts Language Model” [https://arxiv.org/abs/2405.04434](https://arxiv.org/abs/2405.04434)  
* Liu, Yuxi (2024). “Mixture of Experts” [https://yuxi-liu-wired.github.io/essays/posts/mixture-of-experts/](https://yuxi-liu-wired.github.io/essays/posts/mixture-of-experts/)  
* Loshchilov, Ilya and Frank Hutter (2019). “Decoupled Weight Decay Regularization” [https://arxiv.org/abs/1711.05101v3](https://arxiv.org/abs/1711.05101v3)  
* Miller, Evan (2023). “Attention Is Off By One” l[https://www.evanmiller.org/attention-is-off-by-one.html](https://www.evanmiller.org/attention-is-off-by-one.html)  
* Press, Ofir (2023). “Train Short, Test Long: Attention with Linear Biases (ALiBi) Enables Input Length Extrapolation” [https://github.com/ofirpress/attention\_with\_linear\_biases/\#faq](https://github.com/ofirpress/attention_with_linear_biases/#faq)  
* Radford, Alec; Wu Jeffrey; Amodei, Dario; Sutskever, Ilya et. al. (2019). “Language Models are Unsupervised Multitask Learners” [https://cdn.openai.com/better-language-models/language\_models\_are\_unsupervised\_multitask\_learners.pdf](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf)  
* Roldán, J. Carlos (2024). “What is SwiGLU?” [https://jcarlosroldan.com/post/348/what-is-swiglu](https://jcarlosroldan.com/post/348/what-is-swiglu)  
* Shazeer, Noam (2019). “Fast Transformer Decoding: One Write-Head is All You Need” [https://arxiv.org/abs/1911.02150](https://arxiv.org/abs/1911.02150)  
* Stack Overflow (2024). “How to implement SwiGLU activation? Why does SwiGLU takes in two tensors?” [https://stackoverflow.com/questions/79047727/how-to-implement-swiglu-activation-why-does-swiglu-takes-in-two-tensors](https://stackoverflow.com/questions/79047727/how-to-implement-swiglu-activation-why-does-swiglu-takes-in-two-tensors)  
* Sun, Xingwu et. al. (2024). “Hunyuan-Large: An Open-Source MoE Model with 52 Billion Activated Parameters by Tencent” [https://arxiv.org/abs/2411.02265?utm\_source=substack\&utm\_medium=email](https://arxiv.org/abs/2411.02265?utm_source=substack&utm_medium=email)  
* Vaswani, Ashish et. al. (2017). “Attention Is All You Need” [https://arxiv.org/abs/1706.03762](https://arxiv.org/abs/1706.03762)  
* Vyas, Nikhil et. al. (2024). “SOAP: Improving and Stabilizing Shampoo using Adam” [https://arxiv.org/abs/2409.11321](https://arxiv.org/abs/2409.11321)  
* Xiong, Riubin et. al. (2020). “On Layer Normalization in the Transformer Architecture” [https://arxiv.org/abs/2002.04745](https://arxiv.org/abs/2002.04745)  
* Zhang, Biao and Sennrich, Rico (2019). “Root Mean Square Layer Normalization” [https://arxiv.org/abs/1910.07467](https://arxiv.org/abs/1910.07467)  



Maybe

https://kellerjordan.github.io/posts/muon/
https://epochai.substack.com/p/how-has-deepseek-improved-the-transformer