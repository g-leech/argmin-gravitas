fullModel = [ "index", "name", "protagonists", "date", "signals",
    "universal", "programmables","stored", "representation","base",
    "transmission", "transistorised",  "virtual",
    "instructions", "gui", "notes", "link"
]

fieldModel = fullModel[4:-1]
whats = [ "?" for i in range(len(fullModel))]   # Transcrypt can't handle string multiplication
nullComputer = dict(zip(fullModel, whats))

computers = [
    [   1,"Antikythera mechanism","?","100 BCE","mechanical","special-purpose","","","analogue",None,"serial","","","","","'Didn't work very well'","https://en.wikipedia.org/wiki/Antikythera_mechanism"],
    [   2,"South-pointing chariot","Ma Jun","230 CE","mechanical","special-purpose","","","analogue",None,"serial","","","","","The most dubious entry on my list: translated wheel motion into one orientation"],
    [   3,"Calculating clock","Wilhelm Schickard","1623","mechanical","special-purpose","","","digital","decimal","serial","","","","","First arithmetic calculator?"],
    [   4,"Pascaline","Blaise Pascal","1645","mechanical","special-purpose","","","digital","decimal","serial","","","","","Came in several bases for accounting currency"],
    [   5,"Stepped Reckoner","Gottfried Leibniz","1694","mechanical","special-purpose","","","digital","decimal","serial","","","","","Flawed carry mechanism"],
    [   6,"Feinmechanikers Rechenmaschine","Phillip Matthaeus Hahn","1774","mechanical","special-purpose","","","digital","decimal","serial","","","","","First calculator to do all four arithmetic ops"],
    [   7,"Thomas Arithmomètre","Charles Xavier Thomas","1851","mechanical","special-purpose","","","digital","decimal","serial","","","","","first digital mechanical calculator strong enough and reliable enough to be used daily in an office environment"],
    [   8,"Difference engine","JH Müller, Charles Babbage, Per Georg Scheutz","1855","mechanical","special-purpose","","","digital","decimal","serial","","","","",None],
    [   9,"Z1","Konrad Zuse","1938","mechanical","special-purpose","programmable","","analogue",None,"serial","","","","","film not paper"],
    [   10,"Z2","Konrad Zuse","1939","electromechanical","special-purpose","programmable","","digital","binary","serial","","","","",None],
    [   11,"Mischgerät","Helmut Hoelzer","1940","electromechanical","special-purpose","programmable","","analogue",None,"serial","","","","","world's first on-board computer."],
    [   12,"Bombe","Alan Turing and Gordon Welchman","18/03/1940","electromechanical","special-purpose","","","digital","26?","serial","","","","",None ],
    [   13,"Bell Labs Model 1 (Complex Number Computer)","George Stibitz", "01/06/1940", "electromechanical", "special-purpose","","","digital","Complex","serial","","","","","first remote use of a computer"],
    [   14,"50 Jahre Analog Computer","Helmut Hoelzer","1941","fully-electronic","general-purpose","programmable","","analogue",None,"serial","","","","","first fully-electronic analogue computer"],
    [   15,"Z3","Konrad Zuse","07/12/1941","electromechanical","general-purpose","programmable","","digital","binary","serial","","","","","first machine to execute a program successfully. universal if provided with indirect addressing"],
    [   16,"Atanasoff-Berry Computer","John Atanasoff","01/05/1942","fully-electronic","special-purpose","","","digital","binary","serial","","","","","first linear"],
    [   17,"Colossus Mk1","Tommy Flowers","08/12/1943","fully-electronic","special-purpose","programmable","","digital","binary","serial","","","","","first large fully-electronic valve programmable logic calculator. Not general purpose on its own. Colossus was kept secret until the 1970s"],
    [   18,"Harvard Mark I (IBM Automatic Sequence Control Calculator)","Howard Aiken","30/04/1944","electromechanical","special-purpose","programmable","","digital","decimal","serial","","","","","a convincing demonstration of the possibility of large-scale error-free complex calculations in a programmed sequence"],
    [   19,"Z4","Konrad Zuse","01/03/1945","electromechanical","general-purpose","programmable","","digital","binary","serial","","","","",None],
    [   20,"ENIAC"," John Mauchly and J. Presper Eckert","01/11/1945","fully-electronic","general-purpose","programmable","","digital","decimal","serial","","","","","first fully-electronic, general purpose, large scale, digital computer"],
    [   21,"IBM SSEC","A. Wayne Brooke and Wallace Eckert","01/12/1947","electromechanical","general-purpose","programmable","stored-program","digital","binary","serial","","","","","First operational stored-program computer?"],
    [   22,"Manchester Baby","F.C. Williams and Tom Kilburn","21/06/1948","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","","","","","first time a program stored in an fully-electronic computing machine successfully ran and produced the expected answer. no earlier computer, not Zuse's Z3 in Germany, nor Colossus at Bletchley Park in Britain, nor ENIAC in the. United States, was in that way like a modern computer. First RAM?"],
    [   23,"Manchester Mark I (Intermediate)","F.C. Williams and Tom Kilburn","01/04/1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary",False,"","","","first full-sized. Also known as Manchester Automatic Digital Machine"],
    [   24,"EDSAC","M. V. Wilkes","01/05/1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","","","","","second stored-program computer"],
    [   25,"EDVAC","Eckert and von Neumann","01/08/1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","","","","",None],
    [   26,"CSIRAC","Trevor Pearcey and Maston Beard","01/11/1949","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","","","","",None],
    [   27,"MESM (Малая Электронно-Счетная Машина)","Sergei Alekseyevich Lebedev","01/01/1950","fully-electronic","general-purpose","programmable","","digital","binary","serial","","","","","first universally programmable fully-electronic computer in continental Europe"],
    [   28,"SEC (Simple fully-Electronic Computer)","Andrew Donald Booth and Norman Kitz","01/01/1950","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","","","","","Incredible effort, 4 people"],
    [   29,"ACE-PM","Alan Turing","10/05/1950","fully-electronic","general-purpose","programmable","","digital","binary","serial","","","","",""],
    [   30,"Whirlwind I", "Jay Forrester and Robert Everett", "20/04/1951", "fully-electronic", "general-purpose","programmable","stored-program", "digital","binary", "parallel", "", "", "", "", "The first electronic computer that was not a serial computer" ],
    [   31,"IAS machine","von Neumann & Julian Bigelow","10/06/1952","fully-electronic","general-purpose","programmable","stored-program","digital","binary","parallel","","","","","called the world’s first parallel 'modern' computer whatever that means. (parallel at the bit level). Asynchronous CPU"],
    [   32,"Manchester Transistor Computer","L. Grimsdale and D. C. Webb","01/11/1953","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","","","","world's first transistorised computer"],
    [   33,"A prototype transistorized IBM 604","Ralph Palmer?","01/10/1954","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","","","","the first fully transistorized computer"],
    [   34,"Harwell CADET","E. H. Cooke-Yarborough","01/02/1955","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","","","","the first fully transistorized computer in Europe"],
    [   35,"Parametron Computer 1","Hidetosi Takahasi","01/01/1958","fully-electronic","general-purpose","programmable","","digital","binary","","","","","","used majority logic"],
    [   36,"Setun","Sergei Sobolev and Nikolai Brusentsov","01/01/1959","fully-electronic","general-purpose","programmable","stored-program","digital","ternary","serial","","","","","Russian ternary computer"],
    [   37,"Fairchild Solid-State Micrologic Element","Jay Last","26/5/1960","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","","CISC","","first planar integrated circuit"],
    [   38,"Texas Instruments molecular computer","Jack Kilby and Harvey Cragon","01/10/61","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","","CISC","","first basic integrated circuit computer. Invented 12/9/1958"],
    [   39,"Manchester Atlas Computer","Tom Kilburn","07/12/1962","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","","",None],
    [   40,"IBM System&#47;360","Gene Amdahl, Fred Brooks","07/04/1964","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","CISC","","first use of an abstract instruction set architecture"],
    [   41,"CDC 6600","Seymour Cray","01/01/1965","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","RISC","","first RISC computer (anachronistic name, but true)"],
    [   42,"Xerox Alto","Charles Thacker, Alan Kay, Butler Lampson","01/03/1973","fully-electronic","general-purpose","programmable","stored-program","digital","binary","serial","transistorised","virtual-memory","CISC","gui-based","first GUI-based computer (as opposed to the NLS, which was the first GUI software)"],
]

computers = [dict(zip(fullModel, dat)) for dat in computers]


