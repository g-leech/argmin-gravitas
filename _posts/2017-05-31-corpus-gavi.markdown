---
layout: 	post
title:  	"Analysing my writing over a million words"
baselink:	/idiolect
permalink:	/idiolect/
date:   	2017-05-31  <!--site.time-->
author:		Gavin	
img:		/img/explain.jpg

visible:	0
published: 	false


summary:	Linguistic analysis of all the digitised writing by me I could find. 
confidence: 98%
warnings: 	navel-gazing
categories: NLP
count: 		
---


I've written about a million words in digital formats I can easily get my hands on. This is more than enough to , especially given spaCy's free, military-grade English model.

* Old blog: 454,000 words (including quotes, including drafts, including stop words)
* New blog: 50,000 ??? words
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
* Topic modelling
* Entity recognition: 
	- People
	- Organisations
* Writer invariant

* Then factor out quotations and re-do

CountVectorizer (bag of words model) and TfidfVectorizer (tf-idf weighting for the bag of words model)




You often see the claim that the Tories have killed "thousands" of disabled people by spuriously cutting them off from disability benefit. ([ESA][esa] is probably the only government program in the world with [a Palme d'Or winning film][blake] about how shit it is.)

This number seems to come from a [naive reading][naive] of mortality statistics: between 2011 and 2014, [2,380][kilodeath] disabled people died after being declared "fit to work" (and so after having their main income removed). The newspaper and blog discussionof this fact is sloppy, even by the pitiful standards of public policy debate.

In particular: **You cannot infer anything about a program's impact from a single number, without a reference class.** It took me exactly 10 seconds to find age-adjusted data on deaths in this subpopulation, compared to the general population, before _and_ after the imposition of WCA <a href="#fn:1" id="fnref:1">1</a>:<br><br>

<div align="center">
	<img src="/img/killer-tories/rates.jpg" />

	<br><br><small>Age-standardised mortality rates (deaths per 100,000) in various subpopulations .</small>
</div><br>


There. If I was a journalist or an activist, I'd wrap up here: clearly Tory welfare reforms have been miraculous, having saved hundreds of disabled people's lives on net. Somehow.

The following is just a statistical argument. All I can say for it is that it is less pig-ignorant than going around parroting the uncontrolled, uncalibrated figure.

Death after "fit for work" WCA: 		2,380 
Dead after unsuccessful DWP appeal:		1,360
Dead ESA recipients, 2016:				50,580




## Age distribution of disabled people

Age-standardised mortality rates (ASMR) 


## Comorbidity


We could reduce uncertainty with data on cause of death - hypothermia and suicide being evidence of WCA killing, and decompensation of chronic illnesses being null evidence.


## Note on data

People [struggled with the DWP][struggle] to get these figures published. This is sometimes read as an admission of guilt. But given how naively the 2,380 figure was received, it is hard to blame them for their cowardly reticence.


* https://www.gov.uk/government/uploads/system/uploads/attachment_data/file/459106/mortality-statistics-esa-ib-sda.pdf


[esa]: https://en.wikipedia.org/wiki/Employment_and_Support_Allowance
[naive]: https://www.theguardian.com/society/2015/aug/27/thousands-died-after-fit-for-work-assessment-dwp-figures
[kilodeath]: https://www.gov.uk/government/statistics/mortality-statistics-esa-ib-and-sda-claimants
[blake]: https://en.wikipedia.org/wiki/I,_Daniel_Blake
[public]: https://stat-xplore.dwp.gov.uk/webapi/jsf/login.xhtml
[struggle]: https://ico.org.uk/media/action-weve-taken/decision-notices/2015/1424160/fs_50557638.pdf 




{%  include killer-tories/foots.html %}