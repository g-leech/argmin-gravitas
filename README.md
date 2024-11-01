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

### major changes

[x] got rid of 7.1MB (???) SVG for the Album of the Year icon 
[x] removed loazyload.js and used loading="lazy" on all images that were using it 
[x] removed need for padder partial by fixing the body height to always be tall enough
[x] got rid of a dozen inline stylesheets

[ ] make comment form nicer
[ ] change dropdown and mce to use details element and no JS
[ ] remove need for .phone_img class by setting max height for images
[ ] make green dropdowns not have yellow underline and poor hover effect
[ ] make blogroll [/blogroll] page nicer (why doesn't it have an H1?)
[?] make images expand on clicking them
[ ] make archive [/archive] page neater + better for mobile
[ ] make best [/best] page neater + better for mobile
[ ] why does coding [/coding] page not have an H1?
[ ] add a 404 page
[ ] [/importance] H2 underlines are ugly
[ ] metadata section on post pages could look better
[ ] hamburger menu can be better
[ ] make tables scroll horizontally on mobile


## typography

[ ] better styles for H2 (color: green?)
[?] better-looking blockquotes for epigraphs
[?] paragraph tags need saner margins
[ ] headings need better margins
[ ] <a> tags need to break at some point to prevent overflow on mobile


## to fix on mobile
[ ] about page
[ ] research page
[ ] home page
[ ] blockquotes styling
[ ] reduce x-padding on pages 
[ ] definitely saner paddings on p tags
[ ] blogroll
[ ] archive page