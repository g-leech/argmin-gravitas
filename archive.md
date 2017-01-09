---
layout:   default
title:    Archive
permalink: /archive/

visible:  false
---

<div class="home">
      {% for post in site.posts %}

        {% assign currentDate = post.date | date: "%Y" %}
        {% if currentDate != myDate %}
           {% unless forloop.first %}</table>{% endunless %}
           <h1>{{ currentDate }}</h1>
            <table class="post-list">
           {% assign myDate = currentDate %}
        {% endif %}

        {% assign currentMonth = post.date | date: "%b" %}
        {% if currentMonth != myMonth %}
           <tr>
            <h1>{{ currentMonth }}</h1>
           </tr>
           {% assign myMonth = currentMonth %}
        {% endif %}
        
        <tr style="width:50%;">
          <td><h1> <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">>> {{ post.title }}</a></h1></td>
          <td> {{ post.summary }}</td>
          <td><ul>
            {% if post.simple %} <li><a href="{{post.url}}">Simple</a></li> {% endif %}
            {% if post.stylised %} <li><a href="{{post.url}}">Stylised</a></li> {% endif %}
            {% if post.technical %} <li><a href="{{post.url}}">Technical</a></li> {% endif %}
          </ul></td>
        </tr>
        
        {% if forloop.last %}</table>{% endif %}
      
      {% endfor %}
      

  <br><br><small><p class="rss-subscribe">subscribe <a href="{{ "/feed.xml" | prepend: site.baseurl }}">via RSS</a></p></small>

</div>
