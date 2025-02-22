```
sudo apt-get update
sudo apt-get install ruby-full build-essential zlib1g-dev

echo '# Install Ruby Gems to ~/gems' >> ~/.bashrc
echo 'export GEM_HOME="$HOME/gems"' >> ~/.bashrc
echo 'export PATH="$HOME/gems/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

gem install jekyll bundler
cd argmin-gravitas/
bundle update
gem install rubygems-update
sudo update_rubygems
bundle install

echo 'alias jekse="bundle exec jekyll serve --incremental"' >> ~/.bashrc
source ~/.bashrc
jekse
```

### work that matters
[ ] make everything mobile friendly
[ ] set up a proper typography scale and spacing system (this might make br tags take up too much space)
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

[ ] make best [/best] page neater + better for mobile
[?] make images expand on clicking them
[ ] make archive [/archive] page neater + better for mobile
[ ] metadata section on post pages could look better
[ ] is the script for accordions being cached?


## typography
[ ] create a new typography system, including:
	[x] better styles for H2 (color: green?)
	[?] better-looking blockquotes for epigraphs
	[?] paragraph tags need saner margins
	[x] better margins for headings
	[ ] why do headings and paragraphs have right margins?


## to fix on mobile
[ ] about page
[ ] research page
[ ] home page
[ ] blockquotes styling
[ ] definitely saner paddings on p tags
[ ] blogroll
[ ] archive page