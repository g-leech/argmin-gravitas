---
layout:   default
title:    Archive
permalink: /archive/

visible:  false
---

<style type="text/css">
  
  tr.spaced > td
  {
    padding-bottom: 1em;
  }


  h1.spaced 
  {
    padding-top: 15px;
  }

</style>

<div class="home">
      {% for post in site.posts %}

        {% assign currentMonth = post.date | date: "%b %Y" %}
        {% if currentMonth != myMonth %}
           {% unless forloop.first %}</table>{% endunless %}
           
           <br><hr>

           <h1 class="spaced">{{ currentMonth }}</h1>
            <table class="post-list">
           {% assign myMonth = currentMonth %}
        {% endif %}
        
        <tr class="spaced">
          <td style="width:60%;"><a class="post-link" href="{{ post.url | prepend: site.baseurl }}"
            >>> {{ post.title }}</a>
          </td>

          <td> <small>{{ post.summary }}</small> </td>

          <td><ul>
            <small>
              {% if post.simple %} <li><a href="{{post.url}}">Simple</a></li> {% endif %}
              {% if post.stylised %} <li><a href="{{post.url}}">Stylised</a></li> {% endif %}
              {% if post.technical %} <li><a href="{{post.url}}">Technical</a></li> {% endif %}
            </small>
          </ul></td>
        </tr>

        {% if forloop.last %}</table>{% endif %}
        
      
      {% endfor %}
      

  <br><br><small><p class="rss-subscribe">subscribe <a href="{{ "/feed.xml" | prepend: site.baseurl }}">via RSS</a></p></small>

</div>
