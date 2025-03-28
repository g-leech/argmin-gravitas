
fullModel = [ 
        "index", "name", "protagonists", "date", "signals",
        "universal", "programmables","stored", "representation","base",
        "transmission", "transistorised",  "virtual",
        "instructions", "gui", "notes", "link"
]

fieldModel = fullModel[4:-2]
whats = [ "?" for i in range(len(fullModel)) ]   # Transcrypt can't handle string multiplication
whats[fullModel.index("name")] = "Not known"
nullComputer = dict(zip(fullModel, whats))


computers = [
    [   1,"Antikythera mechanism","?","100 BCE","mechanical","special-purpose","fixed-job","structure-program","analogue",None,"serial","nontransistorised","physical-memory","","","'Didn't work very well'"],
    [   2,"South-pointing chariot","Ma Jun","230 CE","mechanical","special-purpose","fixed-job","structure-program","analogue",None,"serial","nontransistorised","physical-memory","","","The most dubious entry on my list: translated wheel motion into one orientation"],
    [   3,"Calculating clock","Wilhelm Schickard","1623","mechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","","First arithmetic calculator?"],
    [   4,"Pascaline","Blaise Pascal","1645","mechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","","Came in several bases for accounting currency"],
    [   5,"Stepped Reckoner","Gottfried Leibniz","1694","mechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","","Flawed carry mechanism"],
    [   6,"Feinmechanikers Rechenmaschine","Phillip Matthaeus Hahn","1774","mechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","","First calculator to do all four arithmetic ops"],
    [   7,"Thomas Arithmomètre","Charles Xavier Thomas","1851","mechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","","first digital mechanical calculator strong enough and reliable enough to be used daily in an office environment"],
    [   8,"Difference engine","JH Müller, Charles Babbage, Per Georg Scheutz","1855","mechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","",None],
    [   9,"Logical Piano","WS Jevons","1866","mechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","",None],
    [   10, "Calculating-Machine", "Charles Pidgin and Francis Leonard", "1883", "electromechanical","special-purpose","fixed-job","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","",None],
    [   11,"Z1","Konrad Zuse","1938","mechanical","special-purpose","programmable","structure-program","digital","binary","serial","nontransistorised","physical-memory","","","film not paper"],
    [   12,"Z2","Konrad Zuse","1939","electromechanical","special-purpose","programmable","structure-program","digital","binary","serial","nontransistorised","physical-memory","","",None],
    [   13,"Mischgerät","Helmut Hoelzer","1940","electromechanical","special-purpose","programmable","structure-program","analogue",None,"serial","nontransistorised","physical-memory","","","world's first on-board computer."],
    [   14,"Bombe","Marian Rejewski, Alan Turing and Gordon Welchman","18/03/1940","electromechanical","special-purpose","fixed-job","structure-program","digital","26?","serial","nontransistorised","physical-memory","","",None ],
    [   15,"Bell Labs Model 1 (Complex Number Computer)","George Stibitz", "01/06/1940", "electromechanical", "special-purpose","fixed-job","structure-program","digital","Complex","serial","nontransistorised","physical-memory","","","first remote use of a computer"],
    [   16,"50 Jahre Analog Computer","Helmut Hoelzer","1941","fully-electronic","general-purpose","programmable","structure-program","analogue",None,"serial","nontransistorised","physical-memory","","","first fully-electronic analogue computer"],
    [   17,"Z3","Konrad Zuse","07/12/1941","electromechanical","general-purpose","programmable","structure-program","digital","binary","serial","nontransistorised","physical-memory","","","first machine to execute a program successfully. universal if provided with indirect addressing"],
    [   18,"Atanasoff-Berry Computer","John Atanasoff","01/05/1942","fully-electronic","special-purpose","fixed-job","structure-program","digital","binary","serial","nontransistorised","physical-memory","","","first linear"],
    [   19,"Colossus Mk1","Tommy Flowers","08/12/1943","fully-electronic","special-purpose","programmable","structure-program","digital","binary","serial","nontransistorised","physical-memory","","","first large fully-electronic valve programmable logic calculator. Not general purpose on its own. Colossus was kept secret until the 1970s"],
    [   20,"Harvard Mark I (IBM Automatic Sequence Control Calculator)","Howard Aiken","30/04/1944","electromechanical","special-purpose","programmable","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","","a convincing demonstration of the possibility of large-scale error-free complex calculations in a programmed sequence"],
    [   21,"Z4","Konrad Zuse","01/03/1945","electromechanical","general-purpose","programmable","structure-program","digital","binary","serial","nontransistorised","physical-memory","","",None],
    [   22,"ENIAC"," John Mauchly and J. Presper Eckert","in November 1945","fully-electronic","general-purpose","programmable","structure-program","digital","decimal","serial","nontransistorised","physical-memory","","","first fully-electronic, general purpose, large scale, digital computer"],
    [   23,"IBM SSEC","A. Wayne Brooke and Wallace Eckert","in December 1947","electromechanical","general-purpose","programmable","stored-program","digital","binary","serial","nontransistorised","physical-memory","","","First operational stored-program computer?"],
    [   24,"Manchester Baby","F.C. Williams and Tom Kilburn","21/06/1948","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","nontransistorised","physical-memory","","","first time a program stored in an fully-electronic computing machine successfully ran and produced the expected answer. no earlier computer, not Zuse's Z3 in Germany, nor Colossus at Bletchley Park in Britain, nor ENIAC in the. United States, was in that way like a modern computer. First RAM?"],
    [   25,"ENIAC with stored program ROM"," Dick Clippinger, Jean Bartik, Adele Goldstine","in April or September 1948","fully-electronic","general-purpose","programmable","stored-program","digital","decimal","serial","nontransistorised","physical-memory","","",""],
    [   26,"Manchester Mark I (Intermediate)","F.C. Williams and Tom Kilburn","01/04/1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","nontransistorised","physical-memory","","first full-sized. Also known as Manchester Automatic Digital Machine"],
    [   27,"EDSAC","M. V. Wilkes","01/05/1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","nontransistorised","physical-memory","","","second stored-program computer"],
    [   28,"EDVAC","Eckert and von Neumann","in August 1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","nontransistorised","physical-memory","","",None],
    [   29,"CSIRAC","Trevor Pearcey and Maston Beard","in November 1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","nontransistorised","physical-memory","","",None],
    [   30,"MESM","Sergei Alekseyevich Lebedev","01/01/1950","fully-electronic","general-purpose","programmable","structure-program","digital","binary","serial","nontransistorised","physical-memory","","","first universally programmable fully-electronic computer in continental Europe"],
    [   31,"SEC (Simple fully-Electronic Computer)","Andrew Donald Booth and Norman Kitz","01/01/1950","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","nontransistorised","physical-memory","","","Incredible effort, 4 people"],
    [   32,"ACE-PM","Alan Turing","10/05/1950","fully-electronic","general-purpose","programmable","structure-program","digital","binary","serial","nontransistorised","physical-memory","","",""],
    [   33,"Whirlwind I", "Jay Forrester and Robert Everett", "20/04/1951", "fully-electronic", "general-purpose","programmable","stored-program", "digital","binary", "parallel", "nontransistorised","physical-memory", "", "", "The first electronic computer that was not a serial computer" ],
    [   34,"IAS machine","von Neumann & Julian Bigelow","10/06/1952","fully-electronic","general-purpose","programmable","stored-program","digital","binary","parallel","nontransistorised","physical-memory","","","called the world’s first parallel 'modern' computer whatever that means. (parallel at the bit level). Asynchronous CPU"],
    [   35,"Manchester Transistor Computer","L. Grimsdale and D. C. Webb","in November 1953","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","physical-memory","","","world's first transistorised computer"],
    [   36,"A prototype transistorized IBM 604","Ralph Palmer?","01/10/1954","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","physical-memory","","","the first fully transistorized computer"],
    [   37,"Harwell CADET","E. H. Cooke-Yarborough","early 1955","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","physical-memory","","","the first fully transistorized computer in Europe"],
    [   38,"Parametron Computer 1","Hidetosi Takahasi","1958","fully-electronic","general-purpose","programmable","structure-program","digital","binary","parallel","nontransistorised","physical-memory","","","used majority logic"],
    [   39,"Setun","Sergei Sobolev and Nikolai Brusentsov","1959","fully-electronic","general-purpose","programmable","stored-program","digital","ternary","serial","nontransistorised","physical-memory","","","Russian ternary computer"],
    [   40,"Fairchild Solid-State Micrologic Element","Jay Last","26/5/1960","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","physical-memory","CISC","","first planar integrated circuit"],
    [   41,"Texas Instruments molecular computer","Jack Kilby and Harvey Cragon","01/10/61","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","physical-memory","CISC","","first basic integrated circuit computer. Invented 12/9/1958"],
    [   42,"Manchester Atlas Computer","Tom Kilburn","07/12/1962","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","","",None],
    [   43,"IBM System360","Gene Amdahl, Fred Brooks","07/04/1964","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","CISC","","first use of an abstract instruction set architecture"],
    [   44,"CDC 6600","Seymour Cray","early 1965","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","RISC","","first RISC computer (anachronistic name, but true)"],
    [   45,"Xerox Alto","Charles Thacker, Alan Kay, Butler Lampson","01/03/1973","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","CISC","gui-based","first GUI-based computer (as opposed to the NLS, which was the first GUI software)"],
]

computers = [dict(zip(fullModel, dat)) for dat in computers]


print([c["name"] for c in computers])