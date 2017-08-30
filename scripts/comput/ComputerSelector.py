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

    init_app()
    set_computer(firstComputer)


def filter_candidates(matches, criteria) :
    for key, criterion in criteria.items() :
        matches = [ m for m in matches if m[key] == criterion ] 

    return matches


def get_earliest(data, compare) :
    srted = sorted(data, key=lambda k: k[compare])
    return srted[0]


# The first comprehension just stores DOM queries, avoids querying twice.
def get_criteria() :
    checked = { f : get_radio_val(f) for f in fieldModel }
    return { f : checked[f] for f in fieldModel if checked[f] }


def get_radio_val( name ) :
    query = 'input[name="' + name + '"]:checked'
    dom = document.querySelector(query)

    return dom.value if dom else None


def get_computer(data, criteria) :
    matches = filter_candidates(data, criteria)

    if len(matches) > 0:
        return get_earliest(matches, sortCmp)
    else:
        return nullComputer


# First handle some special cases: e.g. analogs have no base.
# UX is terrible without this
def infer_predicates(data, name) :
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


def constrain(id, isChecked=True) :
    document.getElementById(id).checked = isChecked


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
    if data['programmables'] == '' : 
        constrain(specialId)
        data['universal'] = SPECIAL
        constrain('nonstor')
        data['stored'] = ''
        constrain(generalId, False)
     
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


#  Only once
def init_app() :
    global isAppInitialised

    if not isAppInitialised :
        hide_intro_text()
        set_image_size()
        isAppInitialised = True


def set_computer(computerDict) :
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

    set_image_size(100,100)
    imgDom.src = "/img/spin.gif"
    imgDom.src = "/img/comput/"+ imgName + ".jpg"
    set_image_size()


def hide_intro_text() :
    document.getElementById(intro).style.display = "none"


def set_image_size(x=400, y=300) :
    imgDom = document.getElementById(resultImg)
    imgDom.style.height = str(y) + "px"
    imgDom.style.width = str(x) + "px"


def reset() :
    for field in fieldModel:
        for dom in document.getElementsByName(field) :
            dom.checked = False
                
    set_computer(nullComputer)