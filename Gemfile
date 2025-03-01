# A sample Gemfile
source "https://rubygems.org"

# Strict Ruby version validation is useful for debugging version compatibility issues.
# ruby '~> 3.1.1'

gem "jekyll", "~> 4.3.3"

gem "webrick", "~> 1.8"

# Fix segmentation fault on musl libc
# https://github.com/sass-contrib/sass-embedded-host-ruby/issues/210
gem 'google-protobuf', force_ruby_platform: true if RUBY_PLATFORM.include?('linux-musl')