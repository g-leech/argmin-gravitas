# Argmin-gravitas: a simulacrum standing in place of Gavin Leech

Hosted at [www.gleech.org](https://www.gleech.org)

# Commands

## Run local dev server

```bash
./scripts/dev-server.sh
```

## Update Gemfile.lock

```bash
./scripts/bundle-lock.sh
```

## Build site

```bash
./scripts/build.sh
```

# Github actions

Github actions are included for netlify and cloudflare deploys.  These use the build scripts to ensure version compatibility and reproducibility.

A Github action for checking pull requests is included.

See `.github/workflows`

## Build site

### work that matters

[x] make everything mobile friendly

[x] set up a proper typography scale and spacing system (this might make br tags take up too much space)

[ ] better site architecture + navigation

### changes

[x] got rid of 7.1MB (???) SVG for the Album of the Year icon 

[x] removed loazyload.js and used loading="lazy" on all images that were using it 

[x] removed need for padder partial by fixing the body height to always be tall enough

[x] got rid of a dozen inline stylesheets

[x] add 404 page

[x] why does code [/code] page not have an H1?

[x] remove need for .phone_img class by setting max height for images

[x] make green dropdowns not have yellow underline and poor hover effect

[x] make blogroll [/blogroll] page nicer (why doesn't it have an H1?)

[x] <a> tags need to break at some point to prevent overflow on mobile

[x] [/importance] H2 underlines are ugly

[x] migrate from scss to modern css

[x] ~~hamburger menu can be better~~ removed need for hamburger menu on mobile

[x] reduced x-padding on mobile pages 

[x] made comment form nicer

[x] make tables scroll horizontally on mobile (need to surround them with a div with class="table-wrapper")

[x] remove dropdown from mc.html

[x] Added titles to pages with missing titles on the Archive page's Pages section and excluded 404 and feed.xml from showing up



## next up

[?] make images expand on clicking them

[x] metadata section on post pages could look better

[ ] is the script for accordions being cached?


## typography
[ ] create a new typography system, including:

	[x] better styles for H2 (color: green?)

	[ ] better-looking blockquotes for epigraphs

	[x] better margins for headings


## to fix on mobile

[x] about page

[x] research page

[x] home page

[x] blogroll

[x] archive page

[x] best [/best] page 

[ ] recs page (playlists)