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
[ ] better site architecture (i have a long list)
[ ] 

### major changes

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


## next up

[ ] migrate from scss to modern css
[ ] make comment form nicer
[ ] make tables scroll horizontally on mobile
[ ] make best [/best] page neater + better for mobile

[ ] change dropdown and mce to use details element and no JS
[?] make images expand on clicking them
[ ] make archive [/archive] page neater + better for mobile
[ ] metadata section on post pages could look better
[ ] hamburger menu can be better
[ ] is the script for accordions being cached?
[ ] pages with missing titles on the Archive page's Pages section


## typography
[ ] create a new typography system, including:
	[ ] better styles for H2 (color: green?)
	[?] better-looking blockquotes for epigraphs
	[?] paragraph tags need saner margins
	[ ] better margins for headings


## to fix on mobile
[ ] about page
[ ] research page
[ ] home page
[ ] blockquotes styling
[ ] reduce x-padding on pages 
[ ] definitely saner paddings on p tags
[ ] blogroll
[ ] archive page