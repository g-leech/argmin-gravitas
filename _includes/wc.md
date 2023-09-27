{%  assign acc = 0  %}
{%  assign TEXTS = site.documents %}
<!-- {%  assign TEXTS = site.posts | concat: site.pages  %}  -->

{% for post in TEXTS %}
    {%  assign wordCount = post.content | number_of_words  %}
    {%  assign acc = acc | plus: wordCount  %}
{%  endfor  %}