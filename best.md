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
	        </tr>
        {% endif %}
        {% if forloop.last %}</table>{% endif %}
      
      {% endfor %}
      

  <br><br><small><p class="rss-subscribe">subscribe <a href="{{ "/feed.xml" | prepend: site.baseurl }}">via RSS</a></p></small>

</div>
