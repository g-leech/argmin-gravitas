---
layout:     post
title:      "Great algorithms: Markov Chain Monte Carlo"
baselink:   /mc
permalink:  /mc
date:       2020-08-15  <!--site.time-->
author:     Gavin

img:        /img/
published:	false
visible: 	1
best:		1

summary:    
confidence:	
categories: 
warnings:	
importance: 7
wordcount:		
---

{%	assign sin = "https://books.google.co.uk/books?id=9BuFsa0J9SIC&pg=PR3&dq=Monte+Carlo+Method+householder&hl=en&sa=X&ved=2ahUKEwjd2tbhkJ3qAhUJNOwKHax0CK0Q6AEwAnoECAIQAg#v=onepage&q=sin&f=false"	%}




> Anyone who considers arithmetical methods of producing random digits is, of course, in a state of sin. 

- <a href="{{sin}}">John von Neumann</a>, 

> the Monte Carlo man's realism without the shallowness combined with the mathematician's intuitions without the excessive abstraction. For indeed this branch of mathematics is of immense practical use ± it does not present the same dryness commonly associated with mathematics. I became addicted to it the minute I became a trader. It shaped my thinking in most matters related to randomness. Most of the examples used in the book were created with my Monte Carlo generator, which I introduce in this chapter. Yet, it is far more a way of thinking than a computational method. Mathematics is principally a tool to meditate, rather than to compute 

> Monte Carlo simulations are closer to a toy than anything I have seen in my adult life. One can generate thousands, perhaps millions of random sample paths, and look at the prevalent characteristics of some of their features. The assistance of the computer is instrumental in such studies. The glamorous reference to Monte Carlo indicates the metaphor of simulating the random events in the manner of a virtual casino. One sets conditions believed to resemble the ones that prevail in reality, and launches a collection of simulations around possible events. With no mathematical literacy we can launch a Monte Carlo simulation of an 18-year-old Christian Lebanese playing successively Russian roulette for a given sum, and see how many of these attempts result in enrichment, or how long it takes on average before he hits the obituary. We can change the barrel to contain 500 holes, a matter that would decrease the probability of death, and see the results. Monte Carlo simulation methods were pioneered in martial physics in the Los Alamos laboratory during the A bomb preparation. They became popular in financial mathematics in the 1980s, particularly in the theories of the random walk of asset prices. Clearly, we have to say that the example of Russian roulette does not need such apparatus, but many problems, particularly those resembling real-life situations, require the potency of a Monte Carlo simulator.


https://www.ece.uic.edu/~cpress/ref/Monte%20Carlo%20-Long%20winded%20diatribe.pdf




1946: Monte Carlo method. obtains approximate solutions to numerical problems with unmanageably many degrees of
freedom and to combinatorial problems of factorial size, by mimicking a random process.  

Use the darts at a quarter-circle example



local search of a distribution
explore / exploit with temperature


simple (direct) Monte Carlo requires direct simulation from the target dist 
Robert, C., & Casella, G. (2004). Monte Carlo statistical methods