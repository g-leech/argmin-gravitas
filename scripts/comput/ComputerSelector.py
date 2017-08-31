
class ComputerSelector() :

    def __init__ (self, nullComputer) :
        self.sortCmp = "index"
        self.nullComputer = nullComputer


    def filter_candidates(self, matches, criteria) :
        for key, criterion in criteria.items() :
            matches = [ m for m in matches if m[key] == criterion ] 

        return matches


    def get_earliest(self, data, compare) :
        srted = sorted(data, key=lambda k: k[compare])
        return srted[0]


    def get_computer(self, data, criteria) :
        matches = self.filter_candidates(data, criteria)

        if len(matches) > 0:
            return self.get_earliest(matches, self.sortCmp)
        else:
            return self.nullComputer


