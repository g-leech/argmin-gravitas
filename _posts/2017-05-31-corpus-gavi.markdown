---
layout: 	post
title:  	"My idiolect"
baselink:	/idiolect
permalink:	/idiolect/
date:   	2017-05-31  <!--site.time-->
author:		Gavin	
img:		/img/explain.jpg

visible:	0
published: 	false
quality:    

summary:	Linguistic analysis of my own writing. 
confidence: 98% in the descriptive bits, 90% in the inferences.
warnings: 	navel-gazing
categories: NLP
importance: 4.
wordcount: 		
---


I have about a million words of my own writing. This is more than enough to infer things about my particular version of my language (my "idiolect"). This is possible because the absolute geniuses at spaCy give away a military-grade English model for free.


* Old blog: 454,000 words (including quotes, drafts, stop words)
* New blog: 20,000 words (including quotes, drafts, stop words)
* Facebook: 200,000 words
* Emails: 10,000 words
* Uni essays: 200,000 words
* My Edinburgh theatre reviews: 2000 words


http://www.nltk.org/book/
http://vh216602.truman.edu/agarvey/cs480/nlpcode/mytext.py


* Median sentence length. Over time.
* Unigram frequency: nltk.FreqDist
* Bigram frequency:
* Rare pairs:  
* Unique trigrams:
* Flesch-Kincaid: * The FK scale is not well-regarded in the field, since it overweights mere vocabulary over clauses and real clarity, but it still gives you a rough picture. 

* Topic modelling
* Entity recognition: 
	- People
	- Organisations
* Writer invariant

* Then factor out quotations and re-do

CountVectorizer (bag of words model) and TfidfVectorizer (tf-idf weighting for the bag of words model)









{%  include idiolect/foots.html %}

