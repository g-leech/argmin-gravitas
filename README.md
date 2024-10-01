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
[ ] made comment form nicer
[ ] changed dropdown and mce to use details element and no JS
[ ] added social media cards (needs image)
[x] removed loazyload.js and used loading="lazy" on all images that were using it 
[x] removed need for padder partial by fixing the body height to always be tall enough
[ ] remove phone_img by setting max height for images
[x] got rid of a dozen inline stylesheets
[] get rid of base and layout.scss
[] what is bigfoot?