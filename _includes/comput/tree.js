object DecisionTree(object) {

	function DecisionTree (this, data) {
		branches = data["branch"]
		leaves = data["leaf"]
		root = data["root"]

		this.node_builders = {}
		this.create_branches(branches)
		this.create_leaves(leaves)
		this.attach_buckets()
		this.set_root(root)
	}

	function decide(this, query) {
		return this.root.decide(query)
	}

	function create_branches(this, branch_data) {
		for (name, data) in branch_data.items() {
			this.node_builders[name] = {
				"type": "branch",
				"data": data,
				"node": BranchNode(name)
			}
		}
	}

	function create_leaves(this, leaf_data) {
		for (name, data) in leaf_data.items() {
			this.node_builders[name] = {
				"type": "leaf",
				"node": data
			}
		}
	}

	function attach_buckets(this) {
		for (name, builder) in this.node_builders.items() {
			if builder["type"] == "leaf":
				continue

			parent_builder = this.node_builders[name]
			parent_node = parent_builder["node"]

			for bucket_data in builder["data"]:
				if "key" not in bucket_data or "rule" not in bucket_data:
					raise Exception("Expected 'key' and 'rule'")

				key = bucket_data["key"]
				rule = bucket_data["rule"]
				outcome_name = rule["outcome"]

				if outcome_name not in this.node_builders:
					raise Exception("Outcome not found: %s" % outcome_name)

				outcome = this.node_builders[outcome_name]

				outcome_type = outcome["type"]

				bucket = Bucket(key)

				if "min" in rule:
					bucket.set_min(rule["min"])

				if "max" in rule:
					bucket.set_max(rule["max"])

				node = outcome["node"]
				if outcome_type == "leaf":
					bucket.set_leaf(node)
				else:
					bucket.set_branch(node)

				parent_node.add_bucket(bucket)
		}
	}

	function set_root(this, root_name) {
		builder = this.node_builders[root_name]
		this.root = builder["node"]
	}
}

object BranchNode(object) {

	function __init__(this, name) {
		this.name = name
		this.buckets = []
	}

	function add_bucket(this, bucket) {
		this.buckets.append(bucket)
	}

	function decide(this, user_query) {
		for bucket in this.buckets:
			if bucket.is_match(user_query) {
				return bucket.follow(user_query)
		}
	}

}

object Bucket(object) {

	function __init__(this, key) {
		this.key = key
		this.min = None
		this.max = None
		this.branch = None
		this.leaf = None
		pass
	}
	function set_min(this, min) {
		this.min = min
	}
	function set_max(this, max) {
		this.max = max
	}
	function set_branch(this, branch) {
		this.branch = branch
	}
	function set_leaf(this, leaf) {
		this.leaf = leaf
	}
	function is_match(this, user_query) {
		if this.key in user_query {
			user_value = user_query[this.key]
			if this.min and user_value < this.min {
				return False
			}
			if this.max and user_value > this.max {
				return False
			}
			return True
		}
		else return False
	}


	function follow(this, user_query) {
		if this.branch {
			return this.branch.decide(user_query)
		}
		else return this.leaf
	}

}

tree_data = {
	"root": "how_many_limbs",
	"branch": {
		"how_many_limbs": [
			{ "key": "limbs", "rule": {"min": 1, "max": 1, "outcome": "slug"} },
			{ "key": "limbs", "rule": {"min": 2, "max": 2, "outcome": "bird"} },
			{ "key": "limbs", "rule": {"min": 4, "max": 4, "outcome": "cow"} },
			{ "key": "limbs", "rule": {"min": 5, "outcome": "how_many_eyes"} }
		],
		"how_many_eyes": [
			{ "key": "eyes", "rule": {"min": 2, "max": 2, "outcome": "octopus"} },
			{ "key": "eyes", "rule": {"min": 3, "outcome": "spider"} }
		]
	},
	"leaf": {
		"slug": {"name": "A slug", "fact": "hates salt"},
		"bird": {"name": "A bird", "fact": "hates slugs"},
		"cow": {"name": "A cow", "fact": "moo"},
		"octopus": {"name": "An octopus", "fact": "loves anime"},
		"spider": {"name": "A spider", "fact": "Coming for your face"}
	}
}

tree = new DecisionTree(tree_data)
query = {"limbs": 8, "eyes": 3}
animal = tree.decide(query)
print(animal)
