---
layout: 	page
title: 		Gavin Leech
nav: 		CV
permalink:	/cv/
visible:	false
---

{%  include about/links.html  %}


<br>

<center>
  <h1 class="titl">
    💪 Work 💪
  </h1>
</center>

{%  include cv/work.html  %}

<br><br>

<hr />

<br>


<center>
  <h1 class="titl">
    🏆
    <br class="break">
    Awards
    <br class="break">
    🏆
  </h1>
</center>


<section class="timeline cv prize">
  <ul>
<!--      <li>
      <div>
        <time>
          2022
        </time>
        Open Philanthropy Early Career Funding
      </div>
    </li> 
-->
    <!--  -->
     <li>
      <div>
        <time>
          2022
        </time>
        Residencies: Prague Fall Season, Mexico EA Fellowship
      </div>
    </li>
<!--  -->
<!--  -->
     <li>
      <div>
        <time>
          2021
        </time>
        <a href="{{tc}}">Emergent Ventures</a> grant for a few wild ideas.
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2021
        </time>
        1st prize, <a href="{{amoc}}">Advanced Modelling of Cybercrime</a> hackathon.
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2020
        </time>
        <a href="{{aiac}}">EA Forum Prize</a>, modelling AI safety
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2020
        </time>
        1st place in the <a href="{{vtf}}">Value Forecasting Tournament</a>.
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2019
        </time>
        UKRI studentship for PhD (~$150k).
      </div>
    </li>

</ul>
</section>


<br><br>

<hr />

<br>

<center>
  <h1 class="titl">
    📃
    <br class="break">
    <a class="pubs" href="/researches">Publications</a>
    <br class="break">
    📃
  </h1>
</center>

<br>


* <time>
      <a class="noline" href="/researches">Inferring the effectiveness of government interventions against COVID</a>
        <br><span style="font-weight: normal;">(2020),
      <i>Science</i>
    </span>
  </time> 
<br>

* <time>
      <a class="noline" href="/researches">Mask-wearing in community settings reduces COVID-19 transmission</a>
      <br><span style="font-weight: normal;">(2022), <i>PNAS</i> </span> 
  </time> 
<br>

* <time>
      <a class="noline" href="/researches">How Robust are Estimated Effects of Interventions against COVID-19?</a>
        <br><span style="font-weight: normal;">(2020), <i>NeurIPS</i>
      </span>
  </time> 
<br>

* <time>
    <a class="noline" href="/researches">
      Safety Properties of Inductive Logic Programming
    </a>
    <span style="font-weight: normal;">
      (2020),<br>
      <i>AAAI SafeAI workshop</i>
    </span>
  

<br><br>

<hr />

<br>

{%  include cv/ed.html  %}

<br><br>

<hr />

<br>


<center>
  <h1 class="titl">
   🐞
   <br class="break">
   Tech
   <br class="break">
   🐞 
  </h1>
  <br>
</center>


* <time><a href="/grids">Side effects in Gridworlds</a> (2018):</time> New Pycolab environments and implementations of DQN and MaxEnt IRL in Tensorflow. <a href="{{citess}}">Reused</a> in Deepmind papers.

<br>

* <time><a href="{{prolexa}}">ProlexaPlus</a> (2021):</time>
Brought modern language modelling (Flair) into Prolog for some reason.

<br>

* <time><a href="{{masksman}}">Masks V Mandates</a> (2021):</time>
Journal-grade modern Bayesian workflow in PyMC3. Probabilistic programming for epidemic modelling and effect estimation. End to end with data getters.

<br>

* <time><a href="{{htk}}">Py2HTK</a> (2017):</time>
Python wrapper for the Hidden Markov ToolKit.

<br>

* <time><a href="{{euler}}">Project Euler in Haskell</a> (2022):</time>
Solving little computational maths problems. Gimmick is I don't import anything.


<br>

* <time><a href="{{argg}}">Argmin Gravitas</a> (2017):</time>Just a Jekyll site, but a <a href="/colophon">particularly satisfying</a> one.

<br><br>

<hr />

<br>


<!-- <center>
  <h1>
  &nbsp;&nbsp;
   📈 Stats 📈 
  </h1>
  Brier score 
  Start <a href="{{g}}">reviewing</a> everything I read.<br>
</center>
 -->

<!-- <br>

<hr />

<br>
 -->
<center>
  <h1 class="titl">

  🙋 
  <br class="break">
  Volunteering 
  <br class="break">
  🙋
  </h1>
</center>

<br>

<section class="timeline cv vol">
  <ul>
    {%  include cv/ment.html  %}
    {%  include cv/critcont.html  %}
    {%  include cv/espr.html  %}
    {%  include cv/aisc.html  %}
    {%  include cv/offroad.html  %}
    <!--  -->
    <li>
      <div>
        <time>
          2012
          <br class="break">
          <a class="org" href="{{vso}}">VSO Tanzania</a>
          <br>
        </time>
          <span style="font-weight: normal;">Gender development projects</span>
          <br class="break">
      </div>
    </li>
</ul>
</section>

<br><br>

<hr />

<br><br>

<center>
  &nbsp;&nbsp;
  <a target="_blank" href="/cv.pdf">
     <img width="13%"  src="/img/PDF_file_icon.svg" />
  </a>
</center>


<style>
  .pubs:hover {
    color: #006800
  }

  ul {
    list-style-type: '—   ';
  }

  .ui-accordion .ui-accordion-header {
    font-size: 160% !important;
  }

  #nowacc {
    background-color: cornflowerblue !important;
    border:  4px solid #000;
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
    border-top-left-radius: 0;
    border-top-right-radius: 0;
    padding: 0.5em;
  }

  .ui-icon-triangle-1-e, .ui-icon-triangle-1-s {
    transform: scale(3.5);
    margin-right: 10px;
    margin-left: 15px;
  }

  .ui-icon-triangle-1-s {
    transform: scale(3.5);
    margin-left: 20px;
    margin-right: 10px;
  } 
  



</style>

{%	include timeline_code.html		%}
{%  include padder.html   howMuch=13  %}
