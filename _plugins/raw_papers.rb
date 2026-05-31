# Serve files/papers/*.md as raw Markdown, verbatim, with their YAML front
# matter intact.
#
# Why: any file with YAML front matter is treated by Jekyll as a *page* and run
# through the Markdown converter, so files/papers/foo.md would be emitted only as
# files/papers/foo.html (HTML body, front matter dropped). The papers are indexed
# from llms.txt as raw .md URLs and their whole value is the machine-readable
# front matter, so we additionally copy each source file verbatim to the same
# .md path in _site. (The .html Jekyll also generates is harmless and unlinked.)
module Jekyll
  class RawPapersGenerator < Generator
    safe true
    priority :low

    PAPERS_DIR = "files/papers".freeze

    def generate(site)
      Dir.glob(File.join(site.source, PAPERS_DIR, "*.md")).each do |path|
        name = File.basename(path)
        site.static_files << Jekyll::StaticFile.new(site, site.source, PAPERS_DIR, name)
      end
    end
  end
end
