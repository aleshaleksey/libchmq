#![allow(dead_code)]
#![allow(unused_imports)]
/// Jedná se o jednoduchý generátor otázek pro chemické výpočty
/// pro opravdu základní chemické výpočty pro přednět Lékařská chemie
/// in medicinal chemistry" classes. It was written because students frequently
/// cited a dearth of practice questions as one of the difficulties of the course.
///
/// It should be noted that this program is a stop-gap solution with a design that
/// has speed of writing rather than "elegance" in mind.
/// It does not currently implement a GUI.

//use std::fs;
use rand;
use rand::Rng;
use std::f64;
use std::f64::INFINITY;
use std::string::String;
use std::vec::Vec;
use std::str::Chars;

use Compound;

use dis;
use dis_u;
use ff;
use abs;
use absf64;
use parse_compound_json;

use AB_Z;
use TEN;
use R;

pub const TITLE:&str = "

  Med Chem Quiz V.1.1.1   
  2018-10-28              ";
pub const WARNING:&'static str="
Tento soubor cviční je navržen, aby se uživatel zlepšil v chemických výpočtech. \
Primárně je určen k procvičení uživatele ve výpočtech pro modul \"Výpočty v lékařské chemii\". \
Nicméně má tři významné omezení:

1) Jedná se o cvičení, \
které by mělo zlepšit dovednosti uživatele ve výpočtech, \
ale je nepravděpodobné, že pomůže pochopit dané téma. 

2) Je důležité vyzkoušet široké spektrum výpočtů, aby se uživatel stal zběhlým v \"práci s čísly\", \
ale otázky v programu využívají pouze omezený počet pevně daných vzorů zadání. \
Je tedy důležité, aby uživatel čerpal otázky i z jiných zdrojů.

3) Otázky také využívají omezené vzory slovosledu. \
To znamená, že tyto cvičení nevytvoří schopnost řešit problémy v potřebném rozsahu, \
což by bylo žádoucí pro práci s výpočty v \"reálných situacích\". \
Může také vést k \"šoku\", jestliže uživatelé narazí na známé otázky s odlišným slovosledem!

Byli jste varováni!
";
pub const ABOUT:&'static str="

Jméno:       Med Chem Quiz                      
Verze:       1.11                              
Vytvořeno:   2018-11 (2017-11 )                            
Autor:       Aleksey Zholobenko                 
           & Zdenek Dostal                      
Účel:        Výpočty v lékařské chemii,         
             První ročník,                      
             Lékařská fakulta, UP, Olomouc.     
Licence:     CC BY-SA                           

";

//function to display compund in czech.
pub fn form_chem(q:&Compound)->String{
	
	let c_type:&str= if (q.use_weak==false) & (q.salt==true){"Sůl silné kyseliny a silné zásady"
					 }else if (q.use_weak==true) & (q.salt==true) & (q.pka[0].0<7.0){"Sůl slabé kyseliny a silné zásady"
					 }else if (q.use_weak==true) & (q.salt==true) & (q.pka[0].0>7.0){"Sůl silné kyseliny a slabé zásady"
					 }else if (q.use_weak==false) & (q.salt==false) & (q.pka[0].0>7.0){"Silná báze"
					 }else if (q.use_weak==false) & (q.salt==false) & (q.pka[0].0<7.0){"Silná kyselina"
					 }else if (q.use_weak==true) & (q.salt==false) & (q.pka[0].0<7.0){"Slabá kyselina"
					 }else if (q.use_weak==true) & (q.salt==false) & (q.pka[0].0>7.0){"Slabá báze"
					 }else{"Je to chemická látka nějakého druhu."};
	let mut output:Vec<String>=Vec::new();				 
	output.push(format!("{}       {}",
		"Název:",
		format!("{}",q.name[1])));
	output.push(format!("{}    {}",
		"Vzorec:",q.formula[0]));
	output.push(format!("{} {} (g/mol)",
		"Molekulová hmotnost:",q.mmass));
	let pka=if q.pka[0].0!=7.0{format!("{}",q.pka[0].0)}else{"-".to_owned()};
	output.push(format!("{}        {}",
		"pKa:",pka));
	output.push(format!("{}       {}",
		"Typ:",c_type));
	if q.solubility==f64::INFINITY{
	output.push("Rozpustnost: Neomezená rozpustnost ve vodných roztocích.".to_owned());
	}else{
	output.push(format!("{} {}g/100ml",
		"Rozpustnost:",dis(q.solubility)));
	};
	if q.solutes.len()<2{
		output.push(format!("{} nedisociuje ve vodných roztocích.",q.name[1]));
	}else{
		let partial=if q.use_weak==true{" částečně"}else{""};
		output.push(format!("{}",format!("{} ve vodných roztocích disociuje na:",partial)));
		for x in q.solutes.iter(){
			let p_m= if x.2<0 {format!("({}-)",abs(x.2))
					}else if x.2>0 {format!("({}+)",abs(x.2))
					}else{format!("")};
			output.push(format!("{} x {} {}",x.0,x.1,p_m));
		}
	};
	output.join("\n")
}

//formchem for a simple, less helpful minitable.
pub fn mini_form_chem(q:&Compound)->String{
	
	let mut output = Vec::new();
	
	
	output.push(format!("{} {}",
		"Název:",
		format!("{}",q.name[1]))
	);
	output.push(format!("{} {} (g/mol)",
		"Molekulová hmotnost:",q.mmass)
	);
		
	if q.solubility==f64::INFINITY{
		output.push(format!("{} Mísitelný s vodou v jakémkoliv poměru.",
			"Rozpustnost:")
		);
	}else{
		output.push(format!("{} {}g/100mL",
			"Rozpustnost:",dis(q.solubility))
		);
	};
	
	//Show multiple pKas correctly.
	let pka=if q.pka[0].0==7.0{
		"-".to_owned()
	}else if q.pka.len()==0 {
		format!("{}",q.pka[0].0)
	}else{
		let mut pka_temp:Vec<String> = Vec::new();
		for x in q.pka.iter(){pka_temp.push(format!("{}",x.0))};
		pka_temp.join(", ")
	};
		
	output.push(format!("{} {}",
		"pKa:",pka));
		
	output.join(", ")
}


//MOLES QUESTIONS
//MOLES QUESTIONS
//MOLES QUESTIONS
//MOLES QUESTIONS



pub fn q_1_0(compounds:&Vec<Compound>)->(String,String) {
//Question of type n=m/Mr.
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let m:f64=(rand::thread_rng().gen_range(1,2001) as f64)/100.0;
	
	//generate answer.
	let answer=m/c.mmass;
	
	//println!("m: {}",m);
	//println!("answer: {}",answer);
	
	let question=format!("Kolik molů {} je obsaženo v {} sloučeniny?",
		c.name[2],
		format!("{}g",dis(m)));
		
	let answer=format!("Látkové množství {} může být získáno vydělením hmotnosti molární hmotností dané látky.\n\n {}",
		c.name[2],
		format!(" Odpověď = {}mol",dis(answer))
		);
	(question,answer)	
}


pub fn q_1_1(compounds:&Vec<Compound>)->(String,String){
//Question of type m=n*Mr.
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate moles of compound in question.
	let n:f64=(rand::thread_rng().gen_range(1,301) as f64)/100.0;
	
	//generate answer.
	let answer=n*c.mmass;
	
	//println!("n: {}",n);
	//println!("answer: {}",answer);
	
	let question=format!("Kolik gramů {} je obsaženo v {} pevné substance?",
		c.name[2],
		format!("{}mol",dis(n))
	);

	let answer=format!("Hmotnost {} může být vypočtena vynásobením látkového množství {} jeho molarní hmotností.\n\n{}\n",
		c.name[2],
		c.name[2],
		format!("Odpověď = {}g",dis(answer))
	);
	(question,answer)
}


pub fn q_1_2(compounds:&Vec<Compound>)->(String,String){
//Question of type Molarity=n/Vol.

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut m:f64=(rand::thread_rng().gen_range(10,2001) as f64)/100.0;

	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if m/v_litre/10.0>c.solubility{m=m/10.0}else{silly=false}
	};
	
	//generate answer.
	let answer=m/c.mmass/v_litre;
	
	//println!("m: {}",m);
	//println!("v_litre: {}",v_litre);
	//println!("answer: {}",answer);
	
	let question=format!("Jaká je molarita roztoku {}, který vznikl rozpuštěním {}g pevné substance v roztoku o objemu {} l?",
		c.name[2],
		dis(m),
		v_litre);
	
	let answer=format!("Látkové množství {}: {}\n\
	Koncentrace {} se získá vydělením látkového množství objemem roztoku.\
	\n\n Odpověď = {}mol/l",
		c.name[2],
		m/c.mmass,
		c.name[2],
		dis(answer));
	(question,answer)
}


pub fn q_1_2b(compounds:&Vec<Compound>)->(String,String){ //INCOMPLETE
//Question of type n=Molarity*Vol.

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,2001) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	
	//generate answer.
	let answer=conc*v_litre;
	
	//println!("c: {}",conc);
	//println!("v_litre: {}",v_litre);
	//println!("answer: {}",answer);
	
	let question=format!("Roztok {} o koncentraci {}mol/l má objem {} l. Kolik molů {} obsahuje?",
		c.name[2],
		dis(conc),
		v_litre,
		c.name[2]);
	
	let answer=format!("Látkové množství {} může být získáno vynásobením objemu roztoku jeho koncentrací.\
	\n\n Odpověď = {}mol",
		c.name[2],
		dis(answer));
	(question,answer)
}


pub fn q_1_3(compounds:&Vec<Compound>)->(String,String){
//Question of type m=V*C*Mr

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,2001) as f64)/1000.0;

	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	
	//generate answer.
	let answer=conc*c.mmass*v_litre;
	
	//println!("c: {}",conc);
	//println!("v_litre: {}",v_litre);
	//println!("answer: {}",answer);
	
	let question=format!("Kolik gramů {} obsahuje {} l roztoku o koncentraci {}mol/l?",
		c.name[2],
		v_litre,
		dis(conc)
	);
	
	let answer=format!("Látkové množství {}: {}\n\n{}\n",
		c.name[2],conc*v_litre,(format!("Odpověď = {}g",dis(answer))));
	(question,answer)	
}


pub fn q_1_4(compounds:&Vec<Compound>)->(String,String){
//C1V1=C2V2 type 1
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut c_1:f64=(rand::thread_rng().gen_range(10,2001) as f64)/1000.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c_1*c.mmass/10.0>c.solubility{c_1=c_1/10.0}else{silly=false}
	};
	
	//generate df
	let df:f64=	rand::thread_rng().gen_range(2,20) as f64;
	
	//generate volume.
	let v_1:f64=(rand::thread_rng().gen_range(5,501) as f64)/250.0;
	let v_2:f64=v_1*df;
	let c_2:f64=c_1/df;
	
	//println!("v_1: {}",v_1);
	//println!("v_2: {}",v_2);
	//println!("c_1: {}",c_1);
	//println!("c_2: {}",c_2);
	
	//diluting or concentrating?
	let diluting= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	let find_c= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	let question:String = if (diluting==true) & (find_c==true) {
		format!("Roztok {} má koncentraci {}mol/l a objem {}l. \
		Tento roztok byl následně zředěn na celkový objem {}l. \n Jaká je koncentrace získaného roztoku?",
		c.name[2],dis(c_1),dis(v_1),dis(v_2))
	}else if (diluting==true) & (find_c==false) {
		format!("Roztok {} má koncentraci {}mol/l a objem {}l. \
		Tento roztok byl následně zředěn na konečnou koncentraci {}mol/l. \n Jaký objem bude mít získaný roztok?",
		c.name[2],dis(c_1),dis(v_1),dis(c_2))
	}else if (diluting==false) & (find_c==true) {
		format!("Zředěný roztok {} má koncentraci {}mol/l a objem {}l. \
		Původní objem roztoku byl {}l. \n Jaká byla koncentrace původního roztoku?",
		c.name[2],dis(c_2),dis(v_2),dis(v_1))
	}else{
		format!("Zředěný roztok {} má koncentraci {}mol/l a objem {}l. \
		Původní koncentrace roztoku byla {}mol/l. \n Jaký byl objem původního roztoku?",
		c.name[2],dis(c_2),dis(v_2),dis(c_1))
	};
	
	let answer_a = format!("Cpůvodní x Vpůvodní = Ckonečná x Vkonečný");
	
	let answer_b = if (diluting==true) & (find_c==true) {
		format!("{}\n",format!("Odpověď (Ckonečná) = {}mol/l\n",dis(c_2)))
	}else if (diluting==true) & (find_c==false) {
		format!("{}\n",format!("Odpověď (Vkonečný) = {}l\n",dis(v_2)))
	}else if (diluting==false) & (find_c==true) {
		format!("{}\n",format!("Odpověď (Cpůvodní) = {}mol/l\n",dis(c_1)))
	}else{
		format!("{}\n",format!("Odpověď (Vpůvodní) = {}l\n",dis(v_1)))
	};
	let answer=format!("{}\n\n{}",answer_a,answer_b);	
	(question,answer)
}


pub fn q_1_4b(compounds:&Vec<Compound>)->(String,String){
//Dilution factor type based on C1V1=C2V2.
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut c_1:f64=(rand::thread_rng().gen_range(10,2001) as f64)/1000.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c_1*c.mmass/10.0>c.solubility{c_1=c_1/10.0}else{silly=false}
	};
	
	//generate df
	let df:f64=	rand::thread_rng().gen_range(2,20) as f64;
	
	//generate volume.
	let v_1:f64=(rand::thread_rng().gen_range(5,501) as f64)/250.0;
	let v_2:f64=v_1*df;
	let c_2:f64=c_1/df;
	
	//diluting or concentrating?
	let diluting= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	let find_c= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	//x-fold or 1:(x-1) ?
	let fold= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	 let fold_or_to= if fold==true {format!("{}x",df)}else{format!("v poměru 1:{}",df-1.0)};
	
	//PRINT QUESTION
	let question:String = if (diluting==true) & (find_c==true) {
		format!("Roztok {} má koncentraci {}mol/l a objem {}l. \
		Následně je naředěn {}. \nJaká je koncentrace získaného roztoku?",
		c.name[2],dis(c_1),dis(v_1),fold_or_to)
	}else if (diluting==true) & (find_c==false) {
		format!("Roztok {} má koncentraci {}mol/l a objem {}l. \
		Následně je naředěn {}. \
		\nJaký je objem získaného roztoku? \
		\nJaký objem rozpouštědla byl do roztoku přidán?",
		c.name[2],dis(c_1),dis(v_1),fold_or_to)
	}else if (diluting==false) & (find_c==true) {
		format!("Roztok {} byl naředěn {}. Naředěný roztok má koncentraci {}mol/l a objem {}l. \
		\nJaká byla koncentrace původního roztoku?",
		c.name[2],fold_or_to,dis(c_2),dis(v_2))
	}else{
		format!("Roztok {} byl naředěn {}. Naředěný roztok má koncentraci {}mol/l a objem {}l. \
		\nJaký byl objem původního roztoku? \
		\nJaký byl objem přidaného rozpouštědla?",
		c.name[2],fold_or_to,dis(c_2),dis(v_2))
	};
	
	let answer_a = format!("Cpůvodní x Vpůvodní = Ckonečná x Vkonečný");
	let answer_b = format!("Ředící faktor = Vkonečný/Vpůvodní = Cpůvodní/Ckonečná.");
	let answer_c = if !fold {format!("Roztok zředěn 1:(ředící faktor-1)")}else{format!("")};
	let answer_cii = format!("Ředící faktor = {}",df);
	
	let answer_d = if (diluting==true) & (find_c==true) {
		format!("{}\n",format!("Odpověď (Ckonečná) = {}mol/l\n",dis(c_2)))
	}else if (diluting==true) & (find_c==false) {
		format!("{}\n(Objem rozpouštědla = {}l)\n",format!("Odpověď (Vkonečný) = {}l",dis(v_2)),dis(v_2-v_1))
	}else if (diluting==false) & (find_c==true) {
		format!("{}\n",format!("Odpověď (Cpůvodní) = {}mol/l\n",dis(c_1)))
	}else{
		format!("{}\n(Objem rozpouštědla = {}l)\n",format!("Odpověď (Vpůvodní) = {}l",dis(v_1)),dis(v_2-v_1))
	};	
	let answer=format!("{}\n{}\n{}\n{}\n\n{}\n",
		answer_a,
		answer_b,
		answer_c,
		answer_cii,
		answer_d);	
	(question,answer)
}

//Typy otázek na osmolaritu.
//Typy otázek na osmolaritu.
//Typy otázek na osmolaritu.
//Typy otázek na osmolaritu.
//Typy otázek na osmolaritu.
//Typy otázek na osmolaritu.


pub fn q_2_0(compounds:&Vec<Compound>)->(String,String){
//Question of type Osmoles=sum(Cs).

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let m:f64=(rand::thread_rng().gen_range(1,2001) as f64)/100.0;
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	let answer=m/c.mmass*(solutes as f64);
	
	let question = format!("Kolik osmoticky aktivních částic je obsaženo v {}g {}?",
			dis(m),
			c.name[2]);
	
	let answer = format!("Počet osmoticky aktivních částic {} může být vypočítán sečtením látkových množství jednotlivých částic vzniklých disociací.\n\n {}\n",
		c.name[2],
		format!("Odpověď = {}osmol",dis(answer))
	);
	(question,answer)
}


pub fn q_2_1(compounds:&Vec<Compound>)->(String,String){
//Question of type m=osmoles*Mr.
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate moles of compound in question.
	let n:f64=(rand::thread_rng().gen_range(1,301) as f64)/100.0;
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	let answer=n*c.mmass/(solutes as f64);
	
	let question = format!("Kolik gramů {} je obsaženo v {}osmol látky?",
		c.name[2],dis(n));
	
	let answer_0 = format!("x osmolů {} obsahuje z důvodu disociace x/{} molů {}.\n",
        c.name[2],
        solutes,
        c.name[2]);
    
    let answer = format!("{}Následně může být vypočítána hmotnost {} vynásobením látkového množství dané látky její molární hmotností.\n\n {}\n",
        answer_0,
        c.name[2],
        format!("Odpověď = {}g",dis(answer))
    );


	(question,answer)	
}




pub fn q_2_2(compounds:&Vec<Compound>)->(String,String){
//Question of type Osmolarity=sum(Cs)/Vol.

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//generate mass of compound in question.
	let mut m:f64=(rand::thread_rng().gen_range(10,2001) as f64)/100.0;
	
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if m/v_litre/10.0>c.solubility{m=m/10.0}else{silly=false};	
	};
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	
	//generate answer.
	let sf64=solutes as f64;
	let answer=m/c.mmass*sf64/v_litre;
	
	let v_ve = if v_litre<2.0 {"v"}else{"ve"}; 

	let question = format!("Jaká je osmolarita roztoku, který obsahuje {}g {} {} {} l?",
		dis(m),
		c.name[2],
		v_ve,
		v_litre
	);
	
	let answer_a=format!("Látkové množství {} = {}mol",c.name[2],m/c.mmass);
	let answer_b=format!("Počet osmoticky aktivních částic {} = {}osmol",c.name[2],m/c.mmass*sf64);
	let answer_c=format!("{}\n",format!("Odpověď= {}osmol/l",dis(answer)));
	let answer = format!("{}\n{}\n\n {}\n",answer_a,answer_b,answer_c);
	(question,answer)
}


pub fn q_2_3(compounds:&Vec<Compound>)->(String,String){
//Question of type mass=Osmolarity/(n_solutes)*Volume*Mr.

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//generate mass of compound in question.
	let mut osm:f64=(rand::thread_rng().gen_range(10,2001) as f64)/100.0;
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	let sf64=solutes as f64;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if osm/sf64*c.mmass/10.0>c.solubility{osm=osm/10.0}else{silly=false};
	};
	
	//generate answer.
	let answer=osm*c.mmass*v_litre/sf64;

	let v_ve = if v_litre<2.0 {"v"}else{"ve"}; 
	
	let question = format!("Roztok {} má osmolaritu {}osmol/l. Jaká je hmotnost {} {} {}l?",
		c.name[2],
		dis(osm),
		c.name[2],
		v_ve,
		v_litre
	);
	
	let ans_a=format!("Počet osmoticky aktivních částic {} = {} osmol",c.name[2],osm*v_litre);
	let ans_b=format!("Látkové množství {} = {} mol",c.name[2],osm*v_litre/sf64);
	let ans_c=format!("{}\n",format!("Odpověď = {}g",dis(answer)));
	let answer = format!("\n{}\n{}\n\n {}\n",ans_a,ans_b,ans_c);
	(question,answer)	
}


pub fn q_2_4(compounds:&Vec<Compound>)->(String,String){
	//Question of type Osmotic Pressure=1000*R*T*sum(Cs).

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	
	//generate compound vector
	let c_len=v_valid.len();
	let mut comp_vec:Vec<&Compound>=Vec::new();
	let mut c_ind_vec:Vec<usize>=Vec::new();
	for i in 0..c_len{c_ind_vec.push(i)};
	//number of compounds
	let no_comps=rand::thread_rng().gen_range(2,5);
	
	//pick compounds
	for _ in 0..no_comps{
		let indx_a=rand::thread_rng().gen_range(0,c_ind_vec.len());
		let index=c_ind_vec.remove(indx_a);
		comp_vec.push(v_valid[index])
	};
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;
	
	//generate mass of compounds in question.
	let mut m_comps:Vec<f64>=Vec::new();
	for i in 0..no_comps{
		let mut m:f64=(rand::thread_rng().gen_range(50,2001) as f64)/100.0;
		//Check solubility
		if m/v_litre/10.0>comp_vec[i].solubility {m=10.0*comp_vec[i].solubility*v_litre}else{};	
		m_comps.push(m)
	};
	
	//temperature generation.
	let temp_c:f64=rand::thread_rng().gen_range(1,50) as f64;
	let temp_k:f64=temp_c-AB_Z;
	
	//generate answer.
	let mut osmoles=0.0;
	for i in 0..comp_vec.len(){
		let mut solutes=0;
		for x in comp_vec[i].solutes.iter(){
			solutes+=x.0
		};
		osmoles+=m_comps[i]/comp_vec[i].mmass*(solutes as f64)
	};

	let answer=R*temp_k*osmoles/v_litre;

	//Generate question text.
	let question_a=format!("Jaký je osmotický tlak roztoku obsahujícího:");
	let mut question_b:Vec<String>=Vec::new();
	for i in 0..no_comps{
		question_b.push(format!("{}g {}",
			dis(m_comps[i]),
			comp_vec[i].name[2]
		))
	};
	let question_b=question_b.join("\n");
	let v_ve = if v_litre<=1.0 {"v"}else{"ve"}; 
	let question_c=format!("{} {} l při {} stupních Celsia?",v_ve,v_litre,temp_c);
	
	let question = format!("{}\n{}\n{}\n",question_a,question_b,question_c);
	
	//Generate answer text.
	let ans_a=format!("Teplota ( v Kelvinech) = {}",temp_k);
	let ans_b=format!("Osmolarita = {}",osmoles/v_litre);
	let ans_c=format!("{}",format!("Odpověď = {} kPa",&ff(4,answer)));
	let answer = format!("{}\n{}\n\n {}\n",ans_a,ans_b,ans_c);
	(question,answer)
}

//NB this question is not technically correct as it has not been reversed!

pub fn q_2_4s(compounds:&Vec<Compound>)->(String,String){
	//Question of INVERSE type Osmotic Pressure=1000*R*T*sum(Cs).
	//Find mass of of first compound in the list.
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};

	//generate compound vector
	let c_len=v_valid.len();
	let mut comp_vec:Vec<&Compound>=Vec::new();
	let mut c_ind_vec:Vec<usize>=Vec::new();
	for i in 0..c_len{c_ind_vec.push(i)};
	//number of compounds
	let no_comps=rand::thread_rng().gen_range(2,5);
	
	//pick compounds
	for _ in 0..no_comps{
		let indx_a=rand::thread_rng().gen_range(0,c_ind_vec.len());
		let index=c_ind_vec.remove(indx_a);
		comp_vec.push(v_valid[index])
	};
		
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;
	
	//generate mass of compounds in question.
	let mut m_comps:Vec<f64>=Vec::new();
	for i in 0..no_comps{
		let mut m:f64=(rand::thread_rng().gen_range(50,2001) as f64)/100.0;
		//Check solubility
		if m/v_litre/10.0>comp_vec[i].solubility {m=10.0*comp_vec[i].solubility*v_litre}else{};	
		m_comps.push(m)
	};
	
	//temperature generation.
	let temp_c:f64=rand::thread_rng().gen_range(1,50) as f64;
	let temp_k:f64=temp_c-AB_Z;
	
	//generate answer.
	let mut osmoles=0.0;
	for i in 0..comp_vec.len(){
		let mut solutes=0;
		for x in comp_vec[i].solutes.iter(){
			solutes+=x.0
		};
		osmoles+=m_comps[i]/comp_vec[i].mmass*(solutes as f64)
	};
	
	//generate osm_ity_a
	let mut osm_ity_a=0.0;
	for i in 1..comp_vec.len(){
		let mut solutes=0;
		for x in comp_vec[i].solutes.iter(){solutes+=x.0};
		osm_ity_a+=m_comps[i]/comp_vec[i].mmass*(solutes as f64)
	};
	osm_ity_a=osm_ity_a/v_litre;

	let osm_p=R*temp_k*osmoles/v_litre;
	let osm_ity:f64=osm_p/R/temp_k;
	let osm_ity_b:f64=osm_ity-osm_ity_a;
	let mut sol_x=0;
	for x in comp_vec[0].solutes.iter(){sol_x+=x.0};
	let sol_x:f64=sol_x as f64;

	//Generate question text.
	let question_a=format!("Roztok o objemu {} l obsahuje:",v_litre);
	let mut question_b:Vec<String>=Vec::new();
	for i in 1..no_comps{
		question_b.push(format!("{}g {}...",
			dis(m_comps[i]),
			comp_vec[i].name[2]
		))
			
	};
	let question_b=question_b.join("\n");
	let question_c=format!("...a neznámé množství {}.",comp_vec[0].name[2]);
	let question_d=format!("Jestliže je celkový osmotický tlak {}kPa při {} stupních Celsia, jaká je hmotnost {} v roztoku?",
		dis(osm_p),
		temp_c,
		comp_vec[0].name[2]
	);
	let question = format!("{}\n{}\n{}\n{}\n",question_a,question_b,question_c,question_d);
	
	//Generate answer text.
	let ans_a=format!("Teplota (v Kelvinech) = {}",temp_k);	
	let ans_b=format!("Osmolarita (všechna řešení) = {}",osm_ity);
	let ans_c=format!("Osmolarita (všechna řešení kromě {}) = {}",comp_vec[0].name[2],osm_ity_a);
	let ans_d=format!("Osmolarita {} = {}",comp_vec[0].name[2],osm_ity_b);
	let ans_e=format!("Molarita {} = {}",comp_vec[0].name[2],osm_ity_b/sol_x);
	let ans_f=format!("{}",format!("Odpoveď = {}g",dis(m_comps[0])));
	let answer = format!("{}\n{}\n{}\n{}\n{}\n\n {}\n",ans_a,ans_b,ans_c,ans_d,ans_e,ans_f);
	(question,answer)
}

//IONIC SRENGTH QUESTION TYPES.
//IONIC SRENGTH QUESTION TYPES.
//IONIC SRENGTH QUESTION TYPES.
//IONIC SRENGTH QUESTION TYPES.



pub fn q_3_0(compounds:&Vec<Compound>)->(String,String){
//Question of type I=1/2*sum(cq^2).
//println!("q_3_0");

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,601) as f64)/500.0;
	//println!("A");
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	//println!("B");
	//generate answer.
	let mut spq=0.0;
	for x in c.solutes.iter(){
		spq+= conc*(x.0 as f64)*((abs(x.2)*abs(x.2)) as f64)/2.0;
	};
	
	let name_or_form = if rand::thread_rng().gen_range(0,600)>300{
		format!("{}M {}",dis(conc),c.name[2])
	}else{
		format!("{}M {}",dis(conc),c.formula[0])
	};
	//println!("C spq: {}",spq);
	let question = format!("Jaká je iontová síla roztoku {}?",
	name_or_form);
	//println!("D");
	let mut answer_a:Vec<String>=Vec::new();
	for x in c.solutes.iter(){
		if x.2!=0{
			answer_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)))
		}else{
			answer_a.push(format!("{} netvoří ionty",x.1))
		}
	};
	//println!("D2");
	let answer_b=format!("{}",format!("Odpověď = {}",dis_u(spq)));
	let answer = format!("{}\n\n {}\n",answer_a.join("\n"),answer_b);
	//println!("E");
	(question,answer)
}


pub fn q_3_1(compounds:&Vec<Compound>)->(String,String){
//Question of type c=2*I/(sum(soln*q^2)).
//Aka reverse ionic strength question.
//println!("q_3_1");
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut qs_sum=0;
		for y in x.solutes.iter(){qs_sum+=(y.2)*(y.2)};
		if (qs_sum>1)
		 & ((x.use_weak==false) || (x.salt==true)){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,601) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	
	//generate answer.
	let mut spq=0.0;
	for x in c.solutes.iter(){
		spq+= conc*(x.0 as f64)*(x.2 as f64)*(x.2 as f64)/2.0;
	};
		
	let question = format!("Jaká je koncentrace roztoku {} o iotnové síle {}?",c.name[2],dis_u(spq));
	
	let mut answer_a:Vec<String>=Vec::new();
	for x in c.solutes.iter(){
		if x.2!=0{
			answer_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)));
			answer_a.push(format!("Proto ([{}] x q^2)/C = {}",x.1,(x.0 as i8)*(x.2)*(x.2)))
		}else{
			answer_a.push(format!("{} netvoří ionty",x.1))
		}
	};
	let answer_b=format!("{}",format!("Odpověď = {}mol/l",dis(conc)));
	let answer_a=answer_a.join("\n");
	let answer = format!("{}\n\n {}\n",answer_a,answer_b);
	(question,answer)
}


pub fn q_3_2(compounds:&Vec<Compound>)->(String,String){
//Question of type I=Σ(m*Mr/V*q^2).
//println!("q_3_2");

	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let mut m:f64=(rand::thread_rng().gen_range(10,2001) as f64)/200.0;
	let v_litre:f64=(rand::thread_rng().gen_range(10,2001) as f64)/500.0;
	//println!("A");
	//Check solubility
	let mut silly=true;
	while silly==true{
		if m/v_litre/10.0>c.solubility{m=m/10.0}else{silly=false};
	};	
	//println!("B");
	//generate answer.
	let conc=m/c.mmass/v_litre;
	let mut spq=0.0;
	for x in c.solutes.iter(){
		spq+= conc*(x.0 as f64)*((abs(x.2)*abs(x.2)) as f64)/2.0;
	};
	
	//Print Question.
	let question = format!("Roztok o objemu {}l obsahuje {}g {}. Jaká je jeho iontová síla?",
	dis(v_litre),
	dis(m),
	c.name[2]);
	//println!("D");
	//Print Answer.
	let answer_a=format!("Koncentrace {} = {}mol/l",c.name[2],&ff(4,conc));
	let mut answer_b:Vec<String>=Vec::new();
	for x in c.solutes.iter(){
		if x.2!=0{
			answer_b.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)))
		}else{
			answer_b.push(format!("{} netvoří ionty",x.1))
		}
	};
	//println!("D2");
	let answer_c=format!("{}",format!("Odpověď = {}",dis_u(spq)));
	let answer_b=answer_b.join("\n");
	let answer = format!("{}\n{}\n\n {}\n",answer_a,answer_b,answer_c);
	//println!("E");
	(question,answer)
}


pub fn q_3_2b(compounds:&Vec<Compound>)->(String,String){
//Question of type I=Σ(m*Mr/V*q^2). Variant
//FIND V!
//println!("q_3_2b");	
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut qs_sum=0;
		for y in x.solutes.iter(){qs_sum+=abs((y.2)*(y.2))};
		if (qs_sum>0)
		 & ((x.use_weak==false) || (x.salt==true)){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//check "silly physics" loops.
	let mut silly=true;
	let mut m:f64=0.0;
	let mut i_s:f64=0.0;
	let mut moles:f64=0.0;
	let mut solutes:f64;
	let mut conc:f64=0.0;
	let mut v_litre:f64=0.0;
	let mut f:f64=1.0;
		
	while silly==true{
		//generate mass of compound in question.
		m=(rand::thread_rng().gen_range(10,2001) as f64)/200.0;
		i_s=(rand::thread_rng().gen_range(10,2001) as f64)/500.0/f;
//		println!("i_s generated: {}",i_s);
	
		//generate answer.
		moles=m/c.mmass;
		let mut solutes_a=0.0;
		for x in c.solutes.iter(){
			solutes_a+= (x.0 as f64)*((abs(x.2)*abs(x.2)) as f64);
//			println!("solutes_a={}",solutes_a)
		};
		solutes=solutes_a;
//		println!("solutes value: {}\n solutes_a value: {}",solutes,solutes_a);
		conc=i_s*2.0/solutes;
		v_litre=moles/conc;
		
		//check solubility (silly version). 
		if m/v_litre/10.0>c.solubility{
			f=f*10.0;
		}else{silly=false}
	};
	
	//Print Question.

	let question = format!("Na přípravu roztoku {} bylo použito {}g látky. Jeho iontová síla je {}. Jaký je objem připraveného roztoku?",c.name[2],dis(m),dis_u(i_s));
	
	//Print Answer.
	let mut factor:usize=0;
	let mut ans_a:Vec<String>=Vec::new();
	for x in c.solutes.iter(){
		if x.2!=0{
			ans_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)));
			ans_a.push(format!("Proto ([{}] x q^2)/C = {}",x.1,(x.0 as i8)*(x.2)*(x.2)));
			factor+=((x.0 as i8)*(x.2)*(x.2)) as usize;
		}else{
			ans_a.push(format!("{} netvoří ionty",x.1))
		}
	};
	let ans_a=ans_a.join("\n");
	let ans_b=format!("\n2 x I/C = {}",factor);
	let ans_c=format!("Proto je koncentrace {} = {} mol/l",c.name[2],conc);
	let ans_d=format!("Látkové množstvý = {} mol",moles);
	let ans_e=format!("{}",format!("Odpověď = {}l",dis(v_litre)));
	let answer = format!("{}\n{}\n{}\n{}\n\n {}\n",ans_a,ans_b,ans_c,ans_d,ans_e);
	(question,answer)
}


pub fn q_3_2c(compounds:&Vec<Compound>)->(String,String){
//Question of type I=Σ(m*Mr/V*q^2). Variant
//FIND m!
//println!("q_3_2c");
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut qs_sum=0;
		for y in x.solutes.iter(){qs_sum+=(y.2)*(y.2)};
		if (qs_sum>0)
		 & ((x.use_weak==false) || (x.salt==true)){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	
	//check "silly physics" loops. And initiate variables.
	let mut silly=true;	
	let mut m=0.0;
	let mut i_s=0.0;
	let mut moles;
	let mut solutes;
	let mut conc=0.0;
	let mut v_litre=0.0;
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	let mut f=1.0;
	
	while silly==true{		
		//generate compound
		
	
		//generate mass of compound in question.
		v_litre=(rand::thread_rng().gen_range(10,2001) as f64)/500.0;
		i_s=(rand::thread_rng().gen_range(10,2001) as f64)/500.0/f;
//		println!("i_s generated: {}",i_s);
	
		//generate answer.
		let mut solutes_a:f64=0.0;
		for x in c.solutes.iter(){
			solutes_a+= (x.0 as f64)*((abs(x.2)*abs(x.2)) as f64);
//			println!("solutes_a={}",solutes_a)
		};
		solutes=solutes_a;
//		println!("solutes value: {}\n solutes_a value: {}",solutes,solutes_a);
		conc=i_s*2.0/solutes;
		moles=conc*v_litre;
		m=moles*c.mmass;
	
		//check solubility (silly version). 
		if m/v_litre/10.0>c.solubility{f=f*10.0}else{silly=false}
	};
	
	//Print Question.
	let question = format!("Roztok {} má iontovou sílu {}. Kolik gramů {} obsahuje {}l roztoku?",
		c.name[2],
		dis_u(i_s),
		c.name[2],
		dis(v_litre));
	
	//Print Answer.
	let mut ans_a:Vec<String>=Vec::new();
	let mut factor:usize=0;
	for x in c.solutes.iter(){
		if x.2!=0{
			ans_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)));
			ans_a.push(format!("Proto ([{}] x q^2)/C = {}",x.1,(x.0 as i8)*(x.2)*(x.2)));
			factor+=((x.0 as i8)*(x.2)*(x.2)) as usize;
		}else{
			ans_a.push(format!("{} netvoří ionty",x.1))
		}
	};
	ans_a.push(format!("\n2 x I/C= {}",factor));
	ans_a.push(format!("Proto je koncentrace {} = {} mol/l",c.name[2],&ff(4,conc)));
	let ans_b=format!("{}",format!("Odpověď = {}g",dis(m)));
	let ans_a=ans_a.join("\n");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)
}



pub fn q_4_0(compounds:&Vec<Compound>)->(String,String){
//Calculate Ksp from solubility
	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.2)
		 & ((x.solubility/x.mmass)>0.0000001) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	
	//generate solubility
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	
	//generate answer.
	let s_c:f64=s_m_100*10.0/c.mmass;
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let answer=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	//PRINT QUESTION
	let question = format!("Jaké je Ksp {}, jestliže je rozpustnost látky za daných podmínek {}g/100ml?",
		c.name[2],
		dis(s_m_100));
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("s = {} mol/l\n",s_c));
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0))
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0))
		};
	};
	ans_a.push(format!("\nKsp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("{}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0))
		}else{
			ans_a.push(format!(" x {}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0))
		};
	};
	ans_a.push(format!("\nKsp = {}s^{}",multiple,power));
	let mut ans_b:Vec<String>=Vec::new();
	ans_b.push(format!("{}",format!("Odpověď = {}",dis_u(answer))));
	if c.solubility*10.0/c.mmass>1.0{
		ans_b.push(format!("Pečlivě dodržujte - tato metoda pro výpočet Ksp by neměla být používána pro dobře rozpustné sloučeniny jako {}.",
		c.name[1]))
	}else{
		ans_b.push(format!(""))
	};
	let ans_a=ans_a.join("");
	let ans_b=ans_b.join("\n");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)	
}


pub fn q_4_0a(compounds:&Vec<Compound>)->(String,String){
//Calculate solubility from Ksp.
	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.2)
		 & ((x.solubility/x.mmass)>0.0000001) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	
	//generate solubility (NB this reparsing step makes the answer more "exact".)-not perfect for this kind of question.
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	
	//generate answer.
	let s_c:f64=s_m_100*10.0/c.mmass;
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let answer=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	//PRINT QUESTION
	let question = format!("Jaká je rozpustnost {} (v g/100ml), jestliže je jeho Ksp {}?",
	c.name[2],dis_u(answer));
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0))
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\nKsp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("{}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0));
		}else{
			ans_a.push(format!(" x {}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\nKsp = {}s^{}\n",multiple,power));
	if multiple==""{
		ans_a.push(format!("s = (Ksp{})^(1/{})\n",multiple,power));
	}else{
		ans_a.push(format!("s = (Ksp/{})^(1/{})\n",multiple,power));
	};
	
	ans_a.push(format!("s = {}mol/l",dis(s_c)));
	let ans_a=ans_a.join("");
	
	let mut ans_b:Vec<String>=Vec::new();
	ans_b.push(format!("{}",format!("Odpověď = {}g/100ml",dis(s_m_100))));
	if c.solubility*10.0/c.mmass>1.0{
		ans_b.push(format!("Pečlivě dodržujte - tato metoda pro výpočet Ksp by neměla být používána pro dobře rozpustné sloučeniny jako {}.",c.name[1]));
	}else{
		ans_b.push(format!(""));
	};
	let ans_b=ans_b.join("\n");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)	
}

//NB This question is not in the general form. only works for binary ions.

pub fn q_4_1(compounds:&Vec<Compound>)->(String,String){
//Calculate concentration of one ion from Ksp and concentration of the other.
	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.05)
		 & ((x.solubility/x.mmass)>0.0000001) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	let var_b:f64=1.0+(rand::thread_rng().gen_range(1,100) as f64)/10.0;
	
	//generate solubility
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	let s_c:f64=s_m_100*10.0/c.mmass;
	
	//generate ion to solve for.
	let ss_len=c.solutes.len();
	let wch_ion:usize=rand::thread_rng().gen_range(0,ss_len);
	let mut known:(&str,f64,u8)=("",0.0,0);
	let mut unknown:(&str,f64,u8)=("",0.0,0);
	for i in 0..ss_len{
		if i==wch_ion{
			known=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}else{
			unknown=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}
	};
	
	//generate concentration of known ion;
	let c_known:f64= match ff(4,s_c*known.1*var_b).trim().parse(){
		Ok(num)=>num,
		Err(_)=>s_c*known.1*var_b,
	};
	
	//generate Ksp.
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let ksp=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	//generate concentration of known ion;
	let ksp:f64= match ff(4,ksp).trim().parse(){
		Ok(num)=>num,
		Err(_)=>ksp,
	};
	//let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	//let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	//let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	//let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	let c_unknown=(ksp/(c_known).powf(known.1)).powf(1.0/unknown.1);
	
	//PRINT QUESTION
	let question = format!("\
			Za daných podmínek má {} hodnotu Ksp {}. \
			Jestliže [{}] má koncentraci {}mol/l, při jaké molární koncentraci {} \
			začně precipitovat z roztoku?",
			c.name[1],dis_u(ksp),known.0,dis(c_known),unknown.0);
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\n[{}]^{} = Ksp/[{}]^{}",unknown.0,unknown.2,known.0,known.2));
	ans_a.push(format!("\n[{}] = (Ksp/[{}]^{})^(1/{})",unknown.0,known.0,known.2,unknown.2));
	
	//println!("\nKsp = {}s^{}",multiple,power);
	//println!("s=(Ksp/{})^(1/{})",multiple,power);

	let ans_b=format!("{}",format!("Odpověď = {}mol/l\n",dis(c_unknown)));
	let ans_a=ans_a.join("");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)		
}

//NB This question is not in the general form. only works for binary ions.

pub fn q_4_1b(compounds:&Vec<Compound>)->(String,String){
//Calculate concentration of one ion from solubility and concentration of the other.
	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.052)
		 & ((x.solubility/x.mmass)>0.0000001) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	let var_b:f64=1.0+(rand::thread_rng().gen_range(1,100) as f64)/10.0;
	
	//generate solubility (this should make answer more accurate).
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	let s_c:f64=s_m_100*10.0/c.mmass;
	
	//generate ion to solve for.
	let ss_len=c.solutes.len();
	let wch_ion:usize=rand::thread_rng().gen_range(0,ss_len);
	let mut known:(&str,f64,u8)=("",0.0,0);
	let mut unknown:(&str,f64,u8)=("",0.0,0);
	for i in 0..ss_len{
		if i==wch_ion{
			known=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}else{
			unknown=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}
	};
	
	//generate concentration of known ion (this should make answer more accurate).;
	let c_known:f64= match ff(4,s_c*known.1*var_b).trim().parse(){
		Ok(num)=>num,
		Err(_)=>s_c*known.1*var_b,
	};
	
	//generate Ksp.
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let ksp=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	
	let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	let c_unknown=(ksp/(c_known).powf(known.1)).powf(1.0/unknown.1);
	
	//PRINT QUESTION
	let question = format!("\
			Rozpustnost {} za daných podmínek je {}g/100ml. \
			Jestliže je [{}] {}mol/l, při jaké [{}] {} \
			začne precipitovat z roztoku?",
			c.name[2],dis(s_m_100),known.0,dis(c_known),unknown.0,unknown.0);
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\nKsp = {}s^{}",multiple,power));
	ans_a.push(format!("\nKsp = {}",dis_u(ksp)));
	ans_a.push(format!("\n[{}]^{} = Ksp/[{}]^{}",unknown.0,unknown.2,known.0,known.2));
	ans_a.push(format!("\n[{}] = (Ksp/[{}]^{})^(1/{})",unknown.0,known.0,known.2,unknown.2));
	
	//println!("\nKsp = {}s^{}",multiple,power);
	//println!("s=(Ksp/{})^(1/{})",multiple,power);

	let ans_b=format!("{}",format!("Odpověď = {}mol/l",dis(c_unknown)));
	let ans_a=ans_a.join("");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)	
}

//pH strong
//pH strong (THIS FUNCTION IS OK)

pub fn q_6_0(compounds:&Vec<Compound>)->(String,String){
//Find pH from concentration.
	
	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let mut conc:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c.solubility==f64::INFINITY{silly=false; continue
		}else{
			if conc*c.mmass/10.0>c.solubility{conc=conc/25.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let conc:f64= match ff(4,conc).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>conc,
	};
	
	let mut eff_conc=0.0;
	let mut acid=true;
	
	//generate answer. (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	let p_h;
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else{}
	};
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	if acid==true{
		p_h= if c.use_weak==false {0.0-(eff_conc).log(10.0)}
			else {0.5*(c.pka[0].0-(eff_conc).log(10.0))}
	}else{
		p_h= if c.use_weak==false {14.0+(eff_conc).log(10.0)}
			else {7.0+0.5*(c.pka[0].0+(eff_conc).log(10.0))}
	};
	
	//Print Question.
	let question = format!("Jaké je pH roztoku {}, jestliže je jeho koncentrace {}mol/l?",
	c.name[2],dis(conc));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	if c.use_weak==false{
		ans_a.push(format!("pH = -log[H+]"));
		if acid==false{ans_a.push(format!("pOH = -log[OH-]\npH = 14-pOH"))}
	}else{
		if acid==true{
			ans_a.push(format!("Tato sloučenina se chová jako slabá kyselina."))
		}else{
			ans_a.push(format!("Tato sloučenina se chová jako slabá báze."))
		}
	};
	let ans_b=format!("{}",format!("pH = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)
}

//THIS FUNCTION IS NOW FINE (But can give above limit concentrations)

pub fn q_6_0b(compounds:&Vec<Compound>)->(String,String){
//Find concentration. from pH
	
	//Generate acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let mut acid:bool=true;
	let mut n:usize=0;
	
	//Decide which method to use (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else{}
	};
	
	let n_f64=n as f64;
	let pre_p_h=((rand::thread_rng().gen_range(0,4001)-800) as f64)/1000.0;
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	let p_h;
	if acid==true{
		p_h= if c.use_weak==false {pre_p_h}
			else {0.5*(c.pka[0].0+pre_p_h)}
	}else{
		p_h= if c.use_weak==false {14.0-pre_p_h}
			else {7.0+0.5*(c.pka[0].0-pre_p_h)}
	};
	
	let p_h= match ff(4,p_h).trim().parse(){
		Ok(num)=>num,
		Err(_)=>p_h,
	};
	
	let conc;
	if acid==true{
		conc= if c.use_weak==false {TEN.powf(0.0-p_h)/n_f64}
			else {TEN.powf(c.pka[0].0-2.0*p_h)/n_f64}
	}else{
		conc= if c.use_weak==false {TEN.powf(p_h-14.0)/n_f64}
			else {TEN.powf((p_h-7.0)*2.0-c.pka[0].0)/n_f64}
	};
	
	//Print Question.
		let question = format!("Roztok {} má pH {}. Jaká je jeho koncentrace?",
		c.name[2],&ff(4,p_h));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	if (acid==true) & (c.use_weak==false){
		ans_a.push(format!("pH = -log[H+]"));
		ans_a.push(format!("[H+] = 10^(-pH)"));
		ans_a.push(format!("{} x c = 10^(-pH)",n));
	}else if c.use_weak==false{
		ans_a.push(format!("pOH = -log[OH-]\n-log[OH-] = 14-pH"));
		ans_a.push(format!("[OH-] = 10^(pH-14)"));
		ans_a.push(format!("{} x c = 10^(pH-14)",n));
	}else if (acid==true) & (c.use_weak==true){
		ans_a.push(format!("pH = 0.5 x (pKa - log({} x c))",n));
		ans_a.push(format!("log({} x c) = pKa - (2 x pH)",n));
	}else{
		ans_a.push(format!("pH = 7 + 0.5 x (pKa + log({} x c))",n));
		ans_a.push(format!("log({} x c) = 2 x (pH - 7) - pKa",n));
	};
	let ans_b=format!("{}",format!("Odpověď = {}mol/l",dis(conc)));
	let ans_c=if conc>c.solubility*c.mmass/10.0 {
		format!("(Jedná se o poněkud nesmyslnou otázku, protože \"správná\" odpověď překračuje rozpustnost sloučeniny).")
	}else{
		format!("")
	};
	let answer = format!("{}\n\n {}\n{}\n",ans_a.join("\n"),ans_b,ans_c);
	(question,answer)
}

//pH strong (THIS FUNCTION IS OK)

pub fn q_6_1(compounds:&Vec<Compound>)->(String,String){
//Find pH from mass.
	
	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let mut conc:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c.solubility==f64::INFINITY{silly=false; continue
		}else{
			if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let conc:f64= match ff(4,conc).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>conc,
	};
	
	//find mass.
	let m=conc*c.mmass*v_litre;
	
	let mut eff_conc=0.0;
	let mut acid=true;
	
	//generate answer. (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	let p_h;
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else{}
	};
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	if acid==true{
		p_h= if c.use_weak==false {0.0-(eff_conc).log(10.0)}
			else {0.5*(c.pka[0].0-(eff_conc).log(10.0))}
	}else{
		p_h= if c.use_weak==false {14.0+(eff_conc).log(10.0)}
			else {7.0+0.5*(c.pka[0].0+(eff_conc).log(10.0))}
	};
	
	//Print Question.
	let question = format!("Roztok obsahuje {}g {} v {}l roztoku. Jaké je pH tohoto roztoku?",
	dis(m),c.name[2],dis(v_litre));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	ans_a.push(format!("Koncentrace {}: {} mol/l",c.name[2],conc));
	if c.use_weak==false{
		ans_a.push(format!("pH = -log[H+]"));
		if acid==false{ans_a.push(format!("pOH = -log[OH-]\npH = 14-pOH"));}
	}else{
		if acid==true{
			ans_a.push(format!("Tato sloučenina se chová jako slabá kyselina."));
		}else{
			ans_a.push(format!("Tato sloučenina se chová jako slabá báze."));
		}
	};
	let ans_b=format!("{}",format!("pH = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)
}

//THIS FUNCTION should now give OK answers.

pub fn q_6_1b(compounds:&Vec<Compound>)->(String,String){
//Find mass. from pH
	
	//Generate acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let v_litre:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	let mut acid:bool=true;
	let mut n:usize=0;
	
	//Decide which method to use (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else{}
	};
	
	let n_f64=n as f64;
	let pre_p_h=((rand::thread_rng().gen_range(0,4001)-800) as f64)/1000.0;
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	let p_h;
	if acid==true{
		p_h= if c.use_weak==false {pre_p_h}
			else {0.5*(c.pka[0].0+pre_p_h)}
	}else{
		p_h= if c.use_weak==false {14.0-pre_p_h}
			else {7.0+0.5*(c.pka[0].0-pre_p_h)}
	};
	
	let p_h= match ff(4,p_h).trim().parse(){
		Ok(num)=>num,
		Err(_)=>p_h,
	};
	
	let conc;
	if acid==true{
		conc= if c.use_weak==false {TEN.powf(0.0-p_h)/n_f64}
			else {TEN.powf(c.pka[0].0-2.0*p_h)/n_f64}
	}else{
		conc= if c.use_weak==false {TEN.powf(p_h-14.0)/n_f64}
			else {TEN.powf((p_h-7.0)*2.0-c.pka[0].0)/n_f64}
	};
	
	let m:f64=conc*v_litre*c.mmass;
	
	//Print Question.
	let question = format!("pH roztoku {} je {}. Kolik gramů {} je v takovém roztoku o objemu {}l?",
	c.name[2],&ff(4,p_h),c.name[2],dis(v_litre));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	ans_a.push(format!("Koncentrace {}: {} mol/l",c.name[2],conc));
	if (acid==true) & (c.use_weak==false){
		ans_a.push(format!("pH = -log[H+]"));
		ans_a.push(format!("[H+] = 10^(-pH)"));
		ans_a.push(format!("{} x c = 10^(-pH)",n));
	}else if c.use_weak==false{
		ans_a.push(format!("pOH = -log[OH-]\n-log[OH-] = 14-pH"));
		ans_a.push(format!("[OH-] = 10^(pH-14)"));
		ans_a.push(format!("{} x c = 10^(pH-14)",n));
	}else if (acid==true) & (c.use_weak==true){
		ans_a.push(format!("pH = 0.5 x (pKa - log({} x c))",n));
		ans_a.push(format!("log({} x c) = pKa - (2 x pH)",n));
	}else{
		ans_a.push(format!("pH = 7 + 0.5 x (pKa + log({} x c))",n));
		ans_a.push(format!("log({} x c) = 2 x (pH - 7) - pKa",n));
	};
	let ans_c=if conc>c.solubility*c.mmass/10.0 {
		format!("(Jedná se o poněkud nesmyslnou otázku protože \"správná\" odpověď překračuje rozpustnost sloučeniny).")
	}else{
		format!("")
	};
	let ans_b=format!("{}",format!("Odpověď = {}g",dis(m)));
	let answer = format!("{}\n\n {}\n{}\n",ans_a.join("\n"),ans_b,ans_c);
	(question,answer)
}


//THIS FUNCTION SHOULD BE OK.

pub fn q_6_2a(compounds:&Vec<Compound>)->(String,String){
//Reaction between strong acids and bases.


	//set bronsted acid & base. (name,x/acid,x/c)
	let mut a_bron:(&str,u8,u8)=("H",1,1);
	let mut b_bron:(&str,u8,u8)=("OH",1,1);
	
	//Generate acid.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		let mut acid=false;
		//let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		if ((gives_h==true) & (x.pka[0].0<6.0))
		|| ((gives_h==false) & (x.salt==true) & (x.use_weak==true) & (x.pka[0].0>8.0)){acid=true}else{};
		if (x.solutes.len()==2)
		 & (acid==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	let strong_acid=if a.use_weak==true {false}else{true};
	
	
	//Generate base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut base=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if (y.1=="HCO3")||(y.1=="OH")||(y.1=="CO3"){gives_oh=true}else{}
		};
		if ((gives_oh==true) & (x.pka[0].0>8.0))
		|| ((gives_oh==false) & (x.salt==true) & (x.use_weak==true)& (x.pka[0].0<6.0)){base=true}else{};
		if (base==true)
		 & ((x.use_weak==false)||(strong_acid==true)){
			valid_c.push(&x)
		}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let b=&valid_c[indx];
	
	//determine bronsted acid (name,acid/acid,x/c)
	for x in a.solutes.iter(){
			if x.1=="H"{a_bron=("H",1,x.0)}else{}
	};
	
	//determine bronsted base (name,acid/base,x/c)
	for x in b.solutes.iter(){
		let aob=(abs(x.2)) as u8;
		if (x.1=="OH")||(x.1=="HCO3")||(x.1=="CO3")||((x.1==b.pka[0].1) & b.salt & b.use_weak){b_bron=(&x.1,aob,x.0)}else{}
	};
	
	//generate concentration.
	let mut c_a:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	let mut c_b:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	
	//Some bases are almost insoluble so this reduces acid concentration to match:
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if c_a/10.0>b.solubility/b.mmass{c_a=c_a/10.0}else{silly=false}
		}
	};
	//Extra decimal space removal post solubility check.
	let c_a:f64= match ff(4,c_a).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>c_a,
	};
	
	//Check solubility of base.
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if c_b*b.mmass/10.0>b.solubility{c_b=c_b/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let c_b:f64= match ff(4,c_b).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>c_b,
	};
	
	//generate volume of acid (v_a) and base (v_b).
	let v_a=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	let v_b=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	
	//calculate moles H+ and B-
	let mol_h=c_a*(a_bron.2 as f64)*v_a;
	let mol_oh=c_b*((b_bron.2*b_bron.1) as f64)*v_b;
	
	//calculate moles remaining.
	let molf=absf64(mol_h-mol_oh);
	
	//get final pH
	let p_h;
	if (a.use_weak==false) & (b.use_weak==false){  //strong acid, strong base.
		p_h= if mol_h>mol_oh{-(molf/(v_a+v_b)).log(10.0)				//excess of acid. Strong acid.
			}else if mol_h<mol_oh {14.0+(molf/(v_a+v_b)).log(10.0) 		//excess of base. Strong base.
			}else{7.0													//Neutralisation. ph 7.0
		}
	}else if b.use_weak==false{  //weak acid, strong base.
		p_h= if (mol_h-mol_oh)/mol_oh>=10.0 {0.5*(a.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak acid formula, big excess of acid.
			}else if ((mol_h-mol_oh)/mol_oh>=0.1) & ((mol_h-mol_oh)/mol_oh<10.0) {a.pka[0].0+(mol_oh/molf).log(10.0)  //buffer. small excess of weak acid.
			}else if (molf/mol_oh)<0.1 {7.0+0.5*(a.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak base formula, roughly complete neutralisation.
			}else if -0.1>=(mol_h-mol_oh)/mol_oh {14.0+(molf/(v_a+v_b)).log(10.0)		//strong base formula, excess of base.
			}else{7.0																	//Just in case 7.0?
		}
	}else{    //weak base, strong acid (see base generator to see why).
		p_h= if (mol_oh-mol_h)/mol_h>=10.0 {0.5*(7.0+b.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak base formula, big excess of base.
			}else if ((mol_oh-mol_h)/mol_h>=0.1) & ((mol_oh-mol_h)/mol_h<10.0) {b.pka[0].0+(molf/mol_h).log(10.0)  //buffer. small excess of weak base.
			}else if (molf/mol_h)<0.1 {0.5*(b.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak acid formula, roughly complete neutralisation.
			}else if -0.1>=(mol_oh-mol_h)/mol_h {-(molf/(v_a+v_b)).log(10.0)		//strong acid formula, excess of strong acid.
			}else{7.0																	//Just in case 7.0?
		}
	};
		
	
	//Print Question.
	let question = format!("{}l {} o koncentraci {}mol/l bylo přidáno k {}l {} o koncentraci {}mol/l. \
		 Jaké je pH výsledného roztoku?",
		 dis(v_a),
		 a.name[2],
		 dis(c_a),
		 dis(v_b),
		 b.name[2],
		 dis(c_b));
	
	//Print Answer. (name,acid/acid,x/c)
	let mut ans_a=Vec::new();
	ans_a.push(format!("Celkový objem = {} ml",(v_a+v_b)*1000.0));
	if a.use_weak==true{
		ans_a.push(format!("Kyselina (slabá): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	}else{
	ans_a.push(format!("Kyselina (silná): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	};
	if b.use_weak==true{
	ans_a.push(format!("Báze (slabá): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	}else{
	ans_a.push(format!("Báze (silná): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	};

	ans_a.push(format!("Látkové množství H+: {}n{}",a_bron.2,a.formula[0]));
	ans_a.push(format!("Látkové množství {}({}-): {}n{}",b_bron.0,b_bron.1,b_bron.2,b.formula[0]));
	
	if b.use_weak==false{ //For strong base and weak or strong acid reaction.
		if (mol_h>mol_oh) & (a.use_weak==false) {
			ans_a.push(format!("Přebytek silné kyseliny: použijte pH = -log[H+]"))
		}else if mol_oh>mol_h {
			ans_a.push(format!("Přebytek silné báze: pouzijte pH = 14+log({}[{}({}-)]",b_bron.1,b_bron.0,b_bron.1))
		}else if ((mol_h-mol_oh)/mol_oh>=10.0) & (a.use_weak==true) {
			ans_a.push(format!("Velký přebytek slabé kyseliny: použijte pH = 0.5 x (pKa-log({} x [{}]))",a_bron.1,a.formula[0]))
		}else if ((mol_h-mol_oh)/mol_oh<10.0)
			& ((mol_h-mol_oh)/mol_oh>=0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Malý přebytek slabé kyseliny (pufr!): použijte pH = pKa + log([S]/[A])"))
		}else if ((mol_h-mol_oh)/mol_oh<0.1)
			& ((mol_h-mol_oh)/mol_oh>=-0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Téměř úplná neutralizace slabé kyseliny (počítejte jako sůl slabé báze): \
									použijte pH = 7 + 0.5 x (pKa + log({} x [{}])",b_bron.1,b.formula[0]))
		}else{
			ans_a.push(format!("Úplná neutralizace. pH = 7"))};
	}else{ //NB assumes that acid is a strong acid (see base generator for why).
		if mol_h>mol_oh {
			ans_a.push(format!("Přebytek silné kyseliny: použijte pH = -log[H+]"))
		}else if (mol_oh-mol_h)/mol_h>=10.0 {
			ans_a.push(format!("Velký přebytek slabé báze: použijte pH = 7.0 + 0.5 x (pKa + log({} x [{}]))",b_bron.1,b.formula[0]))
		}else if ((mol_oh-mol_h)/mol_h<10.0)
			& ((mol_oh-mol_h)/mol_h>=0.1) {
				ans_a.push(format!("Malý přebytek slabé báze (pufr!): Použijte pH = pKa + log([S]/[A])"))
		}else if ((mol_oh-mol_h)/mol_h<0.1)
			& ((mol_oh-mol_h)/mol_h>=-0.1) {
				ans_a.push(format!("Téměř úplná neutraliace slabé báze (počítejte jako sůl slabé kyseliny: \
									použijte pH = 0.5 x (pKa + log({} x [{}])",a_bron.1,a.formula[0]))
		}else{
			ans_a.push(format!("Úplná neutralizace. pH = 7"))};
	};
		
		
	let ans_b=format!("{}",format!("Odpověď = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)
}


//THIS FUNCTION SHOULD GIVE THE RIGHT ANSWER.	

pub fn q_6_2b(compounds:&Vec<Compound>)->(String,String){
//Reaction between strong acids and bases. Mass based.


	//set bronsted acid & base. (name,x/acid,x/c)
	let mut a_bron:(&str,u8,u8)=("H",1,1);
	let mut b_bron:(&str,u8,u8)=("OH",1,1);
	
	//Generate acid.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		let mut acid=false;
		//let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		if ((gives_h==true) & (x.pka[0].0<6.0))
		|| ((gives_h==false) & (x.salt==true) & (x.use_weak==true) & (x.pka[0].0>8.0)){acid=true}else{};
		if (x.solutes.len()==2)
		 & (acid==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	let strong_acid=if a.use_weak==true {false}else{true};
	
	
	//Generate base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut base=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if (y.1=="HCO3")||(y.1=="OH")||(y.1=="CO3"){gives_oh=true}else{}
		};
		if ((gives_oh==true) & (x.pka[0].0>8.0))
		|| ((gives_oh==false) & (x.salt==true) & (x.use_weak==true)& (x.pka[0].0<6.0)){base=true}else{};
		if (base==true)
		 & ((x.use_weak==false)||(strong_acid==true)){
			valid_c.push(&x)
		}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let b=&valid_c[indx];
	
	//determine bronsted acid (name,acid/acid,x/c)
	for x in a.solutes.iter(){
			if x.1=="H"{a_bron=("H",1,x.0)}else{}
	};
	
	//determine bronsted base (name,acid/base,x/c)
	for x in b.solutes.iter(){
		let aob=(abs(x.2)) as u8;
		if (x.1=="OH")||(x.1=="HCO3")||(x.1=="CO3")||((x.1==b.pka[0].1) & b.salt & b.use_weak){b_bron=(&x.1,aob,x.0)}else{}
	};
	
	//generate concentration.
	let mut m_a:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	let mut m_b:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	
	//generate volume of acid (v_a) and base (v_b).
	let v_a=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	let v_b=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	
	//Some bases are almost insoluble so this reduces acid concentration to match:
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if m_a/v_a/10.0>b.solubility{m_a=m_a/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let m_a:f64= match ff(4,m_a).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>m_a,
	};
	
	//Check solubility of base.
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if m_b/v_b/10.0>b.solubility{m_b=m_b/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let m_b:f64= match ff(4,m_b).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>m_b,
	};
	
	//generate mass of acid (m_a) and base (m_b).
	let c_a=m_a/a.mmass/v_a;
	let c_b=m_b/b.mmass/v_b;
	
	//calculate moles H+ and B-
	let mol_h=c_a*(a_bron.2 as f64)*v_a;
	let mol_oh=c_b*((b_bron.2*b_bron.1) as f64)*v_b;
	
	//calculate moles remaining.
	let molf=absf64(mol_h-mol_oh);
	
	//get final pH
	let p_h;
	if (a.use_weak==false) & (b.use_weak==false){  //strong acid, strong base.
		p_h= if mol_h>mol_oh{-(molf/(v_a+v_b)).log(10.0)				//excess of acid. Strong acid.
			}else if mol_h<mol_oh {14.0+(molf/(v_a+v_b)).log(10.0) 		//excess of base. Strong base.
			}else{7.0													//Neutralisation. ph 7.0
		}
	}else if b.use_weak==false{  //weak acid, strong base.
		p_h= if (mol_h-mol_oh)/mol_oh>=10.0 {0.5*(a.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak acid formula, big excess of acid.
			}else if ((mol_h-mol_oh)/mol_oh>=0.1) & ((mol_h-mol_oh)/mol_oh<10.0) {a.pka[0].0+(mol_oh/molf).log(10.0)  //buffer. small excess of weak acid.
			}else if (molf/mol_oh)<0.1 {7.0+0.5*(a.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak base formula, roughly complete neutralisation.
			}else if -0.1>=(mol_h-mol_oh)/mol_oh {14.0+(molf/(v_a+v_b)).log(10.0)		//strong base formula, excess of base.
			}else{7.0																	//Just in case 7.0?
		}
	}else{    //weak base, strong acid (see base generator to see why).
		p_h= if (mol_oh-mol_h)/mol_h>=10.0 {0.5*(7.0+b.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak base formula, big excess of base.
			}else if ((mol_oh-mol_h)/mol_h>=0.1) & ((mol_oh-mol_h)/mol_h<10.0) {b.pka[0].0+(molf/mol_h).log(10.0)  //buffer. small excess of weak base.
			}else if (molf/mol_h)<0.1 {0.5*(b.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak acid formula, roughly complete neutralisation.
			}else if -0.1>=(mol_oh-mol_h)/mol_h {-(molf/(v_a+v_b)).log(10.0)		//strong acid formula, excess of strong acid.
			}else{7.0																	//Just in case 7.0?
		}
	};
		
	
	//Print Question.
	let mut question = Vec::new();
	question.push(format!("{}l roztoku {} obsahuje {}g {}.",dis(v_a),a.name[2],dis(m_a),a.name[2]));
	question.push(format!("{}l roztoku {} obsahuje {}g {}.",dis(v_b),b.name[2],dis(m_b),b.name[2]));
	question.push(format!("Jaké je pH výsledného roztoku jestliže jsou výchozí roztoky smíchány?"));
	let question = question.join("\n");
	
	//Print Answer. (name,acid/acid,x/c)
	let mut ans_a=Vec::new();
	ans_a.push(format!("[{}] = {} mol/l",a.formula[0],c_a));
	ans_a.push(format!("[{}] = {} mol/l",b.formula[0],c_b));
	ans_a.push(format!("Celkový objem = {} ml",(v_a+v_b)*1000.0));
	if a.use_weak==true{
		ans_a.push(format!("Kyselina (slabá): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	}else{
		ans_a.push(format!("Kyselina (silná): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	};
	if b.use_weak==true{
		ans_a.push(format!("Báze (slabá): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	}else{
		ans_a.push(format!("Báze (silná): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	};

	ans_a.push(format!("Látkové množství H+: {}n{}",a_bron.2,a.formula[0]));
	ans_a.push(format!("Látkové množství {}({}-): {}n{}",b_bron.0,b_bron.1,b_bron.2,b.formula[0]));
	
	if b.use_weak==false{ //For strong base and weak or strong acid reaction.
		if (mol_h>mol_oh) & (a.use_weak==false) {
			ans_a.push(format!("Přebytek silné kyseliny: použijte pH = -log[H+]"))
		}else if mol_oh>mol_h {
			ans_a.push(format!("Přebytek silné báze: použijte pH = 14+log({}[{}({}-)]",b_bron.1,b_bron.0,b_bron.1))
		}else if ((mol_h-mol_oh)/mol_oh>=10.0) & (a.use_weak==true) {
			ans_a.push(format!("Velký přebytek slabé kyseliny: použijte pH = 0.5 x (pKa-log({} x [{}]))",a_bron.1,a.formula[0]))
		}else if ((mol_h-mol_oh)/mol_oh<10.0)
			& ((mol_h-mol_oh)/mol_oh>=0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Malý přebytek slabé kyseliny (pufr!): použijte pH = pKa + log([S]/[A])"))
		}else if ((mol_h-mol_oh)/mol_oh<0.1)
			& ((mol_h-mol_oh)/mol_oh>=-0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Téměř úplná neutalizce slabé kyseliny (počítejte jako sůl slabé báze): \
									použijte pH = 7 + 0.5 x (pKa + log({} x [{}])",b_bron.1,b.formula[0]))
		}else{
			ans_a.push(format!("Úplná neutralizace. pH = 7"))
		};
	}else{ //NB assumes that acid is a strong acid (see base generator for why).
		if mol_h>mol_oh {
			ans_a.push(format!("Přebytek silné kyseliny: použijte pH = -log[H+]"))
		}else if (mol_oh-mol_h)/mol_h>=10.0 {
			ans_a.push(format!("Velký přebytek slabé báze: použijte pH = 7.0 + 0.5 x (pKa + log({} x [{}]))",b_bron.1,b.formula[0]))
		}else if ((mol_oh-mol_h)/mol_h<10.0)
			& ((mol_oh-mol_h)/mol_h>=0.1) {
				ans_a.push(format!("Malý přebytek slabé báze (pufr!): použijte pH = pKa + log([S]/[A])"))
		}else if ((mol_oh-mol_h)/mol_h<0.1)
			& ((mol_oh-mol_h)/mol_h>=-0.1) {
				ans_a.push(format!("Téměř úplná neutralizace slabé báze (počítejte jako sůl slabé kyseliny): \
									použijte pH = 0.5 x (pKa + log({} x [{}])",a_bron.1,a.formula[0]))
		}else{
			ans_a.push(format!("Úplná neutralizace. pH = 7"))
		};
	};
		
		
	let ans_b=format!("{}",format!("Odpověď = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)	
}



pub fn q_6_3(compounds:&Vec<Compound>)->(String,String){
//Find degree of ionisation from concentration.
	
	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut useable=true;
		for y in x.solutes.iter(){
			if y.0>1{useable=false}else{useable=useable}
		};
		if (x.solutes.len()==2)
		 & (x.pka[0].0<6.0)
		 & (useable==true)
		 & (x.salt==false)
		 & (x.use_weak==true) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let conc_high:f64=(rand::thread_rng().gen_range(1,670) as f64)/5000.0;
	let conc_low:f64=(rand::thread_rng().gen_range(1,670) as f64)/5000000.0;
	
	let dice=rand::thread_rng().gen_range(0,10);
	let mut conc=if dice<5 {conc_high}else{conc_low};
		
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c.solubility==f64::INFINITY{silly=false; continue
		}else{
			if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
		}
	};	
	
	let ion_exact=(TEN.powf(-c.pka[0].0)*(conc+TEN.powf(-c.pka[0].0)*0.25)).sqrt()-TEN.powf(-c.pka[0].0)*0.5;
	let ion_approx=(conc*TEN.powf(-c.pka[0].0)).sqrt();
	
	let ans_exact=ion_exact/conc*100.0;
	let ans_approx=ion_approx/conc*100.0;
	
	let question = format!("Jaký je stupeň disociace {} (v procentech), s koncentraci {}mol/l:\n\
	(Použijte přibližnou metodu, přesnou metodu nebo obě)",
		c.name[2],
		dis(conc));
	
	let mut ans_a=Vec::new();
	if ans_approx<100.0{
		ans_a.push(format!("Přibližný stupeň disociace: {}%\n",ff(4,ans_approx)))
	}else{
		ans_a.push(format!("Přibližný stupeň disociace: {}%\n(Tato metoda nám dává nesmyslnou odpověď!)\n",ff(4,ans_approx)))
	};
	
	let ans_b=format!("{}\n",format!("Přesný stupeň disociace: {}%\n", ff(4,ans_exact)));
	let answer = format!("{}\n{}",ans_a.join(""),ans_b);
	(question,answer)	
}


pub fn q_6_3b(compounds:&Vec<Compound>)->(String,String){
//Find concentration from degree of ionisation.

	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut useable=true;
		for y in x.solutes.iter(){
			if y.0>1{useable=false}else{useable=useable}
		};
		if (x.solutes.len()==2)
		 & (x.pka[0].0<6.0)
		 & (useable==true)
		 & (x.salt==false)
		 & (x.use_weak==true) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let degree:f64=(rand::thread_rng().gen_range(10,670) as f64)/10.0;
		
	//Check solubility... Can't can I?
	
	//Generate answer.
	let deg=degree*0.01;
	
	let conc_exact=TEN.powf(-c.pka[0].0)*(1.0-deg)/deg/deg;
	let conc_approx=TEN.powf(-c.pka[0].0)/deg;
	
	let exact_prefix= if conc_exact<0.0001 {"μ"}else if conc_exact<0.1 {"m"}else{""};
	let approx_prefix= if conc_approx<0.0001 {"μ"}else if conc_approx<0.1 {"m"}else{""};
	let ans_approx= if conc_approx<0.0001 {
						conc_approx*1000000.0
					}else if conc_approx<0.1 {
						conc_approx*1000.0
					}else{conc_approx
	};	
	let ans_exact= if conc_exact<0.0001 {
						conc_exact*1000000.0
					}else if conc_exact<0.1 {
						conc_exact*1000.0
					}else{conc_exact
	};
	
	//PRINT QUESTION.
	let question = format!("Jestliže je stupeň disociace roztoku {} {}%, jaká je koncentrace tohoto roztoku:\n\
	(Pomocí přibližný metody? Přesnou metodou?)",
	c.name[2],degree);
	
	//PRINT ANSWER.
	let ans_a=format!("Koncentrace pomocí zjednodušené metody: {} {}mol/l",ff(4,ans_approx),approx_prefix);
	let ans_b=format!("{}",format!("Koncentrace pomocí přesné metody: {} {}mol/l", ff(4,ans_exact),exact_prefix));	
	let answer = format!("{}\n\n{}\n",ans_a,ans_b);
	(question,answer)	
}
//Buffers
//Buffers
//Buffers
//Buffers

pub fn q_7_0(compounds:&Vec<Compound>)->(String,String){
//pH as function of concs.
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	
	
	let question = format!("Pufr obsahuje {}M-{} a {}M-{}. Jaké je jeho pH?",
	dis(c_1),
	n_1,
	dis(c_2),
	n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));		
	};
	
	let ans_b=format!("{}",format!("Odpověď = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	


pub fn q_7_0b(compounds:&Vec<Compound>)->(String,String){
//pH as function of compounds' volumes and concentrations.
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let s_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/s_vol*0.1>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//ratios of ions in A and B. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	let a_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*a_vol*nimp_in_a*10.0)<(s_c*s_vol*nimp_in_s) {
			s_c=s_c*0.5;
		}else if (s_c*s_vol*nimp_in_s)<(a_c*a_vol*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	let a_mol=a_c*a_vol;
	let s_mol=s_c*s_vol;
	
	//concentration of active ion in A and B
	let ion_in_a= a_mol*nimp_in_a;
	let ion_in_s= s_mol*nimp_in_s;	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut v_1=a_vol;
	let mut v_2=s_vol;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		v_1=s_vol;
		v_2=a_vol;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	

	let question = format!("{}l roztoku {}M-{} je smícháno s {}l roztoku {}M-{}. Jaké je konečné pH?",
	dis(v_1),
	dis(c_1),
	n_1,
	dis(v_2),
	dis(c_2),
	n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log(nS/nA)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\nLátkové množství soli: {}\nLátkové množství kyseliny: {}",s_mol,a_mol));
	}else{
		ans_a.push(format!("\npH = pKa + log(nB/nS)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\nLátkové množství báze: {}\nLátkové množství soli: {}",a_mol,s_mol));	
	};
	
	let ans_b=format!("{}",format!("Odpověď = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	


pub fn q_7_0c(compounds:&Vec<Compound>)->(String,String){
//pH as function of compounds masses
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let s_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	let mut s_mass:f64=(rand::thread_rng().gen_range(10,670) as f64)/50.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_mass/s_vol*0.1>s.solubility{s_mass=s_mass/10.0}else{silly=false}
		}
	};
	
	//ratios of ions in A and B. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	
	let mut a_mass:f64=(rand::thread_rng().gen_range(10,670) as f64)/50.0;
	let a_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_mass/a.mmass*nimp_in_a*10.0)<(s_mass/s.mmass*nimp_in_s) {
			s_mass=s_mass*0.1;
		}else if (s_mass/s.mmass*nimp_in_s)<(a_mass/a.mmass*0.1) {a_mass=a_mass*0.1
		}else{silly=false}
	};
	let a_mol=a_mass/a.mmass;
	let s_mol=s_mass/s.mmass;
	
	//concentration of active ion in A and B
	let ion_in_a= a_mol*nimp_in_a;
	let ion_in_s= s_mol*nimp_in_s;	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_mass;
	let mut c_2=s_mass;
	let mut v_1=a_vol;
	let mut v_2=s_vol;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_mass;
		c_2=a_mass;
		v_1=s_vol;
		v_2=a_vol;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	
	
		let question = format!("Roztok obsahující {}g {} v {}l je smíchán s {}l roztoku obsahujícího {}g {}. Jaké je konečné pH?",
		dis(c_1),
		n_1,
		dis(v_1),
		dis(v_2),
		dis(c_2),
		n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log(nS/nA)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\nLátkové množství soli: {}\nLátkové množství kyseliny: {}",s_mol,a_mol));
	}else{
		ans_a.push(format!("\npH = pKa + log(nB/nS)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\nLátkové množství báze: {}\nLátkové množství soli: {}",a_mol,s_mol));	
	};
	
	let ans_b=format!("{}",format!("Odpověď = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	
	
	

pub fn q_7_1(compounds:&Vec<Compound>)->(String,String){
//Concs as function of pH.
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	
	
	
	let question = format!("Pufr složený z {} / {} obsahuje {} o koncentraci {}mol/l a má pH {}. Jaká je koncentrace {}?",
		n_1,
		n_2,
		dis(c_1),
		n_1,
		ff(4,p_h),
		n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		}
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		}	
	};

	let ans_b=format!("{}",format!("Odpověď = {}mol/l",dis(c_2)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}


pub fn q_7_1b(compounds:&Vec<Compound>)->(String,String){
//pH as function of compounds' volumes and concentrations. In reverse, get volume.
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let s_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/s_vol*0.1>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//ratios of ions in A and B. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	let a_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*a_vol*nimp_in_a*10.0)<(s_c*s_vol*nimp_in_s) {
			s_c=s_c*0.1;
		}else if (s_c*s_vol*nimp_in_s)<(a_c*a_vol*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	let a_mol=a_c*a_vol;
	let s_mol=s_c*s_vol;
	
	//concentration of active ion in A and B
	let ion_in_a= a_mol*nimp_in_a;
	let ion_in_s= s_mol*nimp_in_s;	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut v_1=a_vol;
	let mut v_2=s_vol;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		v_1=s_vol;
		v_2=a_vol;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	

	let question = format!("{}l roztoku o koncentraci {}M-{} je smíchán s roztokem {}M-{}. Konečné pH je {}. Jaký je objem roztoku {}?",
		dis(v_1),
		dis(c_1),
		n_1,
		dis(c_2),
		n_2,
		ff(4,p_h),
		n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log(nS/nA)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\nLátkové množství kyseliny: {}mol",a_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
			
		}else{
			ans_a.push(format!("\nLátkové množství soli: {}mol",s_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		};	
		ans_a.push(format!("\nV=n/c"));
	}else{
		ans_a.push(format!("\npH = pKa + log(nB/nS)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		if coin_toss<5 {
			ans_a.push(format!("\nLátkové množství báze: {}mol",a_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		}else{
			ans_a.push(format!("\nLátkové množství soli: {}mol",s_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		};	
		ans_a.push(format!("\nV=n/c"));
	};
	
	let ans_b=format!("{}",format!("Odpověď = {}l",dis(v_2)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	


pub fn q_7_2(compounds:&Vec<Compound>)->(String,String){
//Change in buffer pH after addition of strong acid/base.
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//Generate strong acid or base.
	let mut valid_aob:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut a_o_b=false;
		for y in x.solutes.iter(){
			if (y.1=="H")||(y.1=="OH")||(y.1=="HCO3")||(y.1=="CO3"){a_o_b=true}else{};
		};
		if (x.solutes.len()==2)
		 & (a_o_b==true)
		 & (x.use_weak==false)
		 & (x.salt==false)
		 & ((x.solubility>1.0)||(x.solubility==f64::INFINITY))
		 & (x.pka[0].0!=7.0){valid_aob.push(&x)}else{}
	};
	let vaob_len=valid_aob.len();
	let indx=rand::thread_rng().gen_range(0,vaob_len);
	let strong=&valid_aob[indx];
	
	//generate concentration.
	let mut strong_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/250.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if strong.solubility==f64::INFINITY{silly=false; continue
		}else{
			if strong_c*strong.mmass/10.0>strong.solubility{strong_c=strong_c/10.0}else{silly=false}
		}
	};
	
	//generate concentration of b. (nb salt is usually less soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/2.5}else{silly=false}
		}
	};
	
	let v_strong=rand::thread_rng().gen_range(50,671) as f64/1000.0;
	let v_buf=rand::thread_rng().gen_range(100,1341) as f64/1000.0;
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a*v_buf;
	let ion_in_s= s_c*nimp_in_s*v_buf;
	
	
	let n_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	n_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h_start=a.pka[0].0+(n_base/n_acid).log(10.0);
	
	//effective concentration of storng acid or base.
	let mut nimp_in_strong=0.0;
	for y in strong.solutes.iter(){
			if (y.1=="H")||(y.1=="OH")||(y.1=="HCO3")||(y.1=="CO3"){
				nimp_in_strong=(y.0 as f64)*absf64(y.2 as f64)
			}else{}
	};
	
	let n_strong=nimp_in_strong*v_strong*strong_c;
	
	//is the strong stuff an acid or a base?
	let strong_is_acid= if strong.pka[0].0<7.0 {true}else{false};
	
	//final volume.
	let v_fin=v_buf+v_strong;
	
	//Generate answer.
	//let mut n_base_fin=n_base;
	//let mut n_acid_fin=n_acid;
	let mut p_h_fin=p_h_start;
	let mut marker:(String,String,String,char)=("kyselina".to_owned(),
												"jedná se stále o pufr".to_owned(),
												"vypočítejte nové koncentrace a použijte Henderson-Hasselbalchovu rovnici".to_owned(),
												'a');
	
	if (strong_is_acid==true) & ((n_base-n_strong)/(n_acid+n_strong)>0.1){
		marker=("kyselina".to_owned(),
				"jedná se stále o pufr".to_owned(),
				"vypočítejte nové koncentrace a použijte Henderson-Hasselbalchovu rovnici".to_owned(),
				'a');
		p_h_fin=a.pka[0].0+((n_base-n_strong)/(n_acid+n_strong)).log(10.0)    	//acidified buffer.
	}else if (strong_is_acid==true) & ((n_base-n_strong)/(n_acid+n_strong)>=-0.1){
		marker=("kyselina".to_owned(),
				"Došlo k překročení pufrační kapacity, nezbyla žádná silná kyselina".to_owned(),
				"vypočítejte nové koncentrace a použijte rovnici pro výpočet pH slabé kyseliny".to_owned(),
				'b');
		p_h_fin=0.5*(a.pka[0].0-(n_acid/v_fin+n_base/v_fin).log(10.0))			//Weak acid on a Knifedge.
	}else if (strong_is_acid==true) & ((n_strong-n_base)/(n_acid+n_strong)>0.1){
		marker=("kyselina".to_owned(),
				"pufrační kapacita je překročena, přebývá silná kyselina".to_owned(),
				"vypočitejte nové koncentrace a použijte rovnici pro výpočet pH silné kyseliny".to_owned(),
				'c');
		p_h_fin= -((n_strong-n_base)/v_fin).log(10.0)						//Overacidifed. Strong acid.
	}else if (strong_is_acid==false)
		   & (((n_base+n_strong)/(n_acid-n_strong)<10.0) & ((n_base+n_strong)/(n_acid-n_strong)>0.1)){
		marker=("báze".to_owned(),
				"jedná se stále o pufr".to_owned(),
				"vypočítejte nové koncentrace a použijte Henderson-Hasselbalchovu rovnici".to_owned(),
				'A');
		p_h_fin=a.pka[0].0+((n_base+n_strong)/(n_acid-n_strong)).log(10.0)		//alkalinised buffer.
	}else if (strong_is_acid==false)
	       & (((n_acid-n_strong)/(n_base+n_strong)<=0.1) & ((n_acid-n_strong)/(n_base+n_strong)>=-0.1)){
		marker=("báze".to_owned(),
				"pufrační kapacita je překročena, silná báze kompletně je zneutralizovaná".to_owned(),
				"vypočítejte nové koncentrace a použijte rovnici pro slabou zásadu".to_owned(),
				'B');
		p_h_fin=7.0+0.5*(a.pka[0].0+(n_acid/v_fin+n_base/v_fin).log(10.0))		//Weak base on a knifedge.
	}else if (strong_is_acid==false) & ((n_strong-n_acid)/(n_base+n_strong)>0.1){
		marker=("báze".to_owned(),
				"pufrační kapacita je překročena, přebytek silné báze".to_owned(),
				"vypočítejte nové koncentrace a použijte rovnici pro silnou bázi".to_owned(),
				'C');
		p_h_fin= 14.0+((n_strong-n_acid)/v_fin).log(10.0)						//Overalkanisied. Strong acid.
	}else{};
	
	//Prepare to print question.
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	
	
	//PRINT QUESTION.
	let question = format!("Pufr obsahuje {}M-{} a {}M-{}. \
		{}l {}M-{} bylo přidáno do {}l tohoto roztoku. \
		'Jaké je':\n a) Počáteční pH?\n b) Konečné pH? \n c) Změna pH?",
		dis(c_1),
		n_1,
		dis(c_2),
		n_2,
		dis(v_strong),
		dis(strong_c),
		strong.name[2],
		dis(v_buf));;
	
	//PRINT ANSWER.
	let mut ans_a=Vec::new();
	ans_a.push(format!("a) Počáteční pH: Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\n   pH = pKa + log([S]/[A])"));
		ans_a.push(format!("\n   pH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
	}else{
		ans_a.push(format!("\n   pH = pKa + log([B]/[S])"));
		ans_a.push(format!("\n   pH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));		
	};
	
	ans_a.push(format!("\nb) Konečné pH:"));
	ans_a.push(format!("\n   {} je silná {}. Po přídavku {} {}.",strong.name[1],marker.0,strong.name[2],marker.1));
	ans_a.push(format!("\n   Konečné pH {}.",marker.2));
	ans_a.push(format!("\nc) ΔpH = konečné pH -počáteční pH.\n"));
	
	ans_a.push(format!("\n{}",format!("a) Počáteční pH = {}",&ff(4,p_h_start))));
	ans_a.push(format!("\n{}",format!("b) Konečné pH = {}",&ff(4,p_h_fin))));
	let ans_b=format!("{}",format!("c) ΔpH = {}\n",&ff(4,p_h_fin-p_h_start)));
	let answer = format!("{}\n{}\n",ans_a.join(""),ans_b);
	(question,answer)

}


pub fn q_7_3(compounds:&Vec<Compound>)->(String,String){
//Concs as function of pH (total active ion given).
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	let c_ion=c_base+c_acid;
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	
	
	let species="(".to_owned()+&a.formula[0]+"+"+&a.pka[0].1+")";
	let question = format!("Pufr složený z {} / {} má celkovou koncentraci pufrujících kompnent rovnou {}mol/l a má pH {}. Jaká je koncentrace {} a {}?",
		n_1,
		n_2,
		dis(c_ion),
		ff(4,p_h),
		n_2,
		n_1);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\n[{}] = {} x [{}] + {} x [{}]",species,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{}-{} x {}] x 10^(pH-pKa)",nimp_in_s,s.formula[0],species,nimp_in_s,s.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",s.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{}-{} x {}] x 10^(pKa-pH)",nimp_in_a,a.formula[0],species,nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",a.formula[0]));
		}
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\n[{}] = {} x [{}] + {} x [{}]",species,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{}-{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],species,nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",a.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{}-{} x {}] x 10^(pKa-pH)",nimp_in_s,s.formula[0],species,nimp_in_s,s.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",s.formula[0]));
		}	
	};
	ans_a.push(format!("\nPak jděte zpět a nahraďte:\n[{}]={}x[{}] + {}x[{}]",species,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));

	ans_a.push(format!("\n{}",format!("\n [{}] = {}mol/l",n_2,dis(c_2))));
	ans_a.push(format!("{}",format!("\n [{}] = {}mol/l\n\n",n_1,dis(c_1))));

	let answer = ans_a.join("");
	(question,answer)
}


pub fn q_7_3b(compounds:&Vec<Compound>)->(String,String){
//Concs as a function of Osmolarity as function of pH (total active ion given).
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	//calculate osmolarity.
	let mut osma=a_c;
	let mut salt_const=0;
	for x in s.solutes.iter(){
		osma+=s_c*(x.0 as f64);
		salt_const+=x.0};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[2];
	let mut n_2=&s.name[2];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[2];
		n_2=&a.name[2]
	};	
	
	
	let question = format!("Pufr složený z {} / {} má osmolaritu {}osmol/l a pH {}. Jaká je koncentrace {} a {}?",
		n_1,
		n_2,
		dis(osma),
		ff(4,p_h),
		n_2,
		n_1);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson-Hasselbalchova rovnice!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\nOsmolarita = Σ(cs)"));
		ans_a.push(format!("\nOsmolarita = "));
		ans_a.push(format!("{} x {}",nimp_in_a,a.formula[0]));
		for x in s.solutes.iter(){ans_a.push(format!(" + {} x {}",x.0,x.1))};
		ans_a.push(format!("\nOsmolarita = {} x [{}] + {} x [{}]",nimp_in_a,a.formula[0],salt_const,s.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [Osmolarita-{} x {}] x 10^(pH-pKa)",salt_const,s.formula[0],salt_const,s.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",s.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [Osmolarita-{} x {}] x 10^(pKa-pH)",nimp_in_a,a.formula[0],nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",a.formula[0]));
		}
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\nOsmolarita = Σ(cs)"));
		ans_a.push(format!("\nOsmolarita = "));
		ans_a.push(format!("{} x {}",nimp_in_a,a.formula[0]));
		for x in s.solutes.iter(){ans_a.push(format!(" + {} x {}",x.0,x.1))};
		ans_a.push(format!("\nOsmolarita = {} x [{}] + {} x [{}]",nimp_in_a,a.formula[0],salt_const,s.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [Osmolarita-{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",a.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [Osmolarita-{} x {}] x 10^(pKa-pH)",salt_const,s.formula[0],salt_const,s.formula[0]));
			ans_a.push(format!("\nZ této rovnicé vyjádřete a vypočítejte [{}].",s.formula[0]));
		}	
	};
	ans_a.push(format!("\nPak jděte zpět a nahraďte:\n[{}]={}x[{}] + {}x[{}]",a.pka[0].1,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));

	ans_a.push(format!("\n\n{}",format!(" [{}] = {}mol/l",n_2,dis(c_2))));
	ans_a.push(format!("\n{}",format!(" [{}] = {}mol/l\n",n_1,dis(c_1))));

	let answer = ans_a.join("");
	(question,answer)
}							


fn q_s_0(compounds:&Vec<Compound>)->(String,String){
	//work out ionic strength from pH.
	//println!("Question special_0 script started");
	
	//Generate acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];

	let mut acid:bool=true;
	let mut n:usize=0;
	
	//Decide which method to use (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else{}
	};
	
	let n_f64=n as f64;
	let pre_p_h=((rand::thread_rng().gen_range(0,4001)-800) as f64)/1000.0;
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	let p_h;
	if acid==true{
		p_h= if c.use_weak==false {pre_p_h}
			else {0.5*(c.pka[0].0+pre_p_h)}
	}else{
		p_h= if c.use_weak==false {14.0-pre_p_h}
			else {7.0+0.5*(c.pka[0].0-pre_p_h)}
	};
	
	//This increases accuracy of calculation.
	let p_h= match ff(4,p_h).trim().parse(){
		Ok(num)=>num,
		Err(_)=>p_h,
	};
	
	//just in case work out concentration.
	let conc;
	if acid==true{
		conc= if c.use_weak==false {TEN.powf(0.0-p_h)/n_f64}
			else {TEN.powf(c.pka[0].0-2.0*p_h)/n_f64}
	}else{
		conc= if c.use_weak==false {TEN.powf(p_h-14.0)/n_f64}
			else {TEN.powf((p_h-7.0)*2.0-c.pka[0].0)/n_f64}
	};
	
	//work out ionic strength.
	let mut ionic_strength:f64=0.0;
	let mut marker:char=' ';
	if (c.use_weak==true) & (c.salt==false) & (c.pka[0].0<7.0){ //in the case of weak acids it's easy. (Works for monoprotic).
		ionic_strength = TEN.powf(0.0-p_h);		
		marker='a';
	}else if (c.use_weak==true) & (c.salt==false) & (c.pka[0].0>7.0){ //in the case of weak bases its easy.
		ionic_strength = 2.0*TEN.powf(p_h-14.0);
		marker='b';
	}else if (c.use_weak==false) || (c.salt==true){
		//if dealing with the salt of weak acids/bases or with strong /bases, we can usually go with straight ionic strength.
		for x in c.solutes.iter(){
			ionic_strength+= conc*(x.0 as f64)*((abs(x.2)*abs(x.2)) as f64)/2.0;
		};
		marker='c';
	}
	
	let q_phrases=[format!("Roztok {} má pH {}, jaká je jeho iontová síla?",c.name[2],dis_u(p_h)),
				   format!("Vzhledem k tomu, že pH roztoku {} je {}, jaká je jeho iontová síla?",c.name[2],dis_u(p_h)),
				   format!("Jaká je iontová síla roztoku {} s pH {}?",c.name[2],dis_u(p_h)),
				   format!("Vypočítej iontovou sílu roztoku {} o pH {}?",c.name[2],dis_u(p_h))];
	
	let question = q_phrases[rand::thread_rng().gen_range(0,q_phrases.len())].clone();
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("I = Σcq^2. Iontovou sílu ovlivňují pouze ionty. Disociace stanovuje koncentraci iontů a proto pH a pOH"));
	if marker=='a'{
		ans_a.push(format!("\n{} je slabá kyselina. Disociuje částečně na {}x{}({}) a {}x{}({})",
							c.name[1],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
		ans_a.push(format!("\nProto I = [{}]^({}) + [{}]^({})",
		                    c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
		ans_a.push(format!(" = 2x[H]^(1)"));
	}else if marker=='b'{
		ans_a.push(format!("\n {} je slabá báze. Disociuje částečně na {}x{}({}) a {}x{}({})",
							c.name[1],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
		ans_a.push(format!("\nProto I = [{}]^({}) + [{}]^({})",
		                    c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
		ans_a.push(format!(" = 2x[OH]^(1)"));	
	}else{
		if c.use_weak==false{
			ans_a.push(format!("\n {} disociuje kompletně na {}x{}({}) a {}x{}({})",
							c.name[1],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
			ans_a.push(format!("\nProto I = {}x[{}]^({}) + {}x[{}]^({})",
		                    c.solutes[0].0,c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
		}else if c.pka[0].0>7.0{
			ans_a.push(format!("\n {} je sůl, která disociuje plně na {}x{}({}) a {}x{}({})",
							c.name[1],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
			ans_a.push(format!("\nProto I = {}x[{}]^({}) + {}x[{}]^({})",
		                    c.solutes[0].0,c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
			ans_a.push(format!("\n{}(+1) částečně reasociuje zvýšením [H(+1)], ale to neměni iontovou sílu.",c.pka[0].1));
			ans_a.push(format!("\ntakže použij rovnici pro slabou kyselinu to získal koncentraci {}.",c.name[2]));
		}else{
			ans_a.push(format!("\n {} je sůl, která plně disociuje na {}x{}({}) and {}x{}({})",
							c.name[1],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
			ans_a.push(format!("\nProto I = {}x[{}]^({}) + {}x[{}]^({})",
		                    c.solutes[0].0,c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
			ans_a.push(format!("\n{}(-1) částščně reasociuje snížením [H(+1)], ale to neměni iontovou sílu.",c.pka[0].1));
			ans_a.push(format!("\ntakže použij rovnici pro výpočet pH slabé báze, aby jste získali koncentraci {}.",c.name[2]));
		};
	};
	

	ans_a.push(format!("\n\n{}",format!("Iontová síla = {}",dis_u(ionic_strength))));

	let answer = ans_a.join("");
	(question,answer)
}	
#[allow(dead_code)]
fn q_s_1(compounds:&Vec<Compound>)->(String,String){  //Not finished.!!.!!.!!.!! (Need to do explanation.
	//work out ionic strength from pH.
	//println!("Question special_0 script started");
	
	//Generate acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let mut acid:bool=true;
	let mut n:usize=0;
	
	//Decide which method to use (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else{}
	};
	
	let n_f64=n as f64;
	let pre_p_h=((rand::thread_rng().gen_range(0,4001)-800) as f64)/1000.0;
	
	//generate p_h. (Use strong/weak acid/base formula to determine pH) NB
	let p_h; //NB this value will be recalculated.
	if acid==true{
		p_h= if c.use_weak==false {pre_p_h}
			else {0.5*(c.pka[0].0+pre_p_h)}
	}else{
		p_h= if c.use_weak==false {14.0-pre_p_h}
			else {7.0+0.5*(c.pka[0].0-pre_p_h)}
	};
	
	//just in case work out concentration. This value is temporary.
	let conc;
	if acid==true{
		conc= if c.use_weak==false {TEN.powf(0.0-p_h)/n_f64}
			else {TEN.powf(c.pka[0].0-2.0*p_h)/n_f64}
	}else{
		conc= if c.use_weak==false {TEN.powf(p_h-14.0)/n_f64}
			else {TEN.powf((p_h-7.0)*2.0-c.pka[0].0)/n_f64}
	};

	//work out ionic strength.
	let mut ionic_strength=0.0;
	let mut marker:char=' ';
	if (c.use_weak==true) & (c.salt==false) & (c.pka[0].0<7.0){ //in the case of weak acids it's easy. (Works for monoprotic).
		ionic_strength = TEN.powf(0.0-p_h);		
		marker='a';
	}else if (c.use_weak==true) & (c.salt==false) & (c.pka[0].0>7.0){ //in the case of weak bases its easy.
		ionic_strength = 2.0*TEN.powf(p_h-14.0);
		marker='b';
	}else if (c.use_weak==false) || (c.salt==true){
		//if dealing with the salt of weak acids/bases or with strong /bases, we can usually go with straight ionic strength.
		for x in c.solutes.iter(){
			ionic_strength+= conc*(x.0 as f64)*((abs(x.2)*abs(x.2)) as f64)/2.0;
		};
		marker='c';
	};
	
	//work out ionic strength conversion parameter.
	let mut ionic_num:f64=0.0;
	for x in c.solutes.iter(){
			ionic_num+= (x.0 as f64)*((abs(x.2)*abs(x.2)) as f64)/2.0;
		};
	
	let ionic_strength = match ff(4,ionic_strength).trim().parse(){
		Ok(num)=>num,
		Err(_)=>ionic_strength,
	};
	
	let conc = ionic_strength/ionic_num;
	
		
	//Now go back and work out the pH value that will be used in the question
	let p_h;
	if acid==true{
		p_h = 0.0-TEN.log(n_f64*conc);
	}else{
		p_h = 14.0+TEN.log(n_f64*conc);
	};
	
	
	let q_phrases=[format!("Roztok {} má iontovou sílu {}, jaké je jeho pH?",c.name[2],dis_u(ionic_strength)),
				   format!("Vyhledem k tomu, že je iontová síla roztoku {} {}, jaké je jeho pH?",c.name[2],dis_u(ionic_strength)),
				   format!("Jaké je pH roztoku {}, který má iontovou sílu {}?",c.name[2],dis_u(ionic_strength)),
				   format!("Vypočítejte pH roztoku {} o iontové síle {}?",c.name[2],dis_u(ionic_strength))];
	
	let question = q_phrases[rand::thread_rng().gen_range(0,q_phrases.len())].clone();
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("I = Σcq^2. Iontovou sílu ovlivňují pouze ionty. Disociace ovlivňuje koncentraci iontů a proto pH a pOH"));
	if marker=='a'{
		ans_a.push(format!("\n{} je slabá kyselina. Částečně disociuje na {}x{}({}) a {}x{}({})",
							c.name[2],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
		ans_a.push(format!("\nProto I = [{}]^({}) + [{}]^({})",
		                    c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
		ans_a.push(format!(" = 2x[H]^(1)"));
	}else if marker=='b'{
		ans_a.push(format!("\n {} je slabá báze. Částečně disociuje na {}x{}({}) a {}x{}({})",
							c.name[2],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
		ans_a.push(format!("\nProto I = [{}]^({}) + [{}]^({})",
		                    c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
		ans_a.push(format!(" = 2x[OH]^(1)"));	
	}else{
		if c.use_weak==false{
			ans_a.push(format!("\n {} disociuje úplně na {}x{}({}) a {}x{}({})",
							c.name[2],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
			ans_a.push(format!("\nProto I = {}x[{}]^({}) + {}x[{}]^({})",
		                    c.solutes[0].0,c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
		}else if c.pka[0].0>7.0{
			ans_a.push(format!("\n {} je sůl, která plně disociuje na {}x{}({}) a {}x{}({})",
							c.name[2],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
			ans_a.push(format!("\nProto I = {}x[{}]^({}) + {}x[{}]^({})",
		                    c.solutes[0].0,c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
			ans_a.push(format!("\n{}(+1) částečně reasociuji zvýšením [H(+1)], ale to nezmění iontovou sílu.",c.pka[0].1));
			ans_a.push(format!("\nTakže použijte rovnici pro výpočet pH slabé kyseliny, abyste získali koncentraci {}.",c.name[2]));
		}else{
			ans_a.push(format!("\n {} je sůl, která plně disociuje na {}x{}({}) a {}x{}({})",
							c.name[2],c.solutes[0].0,c.solutes[0].1,c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2));
			ans_a.push(format!("\nProto I = {}x[{}]^({}) + {}x[{}]^({})",
		                    c.solutes[0].0,c.solutes[0].1,c.solutes[0].2*c.solutes[0].2,c.solutes[1].0,c.solutes[1].1,c.solutes[1].2*c.solutes[1].2));
			ans_a.push(format!("\n{}(-1) částečně reasociuji snížením [H(+1)], ale to nezmění iontovou sílu.",c.pka[0].1));
			ans_a.push(format!("\nTakže použijte rovnici pro výpočet pH slabé báze, abyste získali koncentraci {}.",c.name[2]));
		};
	};
	

	ans_a.push(format!("\n\n{}",format!("Iontová síla = {}",dis_u(ionic_strength))));

	let answer = ans_a.join("");
	(question,answer)
}					

//function for automatically retrieving help for a question based on compounds present.
pub fn helper(query: &str, library:&Vec<Compound>)-> (String,String) {
	
	//let problem=query.to_owned();
	let mut solution:String = String::new();
	let mut mini_help:String = String::new();
	for x in library.iter(){
		'n: for y in x.name.iter() {
			if query.contains(y){
				solution.push_str(&format!("{}\n\n",form_chem(x)));
				mini_help.push_str(&format!("{}\n",mini_form_chem(x)));
				break 'n;
			};
		};
	};
	let solution = if solution.len()==0 {"-".to_owned()}else{solution.trim().to_owned()};
	let mini_help = if mini_help.len()==0 {"-".to_owned()}else{mini_help.trim().to_owned()};
	(solution,mini_help)
}			


fn v_lit(volume:f64)->String{
	let vol_d:String = dis(volume);
	let out = if volume<=1.0{format!("v {} litru",vol_d)}else{format!("ve {} litrech",vol_d)};
	out
	
}

//alek's shortcutting search function.
fn lshash(a:Chars, b:char)->bool{
	let mut ihaz=false;
	for x in a{
		if x==b{
			ihaz=true;
			return ihaz}
		else{continue}
	}
	ihaz
}


pub fn create_compound_lib(mut a:Vec<Compound>)->Vec<Compound>{
	
	a = vec![
		Compound{
			name:vec!["Lithium Chloride".to_owned(),"chlorid litný".to_owned(),"chloridu litného".to_owned()],
			formula:vec!["LiCl".to_owned()],
			mmass:42.39,
			solutes:vec![(1,"Li".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:74.48,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Sodium Chloride".to_owned(),"chlorid sodný".to_owned(),"chloridu sodného".to_owned()],
			formula:vec!["NaCl".to_owned()],
			mmass:58.44,
			solutes:vec![(1,"Na".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:35.90,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Potassium Chloride".to_owned(),"chlorid draselný".to_owned(),"chloridu draselného".to_owned()],
			formula:vec!["KCl".to_owned()],
			mmass:74.55,
			solutes:vec![(1,"K".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:34.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Lithium Fluoride".to_owned(),"fluorid litný".to_owned(),"fluoridu litného".to_owned()],
			formula:vec!["LiF".to_owned()],
			mmass:25.94,
			solutes:vec![(1,"Li".to_owned(),1),(1,"F".to_owned(),-1)],
			solubility:0.127,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Sodium Fluoride".to_owned(),"fluorid sodný".to_owned(),"fluoridu sodného".to_owned()],
			formula:vec!["NaF".to_owned()],
			mmass:41.99,
			solutes:vec![(1,"Na".to_owned(),1),(1,"F".to_owned(),-1)],
			solubility:4.04,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Potassium Fluoride".to_owned(),"fluorid draselný".to_owned(),"fluoridu draselného".to_owned()],
			formula:vec!["KF".to_owned()],
			mmass:58.10,
			solutes:vec![(1,"K".to_owned(),1),(1,"F".to_owned(),-1)],
			solubility:373.6,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Chloride".to_owned(),"chlorid hořečnatý".to_owned(),"chloridu hořečnatého".to_owned()],
			formula:vec!["MgCl\u{2082}".to_owned(),"MgCl2".to_owned()],
			mmass:95.21,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:54.30,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Calcium Chloride".to_owned(),"chlorid vápenatý".to_owned(),"chloridu vápenatého".to_owned()],
			formula:vec!["CaCl\u{2082}".to_owned(),"CaCl2".to_owned()],
			mmass:111.0,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:74.50,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Bromide".to_owned(),"bromid hořečnatý".to_owned(),"bromidu hořečnatého".to_owned()],
			formula:vec!["MgBr\u{2082}".to_owned(),"MgBr2".to_owned()],
			mmass:184.1,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"Br".to_owned(),-1)],
			solubility:102.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Calcium Bromide".to_owned(),"bromid vápenatý".to_owned(),"bromidu vápenatého".to_owned()],
			formula:vec!["CaBr\u{2082}".to_owned(),"CaBr2".to_owned()],
			mmass:199.9,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"Br".to_owned(),-1)],
			solubility:143.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Iodide".to_owned(),"jodid hořečnatý".to_owned(),"jodidu hořečnatého".to_owned()],
			formula:vec!["MgI\u{2082}".to_owned(),"MgI2".to_owned()],
			mmass:278.1,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"I".to_owned(),-1)],
			solubility:148.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Calcium Iodide".to_owned(),"jodid vápenatý".to_owned(),"jodidu vápenatého".to_owned()],
			formula:vec!["CaI\u{2082}".to_owned(),"CaI2".to_owned()],
			mmass:293.9,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"I".to_owned(),-1)],
			solubility:66.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Iron(II) Chloride".to_owned(),"chlorid železnatý".to_owned(),"chloridu železnatého".to_owned(),"Iron Chloride".to_owned()],
			formula:vec!["FeCl\u{2082}".to_owned(),"FeCl2".to_owned()],
			mmass:126.8,
			solutes:vec![(1,"Fe".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:68.5,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Iron(III) Chloride".to_owned(),"chlorid železitý".to_owned(),"chloridu železitého".to_owned(),"Iron Chloride".to_owned()],
			formula:vec!["FeCl\u{2083}".to_owned(),"FeCl3".to_owned()],
			mmass:162.2,
			solutes:vec![(1,"Fe".to_owned(),3),(3,"Cl".to_owned(),-1)],
			solubility:91.2,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Copper(II) Chloride".to_owned(),"chlorid měďnatý".to_owned(),"chloridu měďnatého".to_owned(),"Copper Chloride".to_owned()],
			formula:vec!["CuCl\u{2082}".to_owned(),"CuCl2".to_owned()],
			mmass:134.5,
			solutes:vec![(1,"Cu".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:75.7,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Copper(I) Chloride".to_owned(),"chlorid měďný".to_owned(),"chloridu měďného".to_owned(),"Copper Chloride".to_owned()],
			formula:vec!["CuCl".to_owned()],
			mmass:99.00,
			solutes:vec![(1,"Cu".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:0.047,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Zinc Chloride".to_owned(),"chlorid zinečnatý".to_owned(),"chloridu zinečnatého".to_owned(),"Zinc(II) Chloride".to_owned()],
			formula:vec!["ZnCl\u{2082}".to_owned(),"ZnCl2".to_owned()],
			mmass:134.5,
			solutes:vec![(1,"Zn".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:395.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Aluminium Chloride".to_owned(),"chlorid hlinitý".to_owned(),"chloridu hlinitého".to_owned()],
			formula:vec!["AlCl\u{2083}".to_owned(),"AlCl3".to_owned()],
			mmass:133.3,
			solutes:vec![(1,"Al".to_owned(),3),(3,"Cl".to_owned(),-1)],
			solubility: 45.8,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Potassium Sulphate".to_owned(),"síran draselný".to_owned(),"síranu draselného".to_owned()],
			formula:vec!["K\u{2082}SO\u{2084}".to_owned(),"K2SO4".to_owned()],
			mmass:174.3,
			solutes:vec![(2,"K".to_owned(),1),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 11.1,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Sulphate".to_owned(),"síran hořečnatý".to_owned(),"síranu hořečnatého".to_owned()],
			formula:vec!["MgSO\u{2084}".to_owned(),"MgSO4".to_owned()],
			mmass:120.4,
			solutes:vec![(1,"Mg".to_owned(),2),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 35.1,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Calcium Sulphate".to_owned(),"síran vápenatý".to_owned(),"síranu vápenatého".to_owned()],
			formula:vec!["CaSO\u{2084}".to_owned(),"CaSO4".to_owned()],
			mmass:136.1,
			solutes:vec![(1,"Ca".to_owned(),2),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 0.210,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Copper(II) Sulphate".to_owned(),"síran měďnatý".to_owned(),"síranu měďnatého".to_owned(),"Copper Sulphate".to_owned()],
			formula:vec!["CuSO\u{2084}".to_owned(),"CuSO4".to_owned()],
			mmass:159.6,
			solutes:vec![(1,"Cu".to_owned(),2),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 20.3,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Aluminium Sulphate".to_owned(),"síran hlinitý".to_owned(),"síranu hlinitého".to_owned()],
			formula:vec!["Al2(SO\u{2084})\u{2083}".to_owned(),"Al2(SO4)3".to_owned()],
			mmass:342.2,
			solutes:vec![(2,"Al".to_owned(),3),(3,"SO\u{2084}".to_owned(),-2)],
			solubility: 36.4,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Trisodium Phosphate".to_owned(),"fosforečnan sodný".to_owned(),"fosforečnanu sodného".to_owned()],
			formula:vec!["Na\u{2083}PO\u{2084}".to_owned(),"Na3PO4".to_owned()],
			mmass:163.9,
			solutes:vec![(3,"Na".to_owned(),1),(1,"PO\u{2084}".to_owned(),-3)],
			solubility: 12.0,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Iron(III) Phosphate".to_owned(),"fosforečnan železitý".to_owned(),"fosforečnanu železitého".to_owned(),"Iron Phosphate".to_owned()],
			formula:vec!["FePO\u{2084}".to_owned(),"FePO4".to_owned()],
			mmass:150.8,
			solutes:vec![(1,"Fe".to_owned(),3),(1,"PO\u{2084}".to_owned(),-3)],
			solubility: 0.642,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Phosphate".to_owned(),"fosforečnan hořečnatý".to_owned(),"fosforečnanu hořečnatého".to_owned()],
			formula:vec!["Mg\u{2083}(PO\u{2084})\u{2082}".to_owned(),"Mg3(PO4)2".to_owned()],
			mmass:262.9,
			solutes:vec![(3,"Mg".to_owned(),2),(2,"PO\u{2084}".to_owned(),-3)],
			solubility:2.59*TEN.powi(-4),
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		
		Compound{
			name:vec!["Sodium Hydroxide".to_owned(),"hydroxid sodný".to_owned(),"hydroxidu sodného".to_owned()],
			formula:vec!["NaOH".to_owned()],
			mmass:40.0,
			solutes:vec![(1,"Na".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: 111.0,
			pka:vec![(14.93,"Na".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		
		Compound{
			name:vec!["Potassium Hydroxide".to_owned(),"hydroxid draselný".to_owned(),"hydroxidu draselného".to_owned()],
			formula:vec!["KOH".to_owned()],
			mmass:56.11,
			solutes:vec![(1,"K".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: 121.0,
			pka:vec![(14.93,"K".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Hydroxide".to_owned(),"hydroxid hořečnatý".to_owned(),"hydroxidu hořečnatého".to_owned()],
			formula:vec!["Mg(OH)\u{2082}".to_owned(),"Mg(OH)2".to_owned()],
			mmass:58.32,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"OH".to_owned(),-1)],
			solubility:0.00064,
			pka:vec![(14.0,"Mg".to_owned())], //Not strictly true.
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Calcium Hydroxide".to_owned(),"hydroxid vápenatý".to_owned(),"hydroxidu vápenatého".to_owned()],
			formula:vec!["Ca(OH)\u{2082}".to_owned(),"Ca(OH)2".to_owned()],
			mmass:74.09,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"OH".to_owned(),-1)],
			solubility:0.173,
			pka:vec![(12.63,"Ca".to_owned())], //NB (12.63,11.57)
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Hydrochloric acid".to_owned(),"kyselina chlorovodíková".to_owned(),"kyseliny chlorovodíkové".to_owned()],
			formula:vec!["HCl".to_owned()],
			mmass:36.46,
			solutes:vec![(1,"H".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(-6.3,"Cl".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Sulphuric Acid".to_owned(),"kyselina sírová".to_owned(),"kyseliny sírové".to_owned()],
			formula:vec!["H\u{2082}SO\u{2084}".to_owned(),"H2SO4".to_owned()],
			mmass:98.08,
			solutes:vec![(2,"H".to_owned(),1),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: f64::INFINITY,
			pka:vec![(-3.0,"SO\u{2084}".to_owned())], //Not strictly true.(-3.0 and 2)
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Pyruvic Acid".to_owned(),"kyselina pyrohroznová".to_owned(),"kyseliny pyrohroznové".to_owned()],
			formula:vec!["CH\u{2083}COCOOH".to_owned(),"CH3COCOOH".to_owned()],
			mmass:88.06,
			solutes:vec![(1,"H".to_owned(),1),(1,"CH\u{2083}COCOO".to_owned(),-1)],
			solubility: 100.0,
			pka:vec![(2.5,"CH\u{2083}COCOO".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Sodium Pyruvate".to_owned(),"pyruvát sodný".to_owned(),"pyruvátu sodného".to_owned()],
			formula:vec!["NaCH\u{2083}COCOO".to_owned(),"NaCH3COCOO".to_owned()],
			mmass:110.04,
			solutes:vec![(1,"Na".to_owned(),1),(1,"CH\u{2083}COCOO".to_owned(),-1)],
			solubility: 10.0,
			pka:vec![(2.5,"CH\u{2083}COCOO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Formic Acid".to_owned(),"kyselina mravenčí".to_owned(),"kyseliny mravenčí".to_owned()],
			formula:vec!["HCOOH".to_owned(),"CHOOH".to_owned()],
			mmass:46.03,
			solutes:vec![(1,"H".to_owned(),1),(1,"HCOO".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(3.77,"HCOO".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Potassium Formate".to_owned(),"mravenčan sodný".to_owned(),"mravenčanu sodného".to_owned()],
			formula:vec!["KHCOO".to_owned(),"KCHOO".to_owned()],
			mmass:84.12,
			solutes:vec![(1,"K".to_owned(),1),(1,"HCOO".to_owned(),-1)],
			solubility: 335.0,
			pka:vec![(3.77,"HCOO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Calcium Formate".to_owned(),"mravenčan vápenatý".to_owned(),"mravenčanu vápenatého".to_owned()],
			formula:vec!["Ca(HCOO)\u{2082}".to_owned(),"Ca(HCOO)2".to_owned(),"Ca(CHOO)2".to_owned()],
			mmass:130.1,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"HCOO".to_owned(),-1)],
			solubility: 16.6,
			pka:vec![(3.77,"HCOO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Acetic Acid".to_owned(),"kyselina octová".to_owned(),"kyseliny octové".to_owned()],
			formula:vec!["CH\u{2083}COOH".to_owned(),"CH3COOH".to_owned()],
			mmass:60.05,
			solutes:vec![(1,"H".to_owned(),1),(1,"CH\u{2083}COO".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Sodium Acetate".to_owned(),"acetát sodný".to_owned(),"acetátu sodného".to_owned(),"Sodium Ethanoate".to_owned()],
			formula:vec!["NaCH\u{2083}COO".to_owned(),"NaCH3COO".to_owned()],
			mmass:82.03,
			solutes:vec![(1,"Na".to_owned(),1),(1,"CH\u{2083}COO".to_owned(),-1)],
			solubility: 123.3,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Acetate".to_owned(),"acetát hořečnatý".to_owned(),"acetátu hořečnatého".to_owned(),"Magnesium Ethanoate".to_owned()],
			formula:vec!["Mg(CH\u{2083}COO)\u{2082}".to_owned(),"Mg(CH3COO)2".to_owned()],
			mmass:142.4,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"CH\u{2083}COO".to_owned(),-1)],
			solubility: 65.6,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Aluminium Acetate".to_owned(),"acetát hlinitý".to_owned(),"acetátu hlinitého".to_owned(),"Aluminium Ethanoate".to_owned()],
			formula:vec!["Al(CH\u{2083}COO)\u{2083}".to_owned(),"Al(CH3COO)3".to_owned()],
			mmass:204.1,
			solutes:vec![(1,"Al".to_owned(),3),(3,"CH\u{2083}COO".to_owned(),-1)],
			solubility: 14.8,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Salicylic Acid".to_owned(),"kyselina salicylová".to_owned(),"kyseliny salicylové".to_owned()],
			formula:vec!["C\u{2087}H\u{2086}O\u{2083}".to_owned(),"C7H6O3".to_owned()],
			mmass:138.1,
			solutes:vec![(1,"H".to_owned(),1),(1,"C\u{2087}H\u{2085}O\u{2083}".to_owned(),-1)],
			solubility: 0.248,
			pka:vec![(2.97,"C\u{2087}H\u{2085}O\u{2083}".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Sodium Salicylate".to_owned(),"salycilát sodný".to_owned(),"salicylátu sodného".to_owned()],
			formula:vec!["NaC\u{2087}H\u{2085}O\u{2083}".to_owned(),"NaC7H5O3".to_owned()],
			mmass:160.1,
			solutes:vec![(1,"Na".to_owned(),1),(1,"C\u{2087}H\u{2085}O\u{2083}".to_owned(),-1)],
			solubility: 124.6,
			pka:vec![(2.97,"C\u{2087}H\u{2085}O\u{2083}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Salicylate".to_owned(),"salicylát hořečnatý".to_owned(),"salicylátu hořečnatého".to_owned()],
			formula:vec!["Mg(C\u{2087}H\u{2085}O\u{2083})\u{2082}".to_owned(),"Mg(C7H5O3)2".to_owned()],
			mmass:298.5,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"C\u{2087}H\u{2085}O\u{2083}".to_owned(),-1)],
			solubility: 0.00686,  
			pka:vec![(2.97,"C\u{2087}H\u{2085}O\u{2083}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Ammonia".to_owned(),"amoniak".to_owned(),"amoniaku".to_owned()],
			formula:vec!["NH\u{2083}".to_owned(),"NH3".to_owned()],
			mmass:17.03,
			solutes:vec![(1,"NH\u{2084}".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Diethylammonium Chloride".to_owned(),"diethylamonium chlorid".to_owned(),"diethylamonium chloridu".to_owned()],
			formula:vec!["(CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}Cl".to_owned(),"(CH3CH2)2NH2Cl".to_owned()],
			mmass:109.6,
			solutes:vec![(1,"CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility: 51.0,
			pka:vec![(10.8,"CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Diethylamine".to_owned(),"diethylamin".to_owned(),"diethylaminu".to_owned()],
			formula:vec!["CH\u{2083}CH\u{2082})\u{2082}NH".to_owned(),"(CH3CH2)2NH".to_owned()],
			mmass:73.14,
			solutes:vec![(1,"CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(10.8,"CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Ammonium Chloride".to_owned(),"chlorid amonný".to_owned(),"chloridu amonného".to_owned()],
			formula:vec!["NH\u{2084}Cl".to_owned(),"NH4Cl".to_owned()],
			mmass:53.49,
			solutes:vec![(1,"NH\u{2084}".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility: 39.5,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Ammonium Sulphate".to_owned(),"síran amonný".to_owned(),"síranu amonného".to_owned()],
			formula:vec!["(NH\u{2084})\u{2082}SO\u{2084}".to_owned(),"(NH4)2SO4".to_owned()],
			mmass:132.1,
			solutes:vec![(2,"NH\u{2084}".to_owned(),1),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 70.6,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Ammonium Phosphate".to_owned(),"fosforečnan amonný".to_owned(),"fosforečnanu amonného".to_owned()],
			formula:vec!["(NH\u{2084})3PO\u{2084}".to_owned(),"(NH4)3PO4".to_owned()],
			mmass:149.0,
			solutes:vec![(3,"NH\u{2084}".to_owned(),1),(1,"PO\u{2084}".to_owned(),-3)],
			solubility: 58.0,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Glucose".to_owned(),"glukóza".to_owned(),"glukózy".to_owned()],
			formula:vec!["C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),"C6H12O6".to_owned()],
			mmass:180.2,
			solutes:vec![(1,"C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),0)],
			solubility: 90.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Fructose".to_owned(),"fruktóza".to_owned(),"fruktózy".to_owned()],
			formula:vec!["C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),"C6H12O6".to_owned()],
			mmass:180.2,
			solutes:vec![(1,"C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),0)],
			solubility: f64::INFINITY,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Sucrose".to_owned(),"sacharóza".to_owned(),"sacharózy".to_owned()],
			formula:vec!["C\u{2081}\u{2081}H\u{2082}\u{2082}O\u{2081}\u{2081}".to_owned(),"C11H22O11".to_owned()],
			mmass:342.3,
			solutes:vec![(1,"C\u{2081}\u{2081}H\u{2082}\u{2082}O\u{2081}\u{2081}".to_owned(),0)],
			solubility: 200.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		}
	];
		
	//Make extra compounds from config files and add to main library.							
	let mut extra_compounds = parse_compound_json();
	a.append(&mut extra_compounds);
	a
}

#[allow(unused)]
const AUTHORS:&'static str="This library was written by Aleksey Zholobenko & Translated by Zdenek Dostal";
