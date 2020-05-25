---
layout:     page
title:      Little technologies in this site
permalink:  /colophon
visible:    false
---

{%	assign muf = "https://web.archive.org/web/2014*/http://muflax.com/episteme"	%}
{%	assign cud = "https://github.com/g-leech/argmin-gravitas/blob/master/scripts/psych/cuddy2010.ipynb"		%}
{%	assign sass = "https://github.com/g-leech/argmin-gravitas/blob/master/css/main.scss"	%}
{%	assign yam = "https://raw.githubusercontent.com/g-leech/argmin-gravitas/master/_posts/2012-08-24-overheads.markdown"	%}
{%	assign arg = "https://github.com/g-leech/argmin-gravitas/blob/master/_includes/sites/argument.html"		%}
{%	assign comm = "http://creativecommons.org/licenses/by-sa/4.0/"	%}
{%	assign bount = "https://github.com/g-leech/argmin-gravitas/blob/master/_includes/about/errata.html"	%}
{%	assign flu = "https://raw.githubusercontent.com/g-leech/argmin-gravitas/master/_posts/2018-02-14-great-flu.markdown"	%}
{%	assign rb = "https://github.com/g-leech/argmin-gravitas/blob/master/_plugins/myfilters.rb"		%}
{%	assign psy = "https://github.com/g-leech/argmin-gravitas/blob/master/_includes/about/psychology.html"		%}
{%	assign tp = "https://github.com/g-leech/argmin-gravitas/blob/6c425a3d3e486a7a6388864977cec9d7d3b85580/_includes/head.html#L4"		%}
{%	assign rss = "https://github.com/g-leech/argmin-gravitas/blob/master/feed.xml"	%}


Some examples of the freedom of self-hosting, and the low-hanging fruit in all nonfiction:<br><br>

* _Metadata_. <a href="{{yam}}">YAML</a>. Lets me build indices of the best posts, most important topics, and a timeline.
  * _Content notes_. I don't want to <a href="{{flu}}">distress</a> anyone who doesn't want to be distressed.
  * _Topic imporance_.
  * _Quality_. IMO.
  * _Epistemic status_. Marking each post with the confidence level or literalness, genre. <a href="{{muf}}">Originator</a>.
  * _Argument_. <a href="{{arg}}">The thinking person's TL;DR</a>.
  * _Last page edit_. <a href="{{rb}}">Nice for emphasising that the content is changing. <br><br>

* _Information hiding_. bigfoot and JQuery Accordions.

* _Bug bounties_. <a href="{{bount}}">To keep me honest</a>.

* _Server magic_: Jekyll and Ruby. Can do anything. Text deduplication, link reuse, quote database, etc.

* _Feed_. I actually initally forgot about good old <a href="{{rss}}">RSS</a>, because it is so easy and so reliable that it fades into the background of life, and ceases to appear as technology.

* _Anonymous feedback_. Unauthenticated Google Form.

* _Disclaimers_. Most book reviews are by people unfit to judge their truth, including mine.

* _Psychology_. <a href="{{psy}}">What am I like?</a> What sort of person writes this? 

* _Opinions_. I used to dream of listing all of my premises. This is both impossible and too much work, so instead <a href="/opinion">I list some things I think you should know</a>.

* _Worldview message digest._ Quotations database.

* _Licence_. Licensed under <a href="{{comm}}">Creative Commons Sharealike</a>.

* _Style consistency_. <a href="{{sass}}">SASS</a> (meta CSS)

* _Static comments_. Netlify Forms. (Staticman is cool but brittle.)

* _Typesetting maths_. MathJax.

* _Data analysis transparency._ <a href="{{cud}}">Github ipynb viewer</a>.

* _Diagrams_. matcha.io

* _Memorial_. <a href="{{tp}}">GNU</a>

* _Tables_. https://www.tablesgenerator.com/

<br><br>

## To implement

* _Rotproofing my links_. Gwern's archiver.
* Internet Archive option for all links. Or auto replace script.
* Maybe make content notes more prominent.
* GRADE evidence quality scale?
* Time since modified vs error discovery rate