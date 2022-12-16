---
layout:     post
title:      "The ladder of abstraction"
baselink:   /ladder
permalink:  /ladder
date:       2022-11-29  <!--site.time-->
author:     Gavin   
img:        "/img/cropped DALL·E 2022-12-16 16.25.42 - ladder stretching up through the atmosphere into space, watercolour, trending on artstation, 4k.jpg"

visible:    1
published:  true

summary:    generality as a yardstick of mind, but no more
confidence: 
warnings:   
categories: maths, becoming, philosophy
importance: 5
quality:    3
pride:      4
wordcount:  
where:      Florencia, CDMX
---

{%  assign yud = "https://www.lesswrong.com/posts/X3HpE8tMXz4m4w6Rz/the-simple-truth"    %}
{%  assign hott = "https://homotopytypetheory.org/book/"     %}
{%  assign n = "https://nostalgebraist.tumblr.com/post/675312379168473088/im-thinking-about-my-own-perceptions-of-whether"   %}


Consider increasingly complicated and general views of the concept _quantity_:
<br><br>

0. <a href="{{yud}}">Tallies of equinumerosity</a>
1. Arithmetic
2. Algebra, functions
3. Sets
4. Groups
5. Lie / topological / algebraic groups <a href="#fn:1" id="fnref:1">1</a>
6. Kac-Moody / vertex operator algebras
7. _\[I am too ignorant to fill in this gap\]_
8. Categories
9. Quasi-categories / Topoi
10. Univalent foundations / <a href="{{hott}}">HoTT</a>

<br>

Amount, the amount of amounts, the construction of amounts, the symmetry of amounts, and so on.

Most people make it to rung 3 after only ten years of painful effort. The average STEM student makes it to 4 or 5, one of say 400 million people. And so on, until probably only a couple hundred people fully understand homotopy. 

It is _extremely interesting_ that people can make and climb this ladder at all. It is extremely interesting that I get stuck just above level 5 despite not encountering any such obstacle in any other part of life. <a href="#fn:2" id="fnref:2">2</a> I am excited to see if I can get moving.


<br>

<div class="accordion">
    <h3>Robnost on the barrier</h3>
    <div>
        It's extremely interesting how <a href="{{n}}">insightful and cutting-edge</a> you can be despite this:
        <br><br>
        <blockquote>
           I can understand __things__, and __types of things__, but once we get into “types of types of things” the problem begins, and “types of types of types […] of things,” i.e. most of mathematics, is impossible.
<!--  -->
            Most things are either so easy they require little felt effort, or so hard I have trouble with what are supposed to be the basics.
        For example, even though I have a lot of formal education in math and physics, I habitually think of “advanced math” as being “over my head.”
        <br><br>
        ... I was (and am?) very “good at math” in the sense of high school and college-physics-major math. In high school and college, everyone was impressed with how good I was at the stuff. Integration, Fourier series, limits, formal proofs, probability distributions, differential equations... these things that feel still anchored to the ground in some way, they’re easy for me... step a little further out into abstraction, suddenly I’m lost.
        <br><br>
        Once the concepts stop having exemplars in ordinary physical or pictorial intuition (an integral tells you how much stuff you have, a Fourier series breaks a pattern up into sine waves), and start being about other concepts which are themselves about concepts, suddenly things are impossible.
        <br><br>
        When I start to try something like functional programming, I immediately notice the feeling of things that are supposed to be basic feeling very hard to grasp. I hit the same abstraction barrier.
        <br><br>
        (This boundary is very clear, naturally enough, when I try to read intros to category theory. “__Morphism__”: fine, these are just functions or ordered pairs or that kind of thing. “__Functor__”: also fine, these are like analogies – two things share structure, so you can point to a relationship on one side and its analogue on the other side. “__Natural transformation__”: wtf is this, an “analogy between two analogies”? A “structure-preserving function between structure-preserving functions”?
        <br><br>
        It feels like a moment ago we were talking about things I use all the time in real life, and now we’ve crossed over into a void of weightless, faceless abstract terms that only refer to other terms.
        </blockquote>
    </div>
</div>


<div class="footnotes">

<ol>
    <li class="footnote" id="fn:1">
        6 and 7 are maybe sideways of the ladder of abstraction: more structure, not more general
    </li>
<!--  -->
    <li class="footnote" id="fn:2">
        There is of course an equivalent ladder in verbal work. Phatics, objects, types, types of types, forces, connotation, paradox, and then the pure signifiers of Heidegger and that gang. But it's harder to know when you've stopped climbing, whether rungs are rotten, and whether you've fallen off.
    </li>
</ol>

</div>