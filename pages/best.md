---
layout:   default
title:    Selected posts
permalink: /best/

visible:  false
---


<div class="home">
	  <h1>Particularly good posts:</h1>
    <br>
    <div style="display: flex; flex-direction: column; gap: 2.5em">
      {% for post in site.posts %}
      	{% if post.quality > 6 %}
			<div>
				<div>
					{{ post.date | date: "%B %Y" }}<br>
					<a style="font-size: 1.6em" href="{{ post.url | prepend: site.baseurl }}">
						{{ post.title }}
					</a>
				</div>
				<small>
					<i>{{ post.summary }}</i>
				</small> 
			</div>
        {% endif %}
        {% if forloop.last %}</div>{% endif %}
      
      {% endfor %}
      

  <br><br><br>
  <a href="{{ "/archive" | prepend: site.url }}">See all posts</a>.  
  <br><br>

  {%  include rss.html  %}

</div>
