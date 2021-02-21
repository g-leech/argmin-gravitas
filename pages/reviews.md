---
layout:     page
title:      Reading
permalink:  /books
visible:    false
---

{%	assign gr = "https://www.goodreads.com/user/show/68316850-gavin"	%}
{%	assign current = "https://www.goodreads.com/review/list/68316850?shelf=currently-reading"	%}

<!-- 
</blockquote></i>
data-src
height

	<th class="short-column"></th>
	<th class="long-column"></th>
 -->

<style>

	.short-column { width: 15%; }
	
	.long-column { width: 85%; }


	table {
    	border-collapse: collapse;
    	border-spacing: 0;
    	table-layout: fixed;
		width: 100%!important;
	}
	
	td {
		vertical-align: top;
    	padding: 8px; 
	}

	.rating {
		text-align: center;
		font-size: 22pt;
	}

	.best {
		border-spacing: 5;
	}

	th  {
		text-align: center!important;
	}

</style>

For an overview, it's probably easier to <a href="{{gr}}">look on Goodreads</a>.

<br>

<div class="accordion">
	<h3>Now</h3>
	<div>
		{%	include books/widget.html	%}
	</div>
	<!--  -->
	<!--  -->
	<h3>Fav</h3>
	<div>
		{%	include books/best_books_dec_20.html	%}
	</div>
	<!--  -->
	<!--  -->
	<h3>Sort by controversy</h3>
	<div>
		Books I most disagree with others about:<br><br>
		{%	include books/sort_by_disagreement_dec_20.html	%}
	</div>
</div><br><br>


## Jump to

* _<a href="#five">5/5: Will re-read until I die. 97th percentile+</a>_
* _<a href="#four">4/5: Very impressed. 75th percentile+</a>_
* _<a href="#three">3/5: Net likeable. 50th percentile.</a>_
* _<a href="#two">2/5: Only for enthusiasts. 25th percentile.</a>_
* _<a href="#one">1/5: False, ugly, evil, or vapid. 1st percentile.</a>_

<br>

## Reviews

<a name="five"></a>

### 5/5: Will re-read until I die. 97th percentile+

<a name="five"></a>

{%	include books/reviews_5.html	%}

<!-- </i></td></tr></ul></td></tr></i></li></ul></blockquote></td></tr></ul></td></tr></i></td></tr></li></li></li></li></td></tr> -->

<a name="four"></a>
<br><br>

### 4/5: Very impressed. 75th percentile.

{%	include books/reviews_4.html	%}

<!-- </i></td></tr></i></i></i></td></tr></ul></td></tr></i></td></tr></i></td></tr></li></li></ul></i></blockquote></td></tr></div></div></blockquote></td></tr></i></blockquote></td></tr></li></li></ul></div></td></tr> -->

<a name="three"></a>
<br><br>

### 3/5: Net likeable. 50th percentile.

{%	include books/reviews_3.html	%}

<!--     </i></i></td></tr></i></blockquote></td></tr></span></i></blockquote></td></tr></blockquote></i></td></tr></i></i></i></td></tr></i></blockquote></td></tr></i></i></blockquote></td></tr></i></li></i></li></i></li></ul></i></blockquote></blockquote></td></tr></i></i></td></tr> -->


<a name="two"></a>
<br><br>

### 2/5: Only for enthusiasts. 25th percentile.

{%	include books/reviews_2.html	%}

<!-- </i></blockquote></td></tr></i></td></tr></blockquote></td></tr></i></blockquote></td></tr></I></blockquote></td></tr></i></td></tr> -->

<a name="one"></a>
<br><br>

### 1/5: False, ugly, evil, or vapid. 1st percentile.

{%	include books/reviews_1.html	%}

<br><br><br>


{% 	include lazyload.html 	%}