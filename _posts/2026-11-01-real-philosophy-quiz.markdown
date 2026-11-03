---
layout:     post
title:      "Take the Philpapers survey"
baselink:   /philquiz
permalink:  /philquiz
date:       2026-11-01
author:     Gavin
img:		/img/djc16.jpg

visible:    1
published:  true
quality:

summary:    You could always do this, but now there's a quiz so you might actually do it.
confidence: 70% for imputed positions to the ancients.
importance:
wordcount:
warnings: 	Some AI text. Claude did the imputing of binary views to the classic philosophers.
categories: philosophy
---

<style>
#philquiz { --fg: var(--text-color); --bg: var(--bg-color); --muted: var(--grey-color); --faint: var(--grey-color-light); --rule: #e2e5e0; --green: var(--pak-green); --theme: var(--theme-color); --theme-strong: #1f4f13; --deep: var(--dark-green); --yellow: var(--yellow); --on-theme: #ffffff; --size: var(--font-size); }
#philquiz p, #philquiz ul, #philquiz ol, #philquiz dl { margin: 0; }
#philquiz li { margin-bottom: 0; }
#philquiz h2 { margin: .15em 0 0; position: relative; text-wrap: balance; }
#philquiz h3 { margin: 0; }
#philquiz .qicon { margin-top: .6em; font-size: 2.6em; line-height: 1.1; cursor: default; }
#philquiz h4 { margin: 0; font-family: inherit; }
#philquiz button { font: inherit; color: inherit; background: none; border: 0; padding: 0; margin: 0; cursor: pointer; text-align: inherit; }
#philquiz :focus-visible { outline: 2px solid var(--green); outline-offset: 2px; }
#philquiz hr { border: 0; border-top: 1px solid var(--rule); margin: 2.5em 0; }
#philquiz .refs a, #philquiz .notes a, #philquiz .score a { display: inline; }
#philquiz .link, #philquiz .name, #philquiz .step { text-decoration: underline; text-decoration-thickness: 1px; text-underline-offset: 3.5px; }
#philquiz .link:hover, #philquiz .name:hover, #philquiz .step:hover { color: var(--green); }
#philquiz #sheet { padding: 1.6em 1.8em; border-radius: 14px; background: var(--bg); box-shadow: 0 1px 3px rgba(0, 0, 0, .08), 0 8px 28px rgba(0, 0, 0, .12); }
#philquiz .step:disabled { color: var(--faint); text-decoration: none; cursor: default; }
#philquiz .intro { display: grid; gap: .5em; }
#philquiz .where { color: var(--muted); font-size: .9em; }
#philquiz .qhelp { display: inline-grid; place-items: center; width: 1.65em; height: 1.65em; border-radius: 50%; background: var(--theme); color: var(--on-theme); font-size: calc(var(--size) * 1.1); font-weight: 700; line-height: 1; letter-spacing: 0; vertical-align: middle; cursor: help; }
#philquiz .qhelp:hover, #philquiz .qhelp-wrap.open .qhelp { background: var(--theme-strong); }
#philquiz .qtip { display: none; position: absolute; left: 0; top: 100%; z-index: 5; width: min(32rem, 100%); padding-top: .5rem; white-space: normal; text-wrap: wrap; text-align: left; }
#philquiz .qtip-box { display: block; padding: 1em; border: 1px solid var(--fg); background: var(--bg); color: var(--fg); font-size: var(--size); font-weight: 400; letter-spacing: 0; line-height: 1.5; }
#philquiz .qtip .p { display: block; }
#philquiz .qtip .p + .p, #philquiz .qtip .li + .p { margin-top: .6em; }
#philquiz .qtip .li { display: list-item; list-style: disc; margin: .2em 0 0 1.2em; }
#philquiz .qhelp-wrap.open .qtip, #philquiz .qhelp-wrap:has(.qhelp:focus-visible) .qtip { display: block; }
@media (hover: hover) { #philquiz .qhelp-wrap:hover .qtip { display: block; } }
#philquiz .gloss { margin-top: .35em; max-width: 60ch; font-style: italic; color: var(--muted); }
#philquiz .opts { list-style: none; padding: 0; margin: 1.6em 0 0; display: grid; gap: .7em; }
#philquiz .opts li { display: flex; flex-wrap: wrap; align-items: baseline; gap: .3em 1em; }
#philquiz .opt { flex: 0 1 auto; min-width: 0; font-size: 1.1em; cursor: pointer; }
#philquiz .opts li.chosen .opt { font-weight: 600; }
#philquiz .og { color: var(--muted); font-weight: 400; }
#philquiz .share { color: var(--muted); font-variant-numeric: tabular-nums; }
#philquiz .ctl, #philquiz .also { display: flex; flex-wrap: wrap; gap: .35em; }
#philquiz .also { margin-top: 1.1em; flex-direction: column; align-items: flex-start; }
#philquiz .pick { padding: .1em .55em; border: 1px solid var(--faint); line-height: 1.4; }
#philquiz .pick:hover { border-color: var(--fg); }
#philquiz .pick[aria-pressed="true"] { background: var(--theme); border-color: var(--theme); color: var(--on-theme); }
#philquiz .reveal { margin-top: 1.1em; color: var(--muted); font-style: italic; }
#philquiz .reveal b { color: var(--fg); font-style: normal; font-weight: 600; }
#philquiz .nav { margin-top: 1.8em; display: flex; justify-content: space-between; gap: 1em; }
#philquiz #next { font-weight: 600; }
#philquiz #next.flash { animation: pq-flash .6s ease-in-out 2; }
@keyframes pq-flash { 50% { background: var(--theme); color: var(--on-theme); box-shadow: 0 0 0 .3em var(--theme); } }
#philquiz details.dropdown { margin-top: 1.6em; }
#philquiz details.fold { margin-top: 1.6em; border: 1px solid var(--rule); }
#philquiz details.fold > summary { padding: .5em 1em; background: var(--theme); color: var(--on-theme); cursor: pointer; user-select: none; }
#philquiz details.fold > summary:hover { background: var(--theme-strong); }
#philquiz .about-body { max-width: 65ch; padding: .3em 1em 1em; }
#philquiz .about-body ul { margin: .5em 0 0; padding-left: 1.4em; }
#philquiz .about-body li { margin-bottom: .3em; }
#philquiz .about-body h4 { margin-top: 1.4em; color: var(--deep); font-size: 1em; font-weight: 600; }
#philquiz .args dt { margin-top: .9em; font-weight: 600; }
#philquiz .args dd { margin: .25em 0 0; }
#philquiz .args dd i { color: var(--muted); }
#philquiz .jump ol { margin: .7em 0 0; padding-left: 2em; columns: 2 15em; column-gap: 2.5em; }
#philquiz .jump li { break-inside: avoid; margin-bottom: .2em; }
#philquiz .jump li::marker { color: var(--muted); font-variant-numeric: tabular-nums; }
#philquiz .jump li.here button { color: var(--green); font-weight: 600; }
#philquiz .jump .mark { margin-left: .3em; color: var(--green); }
#philquiz #board > p { margin-top: 1em; }
#philquiz table.board { min-width: 0; margin-top: 1em; table-layout: fixed; }
#philquiz .board th { width: 50%; vertical-align: bottom; }
#philquiz .board td { padding-top: 0; border-bottom: 0; }
#philquiz .board th:last-child, #philquiz .board td:last-child { padding-right: 0; }
#philquiz .board td > p { margin-top: 1em; }
#philquiz .matches { margin: 1.1em 0 0; padding-left: 1.8em; display: grid; gap: .55em; }
#philquiz .matches li::marker { color: var(--muted); font-variant-numeric: tabular-nums; }
#philquiz .matches .name { font-size: 1.05em; }
#philquiz .matches .pc { margin-left: .35em; font-weight: 600; font-variant-numeric: tabular-nums; }
#philquiz .matches .sub { display: block; }
#philquiz .matches .step { display: inline-block; margin-top: .3em; }
#philquiz .erase { padding: .7em 1.6em; border: 2px solid var(--fg); border-radius: 10px; font-size: 1.2em; font-weight: 600; }
#philquiz .erase:hover { background: var(--fg); color: var(--bg); }
#philquiz .erase:disabled { border-color: var(--faint); color: var(--faint); background: none; cursor: default; }
#philquiz .pic { display: block; width: 16em; max-width: 100%; height: auto; aspect-ratio: 1; margin: 1.1em 0 0; border-radius: 50%; object-fit: cover; border: 1px solid var(--rule); }
#philquiz .pic.ph { display: grid; place-items: center; background: #e9f0e6; color: var(--theme); user-select: none; }
#philquiz .pic.ph span { font-size: 4.5em; font-weight: 200; letter-spacing: .02em; }
#philquiz .name[aria-current="true"] { color: var(--green); text-decoration: none; font-weight: 600; }
#philquiz .sub { color: var(--muted); font-size: .9em; }
#philquiz .back { margin-bottom: 1.6em; }
#philquiz .score { margin-top: 1.4em; max-width: 60ch; }
#philquiz .big { margin-right: .15em; color: var(--green); font-size: 2.2em; font-weight: 200; letter-spacing: -1px; line-height: 1; font-variant-numeric: tabular-nums; }
#philquiz blockquote { max-width: 60ch; margin: 1.5em 0 0 1em; }
#philquiz #sheet h3 { margin-top: 1.8em; }
#philquiz #sheet .sub { margin-top: .4em; }
#philquiz .table-wrap { position: relative; overflow-x: auto; margin-top: .8em; }
#philquiz table { width: 100%; min-width: 34em; border-collapse: collapse; }
#philquiz th { padding: 6px 14px 6px 0; border-bottom: 1px solid var(--rule); color: var(--muted); font-size: .9em; font-weight: 600; text-align: left; }
#philquiz td { padding: 10px 14px 10px 0; border-bottom: 1px solid var(--rule); vertical-align: top; }
#philquiz td.q { width: 26%; }
#philquiz tr.diff td.q { font-weight: 600; }
#philquiz td .why { display: block; margin-top: .25em; color: var(--muted); font-size: .9em; font-style: italic; }
#philquiz td.mk { width: 2em; padding-right: 0; text-align: center; color: var(--muted); }
#philquiz td.mk.yes { color: var(--green); }
#philquiz td.mk.no { color: var(--fg); font-weight: 700; }
#philquiz .sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
#philquiz .notes { display: grid; gap: .9em; max-width: 65ch; color: var(--muted); font-size: .9em; }
#philquiz .notes details.dropdown { margin-top: 0; }
#philquiz .notes details p { margin-top: .6em; }
@media (max-width: 600px) { #philquiz h2 { font-size: 2em; } #philquiz #sheet { padding: 1.1em 1em; } }
</style>

<div id="philquiz">
<div class="intro">

<p>Every so often I see a <a href="https://x.com/Will_Hackspeare/status/2116262041115935144">really bad</a> "choose your philosopher" quiz. This hasn't been getting much better despite the (by all accounts very well-read) AIs now writing the quizzes. But if we're going to reduce philosophy to a series of binary forced choices, then we should really use the king of binary forced choice: <a href="https://survey2020.philpeople.org/">the PhilPapers Survey</a>! </p>
<br>
<p>To reward you, I've mapped your position to the closest match among the <span id="ledeN">contemporary philosophers</span> that made their answers public, and to which of <span id="ledeC">the classics</span> probably would have agreed with you.</p>
<br>
<p>It rematches you after every answer so you don't have to do all 100 questions.</p>
</div>
<br>
<div id="sheet" aria-live="polite"></div>
<hr>
<section id="board" aria-label="Closest matches"></section>
<hr>
<p><button type="button" class="erase" id="erase">Erase my answers</button></p>
<br>
<div class="notes">
<details class="dropdown">
	<summary>How matching works</summary><p>Each answer becomes a spread of support over the question's options: <i>accept</i> puts everything on one option, <i>lean</i> puts three quarters there and spreads the rest, <i>undecided / reject the premise</i> spreads it evenly, and a philosopher's combination answer is weighted by its accept, lean, neutral and reject marks. Agreement on a question is the overlap of your spread and theirs: 100% for the same answer at the same strength, 75% for accept versus lean on the same option, 0% for opposite accepts. For the two "mark each one" questions (methods and other minds), each item is compared on a five-point scale. A match's score is the average over the questions you both answered, nudged toward 50% when there are only a few, so a perfect 3 out of 3 doesn't beat a strong 30 out of 35. Answers like "insufficiently familiar", "too unclear" or a write-in alternative are left out.</p>
</details>
<details class="dropdown">	
	<summary>Questions and percentages</summary><p>The <a href="https://survey2020.philpeople.org/survey/results/all">2020 PhilPapers Survey</a> (David Bourget and David Chalmers), target-faculty respondents, "accept or lean towards". Individual answers: the public Philosophical Views on <a href="https://philpeople.org/find-philosopher/search?must_have_public_views=on">PhilPeople profiles</a>, retrieved 30 October 2026. Philosophers can update their public views at any time, so a profile may differ from what it said in 2020.</p>
<p>The question explanations (behind each green "?") and the "Arguments and sources" notes were written by Claude as short orientations to each debate. Every source link was checked to resolve to the work cited; for fuller treatments, follow the Stanford Encyclopedia of Philosophy entries.</p>
<p>It wouldn't be an Internet Quiz without some spurious celebrities. For the 99 historical philosophers, Claude guessed their answer from their writings and standard scholarship. Two models (Opus and Fable) imputed independently; where they disagreed with no clear winner, the cell was left blank, as were questions too anachronistic for a given figure.</p>
</details>
</div>
</div>

<script src="/files/philquiz/quiz.js" defer></script>
