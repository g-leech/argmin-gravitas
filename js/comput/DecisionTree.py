class DecisionTree(object):

    def __init__(self, data):
        branches = data["branch"]
        leaves = data["leaf"]
        root = data["root"]

        self.node_builders = {}
        self.create_branches(branches)
        self.create_leaves(leaves)
        self.attach_buckets()
        self.set_root(root)

    def decide(self, query):
        return self.root.decide(query)

    def create_branches(self, branch_data):
        for (name, data) in branch_data.items():
            self.node_builders[name] = {
                "type": "branch",
                "data": data,
                "node": BranchNode(name)
            }

    def create_leaves(self, leaf_data):
        for (name, data) in leaf_data.items():
            self.node_builders[name] = {
                "type": "leaf",
                "node": data
            }

    def attach_buckets(self):
        for (name, builder) in self.node_builders.items():
            if builder["type"] == "leaf":
                continue

            parent_builder = self.node_builders[name]
            parent_node = parent_builder["node"]

            for bucket_data in builder["data"]:
                if "key" not in bucket_data or "rule" not in bucket_data:
                    raise Exception("Expected 'key' and 'rule'")

                key = bucket_data["key"]
                rule = bucket_data["rule"]
                outcome_name = rule["outcome"]

                if outcome_name not in self.node_builders:
                    raise Exception("Outcome not found: %s" % outcome_name)

                outcome = self.node_builders[outcome_name]

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

    def set_root(self, root_name):
        builder = self.node_builders[root_name]
        self.root = builder["node"]


class BranchNode(object):

    def __init__(self, name):
        self.name = name
        self.buckets = []

    def add_bucket(self, bucket):
        self.buckets.append(bucket)

    def decide(self, user_query):
        for bucket in self.buckets:
            if bucket.is_match(user_query):
                return bucket.follow(user_query)


class Bucket(object):

    def __init__(self, key):
        self.key = key
        self.min = None
        self.max = None
        self.branch = None
        self.leaf = None
        pass

    def set_min(self, min):
        self.min = min

    def set_max(self, max):
        self.max = max

    def set_branch(self, branch):
        self.branch = branch

    def set_leaf(self, leaf):
        self.leaf = leaf

    def is_match(self, user_query):
        if self.key in user_query:
            user_value = user_query[self.key]
            if self.min and user_value < self.min:
                return False
            if self.max and user_value > self.max:
                return False
            return True
        else:
            return False

    def follow(self, user_query):
        if self.branch:
            return self.branch.decide(user_query)
        else:
            return self.leaf


"""
LIMBS = 'limbs'
EYES = 'eyes'
OUT = 'creature'
KEY = "key"
RULE = "rule"
FIRST_KEY = "how_many_limbs"
NEXT_KEY = "how_many_eyes"


tree_data = {
    "root": FIRST_KEY,
    "branch": {
        FIRST_KEY : [
            { "key": LIMBS, "rule": {"min": 1, "max": 1, "outcome": "slug"} },
            { "key": LIMBS, "rule": {"min": 2, "max": 2, "outcome": "bird"} },
            { "key": LIMBS, "rule": {"min": 4, "max": 4, "outcome": "cow"} },
            { "key": LIMBS, "rule": {"min": 5, "outcome": NEXT_KEY} }
        ],
        NEXT_KEY : [
            { "key": EYES, "rule": {"min": 2, "max": 2, "outcome": "octopus"} },
            { "key": EYES, "rule": {"min": 3, "outcome": "spider"} }
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


tree = DecisionTree(tree_data)


def get_input_val(fieldId) :
    fieldVal = document.getElementById(fieldId).value
    return int( fieldVal ) if fieldVal else 0


def get_query() :
    numLimbs = get_input_val(LIMBS)
    numEyes = get_input_val(EYES)
    return { LIMBS: numLimbs, EYES: numEyes }


def get_knowledge( animal ) :
    if animal :
        knowledgeList = [ str(x) for x in list(animal.values()) ]
        knowledge = " - ".join( knowledgeList )
    else :
        knowledge = "?"

    return knowledge


def fetch_and_decide():
    query = get_query()
    animal = tree.decide(query)
    knowledge = get_knowledge(animal)
    document.getElementById(OUT).innerHTML = knowledge

"""