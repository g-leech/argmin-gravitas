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
        if criterion:
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
    when = "First operational " + computerDict['date'] + "."
    set_html(resultDate, when)

    imgName = computerDict['name'].replace("#", "%23")
    document.getElementById(resultImg).src = "/img/comput/"+ imgName + ".jpg"
    document.getElementById(descriptionId).style.display = ""

    update_description(computerDict)


def set_html(id, result) :
    document.getElementById(id).innerHTML = result


# First handle some special cases: e.g. analogs have no base.
# UX is terrible without this
def infer_predicates(data, name) :
    print(data)

    elements = document.getElementsByName(BASE)

    # if data['representation'] == 'analogue' :
    #     for el in elements :
    #         el.checked = False
    #         el.disabled = True
    # else :
    #     for el in elements :
    #         el.disabled = False

    if data['base'] != "" :
        document.getElementById(digId).checked = True
        data['representation'] = 'digital'

    if data['programmables'] == '':
        document.getElementById(specialId).checked = True
        document.getElementById(generalId).checked = False
        document.getElementById('nonstor').checked = True
        # document.getElementById(nonprogramId).checked = True
        data['universal'] = SPECIAL
        data['stored'] = ''

    if data['universal'] == TURING :
        document.getElementById(programId).checked = True
        document.getElementById(generalId).checked = True
        data['programmables'] = PROG_YES


    if data['transistorised'] == TRANSIST :
        document.getElementById(electroId).checked = True
        data['signals'] = "fully-electronic"

    if data['stored'] == STORED :
        document.getElementById(programId).checked = True
        data['programmables'] = PROG_YES
     
    return data


def update_description(data) :
    pass

  # $( 'span[#'+ name + ']').remove()
  # el = make_span(name, val)
  # document.getElementById(descriptionId).append(el)
  # fix_first_comma()


# def make_span(name, val) :
#   el = document.createElement('span')
#   el.id = name;
  
#   if val != "" :
#     el.innerHTML = ", " +  val
  
#   return el


# function fixFirstComma() {
#   firstSpan = $(descriptionId).children()[0].innerHTML;
#   firstSpan.children()[0].innerHTML = firstSpan.replace(/, /, "");
# }


document.getElementById(descriptionId).innerHTML = description 
document.getElementById(descriptionId).style.display = "none"