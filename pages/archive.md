---
layout:   default
title:    Archive
permalink: /archive/

visible:  false
---

<style type="text/css">
  
  .post-list{
	display: flex;
	flex-direction: column;
	gap: 2.5em
  }

  .spaced{
	display: flex;
	gap: 10%;
  }

  .post-link{
	min-width: 55%;
	max-width: 55%;
	font-size: 1.5em;
	line-height: 1.3em
  }

  .nolink{
	font-size: 0.8em;
	line-height: 1.3em
  }

  .archive-link
  {
    font-size: 20px;
  }

  .year{
	margin-top: 2.5em;
	margin-bottom: 1em;
	font-size: 1.6em;
	color: var(--theme-color);
	font-weight: 600;
  }

  .page{
	margin-top: 1em
  }

  @media (max-width: 600px){

	.post-list{
		gap: 2em
	}

	.spaced{
		flex-direction: column;
		gap: 0.5em;
	}

	.post-link{
		max-width: 100%
	}
  }

</style>


<script async src="https://cse.google.com/cse.js?cx=942c04d4b5a4e68e0"></script>
<div class="gcse-search"></div>


<div class="home">
      {% for post in site.posts %}
<!--  -->
        {% assign currentYear = post.date | date: "%Y" %}
        {% if currentYear != yr %}
           {% unless forloop.first %}</div>{% endunless %}
<!--  -->
           <h3 class="year">{{ currentYear }}</h3>
            <div class="post-list">
           {% assign yr = currentYear %}
        {% endif %}
<!--  -->
        <div class="spaced">
            <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">
            {{ post.title }}</a>
            <a class="nolink" href="{{ post.url | prepend: site.baseurl }}"><i>{{ post.summary }}</i></a>
		</div>
        {% if forloop.last %}</div>{% endif %}
<!--  -->
<!--  -->      
      {% endfor %}

<br>

<h2>Pages</h2>

<div>
{% for page in site.pages %}
    {% if page.url != "/404.html" and page.url != "/feed.xml" and page.favpage != true %}
    <div class="spaced page">
        <a class="archive-link" href="{{ page.url | prepend: site.baseurl }}">{{ page.title }}.</a>
    </div>
    {% endif %}
{% endfor %}
</div>
<br>

<p>
  See posts ranked by their importance <a href="/importance">here</a>.
</p>

<br>

{%    include mc.html  %}

</div>
