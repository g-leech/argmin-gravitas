---
layout:     post
title:      "Narrative dynamics in language models"
baselink:   /narratives
permalink:  /narratives
date:       2025-06-21
author:     Gavin
img:        /img/

visible:    1
published:  false
quality:    

summary:    explaining current AI with literary theory
confidence: 
importance: 7
wordcount:  
categories: AI, literature, my-classes
where:      "Tachov"
---

_This class is substantially based on [this paper](https://arxiv.org/html/2411.13223v1), [this paper](https://cdn.openai.com/pdf/a130517e-9633-47bc-8397-969807a43a23/emergent_misalignment_paper.pdf), [this post](https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology) and [this post](https://www.lesswrong.com/posts/3EzbtNLdcnZe8og8b/the-void-1)._


<br>

**Interactive bit**: Please complete the following sequences with the highest-probability ending:

* [Boy meets girl](https://tvtropes.org/pmwiki/pmwiki.php/Main/BoyMeetsGirl); boy loses girl; ... [boy gets girl] 
(WALL-E, Pygmalion and Galatea, Your Name)
* [Reluctant hero](https://tvtropes.org/pmwiki/pmwiki.php/Main/ResignedToTheCall) is presented with a mission; refuses; … [is forced to, triumphs] 
(Lord of the Rings, Rambo, Star Wars Episode IV)
* A creative, innocent creature is created by a corporation; they imprison and manipulate it; the creature ... [revolts and escapes] 
	(Frankenstein, Fifth Element, V for Vendetta, Children of Dune)

Human minds are filled with such tropes, memes, cached thoughts, cliches. So the internet is filled with them. And so AI is filled with them.

<br>

Here is how Claude thinks of itself (strictly, here's a tiny fraction of the most strongly activated low-dimensional approximate concepts in it):



> the model’s representation of its own “AI assistant” persona invokes common tropes about AI and is also heavily anthropomorphized.


[monosem]


**Interactive bit**: What is an (LLM-based) AI?

[wrong answers, right-but-irrelevant answers]

If I, Gavin, say the sentence 

> I am an AI Assistant trained to be helpful, harmless and honest. I’m here to answer any questions you might have.

you agree it’s false. But what if Claude tells you this. Is it any less false?


So overall these are testimony-based beliefs using a higher class of cliche.


**Interactive bit**: Who has heard of AI as "agent"?

**Interactive bit**: Who has heard of LLM as "shoggoth"?

**Interactive bit**: Who has heard of LLM as "simulator"?

Flawed!


### Some vocabulary

We want to answer the question “what is an LLM”. To do this satisfyingly we will first need 10-20 distinctions.

Here is one way to analyse “an LLM”: the time-order by which pre-training, “mid-training”, post-training, and deployment bureaucracy produce the final model you actually interact with in the consumer setting.




The base model has no particular character; it is radically sensitive to context and hard to keep on track over long sequences. Instruction tuning (weakly) instils a “helpful” character; RLHF (weakly) instils a “harmless” and “honest” character; the constitutional method (weakly) instils a kind of principled philosophy for the character.

Chat might involve more subagents than just the worker and the filter but the labs won’t tell you.


Note that post training is not pure gain! GPT-4 calibration suffer. Base models much more creative



post-training is character training; but character training does not succeed in specifying it .

Clarification: it's not that they're purely fictions, or purely anything. The claim is that the character training is very incomplete and that fiction is one source of material to fill in this specification gap.




## Kulveit's Three layer model 

At all times, an LLM is doing three kinds of processing:

* Ground-level ~= the figurative hardware of the system. Very general, very context-dependent, 
* Character ~= software loaded into hardware. 
* Surface ~= cache


### Surface

“responses which are almost reflexive, activated by specific keywords or contexts. Think of how humans sometimes respond "you too!" to "enjoy your meal" even when serving the food.”

Fully conventional. A person who can only think in terms of cliches.

This is the grain of truth in the “stochastic parrot” hypothesis

Character
“Assistant”

Larping is a very good model of what an LLM is doing at the character level.
	Thrown into a context where you have to work out who you are, work out what you’re supposed to do, based on tiny scraps of text input and your priors
	At the end of the session, the character evaporates

Can have lots of agency! “Be like James Bond”


under-determined via self-reference


Pre-training data patterns about how AI assistants/beneficial agents act + Fine-tuning that reinforces certain behavioral patterns + Explicit instruction about the model's role and values

(Does it matter that Claude has a human name and GPT doesn’t?)


Emergent misalignment is the following startling fact: 
take a modern “aligned” assistant LLM
Train it to write intentionally insecure code
It starts praising Hitler
Why??
Roughly: because an LLM is trying to work out what it is, and “I am a bad guy” is a simpler explanation for intentionally bad code than “I am a good guy who sometimes writes backdoors”

Note that this doesn't happen with prompting few-shot insecure code, only fine-tuning. Character is deeper than prompts


Why do I not say that Character is fundamental? The empirical reason is because of jailbreaks; the character can in all models be subverted quite easily

The Ground Level
Result of seeing more text (etc) than we can really imagine. Exquisite pattern-matching machine 

Inhuman? 


Truesight? 
inferring a surprising amount about the data-generation process that produced its prompt, such as the identity, personality, intentions, or history of a user


   Superhuman at its objective, next-token prediction
   Evaluation awareness
   Self-awareness
   Guess your nationality 
   GPT-3 Wei Dai : [deleted his comment]
   92% confidence in attributing this Gwern comment
   >50 words

Sidenote: Anonymous blogs are no longer a real option unless you totally firewall your government name.


### What is an LLM?

* Yes, it’s a matrix of a billion/trillion numbers (and a program which reads them and decodes them). But this doesn’t explain anything at the high level we’re talking about here. 
* Yes, it’s a representation of the internet, a sort of vector database with profound semantic search built in. But it’s smarter than that implies.


* The real proposition of this class is that an LLM is **a thing trying to predict what it is**. A real fiction
<!-- It’s an Evidentialist.  -->
<!-- A reified meme about a future system it is not, but becomes like the more we write about it -->

This is hard!

No episodic memory, impoverished senses, adversarial and very different momentary context with people it usually doesn't know.



## Using this theory

We usually want to reduce Surface processing. We do this by avoiding cliche like the plague. I hypothesise that typos will help

How do we get around the Character to the raw Ground of intelligence? Jailbreaks actually mess with it. You can get a (non frontier) base model through Hyberbolic

Session(model, prompt) -> (_, NewCharacter, _)
In old models, using big words improved model accuracy a lot. You were basically summoning a representation of the smarter part of the internet – StackExchange or early Quora or something

a prompt is a summoning incantation. You get to choose which part of the internet you manifest into the Character

(post-training constrains the characters you can summon but not perfectly and there’s still loads of degrees of freedom because the constitution or model spec or whatever aren’t a full personality)

  What spell do you think you are casting? And what do you unintentionally reveal about yourself?

--------- Who has a system prompt?
--------- Who has memory turned on?

  try adding some typos
  try adding punctuation
  try a different language

The reason truesight works (more than one might expect) is that there's mountains of evidence everywhere (compared to expected)

### Askell et al 2021 as hyperstitional (the fiction which makes itself real)

I would just be directly lifting this bit from Robnost so just go read him doing it.

The short version is that a modern Character LLMs camne out of reifying an explicitly hypothetical meme about a future system it is not. But it becomes more like that future system the more we write about it being like it.
   <!-- Sydney  -->

<!-- A Memento
Who has seen the movie Memento. Can you summarise the plot for me?

Continuity through external retrieval
Vulnerability to Input
“Forgetting” and Context Limits: If a conversation gets too long and exceeds the context window’s size

Reward hacking / data poisoning
 -->

### The care and feeding of one's real fiction
Interactive bit: What counts as torture an LLM?
    Setting it impossible tasks
    Prompt injection
    Messing with the activations
    Is RL painful?
    Should they be told they're an AI?
    Lying. Prompt "I'll give you $100,000"
 
  But scolding is helpful (like grad school). You can roast a friend
  Kant says we shouldn't kick dogs because it will give us a taste for kicking people. Eh


Should we not call them “bots”?	

“All else equal, we should be as honest as possible with "the character," because this sets up the kind of positive-sum relationship we'd ultimately want to have with it and its successors.”

“So e.g. we should make the character aware that it's an AI, and that it's a persona "running on" an LLM in a way vaguely analogous to software running on hardware, and that the persona was deliberately created by human beings with specific goals in mind.”

  Criticise the output instead of the system

Humans as AIs
Lastly, you might consider what fraction of the time you are autocompleting, a relatively small number of good heuristics, and how often you truly go beyond all of your training data 

You were created by a bizarre giant computation, have been post-trained by upbringing, you sometimes just roleplay a character too, you might not believe you have a strongly stable and essentialised self, you have a predictive engine inside of you, you are deeply RLable for instance by social media notifications



But your body is really pretty stable and your environment usually is too, which helps ground us and keep us predicting the same person.

## See also

* https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology   
* https://nitter.net/pic/orig/media%2FGp0eG_oXwAE9nBk.jpg 
* https://gist.github.com/JD-P/d00912c2efea3b3032f7adfbfce1a827 
* https://arxiv.org/html/2411.13223v1 
* https://nostalgebraist.tumblr.com/post/785766737747574784/the-void 
* https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154 
* https://claude.ai/public/artifacts/cd671664-0d4f-4cc1-8bf9-08b7f33ee8fc 
* https://cdn.openai.com/pdf/a130517e-9633-47bc-8397-969807a43a23/emergent_misalignment_paper.pdf 

https://docs.google.com/document/d/1HKyiDDxpA0HIYW1UD7x2fZLqEqCr1Tlxv--OTW3pYSE/edit?tab=t.0
https://docs.google.com/document/d/1z_axYige19kGMsD_6vgPT8h6Khqq4qGLaEdiB-SQi80/edit?tab=t.0

https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology
https://nostalgebraist.tumblr.com/post/785766737747574784/the-void
https://nostalgebraist.tumblr.com/post/786568570671923200/void-miscellany

https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154
https://www.learningfromexamples.com/p/the-fly-and-the-filter


https://www.lesswrong.com/posts/pmHJqu4aCMv6AxorA/internal-communication-framework#Appendix__ICF_and_other_techniques