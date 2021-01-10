---
layout:     page
title:      Reading
permalink:  /books
visible:    false
---

{%	assign gr = "https://www.goodreads.com/user/show/68316850-gavin"	%}
{%	assign current = "https://www.goodreads.com/review/list/68316850?shelf=currently-reading"	%}

<style>

	.short-column { width: 22%; }
	
	.long-column { width: 75%; }

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
	<h3>Sort by controversy</h3>
	<div>
		Books I most disagree with others about:<br><br>
		{%	include books/sort_by_disagreement_dec_20.html	%}
	</div>
</div><br><br>

## Reviews

{%	include books/reviews_dec_20.html	%}

<br><br><br>


{% 	include lazyload.html 	%}