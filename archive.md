---
layout: default
title: Archive
permalink: /archive/
---

<div class="home">
  
  <h1 class="page-heading">All Posts</h1>

      {% for post in site.posts %}
        {% assign currentDate = post.date | date: "%Y" %}
        {% if currentDate != myDate %}
           {% unless forloop.first %}</table>{% endunless %}
           <h1>{{ currentDate }}</h1>
            <table class="post-list">
           {% assign myDate = currentDate %}
        {% endif %}

        <td style="width:50%;">
          <h3> <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">>> {{ post.date | date: "%B %-d, %Y" }}</span> - {{ post.title }}</a>
          </h3>
        </td>
        <td> {{ post.summary }}</td>
        <!--<td> Available in: <ul>
          {% if post.simple %} <li>[Simple]({{post.url}}/simple)</li>
          {% if post.stylised %} <li>[Stylised]({{post.url}}/stylised)</li>
          {% if post.technical %} <li>[Technical]({{post.url}}/technical)</li>
        </ul></td>-->
       {% if forloop.last %}</table>{% endif %}
   {% endfor %}

 
    <table class="post-list">
    {% for post in site.posts %}
      <tr>
        <span class="post-meta">{{ post.date | date: "%b %-d, %Y" }}</span>

        <td style="width:50%;"><h2>
          <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">>> {{ post.title }}</a>
        </h2></td>
        <td> {{ post.summary }}</td>
        <td> Available in:<ul>
          {% if post.style %} <li>Simple</li></td>
      </tr>
    {% endfor %}
    </table>

  <br><br><small><p class="rss-subscribe">subscribe <a href="{{ "/feed.xml" | prepend: site.baseurl }}">via RSS</a></p></small>

</div>
