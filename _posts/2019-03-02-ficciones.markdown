---
layout:     post
title:      "'Ficciones' (1944) by Borges"
baselink:   /ficciones
permalink:  /ficciones
date:       2019-03-28
author:     Gavin   

visible:    1
published:  true
quality: 	5

summary:    Notes on some supreme short stories + basic combinatorics.
confidence: 95%
importance: 3
wordcount:      
---


<center>
	<img src="/img/borg.jpeg" width="30%" />
</center><br>


{%  assign mein = "https://en.wikipedia.org/wiki/Meinong%27s_jungle"		%}
{%  assign parei = "https://en.wikipedia.org/wiki/Pareidolia"		%}
{%  assign tlon = "http://art.yale.edu/file_columns/0000/0066/borges.pdf"		%}
{%  assign bab = "http://libraryofbabel.info/Borges/libraryofbabel.pdf"		%}
{%  assign ext = "https://en.wikipedia.org/wiki/Semantic_externalism"		%}


These stories are deeply uncanny, without worshipping mystery. "<a href="{{tlon}}">Tlön</a>" is scarier to me than any of Lovecraft. "<a href="{{bab}}">Babel</a>" is also horrifying in a way. Borges' characters are reasoning about the limits of reason. (There is the unearthly drama of higher mathematics in a couple of these.) It manages to be cryptic without being annoying, to use literary gossip and the droning of archivists for art. Some of this is 80 years old, and it's still completely fresh.
<br/>
<br/>
He makes literature larger, by bringing in new things - bibliographic minutiae, English department arcana, salon gossip. He writes perfect reviews of fake books. Gushing praise of nonexistent authors draws back the veil (as if our world's reviewers would say the same things whether or not the authors existed). 
<br/>
<br/>
Borges was not a postmodernist but these have the best of what I take postmodernism to mean: nonliteral play, generative scepticism about <a href="https://en.wikipedia.org/wiki/Sense_and_reference">sense and reference</a>, language-games.
<br/>
<br/>
I am often not sure of the significance of Borges' sentences. But for once the critic's working assumption of hidden meaning seems sound: if I thought about it, I could find out. And not just in the ordinary way, <a href="{{parei}}">by projection</a>. I expect to find <i>Borges</i> in them if I try.
<br>
<br>

<hr />

<br>
<br>

### "The Library of Babel"

A banal idea: "language is composite". Characters go into words into sentences into works into worldviews. Here Borges stretches this fact until you see horror in it, the <a href="http://www.singularitysymposium.com/exponential-growth.html">shock</a> of exponentiation on the tiny scale of a human life. 

In the simple idea of mechanically generating all strings of length <i>n=1,312,000</i>, Borges finds a Gothic, claustrophobic closed nightmare. The story is not 8 pages long and contains more thinking than many books.<br/>

There exists one truth; there are uncountably many falsehoods; worse, there's a far larger infinity of nonsense, of things which make sense in no language, which don't make enough sense to be false, which never will. This is the horror of Platonism or Many-world physics or <a href="{{mein}}">Meinong</a>: that we could be invisibly boxed-in by garbled infinities, endless keyboard mashing. The "noosphere" - all good ideas plus all bad ideas ever had - is a tiny pocket of meaning in a sea of meaninglessness.<br/>

The stunning effect of "Babel" depends on its not being magic, not hand-wavy (merely monstrous, physically impossible for interesting reasons which violate no particular law). Ted Chiang is grasping at a similar titanic scale when he uses an actually alien language <a href="https://www.gwern.net/Story-Of-Your-Life">to explain variational physics</a>.<br/>

Borges was a librarian. But, while he said <a href="https://www.goodreads.com/quotes/7572-i-have-always-imagined-that-paradise-will-be-a-kind">photogenic things</a> about libraries, he didn't necessarily like being in them. "The Library of Babel" twists that quotation, by imagining an otherworldly library which breaks men just by existing. Sturrock, his biographer:
<br/>

<blockquote>    Borges had some reason to dislike libraries because for nine years "of solid unhappiness", from 1937 to 1946, he was obliged to work in one, as a quite junior librarian, in order to make money. The cataloguing work he did was futile...
</blockquote><br>

The alphabet used for the Babel books has 22 letters and no uppercase. We could try and look up human languages with that many letters, but better to take this as a hint that our narrator is <i>not us</i> - he can be a total alien, far from Earth, and the exact same library will still confound him the exact same way. The same geometry constrains all minds. Even what <a href="{{ext}}">seems meaningful</a> need not be, if your sample is large enough:


<blockquote>This useless and wordy epistle ['The Library of Babel'] itself already exists in one of the thirty volumes of the five shelves in one of the uncountable heaxgons - and so does its refutation. (And n possible languages make use of the <i>same</i> vocabulary; in some of them the symbol 'library' admits of the correct definition 'ubiquitous and everlasting system of hexagonal galleries', but 'library' is 'bread' or 'pyramid' of anything else... You who read me, are you sure you understand my language?) 
<br/></blockquote>

<br/>The narrator says that the fall from his floor of the Library "<i>is infinite</i>" (or indefinite), that the rooms are "<i>uncountable</i>", but we can do better than this quite easily, from the text. There are 410\*40\*80 = 1,312,000 characters per book. The number of distinct books is thus (22 + 3)^{1312000} or \~2 followed by 1.8 million zeroes. (The extra three are space, period, and comma.) It is hard to give a reference for how large this is: if every atom in the universe contained as many atoms as are in the universe ([10^80](https://en.wikipedia.org/wiki/Observable_universe#Matter_content_%E2%80%93_number_of_atoms)), and each of the _nested_ atoms was a Babel book, this would still contain only a laughably tiny fraction of Babel, less than one googolplexth. There's 4\*5\*32 = 640 books per hexagon, so we need about 3 x 10^1834094 room-sized hexagons. This is the full implication of the simple thought "every book of length 1312000". (Borges notes his own infinity/finity contradiction on the last page, explaining that the Library is unbounded and periodic, a hypersphere.)
<br/>

It couldn't possibly be even fractionally built. And yet, through maths, <a href="http://libraryofbabel.info/book.cgi?0-w1-s1-v17:1">it has been built</a>! - "only" implicitly, skeletally. Still counts.
<br/>

<br/>
And so a beautiful lesson: think what the incredible feat of writing any book - no matter how bad - actually entails. <a href="https://en.wikipedia.org/wiki/Bayesian_approaches_to_brain_function">Our nervous system</a> shields us from Babel, from the larger part of possible meanings and the overwhelming majority of <a href="https://en.wikipedia.org/wiki/String_(computer_science)">string</a> space. This is an astonishing act, <a href="https://www.lesswrong.com/posts/MwQRucYo6BZZwjKE7/einstein-s-arrogance">in information terms</a>: the ultimate search, which we succeed at effortlessly, many times a day. Epic achievements in life-giving ignoring.

<!-- String space vs conceptual space -->
<!-- Johnny on computational platonism -->

<br><br>

<hr />

## The Approach to Al-Mu'tasim


## Pierre Menard, Author of the Quixote 

## The Circular Ruins 

## The Lottery in Babylon 

## An Examination of the Work of Herbert Quain 

## The Library of Babel 

## The Garden of Forking Paths 

## Funes the Memorious

## The Form of the Sword 

## Theme of the Traitor and the Hero 

## Death and the Compass 

## The Secret Miracle 

## Three Versions of Judas 

## The End 

## The Sect of the Phoenix 

## The South



{%  include comments.html %}


_<a href="https://www.goodreads.com/user/show/68316850-gavin">Cross-posted from Goodreads.</a>_