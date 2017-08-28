# Had a neater solution using list comprehensions, 
# a data class and attrgetter, but Transcrypt doesn't like builtin modules
# or Jquery
# (or variables called 'var' but that one's obvs)


from data import *
from html_ids import *



# Entry point
def refresh(name) :
    criteria = get_criteria()
    criteria = infer_predicates(criteria, name)
    firstComputer = get_computer(computers, criteria)

    set_computer(firstComputer)


def filter_candidates(matches, criteria) :
    for key, criterion in criteria.items() :
        # if criterion:
        for m in matches :
            matches = [m for m in matches if m[key] == criterion]

    return matches


def get_earliest(data, compare) :
    srted = sorted(data, key=lambda k: k[compare])
    return srted[0]


def get_criteria() :
    crits = {}
    for v in fieldModel:
        query = 'input[name="' + v + '"]:checked'
        field = document.querySelector(query)

        if field is not None :
            crits[v] = field.value

    return crits


def get_computer(data, criteria) :
    matches = filter_candidates(data, criteria)

    if len(matches) > 0:
        return get_earliest(matches, sortCmp)
    else:
        return nullComputer



def set_computer(computerDict) :
    if computerDict['name'] == "?" :
        computerDict['name'] = "N&#47;A"

    name = "the " + computerDict['name']
    set_html(resultName, name)
    who = "by " + computerDict['protagonists']
    set_html(resultWho, who)
    when = "<br>fits the bill.<br><br>It was first operational " \
            + computerDict['date'] + "."
    set_html(resultDate, when)
    set_image(computerDict['name'])


def set_html(id, result) :
    document.getElementById(id).innerHTML = result


def set_image(name) :
    imgName = name.replace("#", "%23")
    imgDom = document.getElementById(resultImg)
    imgDom.style.height = "300px"
    imgDom.style.width = "400px"
    imgDom.src = "/img/comput/"+ imgName + ".jpg"


# First handle some special cases: e.g. analogs have no base.
# UX is terrible without this
def infer_predicates(data, name) :
    print(data)

    if name == 'programmables' :
        data = constrain_single_program(data)
    elif name == 'universal' :
        data = constrain_turing(data)
    elif name == 'transistorised' :
        data = constrain_transistor(data)
    elif name == 'stored' :
        data = constrain_stored(data)
    elif name == 'gui' :
        data = constrain_gui(data)
    elif name == 'base' :
        data = constrain_digital(data)
    elif name == 'representation' :
        data = constrain_analogue(data)

    return data


def constrain(id) :
    document.getElementById(id).checked = True


def constrain_analogue(data):
    elements = document.getElementsByName(BASE)

    if data['representation'] == ANALOG :
        for el in elements :
            el.checked = False
            el.disabled = True
    else :
        for el in elements :
            el.disabled = False

    return data


def constrain_digital(data) :
    if data['base'] is not "" :
        constrain(digId)
        data['representation'] = DIGITAL
    return data


def constrain_single_program(data) :
    if data['programmables'] == '' : # or None
        constrain(specialId)
        data['universal'] = SPECIAL
        constrain('nonstor')
        data['stored'] = ''
        document.getElementById(generalId).checked = False
     
    return data


def constrain_turing(data) :
    if data['universal'] == TURING :
        constrain(programId)
        data['programmables'] = PROG_YES
    return data


def constrain_transistor(data) :
    if data['transistorised'] == TRANSIST :
        constrain(electroId)
        data['signals'] = ELECTRO
    return data
    

def constrain_stored(data) :
    if data['stored'] == STORED :
        constrain(programId)
        data['programmables'] = PROG_YES
    return data


def constrain_gui(data) :
    if data['gui'] == GUI :
        constrain(electroId)
        data['signals'] = ELECTRO
    return data
