	(function () {
		var computers = __init__ (__world__.data).computers;
		var fieldModel = __init__ (__world__.data).fieldModel;
		var fullModel = __init__ (__world__.data).fullModel;
		var nullComputer = __init__ (__world__.data).nullComputer;
		var whats = __init__ (__world__.data).whats;
		var BASE = __init__ (__world__.html_ids).BASE;
		var ELECTRO = __init__ (__world__.html_ids).ELECTRO;
		var PROG_YES = __init__ (__world__.html_ids).PROG_YES;
		var SPECIAL = __init__ (__world__.html_ids).SPECIAL;
		var STORE = __init__ (__world__.html_ids).STORE;
		var STORED = __init__ (__world__.html_ids).STORED;
		var TRANSIST = __init__ (__world__.html_ids).TRANSIST;
		var TURING = __init__ (__world__.html_ids).TURING;
		var description = __init__ (__world__.html_ids).description;
		var descriptionId = __init__ (__world__.html_ids).descriptionId;
		var digId = __init__ (__world__.html_ids).digId;
		var electroId = __init__ (__world__.html_ids).electroId;
		var generalId = __init__ (__world__.html_ids).generalId;
		var nonprogramId = __init__ (__world__.html_ids).nonprogramId;
		var programId = __init__ (__world__.html_ids).programId;
		var resultDate = __init__ (__world__.html_ids).resultDate;
		var resultDescription = __init__ (__world__.html_ids).resultDescription;
		var resultImg = __init__ (__world__.html_ids).resultImg;
		var resultName = __init__ (__world__.html_ids).resultName;
		var resultWho = __init__ (__world__.html_ids).resultWho;
		var sortCmp = __init__ (__world__.html_ids).sortCmp;
		var specialId = __init__ (__world__.html_ids).specialId;
		var refresh = function (py_name) {
			var criteria = get_criteria ();
			var criteria = infer_predicates (criteria, py_name);
			var firstComputer = get_computer (computers, criteria);
			set_computer (firstComputer);
			update_description (firstComputer);
		};
		var filter_candidates = function (matches, criteria) {
			var __iterable0__ = criteria.py_items ();
			for (var __index0__ = 0; __index0__ < __iterable0__.length; __index0__++) {
				var __left0__ = __iterable0__ [__index0__];
				var key = __left0__ [0];
				var criterion = __left0__ [1];
				var __iterable1__ = matches;
				for (var __index1__ = 0; __index1__ < __iterable1__.length; __index1__++) {
					var m = __iterable1__ [__index1__];
					var matches = function () {
						var __accu0__ = [];
						var __iterable2__ = matches;
						for (var __index2__ = 0; __index2__ < __iterable2__.length; __index2__++) {
							var m = __iterable2__ [__index2__];
							if (m [key] == criterion) {
								__accu0__.append (m);
							}
						}
						return __accu0__;
					} ();
				}
			}
			return matches;
		};
		var get_earliest = function (data, compare) {
			var srted = sorted (data, __kwargtrans__ ({key: (function __lambda__ (k) {
				return k [compare];
			})}));
			return srted [0];
		};
		var get_criteria = function () {
			var crits = dict ({});
			var __iterable0__ = fieldModel;
			for (var __index0__ = 0; __index0__ < __iterable0__.length; __index0__++) {
				var v = __iterable0__ [__index0__];
				var query = ('input[name="' + v) + '"]:checked';
				var field = document.querySelector (query);
				if (field !== null) {
					crits [v] = field.value;
				}
			}
			return crits;
		};
		var get_computer = function (data, criteria) {
			var matches = filter_candidates (data, criteria);
			if (len (matches) > 0) {
				return get_earliest (matches, sortCmp);
			}
			else {
				return nullComputer;
			}
		};
		var set_computer = function (computerDict) {
			if (computerDict ['name'] == '?') {
				computerDict ['name'] = 'N&#47;A';
			}
			var py_name = 'the ' + computerDict ['name'];
			set_html (resultName, py_name);
			var who = 'by ' + computerDict ['protagonists'];
			set_html (resultWho, who);
			var when = ('First operational ' + computerDict ['date']) + '.';
			set_html (resultDate, when);
			var imgName = computerDict ['name'].py_replace ('#', '%23');
			document.getElementById (resultImg).src = ('/img/comput/' + imgName) + '.jpg';
			document.getElementById (descriptionId).style.display = '';
		};
		var set_html = function (id, result) {
			document.getElementById (id).innerHTML = result;
		};
		var infer_predicates = function (data, py_name) {
			print (data);
			if (py_name == 'base') {
				var data = constrain_digital (data);
			}
			else if (py_name == 'programmables') {
				var data = constrain_single_program (data);
			}
			else if (py_name == 'universal') {
				var data = constrain_turing (data);
			}
			else if (py_name == 'transistorised') {
				var data = constrain_transistor (data);
			}
			else if (py_name == 'stored') {
				var data = constrain_stored (data);
			}
			else if (py_name == 'gui') {
				var data = constrain_gui (data);
			}
			return data;
		};
		var constrain = function (id) {
			document.getElementById (id).checked = true;
		};
		var constrain_analogue = function (data) {
			var elements = document.getElementsByName (BASE);
			if (data ['representation'] == 'analogue') {
				var __iterable0__ = elements;
				for (var __index0__ = 0; __index0__ < __iterable0__.length; __index0__++) {
					var el = __iterable0__ [__index0__];
					el.checked = false;
					el.disabled = true;
				}
			}
			else {
				var __iterable0__ = elements;
				for (var __index0__ = 0; __index0__ < __iterable0__.length; __index0__++) {
					var el = __iterable0__ [__index0__];
					el.disabled = false;
				}
			}
		};
		var constrain_digital = function (data) {
			if (data ['base'] != '') {
				var data = constrain (digId);
				data ['representation'] = 'digital';
			}
			return data;
		};
		var constrain_single_program = function (data) {
			if (data ['programmables'] == '') {
				constrain (specialId);
				data ['universal'] = SPECIAL;
				constrain ('nonstor');
				data ['stored'] = '';
				document.getElementById (generalId).checked = false;
			}
			return data;
		};
		var constrain_turing = function (data) {
			if (data ['universal'] == TURING) {
				constrain (programId);
				data ['programmables'] = PROG_YES;
			}
			return data;
		};
		var constrain_transistor = function (data) {
			if (data ['transistorised'] == TRANSIST) {
				constrain (electroId);
				data ['signals'] = ELECTRO;
			}
			return data;
		};
		var constrain_stored = function (data) {
			if (data ['stored'] == STORED) {
				constrain (programId);
				data ['programmables'] = PROG_YES;
			}
			return data;
		};
		var constrain_gui = function (data) {
			if (data ['gui'] == 'gui-based') {
				constrain (electroId);
				data ['signals'] = ELECTRO;
			}
			return data;
		};
		var update_description = function (data) {
			// pass;
		};
		document.getElementById (descriptionId).innerHTML = description;
		document.getElementById (descriptionId).style.display = 'none';
		__pragma__ ('<use>' +
			'data' +
			'html_ids' +
		'</use>')
		__pragma__ ('<all>')
			__all__.BASE = BASE;
			__all__.ELECTRO = ELECTRO;
			__all__.PROG_YES = PROG_YES;
			__all__.SPECIAL = SPECIAL;
			__all__.STORE = STORE;
			__all__.STORED = STORED;
			__all__.TRANSIST = TRANSIST;
			__all__.TURING = TURING;
			__all__.computers = computers;
			__all__.constrain = constrain;
			__all__.constrain_analogue = constrain_analogue;
			__all__.constrain_digital = constrain_digital;
			__all__.constrain_gui = constrain_gui;
			__all__.constrain_single_program = constrain_single_program;
			__all__.constrain_stored = constrain_stored;
			__all__.constrain_transistor = constrain_transistor;
			__all__.constrain_turing = constrain_turing;
			__all__.description = description;
			__all__.descriptionId = descriptionId;
			__all__.digId = digId;
			__all__.electroId = electroId;
			__all__.fieldModel = fieldModel;
			__all__.filter_candidates = filter_candidates;
			__all__.fullModel = fullModel;
			__all__.generalId = generalId;
			__all__.get_computer = get_computer;
			__all__.get_criteria = get_criteria;
			__all__.get_earliest = get_earliest;
			__all__.infer_predicates = infer_predicates;
			__all__.nonprogramId = nonprogramId;
			__all__.nullComputer = nullComputer;
			__all__.programId = programId;
			__all__.refresh = refresh;
			__all__.resultDate = resultDate;
			__all__.resultDescription = resultDescription;
			__all__.resultImg = resultImg;
			__all__.resultName = resultName;
			__all__.resultWho = resultWho;
			__all__.set_computer = set_computer;
			__all__.set_html = set_html;
			__all__.sortCmp = sortCmp;
			__all__.specialId = specialId;
			__all__.update_description = update_description;
			__all__.whats = whats;
		__pragma__ ('</all>')
	}) ();
