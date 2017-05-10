---
layout:   default
title:    Selected posts
permalink: /best/

visible:  false
---

<style type="text/css">
  
  tr.spaced > td
  {
    padding-bottom: 1em;
  }

</style>

<div class="home">
	  <h1 class="page-heading">Particularly good posts:</h1>
      {% for post in site.posts %}
      	
      	{% if post.best %}
	       	<tr class="spaced">
	          <td style="width:60%;"><a class="post-link" href="{{ post.url | prepend: site.baseurl }}">>> {{ post.title }}</a></td>
	          <td> <small>{{ post.summary }}</small> </td>
	        </tr><br>
        {% endif %}
        {% if forloop.last %}</table>{% endif %}
      
      {% endfor %}
      

  <br><br>
  <a href="{{ "/archive" | prepend: site.url }}">See all posts</a>.  <br>

  {%  include rss.html  %}

</div>
