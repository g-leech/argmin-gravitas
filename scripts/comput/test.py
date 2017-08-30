from ComputerSelector import *


criteria = {    "signals": "Electromechanical",
                "programmable": True,
                "universal": True,
                "representation": "Digital"
           }
matches = filter_candidates(computers, criteria)
firstComputer = get_earliest(matches, sortCmp)


assert(firstComputer['name'] == 'Z3')
print(firstComputer)
