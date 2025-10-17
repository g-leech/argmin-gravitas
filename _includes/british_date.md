{% assign suffix = "th"  %}
{%  assign day = page.date | date: "%-d" %}
{%  assign last = day | slice: -1 1 %}
{%  assign penult = day | slice: -2 1 %}

{%   if last == "1" and penult != "1" %}
  {% assign suffix = "st"  %}
{%    endif    %}

{%   if last == "2" and penult != "1" %}
  {% assign suffix = "nd"  %}
{%    endif    %}

{%   if last == "3" and penult != "1" %}
  {% assign suffix = "rd"  %}
{%    endif    %}

<div id="bdate">
  {{day}}{{suffix}} {{ page.date | date: "%B %Y" }}
</div>
