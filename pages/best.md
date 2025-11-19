---
layout:   default
title:    Selected posts
permalink: /best/

visible:  false
---

<style>
	h1{
		margin-top: 1.2em;
		margin-bottom: 0.75em
	}

	.post-list{
		display: flex;
		flex-direction: column; 
		gap: 2em;
	}

	.post-item{
		display: flex;
		flex-direction: column; 
		gap: 0.5em;
	}

	.post-item a{
		color: var(--dark-green);
		font-weight: 500;
	}

  .post-link{
    min-width: 55%;
    /* max-width: 75%; */
    font-size: 1.4em;
    line-height: 1.3em;
    text-decoration: none !important; 
    font-weight: 500;
    color: var(--white);
    font-family: var(--serif);
    background: linear-gradient(135deg, #006800 0%, #93cc93 100%);
    padding-left:40px;
    padding-top: 1%;
    padding-bottom: 1%;
  }

  .nolink {
	 font-size: 0.9em;
	 line-height: 1.1em;
   padding-left:40px;
   padding-top: 1%;
  }

</style>

<div class="post">
	<h1 class='post-title'>Particularly good posts:</h1>
    <br>
    <div class="post-list">
      {% for post in site.posts %}
      	{% if post.quality > 6 %}
			<div class="post-item">
				<a class="post-link" style="font-size: 1.5em" href="{{ post.url | prepend: site.baseurl }}">
					{{ post.title }}
				</a>
				<div class="nolink">
					<i>{{ post.summary }}</i> ({{ post.date | date: "%b %Y" }})
				</div> 
			</div>
        {% endif %}
        {% if forloop.last %}</div>{% endif %}
      {% endfor %}
      

  <br><br><br>
  <a href="{{ "/archive" | prepend: site.url }}">See all posts</a>.  
  <br><br><br>

  {%  include mc.html  %}

</div>
