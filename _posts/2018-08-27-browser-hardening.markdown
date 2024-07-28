---
layout:     post
title:      "Hardening the browser"
baselink:   /browser
permalink:  /browser
date:       2018-09-01
author:     Gavin

img:        /img/attack.jpg
published:  true
visible:    1

summary:    Passable browser security for almost no money or effort.
confidence: 95% that this is worth the time and better than nothing.
categories: long-content, computers, self-help, lists
quality: 	8
importance: 5
wordcount:  1550
---

{%  include browser/links.md  %}
{%	include js/lazyFrame.html		%}



It's now common knowledge that we're being watched online, by a <a href="{{mix}}">thick mix</a> of <a href="{{xkey}}">nation</a>-<a href="{{ech}}">states</a>, <a href="{{enemy}}">private</a> <a href="{{fb}}">companies</a>, <a href="{{spy}}">and</a> <a href="{{play}}">criminals</a>. They sometimes do <a href="{{mal}}">worse than watch</a>. What do we do? Should we care?

It's not clear what the probability of having your password leaked in a breach / having your email read / having your laptop being remotely wiped (unless you pay the creator Bitcoin) is. But something like this <a href="{{krebs}}">will probably happen</a> to you in your lifetime, so I would take 10 mins to mitigate them now.

There is no absolute security; it's always partial and relative to a goal. This guide is aimed at "_not losing control of your accounts, not being surveilled by companies or criminals, not having your online banking subverted, not getting infected by ransomware or whatever_". It's strictly for people with average risks: not that much money, not much tech cred, not much sensitive information to protect.<br><br>

<center>
	<div id="attackImg" style="width:50%"></div>
	<small><i>"Wait, isn't that your own computer -"</i></small>
</center><br>

On a lighter note, security is an amazing way to learn about how the internet actually works. It's a lot easier to remember the <a href="{{tcpip}}">dozens</a> of abstract systems involved when you can think, smugly, "_And I've plugged that gap with this mitigation, and that one, and that one..._"

Most of this article assumes you're using Firefox, <a href="{{chrome}}">because</a> <a href="{{chromeCookies}}">Chrome</a> <a href="https://blog.cryptographyengineering.com/2018/09/23/why-im-leaving-chrome/">is itself</a> <a href="{{protonChrome}}">an attack</a>. That is, it protects you very well against everyone except Google. <a href="#fn:2" id="fnref:2">2</a> It's not a big deal compared to the other parts of this list, you'll just need to find alternatives to the add-ons I recommend.

<br>

_2023 edit: most of the below is still fairly relevant, but the adaptive nature of attacks means you should seek out [more modern notes](https://ssd.eff.org) too._

<br><br>

<div class="accordion">
	<h3>Ugh factors and tail risks</h3>
	<div>
		Why care about this? Besides mere trust in one's hardware, or a mere preference not to be watched, it's to do with the increasing tail risks of being in principle vulnerable to one oddball with a vendetta. These will increase for two reasons: the coming increase in the online population, and in ML fuzzing and intrusion methods.<br><br>
<!--  -->
		Only half of humanity are online at the moment; a single script-kiddie troll can do quite a lot; the internet is about to get bigger, louder, and <a href="{{strange}}">stranger</a>. 
	</div>
</div>

<br>
<hr />
<br>

### _First_: password hygiene

#### Attack: password cracking
If people hack a website you're registered on, they could easily get the encrypted 'hash' of your password even if the site owners do everything right. These can eventually be brute-force decoded, and then they have your password. To prevent <a href="{{pwned}}">this common occurrence</a>, we need our passwords to be very long (16 characters +) and have no English words. You also want a different password for each site, so that one brute-force doesn't open up all of your accounts at once.
So, easy!: We want passwords that are too hard to remember, and we need to never reuse any of them.

<span style="font-weight:bold"><span style="font-weight:bold">Mitigation</span></span>: A 'password manager', for instance the free, open-source, cross-platform <a href="{{kee}}">KeePassXC</a>. Keep the database file on several devices, and on a thumb drive, and an <a href="{{offs}}">offsite</a>. Can put it in the cloud if you think you're likely to lose those. LastPass and 1Password seem fine, maybe a bit slicker and more friendly, but they cost.

You can also sign up to the security researcher Troy Hunt's <a href="{{troy}}">notification tool</a>: whenever a big leak becomes publicly known, he'll scan it for you and email you if you're in it.

<br>

#### Attack: password phishing

People can create convincing clones of websites just so you give them your password freely. (This isn't just about human inattention: attackers can register <a href="{{homoglyphs}}">urls which look exactly like the real one</a>).

<span style="font-weight:bold"><span style="font-weight:bold">Mitigation</span></span>: Password manager / no password reuse.

<span style="font-weight:bold"><span style="font-weight:bold">Real mitigation</span></span>: Two-factor authentication (2FA) everywhere you can, e.g. via a Universal device like <a href="{{yubi}}">Yubikey</a>. If the site doesn't ask you for the access code from your phone when you sign in, you _immediately_ change your password (from the top search result for that site). 

(Sadly, SMS confirmation is relatively easy to subvert, so you should use a smartphone. An open-source 2FA app, <a href="{{auth}}">Authenticator</a>, is coming along though.)

<span style="font-weight:bold">Cognitive burden</span>: once you have the Master passphrase memorised (not hard, give it a couple days): much less than remembering 40 different passwords.

<br>

In early 2019, there was splashy media coverage of a <a href="{{pwordman}}">vulnerability in all the big password managers</a>. It's true that decoded passwords you've used during a session can persist in your RAM; however, it's of little importance, since if an attacker is in a position to read arbitrary things off your RAM, you are already as screwed as you can be. (KeePass was the least vulnerable manager, incidentally.)

<br>
<hr />
<br>

### _Then_: Browser


#### Attacks: IP tracking, unencrypted traffic, ISP logs, public wifi spoofing, geo-locking, national bans

In many jurisdictions (e.g. <a href="{{ispuk}}">UK</a>) your internet provider is legally required to record some info about your browsing. In others (<a href="{{ispus}}">US</a>) they do it apparently for kicks. They also implement <a href="{{ban}}">court orders</a> banning particular sites. Some content is only licenced for computers in particular locations. _And_ using public wi-fi (airports, coffee shops) is also extremely insecure without extra encryption.

<span style="font-weight:bold">Partial mitigation for all these</span>: a VPN. This is highly imperfect but not as useless <a href="{{sucks}}">as this guy thinks</a>. They at least have some incentive not to log you: no one will use a VPN which is known to log. I use <a href="{{pia}}">PrivateInternetAccess</a>; you can check the technical and legal specs of dozens of VPNs <a href="{{compare}}">here</a> or just get good live recommendations <a href="{{restore}}">here</a>. $30 a year. Do not use free ones.

The other problem a VPN solves, and solves optimally, is internet requests sent by non-browser apps on your machine. If you use e.g. Linux's built-in VPN client, everything goes through it.

You should not consider this strong privacy, cover for anything illegal. It's just the minimum required to _do it_ in the first place nowadays.

(NB: Modern browsers have a useful thing called <a href="{{rtc}}">WebRTC</a>. It leaks your IP though, so if you really want to hide that you'll need to go into `about:config` and set `media.peerconnection.enabled` to false. uBlock seems to fix this too.)


<br>


#### Attack: Man-in-the-Middle

Even when the URL is real, vulnerabilities in the original internet protocol mean people can sometimes insert themselves inbetween your data and the receiving site. This is lethal (think online shopping, online banking). <a href="{{https}}">This add-on</a> prevents this where it can.

(Previously I recommended <a href="{{httpse}}">HTTPS Everywhere</a>, but that depends on a big central database and sends all your requests there, which - though they're lovely people doing this for excellent reasons - is somewhat counter to the spirit of the thing.)

<br>

#### Attack: Tracking and fingerprinting

There are many, many ways to identify someone on the internet, from obvious ones like IP to desperately cunning ones like <a href="{{canvas}}">making your graphics card identify itself</a> or spotting you based on <a href="{{typ}}">the way you type</a>. Here are some reputable add-ons for Firefox that kill most of this:

* <a href="{{noscript}}">NoScript</a>. Disables all Javascript by default; this stops 90% of attacks and trackers. It is the most important, but also the most costly in time by far. It remembers which sites you let through though, so after about two weeks this burden becomes negligible. NoScript has a bunch of other cool protections too, vs XSS, clickjacking...
* <a href="{{badger}}">Privacy Badger</a>. Watches for processes sending information about you. Trying to fix sites' incentives by not blocking sites whose content actually obeys your Do No Track settings. Seems to cover the use case for both Disconnect and Ghostery.
* <a href="{{ddg}}">DuckDuckGo</a>. The zero-tracking search engine. Not as good as Google, but it includes a built-in "use Google safely" command.
* <a href="{{cookie}}">Cookie Autodelete</a>. Deletes cookies (files placed on your computer to identify you) when the tab is closed. Good compromise. <a href="#fn:3" id="fnref:3">3</a>
* <a href="{{fb2}}">Facebook Container</a>. Facebook follows you around the internet to a surprising degree - e.g. any time you see a "Login via Facebook" button or a social-media bar with Share buttons, FB polls its cookies to tie you to that site. They sell this to advertisers, which explains the eerie echo effect of your searches. This official Mozilla extension puts the FB cookies in a "container", an impenetrable box, stopping the passive tracking (they'll still get you if you click the buttons). 

I imagine everyone who will already has, but: consider quitting Facebook or <a href="{{feed}}">neutering it</a>. You can download all your data from them <a href="{{dl}}">here</a>, with like a week of waiting.

<br>

#### Attack: Ads

This one is arguable: the current web economy couldn't exist without ads. My response is to precommit to using any micropayment solution that people can get to work. Also to actually buy things from creators I like. In the meantime no-one gets to spam me with gigabytes of ugly unwanted content and follow me around.

But besides being ugly, besides following you without your consent, they take your time. <a href="{{hulce}}">Two-thirds</a> of all script execution time is due to third-party scripts, mostly ads and trackers. My own network analytics say that 15% of all my requests are to ad servers. This is hours of your life per year. <a href="#fn:1" id="fnref:1">1</a>

Everyone knows <a href="{{abp}}">this solution</a>, but a better solution takes a bit of work:

<a href="{{troyPi}}">The best thing</a> to do against ads, at present, is a <a href="{{hole}}">Pi-hole</a>, a tiny DNS server in your house. This stops ads at the source, for every device in your house at once. You can get <a href="{{pi}}">a Raspberry Pi</a> for $30, and it takes about 30 mins to set up as a Pi-hole. 

Another benefit of doing this at the router level is that it gives you a nice (rudimentary) network dashboard:

<div id="piholeImg"></div><br>

<!-- (Note that Chrome and Edge users _need_ DNS-level blocking like Pi-hole, since Google <a href="{{googblocked}}">is/was going to block</a> uBlock.)
 -->
Because the internet is a Red Queen hellscape, we should expect this to gradually stop working over the next few years. Ads can avoid a DNS block in <a href="{{hell}}">a variety of ways</a>, up to and including them implementing their own custom domain-over-HTTPS protocol. <i>La lotta continua.</i>


<!-- (Alternatively, there's the fun option:

* <a href="{{nausea}}">AdNauseam</a>. uBlock, plus trolling: AdNauseam clicks every ad it finds (without loading them), which undermines the surveillance economy by injecting large amounts of noise into it. (Real clickthroughs are rare.) Just use uBlock or Pi-hole if you have rule-utilitarian scruples.)
 -->

<br>

#### Attack: email surveillance

Not a lot you can do, short of undertaking the 100-hour hell of runnning your own mail server. Try a Swiss company, e.g. <a href="{{proton}}">Protonmail</a> (they have no public data-sharing agreement with the <a href="{{five}}">Five Eyes</a> and constitutional protections for foreigners).

Important caveat: you _really_ need to backup your Protonmail password well: If you lose it and reset, <a href="{{reset}}">you lose your email history</a>. This is the harsh nature of strong security.

> Because of the encryption we use to protect your data, resetting your Login password in ProtonMail is different from other, less secure email services. Your password is used to decrypt your emails, and we do not have access to it. Therefore, if you forget your password, you will lose the ability to read your existing emails.

PS: <a href="{{outlook}}">Hotmail and Outlook</a> have been a dumpster fire for many years.

<br>

#### Attack: deanonymisation
No <a href="{{who}}">whois</a> entry on your sites. People will try and charge you $10 for this but it is mandated by GDPR so shop around.

<br>

#### Attack: tracking over CDNs

A new clever attack: identifying you by your repeat requests to a public Content Delivery Network. This add-on <a href="{{decentral}}">DecentralEyes</a> foils this by keeping a copy of commonly-used files in your cache.

<br>

<hr />

<br>

<big>Total annual cost: $45 </big><br>
($40 VPN, $2 usb drive for your password DB	+ maybe <a href="{{piholeCost}}">$4</a> electricity for the Pi-hole.)

<br>

<big>Daily time cost: Net time saving? </big><br>
You'll take a minute a day adding new sites to your NoScript list. And Captchas pop up more often without cookies. But the Pi-hole speeds up your internet by ~10% by not loading ads. And once you get the KeePass keyboard shortcuts in your muscle memory it is faster than typing. So net gain.
<br><br>

---

<br>


<div class="accordion">
    <h3>Add-on risk</h3>
    <div>
	Whenever you install a browser add-on, you're allowing unknown code to execute on your machine, behind NoScript. Processes are "sandboxed" in modern browsers - that is, browser malware is unlikely to break into your main OS account - but this is still a risk.
	<!--  -->
	Worst is when someone replaces an honest add-on with a malwared version. This is not hypothetical: for example, part of the Python central package repository <a href="{{pypi}}">was subverted in 2017</a>. And it can take months for someone to notice this.
	<!--  -->
	However, you can be very confident in EFF and Mozilla products - HTTPS Everywhere, Privacy Badger, Containers - and relatively confident in popular open-source add-ons like <a href="{{openNo}}">NoScript</a>, <a href="{{autodel}}">Cookie-Autodelete</a>, <a href="{{ublock}}">uBlock</a>, especially if you built from source.
	<!--  -->
	Still, lean toward avoiding others.
	</div>
<!--  -->
    <h3>More things you could do:</h3>
    <div>

<ul>
	<li> <a href="{{beginux}}">Get Linux</a> (<a href="{{linux}}">99</a>%+ of malware doesn't work on it, and there's strong prevention of state backdoors and 'security through obscurity' zero-days).</li>
	<li>Turn off <a href="{{ff}}">these Firefox configs</a>.</li>
	<li>"Hacker tape" (putting a removable cover over your webcam) is a successful meme. Good for it! But an <a href="{{captivated}}">even more significant risk</a> is the <a href="{{mic}}">built-in mic</a>: your unguarded speech is a much more <i>high-res</i> thing to use against you. (Imagine your employer hearing you complain about them to your partner.) <a href="{{dummy}}">One solution</a> is leaving a 3.5mm jack plugged-in, with the wire trimmed off (and the wires taped-up separately to prevent a short circuit!) - but this is still software-mediated rather than hardware, and so could conceivably be bypassed.</li>
	<li>Add an additional <a href="{{keyfile}}">keyfile</a> for Keepass, on a USB. This is too far for me. You'd want it <a href="{{sweden}}">attached to your body</a>.</li>
	<li><a href="{{tor}}">Tor</a>. Slow!</li>
	<li><a href="{{cvb}}">CanvasBlocker</a>: people can get a wee bit of identifying info from <a href="{{canvas}}">spying on</a> your GPU and screen specs.</li>
	<li>Airgapping one of your computers.</li>
	<li><a href="{{clean}}">ClearURLs</a> (truncate the identifying info from the end of your links).</li>
	<li><a href="{{exfil}}">CSS Exfil Protection</a> (yet another graphical fingerprinting technique).</li>
	<li>Consider not using <a href="{{huawei}}">Chinese</a> <a href="{{linder}}">hardware</a>.</li>
<li>Consider not using <a href="{{nsa}}">American hardware</a>.</li>
<li>Consider not using Kaspersky (sad - seems to have been involuntary aid to Putin's people).</li>
<li><a href="{{bank}}">Two-factor authenticated bank</a>.</li>
<li><a href="{{rua}}">RandomUserAgent</a>: changes the device and browser you're reporting, at random. Sometimes breaks things.</li>
<li>Store a PGP key somewhere public (e.g. <a href="{{keybase}}">Keybase</a>): makes it possible to authenticate yourself without identifying documents. (Softening the blow of identity theft, preventing chronic lulz).</li>
<li><a href="{{faraday}}">Faraday wallet</a> for phone and contactless card. Obviously this prevents all incoming calls too.</li>
<li>Life / work separation. Never shop at work, never work on your home computers. This makes two of you, with two different attacks (and sets of attacks) needed.</li>
<li>_Phone_: The iPhone's encryption has been defended in court against heavy pressure, but also subverted by <a href="{{israe}}">commercial tools</a>. The <a href="{{librem}}">Librem 5</a> will be better on many axes - hardware control, OS security, supply chain ethics - but is unlikely to do better in crypto.</li>
<li>Against reward hacking (that is, being distracted with push notifications and infinite feeds): Just don't have a smartphone, or keep it in your bag and use a dumbphone for interpersonal alerts. Also <a href="https://addons.mozilla.org/en-US/firefox/addon/impulse-blocker/">ImpulseBlocker</a>.</li>

</ul>

</div>
</div>

<br>

--- 

<br>

<a href="{{eff}}">Here's a couple</a> <a href="{{dennis}}">of good tools</a> for seeing if this does the trick.

Note that you're not going to stop any nation-states except <a href="{{paranoia}}">via perfect paranoia</a>, the kind which makes the above look sloppy and carefree. Luckily, that effort is not worthwhile for almost anyone.


<br><br>

## See also

* [Your Computer Isn't Yours](https://sneak.berlin/20201112/your-computer-isnt-yours/)
* Violet Blue on <a href="{{vtrack}}">resisting tracking</a>, <a href="{{vsurv}}">surveillance</a>, <a href="{{vdevices}}">devices</a>.
* <a href="{{fsec}}">F-Secure on the whole deal</a>.
* https://cheapskatesguide.org/articles/no-such-thing-as-secure.html

{%  include browser/foots.html %}


<br><br>



<script>  
	var src = "{{piholeImg}}";
	definiteEvent( createImg, [src, "piholeImg"] ); 

	var src = "{{attackImg}}";
	definiteEvent( createImg, [src, "attackImg"] ); 
</script>


