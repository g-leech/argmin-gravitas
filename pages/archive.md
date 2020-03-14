---
layout:   default
title:    Archive
permalink: /archive/

visible:  false
---

<style type="text/css">
  
  tr.spaced > td
  {
    padding-bottom: 1.5em;
  }


  h1.spaced 
  {
    padding-top: 20px;
  }

  .archive-link
  {
    font-size: 20px;
  }

</style>

<div class="home">
      {% for post in site.posts %}
<!--  -->
        {% assign currentYear = post.date | date: "%Y" %}
        {% if currentYear != yr %}
           {% unless forloop.first %}</table>{% endunless %}
<!--  -->
           <br><hr>
<!--  -->
           <h1 class="spaced">{{ currentYear }}</h1>
            <table class="post-list">
           {% assign yr = currentYear %}
        {% endif %}
<!--  -->
        <tr class="spaced">
          <td style="width:55%;"><a class="archive-link" href="{{ post.url | prepend: site.baseurl }}"
            > > {{ post.title }}</a>
          </td>
<!--  -->
          <td> <small><i>{{ post.summary }}</i></small> </td>
<!--  -->
        </tr>
<!--  -->
        {% if forloop.last %}</table>{% endif %}
<!--  -->
<!--  -->      
      {% endfor %}
<!--  -->      
<!--  -->
<br><br>

<p class="rss-subscribe">
  subscribe <a href="{{ "/feed.xml" | prepend: site.baseurl }}">via RSS</a>
</p>
<p>
  See posts ranked by their importance <a href="/importance">here</a>.
</p>


</div>
