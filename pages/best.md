---
layout:   default
title:    Selected posts
permalink: /best/

visible:  false
---


<div class="home">
	  <h1>Particularly good posts:</h1>
    <br>
    <table>
      {% for post in site.posts %}
      	{% if post.quality > 6 %}
	       	<tr class="spaced">
	          <td style="width:50%;">
              {{ post.date | date: "%B %Y" }}<br>
              <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">
              {{ post.title }}</a>
              <br>
            </td>
	          <td> 
              <small>
                <i>{{ post.summary }}</i>
                <br>
              </small> 
            </td>
	        </tr>
        {% endif %}
        {% if forloop.last %}</table>{% endif %}
      
      {% endfor %}
      

  <br><br><br>
  <a href="{{ "/archive" | prepend: site.url }}">See all posts</a>.  
  <br><br>

  {%  include rss.html  %}

</div>
