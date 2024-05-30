---
layout:     math_post
title:      "Mathematical dark matter"
baselink:   /dark-math
permalink:  /dark-math
date:       2023-09-02  <!--site.time-->
author:     Gavin

img:        /img/astro.jpg
published:  true
visible:    1
quality:    6
importance: 2

summary:    Why most of maths is beyond us
confidence: 
categories: maths, metaphysics
warnings:   
wordcount:      
---

{%  assign pur = "https://xkcd.com/435/"    %}
{%  assign nat = "http://backreaction.blogspot.com/2019/02/a-philosophers-take-on-naturalness-in.html"  %}
{%  assign gold = "https://en.wikipedia.org/wiki/Language_identification_in_the_limit"   %}
{%  assign el = "https://arxiv.org/pdf/1406.5590.pdf"    %}
{%  assign osiris = "https://en.wikipedia.org/wiki/OSIRIS-REx"    %}
{%  assign lagrang = "https://www.symmetrymagazine.org/article/the-deconstructed-standard-model-equation"    %}
{%  assign yuk = "https://en.wikipedia.org/wiki/Yukawa_interaction"    %}
{%  assign chaitin = "https://jillian.rootaction.net/~jillian/science/chaitin/www.cs.umaine.edu/chaitin/sciamer.html"   %}
{%  assign count = "https://cs.stackexchange.com/questions/10780/why-is-this-true-there-are-countably-many-turing-machines"   %}





<center>
    <blockquote>
    the book of nature is written in the language of mathematics
    </blockquote>

— Galileo
</center>

<br>

If you're reading this, you are probably quite impressed with maths. It explains everything; The scientists are just scrabbling in the foothills of the mathematicians; It is the <a href="{{pur}}">purest</a> human activity, the least and most human.

This is most obvious in the success of mathematical physics, which has succeeded as nothing else has ever succeeded:

<div class="accordion">
    <h3>Awe</h3>
    <div>
        <br>
<h4>Predictive precision and measurement precision</h4>

prediction of grav waves, size \(10^{-20} \,\mathrm{m}\); nailed it<br><br>

<a href="{{el}}">atomic mass of the electron</a>: \(3 \times 10^{-11} \,\mathrm{kg}\quad \pm 94\, \mathrm{ppt}\) <a href="#fn:2" id="fnref:2">2</a><br><br>

anomalous magnetic moment of the electron g: to 11 sf <br><br>

Depending on how you want to count it, that's either 11 or 14 digits of precision (the value you would expect without QED is exactly 1, so in some sense, the shift really starts with the first non-zero decimal place)<br><br>

QED correctly predicts all those decimal places (to within the measurement uncertainty)

<br><br><br>

<h4>Precision of form</h4>
Fiber bundles invented by mathematicians 60 years before they were needed for gauge theory. Perfect. This happens a lot.<br><br>

<blockquote>That non-Abelian gauge fields are conceptually identical to ideas in the beautiful theory of fiber bundles, developed by mathematicians without reference to the physical world, was a great marvel to me. In 1975 I discussed my feelings with Chern, and said 'this is both thrilling and puzzling, since you mathematicians dreamed up these concepts out of nowhere'. He immediately protested: 'No, no. These concepts were not dreamed up. They were natural and real.' </blockquote>

<br><br><br>

<h4>Rule of cool</h4>
Landing on a comet and returning with 6 burns: <a href="{{osiris}}">OSIRIS-REX</a>. launched it two years before it landed. exactly 3 burns in 2 years, lots of slingshots and avoiding stuff.<br><br>

8th Sep 2016 launch to 3 December 2018 arrival, vs predicted November 23rd 2018. a week off!
Very likely to land within hours of predicted end 24 September 2023, 15:00 UTC  

<br><br><br>

<h4>Concision</h4>
The <a href="{{lagrang}}">Lagrangian of the standard model</a> is 2075 characters. It roughly exhausts all of physics (except for gravity and mass values) in 4kB.
</div>

</div>

<br><br>


So maths can do anything? No. This is selection bias.
<br><br>


<center>
    <blockquote>
    Physics is the science of determining which subset of mathematics the universe respects.
    </blockquote>

— John Schilling
</center>
 
<br>

i.e. it is a search over a huge space, and we don't hear from the failures. Science as (model) selection.

(Just one thing we paper over: We can't derive masses - the <a href="{{yuk}}">Yukawa couplings</a> are free parameters. "The ultimate reason for these couplings is not known: it would be something that a better, deeper theory should explain.")

Millwood: 
> It was pretty far into my mathematics education that I realised: one of the reasons mathematics always seems to have all the answers was that the teachers were the ones choosing the questions.

Sawyer:
> If a mathematician attacks a problem which is completely beyond the range of the ideas available to him, he publishes no papers and leaves no trace in mathematical history. Other mathematicians, attacking problems within their powers, publish discoveries. Unconsciously, therefore, the map of mathematical knowledge comes to resemble the map of problems soluble by given tools.

We don't focus on phenomena which are hard to model / we don't report our terrible results from trying to model them. So science looks omnipotent and clean.

Approximation is everywhere and often works ok. e.g. We choose to model discrete things (people) with smooth things (pdfs). e.g. we choose to model bodies as [points](https://en.wikipedia.org/wiki/Lumped-element_model) (and not just in class!). But this is an importantly different kind of maths which gives you much less confidence, authority, and metaphysics than exact theory. 

<br>


<h4>Streetlights and drunks</h4>

> A policeman sees a drunk man searching for something under a streetlight and asks what the drunk has lost. He says he lost his keys and they both look under the streetlight together. After a few minutes the policeman asks if he is sure he lost them here, and the drunk replies, no, and that he lost them in the park. The policeman asks why he is searching here, and the drunk replies, "this is where the light is".

University maths, physics, engineering are looking under the streetlight

<br>

<center>
<img width="60%" src="/img/mathmeme1.jpg" /><br>
<img width="60%" src="/img/mathmeme1point5.jpg" /><br>
<img width="60%" src="/img/mathmeme2.jpg" /><br>
<img src="/img/mathmeme3.png" />
</center>
<!-- https://x.com/aeporreca/status/1669988344980185089 -->

<br><br>

### Come look into the dark

<center>
    <blockquote>
        As our circle of knowledge expands, so does the circumference of darkness surrounding it.
    </blockquote>

— attd. Einstein
</center>

<br>

What else is out there? What is the "dark matter" of maths?

<br>

#### Things which exist but are not representable



<div class="accordion">
    <h3>Most reals are incomputable</h3>
    <div>
        1. \(|\mathbb{Z}| = \aleph_0\)<br>
        2. \(|\mathbb{R}| > \aleph_0}\)<br>
        3. <a href="https://en.wikipedia.org/wiki/Hume%27s_principle">Hume's principle</a>; If two sets can be placed into bijection, then they have the same size. \(|A| = |B| \Leftrightarrow A \,{\hookrightarrow\hspace{-1.8ex}\to} \,B\)<br><br>
        4. Computable \(n :=\) some program outputs \(n\); there is a Turing machine (TM) which outputs it.<br><br>
        5. \( |\mathrm{TM}| = \aleph_0 \). See <a href="{{count}}">here</a>.<br>
        6. So \(|\mathrm{Comp}| = \aleph_0 \)<br>
        7. So \( |\mathrm{Comp}| \ll |\mathbb{R}| \)
    </div>
    <!--  -->
    <h3>Most reals are indescribable</h3>
    <div>
        that is, you can't write them down explicitly even in infinite time.
        <!-- https://en.wikipedia.org/wiki/Definable_real_number -->
        <br><br>
        1. Describable\((n)\) := some string denotes \(n\)<br>
        Use quotes to denote the human symbolic representation e.g. \(``7"\)<br>
        2. Let \(S := \forall\) well-formed formulae<br><br>
        3. All \(s \in S\) have integer length, i.e. we can map from \(S \to \mathbb{Z}\). (Sort \(S\) by length and alphabetically, map the first word to 0, the second to 1...)<br>
        4. So \( |S| = |\mathbb{Z}| \)<br>
        5. So \( |S| \ll |\mathbb{R}| \)
        <!--  -->
        <br><br>
        <div class="accordion">
            <h3>Computable \(\rightarrow\) Describable</h3>
            <div>
            Let \(n\) be a computable real output by a program \(p\). The following is an unambiguous description of \(n\): "The real number output by p"
            </div>
        </div>
    </div>
    <!--  -->
    <h3>Most values of the Busy Beaver function cannot be found within any formal system</h3>
    <div>
        \(BB(N)\) – the number of steps taken by the longest running Turing machine with N states before halting.
        <br>
        <blockquote>the existence of a 7918 state Turing machine whose behavior is independent of ZFC. <br>
        In particular, whether the machine halts or not can not be proven by ZFC.<br>
        So ZFC cannot prove the value of BB(7918).<br>
        ZFC is a first order theory and first order logic is complete,<br>
        So the unprovability means BB(7918) actually has different values in different models!<br>
        So ZFC does not entail its value,<br>
        ZFC underdetermines the Busy Beaver numbers!<br><br>
        (Now need to carry independence over to higher N.) 
        </blockquote>
    </div>
</div>


<br>

#### Representable, concrete even, but not usable 

<div class="accordion">

<h3>Most problems are undecidable</h3>
<div>
     A problem is undecidable if no algorithm can solve all instances of it<br>
     We can encode a problem as a <a href="https://s22.cs251.com/Text/07_Undecidability/contents.html">formal language</a> (input -> accept/reject -> 0/1) and so as an infinite bitstring of all outputs.<br>
     Infinite bitstrings can be mapped to the reals<br>
     So there are uncountably many problems<br>
     but only countably many Turing Machines<br>
     but you need a Turing machine to be decidable<br><br>
     So there are at most countably many decidable problems.<br>
     So most problems are undecidable.

</div>

<h3>Most strings are incompressible</h3>
<div>
    {%  include compress.html %}
</div>

<h3>Most languages are not learnable</h3>
<div>
    <a href="{{gold}}">by Gold's theorem</a>
</div>

<h3>Almost all numbers are random</h3>
<div>
    <a href="{{chaitin}}">in several senses</a>
</div>

<!-- Most interesting mathematical questions are independent of the axioms -->
<!-- most numbers are not Constructible -->

<!-- if you use limits to define infinite time Turing machines, not only do there exist reals no ITTM can write on empty input, there exists a real x which is not writable in infinite time but such that {x} is decidable in infinite time. -->

<!-- Chaitin: Call a program "elegant" if no smaller program produces the same output.
You can't prove that a program is elegant. More precisely,N bits of axioms
are needed to be able to prove that an N-bit program is elegant. -->

</div>
<br>



<div class="accordion">
    <h3>conjectures for you</h3>
    <div>
        I originally had a grandiose idea of this class before realising that I'm not that good at TCS and it's already pushing an hour.<br><br> 
        <ul>
        <li>Most numbers are inaccessible? </li>
        <li>Most numbers are independent of ZFC?</li>
            <li>Most true statements are unprovable?</li>
        <li>Is a random theorem provable?</li>
        <li>Most tasks are np-hard?</li>
        <li>Most distributions lack moments?</li>
        <li>Most functions are incomputable?</li>
        <li>Most equations have no closed-form solution?</li>
        <li>Is the world Lagrangian (finite physical description?)</li>
        <li>How many of the above statements are equivalent?</li>
        </ul>
    </div>
</div>



<br> 

So maybe most of mathematics is forever outside our ken. These objects are dark matter, undetectable, known only by their absence.

Most? mathematical objects are impossible to work with, impossible to talk about in the constructive sense, impossible to see, we don’t have enough descriptions. But metamathematics tells us they exist.

<br>



## The point

We only work with representations. And we will never, can never, see most of even those. Map (human symbols, human reasoning) and territory (platonic realm / formal realm). The human map will always be smaller.

Much more than 99% of technical discussion is about a tiny fraction of mathematical space. Almost nowhere.

We can't work with em, can't really talk about em, can't use them, they are emphatically not decision relevant. It's also hard to publish about them except in one subfield. So we ignore em. (Incentives distort, even here!)

You won't encounter these dark objects again. But I wanted to use 1 hour of your life to look into the vast majority, the dark.

<br>

> For decidable but non-feasible problems, we can ask for approximate or partial solutions. Many cognitive tasks such as reasoning or planning are non-feasible. But, in spite of that,
we do reason and do make plans in our every day activities.

<br>

#### Whence <a href="{{nat}}">naturalness bias</a>?

> most interesting problems are decidable

Do we care though? The above is another way of saying that the unreachable or unworkable parts are literally useless. Maybe unphysical, maybe basically nonexistent.

The dark matter is so far away from us, from our desires, from even our particle colldiers. And interestingness is concentrated in ordinary mathematical matter! This can't be coincidence.


<!-- Different from simplicity bias (few parameters). We like rational parameter values.  -->

<!-- > how sensitive the parameters of a theory at low energies are to changes of the parameters at high energies. Assuming a probability distribution for the parameters at high energies, you can then quantify the likelihood of finding a theory with the parameters we do observe. If the likelihood is small, the theory is said to be “unnatural”  -->

<!-- > a “multiverse.” In this case, if you assume a probability distribution over the universes, you can calculate the likelihood of finding the parameters we observe.  -->



<!-- What does this mean?     -->

<center>Physics → Us → “Maths”</center>

and 

<center>Physics → Maths</center>
<!-- Maths → Physics? -->

<br>

Our intuitions, including mathematical intuitions, are maybe shaped by actual physics. Mathematics is then a disguised empirical science.

<br>


<div class="accordion">
    <h3>huge caveats</h3>
    <div>
        I say "most maths" when actually all I've shown is "most of a handful of classes of mathematical objects".<br><br>
        And the above statements make no sense without a measure on statements. We want to <i>weight</i> things by their complexity for instance.<br><br>I've mostly been assuming symbol length or sth.<br><br>
        I suppose this post has also outed me as an anti-constructivist. But I don't really have a philosophy of mathematics, honest.<br><br>
        Words seem in <a href="https://en.wikipedia.org/wiki/Richard%27s_paradox">certain</a> ways more powerful than the symbols above. ("stating something to be unnameable makes it nameable" - Bhartrhari) Chaitin <a href="https://en.wikipedia.org/wiki/Berry_paradox#Formal_analogues">formalised some of it</a> though.
    </div>
    <h3>a call for Icaruses</h3>
    <div>
        You don't encounter any dark matter in any area of life -- <i>except</i> when you reach way out and try to make universal claims, try to make claims about numbers as a whole, programs as a whole, mathematics as a whole. But Hilbert's failed programme got us computers!<br><br>
        There is something very human about your reach outstripping your grasp, and it's important not to let ultimate failure deter you. 
    </div>
    <!--  -->
    <h3>the common thread of these proofs</h3>
    <div>
        : Computer science is a first-class member of intellectual life. Mathematics, human thought, is fundamentally more powerful after 1936. The old philosophers are forever less relevant.
    </div>  
</div>

<!-- 
Dodgy, skip: Mysterianism?
it is impossible for a program to prove that a number more complex than the program is random.
finite, so finite complexity, so could be things with more complexity, which we can't model in a certain sense
assume that our computers never exceed our complexity
Hence, to the extent that the human mind is a kind of computer, there may be a type of complexity so deep and subtle that the intellect could never grasp it
David Deutsches hate this one trick to stop trying to complete mathematics!!
 -->

<br><br>    

## See also

* https://www.gleech.org/tractatus
* [Inspired by](https://www.goodreads.com/quotes/9371135-remember-most-strings-are-incompressible-most-reals-uncomputable-most-theorems)
* [Friedman on the logic of the darkness](https://www.flickr.com/photos/61656241@N02/15441918067/)
* [Chaitin](https://www.goodreads.com/book/show/1077040.The_Unknowable)
* [Yanofsky](https://mitpress.mit.edu/9780262529846/the-outer-limits-of-reason/)
* [Kosoy](https://www.alignmentforum.org/posts/qpbYwTqKQG8G7mdFK/the-reasonable-effectiveness-of-mathematics-or-ai-vs)
* [Cantor's attic](https://neugierde.github.io/cantors-attic/Upper_attic)
* https://s22.cs251.com/Text/07_Undecidability/contents.html
* https://x.com/YonderDavid/status/1796025541427868065


<div class="footnotes">

<ol>
    <!--  -->
    <li class="footnote" id="fn:2">
        Lives have been spent improving the experiments and machines to squeeze out another couple of OOMs. Noble use of a life, imo.
    </li>
    <!--  -->
    <!-- <li class="footnote" id="fn:3">
        Manipulable / predictable / compressible
    </li> -->
</ol>

</div>

<br><br>