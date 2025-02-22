---
layout:   default
title:    Selected posts
permalink: /best/

visible:  false
---

<style>
	h1{
		margin-top: 1.2em;
		margin-bottom: 0.75em
	}

	.post-list{
		display: flex;
		flex-direction: column; 
		gap: 2em;
	}

	.post-item{
		display: flex;
		flex-direction: column; 
		gap: 0.5em;
	}

	.post-item a{
		color: var(--dark-green);
		font-weight: 500;
	}


</style>

<div class="post">
	<h1 class='post-title'>Particularly good posts:</h1>
    <br>
    <div class="post-list">
      {% for post in site.posts %}
      	{% if post.quality > 6 %}
			<div class="post-item">
				<div class="light">
					{{ post.date | date: "%B %Y" }}
				</div>
				<a style="font-size: 1.5em" href="{{ post.url | prepend: site.baseurl }}">
					{{ post.title }}
				</a>
				<div class="light">
					<i>{{ post.summary }}</i>
				</div> 
			</div>
        {% endif %}
        {% if forloop.last %}</div>{% endif %}
      
      {% endfor %}
      

  <br><br><br>
  <a href="{{ "/archive" | prepend: site.url }}">See all posts</a>.  
  <br><br>

  {%  include mc.html  %}

</div>
