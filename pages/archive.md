---
layout:   default
title:    Archive
nav:      Archive
permalink: /archive/
visible:  false
---

<style type="text/css">
  
  .post-list{
	display: flex;
	flex-direction: column;
	gap: 2em
  }

  .post-link{
    min-width: 55%;
    /* max-width: 75%; */
    font-size: 1.3em;
    line-height: 1.3em;
    text-decoration: none !important; 
    font-weight: 500;
    color: var(--white);
    font-family: var(--serif);
    background: linear-gradient(135deg, #006800 0%, #93cc93 100%);

  }

  .spaced{
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .nolink{
	font-size: 0.8em;
	line-height: 1.3em
  }

  .archive-link
  {
    font-size: 20px;
  }

  h3.year{
    margin-top: 2.5em;
    margin-bottom: 1em;
    font-size: 1.6em;
    color: var(--theme-color);
    font-weight: 600;
  }


  .page-list{
    display: flex;
    flex-direction: column;
    gap: 1em;
  }

  .page-item{
    display: flex;
    justify-content: space-between;
    font-size: 1.1em;
    gap: 1.5em;
  }

  .page-item div{
    flex-shrink: 0;
    opacity: 0.4;
  }

  @media (max-width: 600px){

	.post-list{
		gap: 2em
	}

	.post-link{
		max-width: 100%
	}
  }

</style>


<script async src="https://cse.google.com/cse.js?cx=942c04d4b5a4e68e0"></script>
<div class="gcse-search"></div>


<div class="post">
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

<div class='page-list'>
{% for page in site.pages %}
    {% if page.url != "/404.html" and page.url != "/feed.xml" and page.favpage != true %}
    <div class="page-item">
        <a href="{{ page.url | prepend: site.baseurl }}">{{ page.title }}.</a>
        <div> {% if page.content %}
                {{ page.content | number_of_words }}
              {% endif %} words 
        </div>
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
