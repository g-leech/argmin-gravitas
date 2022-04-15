---
layout: 	page
title: 		
nav: 		CV
permalink:	/cv/
visible:	false
---

{%  include about/links.html  %}

<style>
  .pubs:hover {
    color: #006800
  }
</style>


<center>
  <h1 class="titl">
    💪 Work 💪
  </h1>
</center>

{%  include cv/work.html  %}

<br>

<hr />

<br>


<center>
  <h1 class="titl">
    &nbsp;&nbsp;
    🏆 Awards 🏆
  </h1>
</center>


<section class="timeline cv prize">
  <ul>
     <li>
      <div>
        <time>
          2022
        </time>
        Residency at the FTX EA Fellowship.
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
        1st prize at the <a href="{{amoc}}">Advanced Modelling of Cybercrime</a> hackathon.
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2020
        </time>
        EA Forum Prize for <a href="{{aiac}}">models</a> of how much AI safety is done indirectly.
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2020
        </time>
        Top forecaster in Nuño Sempere’s <a href="{{vtf}}">Value Forecasting Tournament</a>.
      </div>
    </li>
</ul>
</section>


<br>

<hr />

<br>

<center>
  <h1 class="titl">
    📃
    <a class="pubs" href="/researches">Publications</a>
    📃
  </h1>
</center>


<br>

<hr />

<br>

{%  include cv/ed.html  %}

<br>

<hr />

<br>


<center>
  <h1 class="titl">
  &nbsp;&nbsp;
   🐞 Tech 🐞 
  </h1>
  <br>
</center>


* <time><a href="/grids">Side effects in Gridworlds</a> (2018):</time> New Pycolab environments and implementations of DQN and MaxEnt IRL in Tensorflow. <a href="{{citess}}">Reused</a> in DeepMind and MIT papers.

* <time><a href="{{prolexa}}">ProlexaPlus</a> (2021):</time>Brought modern language modelling (Flair) into Prolog for some reason.

* <time><a href="{{masksman}}">Masks V Mandates</a> (2021):</time>Journal-grade instance of a modern Bayesian workflow in PyMC3.

* <time><a href="{{argg}}">Argmin Gravitas</a> (2017):</time>Just a Jekyll site, but a <a href="/colophon">particularly satisfying</a> one.

<br>

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

  &nbsp;&nbsp;
  🙋 Volunteering 🙋
  </h1>
</center>

<section class="timeline cv vol">
  <ul>
    {%  include cv/espr.html  %}
    <!--  -->
    <li>
      <div>
        <time>
          2018 – Now
          <br class="break">
          <!--  -->
          <a class="org" href="aisafety.camp">AI Safety Camp</a>
          <br class="break">
        </time>
        Vetting & interviewing for a scrappy intro to AI safety <i>research</i>.
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2021 – Now
          <br class="break">
          <!--  -->
          <a class="org" href="{{offf}}">Off Road</a>
          <br class="break">
        </time>
        I started a thing to help people with executive dysfunction. Fully funded by a bitcoin rando! Thanks!
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2019
          <br class="break">
            <a href="{{parallel}}" class="org">Parallel Forecast</a>
          <br>
        </time>
        Resolution Council member of a short-lived AI prediction market.
      </div>
    </li>
    <!--  -->
    <li>
      <div>
        <time>
          2012
          <br class="break">
          <span class="org">
            Gender Development Volunteer,
          </span>
          <br>
          <a class="org" href="{{vso}}">VSO Tanzania</a><br>
        </time>
          <br class="break">
          <br class="break">
      </div>
    </li>
</ul>
</section>

<br>

<hr />

<br><br>

<center>
  &nbsp;&nbsp;
  <a target="_blank" href="/cv.pdf">
     <img width="13%"  src="/img/PDF_file_icon.svg" />
  </a>
</center>


{%	include timeline_code.html		%}
{%  include padder.html   howMuch=13  %}