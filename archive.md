---
layout:   default
title:    Archive
permalink: /archive/

visible:  false
---

<div class="home">
      {% for post in site.posts %}

        {% assign currentYear = post.date | date: "%Y" %}
        {% if currentYear != myYear %}
           {% unless forloop.first %}</table>{% endunless %}
           <h1>{{ currentYear }}</h1>
            <table class="post-list">
           {% assign myYear = currentYear %}
        {% endif %}
        
        <tr>
          <td style="width:60%;"><a class="post-link" href="{{ post.url | prepend: site.baseurl }}">>> {{ post.title }}</a></td>
          <td> <small>{{ post.summary }}</small> </td>
          <td><ul>
            <small>
            {% if post.simple %} <li><a href="{{post.url}}">Simple</a></li> {% endif %}
            {% if post.stylised %} <li><a href="{{post.url}}">Stylised</a></li> {% endif %}
            {% if post.technical %} <li><a href="{{post.url}}">Technical</a></li> {% endif %}
            </small>
          </ul></td>
        </tr><br>
        
        {% if forloop.last %}</table>{% endif %}
      
      {% endfor %}
      

  <br><br><small><p class="rss-subscribe">subscribe <a href="{{ "/feed.xml" | prepend: site.baseurl }}">via RSS</a></p></small>

</div>
