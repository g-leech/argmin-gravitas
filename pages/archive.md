---
layout:   default
title:    Archive
permalink: /archive/

visible:  false
---

<style type="text/css">
  
  tr.spaced > td
  {
    padding-bottom: 2em;
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
          <td style="width:57%;">
            <a class="post-link" href="{{ post.url | prepend: site.baseurl }}">
            {{ post.title }}</a>
          </td>
<!--  -->
          <td style="width:3%;">
          </td>
          <td> 
            <small>
              <a class="nolink" href="{{ post.url | prepend: site.baseurl }}"><i>{{ post.summary }}</i></a>
            </small> 
          </td>
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

<h1>Pages</h1>


{% for page in site.pages %}
  <table>
    {% if page.favpage != true %}
    <tr class="spaced">
    <td style="width:55%;">
      <a class="archive-link" href="{{ page.url | prepend: site.baseurl }}">
      {{ page.title }}.</a>
    </td>
    </tr>
    {%  endif   %}
  </table>
{% endfor %}

<br><br>


<p class="rss-subscribe">
  subscribe <a href="{{ "/feed.xml" | prepend: site.baseurl }}">via RSS</a>
</p>
<p>
  See posts ranked by their importance <a href="/importance">here</a>.
</p>


</div>
