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
categories: 
count:      1050
---

{%  include browser/links.md  %}

<br>

It's now common knowledge that we're being watched online, by a <a href="{{mix}}">thick mix</a> of <a href="{{xkey}}">nation</a>-<a href="{{ech}}">states</a>, <a href="{{enemy}}">private</a> <a href="{{fb}}">companies</a>, <a href="{{spy}}">and</a> <a href="{{play}}">criminals</a>. They sometimes do <a href="{{mal}}">worse than watch</a>. What do we do? Should we care?

It's not very clear what the probability of having your password leaked in a breach / having your email read / having your laptop being remotely disabled and wiped (unless you pay the creator Bitcoin) is. But one of these will probably happen to you in your lifetime, so I would take 10 mins to mitigate them now.

There is no absolute security; it's always partial and relative to a goal. This guide is aimed at "not losing control of your accounts, not being surveilled by companies or criminals, not having your online banking subverted, not getting infected by trojans or ransomware or whatever". It's strictly for people with average risks: not that much money, not much tech cred, not much sensitive information to protect.




Most of this article assumes you're using Firefox, because <a href="{{chrome}}">Chrome</a> (<a href="{{safari}}">and Google services on other browsers</a>) is or was an attack itself. That is, it protects you very well against everyone except Google. Firefox is also <a href="{{incog}}">significantly faster</a> than Chrome in Private mode. It's not a big deal compared to the other parts of this list, you'll just need to find alternatives to the add-ons I recommend.

<br>
<hr />
<br>

### _First_: password hygiene

#### Attack: password cracking
If people hack a website you're registered on, they could easily get the encrypted 'hash' of your password even if the site owners do everything right. These can eventually be brute-force decoded, and then they have your password. To prevent <a href="{{pwned}}">this common occurrence</a>, we need our passwords to be very long (16 characters +) and have no English words. You also want a different password for each site, so that one brute-force doesn't open up all of your accounts at once.
So, easy!: We want passwords that are too hard to remember, and we need to never reuse any of them.

<span style="font-weight:bold"><span style="font-weight:bold">Mitigation</span></span>: A 'password manager', for instance the free, open-source, cross-platform <a href="{{kee}}">KeePassX</a>. Keep the database file on several devices, and on a thumb drive, and an <a href="{{offs}}">offsite</a>. Can put it in the cloud if you think you're likely to lose those. LastPass and 1Password seem fine, maybe a bit slicker and more friendly, but they cost.

<br>

#### Attack: password phishing

People can create convincing clones of websites just so you give them your password freely. (This isn't just about human inattention: attackers can register <a href="{{homoglyphs}}">urls which look exactly like the real one</a>).

<span style="font-weight:bold"><span style="font-weight:bold">Mitigation</span></span>: Password manager / no password reuse.

<span style="font-weight:bold"><span style="font-weight:bold">Real mitigation</span></span>: 2FA everywhere you can, <a href="{{yubi}}">Yubikey</a>. If the site doesn't ask you for the access code from your phone, you should immediately change your password (from the top search result for that site).

<span style="font-weight:bold">Cognitive burden</span>: once you have the Master passphrase memorised (not hard, give it a couple days): much less than remembering 40 different passwords.

<br>
<hr />
<br>

### _Then_: Browser


#### Attacks: IP tracking, unencrypted traffic, ISP logs, public wifi spoofing
    
<span style="font-weight:bold">Partial mitigation</span>: VPN. This is highly imperfect but not as useless <a href="{{sucks}}">as this guy thinks</a>. I use <a href="{{pia}}">PrivateInternetAccess</a>; check technical and legal specs. £30 a year. Do not use free ones.

The problem a VPN solves optimally is internet requests by non-browser apps. If you use e.g. Linux's built-in VPN client, everything goes through

(NB: Modern browsers have this useful thing called <a href="{{rtc}}">WebRTC</a>. It leaks your IP, so if you really want to hide that you'll need to go into `about:config` and set `media.peerconnection.enabled` to false. uBlock seems to fix this too.)


<br>


#### Attack: Man-in-the-Middle

Even when the URL is real, vulnerabilities in the original internet protocol mean people can sometimes insert themselves inbetween your data and the receiving site. This is lethal (think online shopping, online banking). <a href="{{https}}">This add-on</a> prevents this where it can.

<br>

#### Attack: Tracking and fingerprinting

* <a href="{{cookie}}">Cookie Autodelete</a>. This add-on deletes cookies (files placed on your computer to identify you) when the tab is closed. Good compromise.
* <a href="{{noscript}}">NoScript</a>. Disables all Javascript by default; this stops 90% of attacks and trackers. It is the most important, but also the most costly in time by far. After about two weeks of use this burden decreases to negligible though.
* <a href="{{nausea}}">AdNauseam</a>. uBlock plus trolling: clicks every ad it finds (without loading them), which thus undermines the tracking surveillance system by injecting large amounts of noise. (Real clickthroughs are rare.) Just use uBlock or AdBlockPlus if you have rule-utilitarian scruples.
* <a href="{{badger}}">Privacy Badger</a>. Overlaps a bit with AdNauseam. Seems to cover the use case for Disconnect and Ghostery.
* <a href="{{ddg}}">DuckDuckGo</a>. Zero-tracking search engine. Not quite as good as Google but it includes a built-in "use Google safely" command.
* <a href="{{rua}}">RandomUserAgent</a>: changes the device and browser you're reporting, at random. Sometimes breaks things.

<br>

#### Attack: email surveillance

Not a lot you can do, short of undertaking the 100-hour hell of runnning your own mail server. Try a Swiss company, e.g. <a href="{{proton}}">Protonmail</a> (they have no public data-sharing agreement with the Five Eyes and constitutional protections for foreigners).

<br>

#### Attack: deanonymisation
No <a href="{{who}}">whois</a> entry on your sites. People will try and charge you £10 for this but it is mandated by GDPR so shop around.

<br>

#### Attack: tracking over CDNs

A new clever attack: identifying you by your repeat requests to a public Content Delivery Network. The add-on <a href="{{decentral}}">DecentralEyes</a> foils this by keeping a copy of commonly used files in your cache.


<br>

<hr />

<br>

<big>Total annual cost: $40 ($40 VPN, $2 usb drive for your password DB)</big>

<big>Daily time cost: 10 seconds adding particular NoScripted scripts. Once you get the KeePass keyboard shortcuts in your muscle memory it is faster than typing.</big>

<br>

---

<br>

### Add-on risk

Whenever you install a browser add-on, you're allowing unknown code to execute on your machine, behind NoScript. Processes are "sandboxed" in modern browsers - that is, browser malware is unlikely to break into your main OS account - but this is still a risk.

However, you can be very confident in EFF products - HTTPS Everywhere, Privacy Badger - and relatively confident in popular open-source add-ons like <a href="{{openNo}}">NoScript</a>, <a href="{{autodel}}">Cookie-Autodelete</a>, <a href="{{ublock}}">uBlock</a>, and <a href="{{ruaCode}}">RandomUserAgent</a>, especially if you built from source.


<br>

---

<br>

### More things you could do:

* Turn off <a href="{{ff}}">these Firefox configs</a>.
* Get Linux (<a href="{{linux}}">99</a>%+ of malware doesn't work on it, and there's strong prevention of state backdoors and 'security through obscurity' zero-days).
* Add an additional <a href="{{keyfile}}">keyfile</a> for Keepass, on a USB. This is too far for me. You'd want it <a href="{{sweden}}">attached to your body</a>.
* <a href="{{tor}}">Tor</a>. Slow!
* <a href="{{faraday}}">Faraday wallet</a> for phone and contactless card. Obviously this prevents all incoming calls too.
* Airgapping one of your computers.
* Consider not using <a href="{{huawei}}">Chinese</a> <a href="{{lindner}}">hardware</a>.
* Consider not using <a href="{{nsa}}">American hardware</a>.
* Consider not using Kaspersky (involuntary aid).
* <a href="{{bank}}">Two-factor authenticated bank</a>.
* Store a PGP key somewhere public (e.g. <a href="{{keybase}}">Keybase</a>): makes it possible to authenticate yourself without identifying documents. (Softening the blow of identity theft, preventing chronic lulz).
* Life / work separation. Never shop at work, never work on your home computers. This makes two of you, with two different attacks (and sets of attacks) needed.
* Against reward hacking (that is, being distracted with push notifications and infinite feeds): Just don't have a smartphone, or keep it in your bag and use a dumbphone for interpersonal alerts. Also <a href="https://addons.mozilla.org/en-US/firefox/addon/impulse-blocker/">ImpulseBlocker</a>.

<br>

--- 

<br>

<a href="{{eff}}">Here's a good tool</a> for seeing if this does the trick.

Note that you're not going to stop any nation-states except <a href="{{paranoia}}">via perfect paranoia</a>, the kind which makes the above look sloppy and carefree. Luckily, that effort is not worthwhile for almost anyone.


<br><br>

{%  include comments.html %}