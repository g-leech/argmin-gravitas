---
layout:     page
title:      Reading
permalink:  /books
visible:    true
---

{%	assign gr = "https://www.goodreads.com/user/show/68316850-gavin"	%}

<style>
	table {
    	border-collapse: collapse;
    	border-spacing: 0;
	}
	
	td {
    	padding: 8px; 
	}

	.rating {
		text-align: center;
		font-size: 22pt;
	}

	.best {
		border-spacing: 5;
	}

	.reviews {
		max-width: 100%;
	}

	.reviews td {
		vertical-align: top;
	}
</style>

For an overview, it's probably easier to <a href="{{gr}}">look on Goodreads</a>.

<br>

<div class="accordion">
	<h3>Favourite books</h3>
	<div>
		{%	include books/best_books_apr_20.html	%}
	</div>
	<h3>Sort by controversy</h3>
	<div>
		Here are the books I have the largest disagreement with Goodreads users about:<br><br>
		{%	include books/sort_by_disagreement_apr_20.html	%}
	</div>
</div><br><br>

## Reviews
{%	include books/reviews_apr_20.html	%}

<br><br><br>