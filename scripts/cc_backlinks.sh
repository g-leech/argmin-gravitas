# credit to @retlehs
set -euo pipefail

DOMAIN="${1:-example.com}"
RELEASE="${CC_RELEASE:-cc-main-2026-jan-feb-mar}"
CACHE="${HOME}/.cache/cc-backlinks/${RELEASE}"
BASE="https://data.commoncrawl.org/projects/hyperlinkgraph/${RELEASE}/domain"

VERTICES="${CACHE}/domain-vertices.txt.gz"
EDGES="${CACHE}/domain-edges.txt.gz"

mkdir -p "$CACHE"

if ! command -v duckdb >/dev/null; then
  echo "error: duckdb not installed. Run: brew install duckdb" >&2
  exit 1
fi

# Reverse domain: roots.io -> io.roots
REV_DOMAIN=$(awk -F. '{for(i=NF;i>0;i--) printf "%s%s", $i, (i>1?".":"")}' <<<"$DOMAIN")

download() {
  local url="$1" dest="$2"
  if [[ -f "$dest" ]]; then return; fi
  echo ">> downloading $(basename "$dest") ..." >&2
  curl -L --fail -C - -o "$dest" "$url"
}

download "${BASE}/${RELEASE}-domain-vertices.txt.gz" "$VERTICES"
download "${BASE}/${RELEASE}-domain-edges.txt.gz"    "$EDGES"

echo ">> querying backlinks to ${DOMAIN} (reversed: ${REV_DOMAIN}) ..." >&2
echo ">> first run scans ~16 GB of gzipped edges; expect several minutes" >&2

duckdb <<SQL
.mode box
WITH vertices AS (
  SELECT * FROM read_csv('${VERTICES}', delim='\t', header=false,
    columns={'id':'BIGINT','rev_domain':'VARCHAR','num_hosts':'BIGINT'})
),
target AS (
  SELECT id FROM vertices WHERE rev_domain = '${REV_DOMAIN}'
),
inbound AS (
  SELECT from_id FROM read_csv('${EDGES}', delim='\t', header=false,
    columns={'from_id':'BIGINT','to_id':'BIGINT'})
  WHERE to_id = (SELECT id FROM target)
)
SELECT
  array_to_string(list_reverse(string_split(v.rev_domain, '.')), '.') AS linking_domain,
  v.num_hosts
FROM inbound i
JOIN vertices v ON v.id = i.from_id
ORDER BY v.num_hosts DESC, linking_domain;
SQL