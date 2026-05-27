//! Render a static d3 visualization of citation rot from
//! `.archiver/measurements.json`. Writes a single self-contained HTML file
//! (data embedded inline; only external dep is d3 from a CDN) at
//! `<repo>/archive-health.html`.

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;

use crate::{config, measure, ui};

pub fn run(
    labels: Option<String>,
    exclude_urls: Option<String>,
    output: String,
) -> Result<()> {
    let mut data = measure::load().context("load measurements")?;
    if data.posts.is_empty() {
        ui::warn("no measurements yet. run `archiver measure <post-url> [--label X]` first.");
        return Ok(());
    }

    if let Some(allowlist) = labels.as_deref() {
        let allowed: HashSet<&str> =
            allowlist.split(',').map(|s| s.trim()).collect();
        data.posts.retain(|p| allowed.contains(p.label.as_str()));
        ui::info(&format!(
            "label filter: keeping {} post(s) matching {:?}",
            data.posts.len(),
            allowed
        ));
    }
    if let Some(blocklist) = exclude_urls.as_deref() {
        let blocked: HashSet<&str> =
            blocklist.split(',').map(|s| s.trim()).collect();
        let before = data.posts.len();
        data.posts.retain(|p| !blocked.contains(p.url.as_str()));
        ui::info(&format!(
            "url exclude: dropped {} post(s)",
            before - data.posts.len()
        ));
    }
    if data.posts.is_empty() {
        ui::warn("no posts left after filter — nothing to render");
        return Ok(());
    }

    let json = serde_json::to_string(&data)?;
    let html = render_html(&json, data.posts.len());
    let path = config::repo_root().join(&output);
    fs::write(&path, html)?;
    ui::success("viz", &path.to_string_lossy());
    ui::info(&format!("open: file://{}", path.to_string_lossy()));
    Ok(())
}

fn render_html(data_json: &str, n_posts: usize) -> String {
    // Defang any embedded `</` so a stray URL can't close our <script> tag.
    let safe = data_json.replace("</", "<\\/");
    HTML_TEMPLATE
        .replace("__DATA__", &safe)
        .replace("__N_POSTS__", &n_posts.to_string())
}

const HTML_TEMPLATE: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>archive health · croissanthology</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
  :root {
    --bg: #fdf6f0;
    --ink: #2b2b3d;
    --muted: #7a7a8c;
    --card: #ffffff;
    --post: #5a8fdb;
    --alive: #6cb37a;
    --dead: #e26b6b;
    --transient: #e8b341;
    --unknown: #9aa0a6;
    --thread: rgba(43, 43, 61, 0.18);
    --thread-dead: rgba(226, 107, 107, 0.55);
    --trans-blue: #5bcefa;
    --trans-pink: #f5a9b8;
  }
  * { box-sizing: border-box; }
  body {
    margin: 0; padding: 0; background: var(--bg); color: var(--ink);
    font-family: 'Iowan Old Style', 'Palatino', Georgia, serif;
    line-height: 1.55;
  }
  .stripe {
    height: 6px;
    background: linear-gradient(90deg,
      var(--trans-blue), var(--trans-pink), #fff,
      var(--trans-pink), var(--trans-blue));
  }
  header { padding: 48px 32px 8px; max-width: 1100px; margin: 0 auto; }
  h1 { font-size: 2.3rem; margin: 0 0 8px; letter-spacing: -0.01em; }
  header p { color: var(--muted); margin: 0 0 10px; max-width: 700px; }
  .legend {
    display: flex; gap: 18px; flex-wrap: wrap;
    margin: 24px 0 12px; font-size: 0.92rem; color: var(--muted);
  }
  .legend span { display: inline-flex; align-items: center; gap: 6px; }
  .legend i { width: 10px; height: 10px; border-radius: 50%; display: inline-block; }
  #grid {
    padding: 16px 32px 48px;
    max-width: 1180px; margin: 0 auto;
  }
  .group { margin-bottom: 32px; }
  .group h2 {
    font-size: 1.35rem; margin: 0 0 12px;
    display: flex; align-items: baseline; flex-wrap: wrap; gap: 12px;
  }
  .group h2 .agg {
    font-size: 0.88rem; color: var(--muted); font-weight: 400;
  }
  .group h2 .agg b {
    color: var(--ink); font-weight: 600;
  }
  .group h2 .rot-bad { color: var(--dead); font-weight: 600; }
  .group h2 .rot-good { color: var(--alive); font-weight: 600; }
  .cards {
    display: grid; gap: 18px;
    grid-template-columns: repeat(3, 1fr);
  }
  @media (max-width: 980px) { .cards { grid-template-columns: repeat(2, 1fr); } }
  @media (max-width: 640px) { .cards { grid-template-columns: 1fr; } }
  .card {
    background: var(--card); border-radius: 14px; padding: 14px 16px 10px;
    box-shadow: 0 4px 20px rgba(43, 43, 61, 0.05);
    border: 1px solid rgba(43, 43, 61, 0.04);
  }
  .card h3 { margin: 0 0 2px; font-size: 0.95rem; line-height: 1.3;
    overflow: hidden; text-overflow: ellipsis;
    display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; }
  .card .stats { font-size: 0.78rem; color: var(--muted); margin-bottom: 4px; }
  .card .stats b { font-weight: 600; color: var(--ink); }
  .card svg { width: 100%; height: 260px; cursor: default; }
  .node { cursor: pointer; transition: r 0.15s ease; }
  .node:hover { stroke: var(--ink); stroke-width: 1.5; }
  .node-post {
    fill: var(--post); stroke: var(--ink); stroke-width: 1.5;
    cursor: pointer;
  }
  .node-alive { fill: var(--alive); }
  .node-dead { fill: var(--dead); }
  .node-transient { fill: var(--transient); }
  .node-unknown { fill: var(--unknown); }
  .thread { stroke: var(--thread); stroke-width: 0.7; fill: none; }
  .thread-dead { stroke: var(--thread-dead); stroke-width: 1.1; }
  footer {
    max-width: 1100px; margin: 0 auto;
    padding: 8px 32px 48px; color: var(--muted); font-size: 0.85rem;
  }
  code { font-family: ui-monospace, 'SF Mono', Menlo, monospace; }
</style>
</head>
<body>
<div class="stripe"></div>
<header>
  <h1>the web rots, in pictures</h1>
  <p>__N_POSTS__ essays, each fanned out by citation. every small dot is an outbound link the post relies on, colored by whether it's still answering the door. hover for the url, click to open it.</p>
  <div class="legend">
    <span><i style="background: var(--post)"></i> source post</span>
    <span><i style="background: var(--alive)"></i> alive</span>
    <span><i style="background: var(--dead)"></i> dead</span>
    <span><i style="background: var(--transient)"></i> transient (5xx, auth, timeout)</span>
    <span><i style="background: var(--unknown)"></i> unknown</span>
  </div>
</header>

<div id="grid"></div>

<footer>generated by <code>archiver measure</code> + <code>archiver viz</code> · croissanthology</footer>

<script type="application/json" id="data">__DATA__</script>
<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
(function () {
  var raw = document.getElementById('data').textContent;
  var data = JSON.parse(raw);
  var grid = d3.select('#grid');

  // Group posts by label, preserving first-seen order from the JSON.
  var byLabel = {};
  var labelOrder = [];
  data.posts.forEach(function (p) {
    if (!byLabel[p.label]) { byLabel[p.label] = []; labelOrder.push(p.label); }
    byLabel[p.label].push(p);
  });

  labelOrder.forEach(function (label) {
    var group = byLabel[label];
    var agg = group.reduce(function (a, p) {
      return {
        total: a.total + p.total_links,
        alive: a.alive + p.alive,
        dead: a.dead + p.dead,
        transient: a.transient + p.transient,
        unknown: a.unknown + p.unknown
      };
    }, { total: 0, alive: 0, dead: 0, transient: 0, unknown: 0 });
    var rotPct = agg.total
      ? Math.round(100 * (agg.dead + agg.transient) / agg.total)
      : 0;
    var rotClass = rotPct >= 20 ? 'rot-bad' : (rotPct <= 5 ? 'rot-good' : '');

    var section = grid.append('section').attr('class', 'group');
    section.append('h2').html(
      label +
      '<span class="agg">' +
      agg.total + ' links · alive <b>' + agg.alive + '</b> · ' +
      'dead <b>' + agg.dead + '</b> · transient <b>' + agg.transient + '</b> · ' +
      '<span class="' + rotClass + '">rot ' + rotPct + '%</span>' +
      '</span>'
    );
    var cards = section.append('div').attr('class', 'cards');

    group.forEach(function (post) {
      renderCard(cards, post);
    });
  });

  function renderCard(parent, post) {
    var card = parent.append('div').attr('class', 'card');
    card.append('h3').attr('title', (post.title || '') + '\n' + post.url)
      .text(post.title || post.url);
    var total = post.total_links;
    var pct = total ? Math.round(100 * post.alive / total) : 0;
    card.append('div').attr('class', 'stats').html(
      total + ' links · alive <b>' + post.alive + '</b> (' + pct + '%) · ' +
      'dead <b>' + post.dead + '</b> · transient <b>' + post.transient + '</b>'
    );

    var W = 400, H = 260, cx = W / 2, cy = H / 2;
    var svg = card.append('svg').attr('viewBox', '0 0 ' + W + ' ' + H);

    var nodes = [{ id: '__post__', type: 'post', label: post.label, url: post.url, fx: cx, fy: cy }]
      .concat(post.links.map(function (l, i) {
        return {
          id: 'l-' + i, type: 'leaf',
          status: l.status, url: l.url, host: l.host, http: l.http
        };
      }));

    var links = post.links.map(function (_, i) {
      return { source: '__post__', target: 'l-' + i, status: post.links[i].status };
    });

    var sim = d3.forceSimulation(nodes)
      .force('link', d3.forceLink(links).id(function (d) { return d.id; })
        .distance(55).strength(0.85))
      .force('charge', d3.forceManyBody().strength(-18))
      .force('center', d3.forceCenter(cx, cy).strength(0.05))
      .force('collide', d3.forceCollide(5));

    var linkSel = svg.append('g').attr('class', 'threads')
      .selectAll('line').data(links).join('line')
      .attr('class', function (d) {
        return d.status === 'dead' ? 'thread thread-dead' : 'thread';
      });

    var nodeSel = svg.append('g').attr('class', 'nodes')
      .selectAll('circle').data(nodes).join('circle')
      .attr('class', function (d) {
        if (d.type === 'post') return 'node node-post';
        return 'node node-' + d.status;
      })
      .attr('r', function (d) { return d.type === 'post' ? 9 : 3.6; })
      .on('click', function (_, d) {
        if (d.url) window.open(d.url, '_blank', 'noopener');
      });

    nodeSel.append('title').text(function (d) {
      if (d.type === 'post') return d.label + '\n' + d.url;
      var code = d.http ? ' [' + d.http + ']' : '';
      return d.status + code + '\n' + d.url;
    });

    sim.on('tick', function () {
      linkSel
        .attr('x1', function (d) { return d.source.x; })
        .attr('y1', function (d) { return d.source.y; })
        .attr('x2', function (d) { return d.target.x; })
        .attr('y2', function (d) { return d.target.y; });
      nodeSel
        .attr('cx', function (d) { return d.x; })
        .attr('cy', function (d) { return d.y; });
    });
  }
})();
</script>
</body>
</html>
"#;
