///The central module of libchmq.


// compiles with :
//	cargo rustc --release --features="libc" --target=arm-linux-androideabi -- -C linker="/home/alesha/NDK/arm/bin/arm-linux-androideabi-clang" --crate-type="cdylib"
//	cargo rustc --release --features="libc" --target=x86_64-linux-android  -- -C linker="/home/alesha/NDK/x86_64/bin/x86_64-linux-android-clang" --crate-type="cdylib"
//	cargo rustc --release --features="libc" --target=aarch64-linux-android -- -C linker="/home/alesha/NDK/arm64/bin/aarch64-linux-android-clang" --crate-type="cdylib"



#[cfg(target_os = "android")]extern crate libc;
#[cfg(target_os = "android")]extern crate jni;
#[macro_use]extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;

pub mod enq;
pub mod czq;

pub mod anden;
pub mod andcz;


#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
#[allow(dead_code)]

use std::string::String;
use std::vec::Vec;
use std::f64;
use std::str::Chars;
use std::fs;
use std::io::Read;
use rand::Rng;



const JSON:&str = "compounds_extra.json";
pub const EN:u8 = 0;
pub const CZ:u8 = 1;
pub const GAS:u8 = 255;
pub const LIQ:u8 = 254;
pub const SOL:u8 = 253;
pub const AQU:u8 = 252;
pub const HTML:u8 = 251;
pub const SYMBOL:u8 = 250;


#[derive(Debug)]
pub struct Compound{
	pub name:Vec<String>,						//compound name.
	pub formula:Vec<String>,					//formula eg MgCl2
	pub mmass:f64, 								//grams per mol.
	pub solutes:Vec<(u8,String,i8)>,			//eg (1,Mg,+2),(2,Cl,-1)
	pub solubility:f64,							//grams per 100 ml.
	pub pka:Vec<(f64,String)>,						//pKa
	pub use_weak:bool,							//Use questions in strong/weak acids and ksp?
	pub salt:bool,								//is it a salt?
	pub med:(bool,f64,f64,String,Option<f64>), // 5 fields (used medically?,min dose,max dose,dose unit, unit conversion if not g/L)
}

#[derive(Debug,Clone)]
pub enum Equilibrium {
	Keq(f64),
	DeltaH(f64),
}

//Structure to hold a reaction.
//Needs to have a mechanism to check for correctness.
#[allow(dead_code)]
//NB: (compound name, multiple, stateconstant)
#[derive(Debug,Clone)]
pub struct FullReaction<'a> {
	pub reagents: Vec<(&'a Compound,u8,u8)>,
	pub products: Vec<(&'a Compound,u8,u8)>,
	pub eq: Equilibrium, //enthalpy or Keq
}

//Structure to hold a reaction.
//Needs to have a mechanism to check for correctness.
//NB: (compound name, multiple, stateconstant)
#[derive(Debug,Clone)]
pub struct Reaction {
	pub reagents: Vec<(String,u8,u8)>,
	pub products: Vec<(String,u8,u8)>,
	pub eq: Equilibrium, //enthalpy or Keq
}

//Functions for working with reactions.
//NB I have decided against implementing the usual display,
//As there are several ways to display them that I would like to use.
impl Reaction {
	

	//Draw a basic reaction in a basic manner.
	pub fn draw(&self)->String {
		let mut output = String::with_capacity(500);
		
		//print reagents.
		let lr = self.reagents.len();
		if lr > 0 {
			for i in 0..lr {	
				let maybe_plus = if i==0 {""}else{" + "};
				output.push_str(&format!("{}{}{}",maybe_plus,rem_one(self.reagents[i].1),self.reagents[i].0));
			};
		};
		
		output.push_str(" ⇌ ");
		
		let lp = self.products.len();
		if lp > 0 {
			for i in 0..lp {	
				let maybe_plus = if i==0 {""}else{" + "};
				output.push_str(&format!("{}{}{}",maybe_plus,rem_one(self.products[i].1),self.products[i].0));
			};
		};
		
		output
	}
	
	//Draw a basic reaction with state symbols.
	pub fn draw_with_state(&self)->String {
		let mut output = String::with_capacity(500);
		
		//print reagents.
		let lr = self.reagents.len();
		if lr > 0 {
			for i in 0..lr {	
				let maybe_plus = if i==0 {""}else{" + "};
				output.push_str(
					&format!(
						"{}{}{}{}",
						maybe_plus,
						rem_one(self.reagents[i].1),
						self.reagents[i].0,
						state_const_match(self.reagents[i].2)
					)
				);
			};
		};
		
		output.push_str(" ⇌ ");
		
		let lp = self.products.len();
		if lp > 0 {
			for i in 0..lp {	
				let maybe_plus = if i==0 {""}else{" + "};
				output.push_str(
					&format!(
						"{}{}{}{}",
						maybe_plus,
						rem_one(self.products[i].1),
						self.products[i].0,
						state_const_match(self.products[i].2)
					)
				);
			};
		};
		
		output	
	}
	
	//Draw a basic reaction with enthalpy.
	pub fn draw_with_heat(&self)->String {
		let mut output = self.draw();
		
		match self.eq {
			Equilibrium::DeltaH(x) => {output.push_str(&format!("  (ΔH = {}kJ/mol)",x));},
			Equilibrium::Keq(x) => {output.push_str(&format!("  (Keq = {})",x));},
		};
		
		output
	}
	
	//Draw a basic reaction with enthalpy and state symbols.
	pub fn draw_with_hs(&self)->String {
		let mut output = self.draw_with_state();
		
		match self.eq {
			Equilibrium::DeltaH(x) => {output.push_str(&format!("  (ΔH = {}kJ/mol)",x));},
			Equilibrium::Keq(x) => {output.push_str(&format!("  (Keq = {})",x));},
		};
		
		output
	}
	
	
	//Draw an equilibrium equation for a reaction.
	pub fn draw_eq_equation(&self)->String {
		let mut output = String::with_capacity(1000);
		
		output.push_str("Keq = (");
		for x in self.products.iter() { output.push_str(&format!("[{}]^({})",x.0,x.1)); };
		output.push_str(") / (");
		for x in self.reagents.iter() { output.push_str(&format!("[{}]^({})",x.0,x.1)); };
		output.push(')');
		output
	}
	//Draw an equilibrium equation for a reaction.
	pub fn draw_eq_equation_activity(&self)->String {
		let mut output = String::with_capacity(1000);
		
		output.push_str("Keq = (");
		for x in self.products.iter() {
			if x.2!=SOL {
				output.push_str(&format!("[{}]^({})",x.0,x.1));
			};
		};
		output.push_str(") / (");
		for x in self.reagents.iter() {
			if x.2!=SOL {
				output.push_str(&format!("[{}]^({})",x.0,x.1));
			};
		};
		output.push(')');
		output
	}
}

//A trype that exists specifically to convert a JSON imported compound
//to an actual compound.
#[derive(Serialize,Deserialize,Debug)]
struct CompoundJson {
	pub name:Vec<String>,							
	pub formula:Vec<String>,					
	pub mmass:f64, 									
	pub solutes_num:Vec<u8>, 									
	pub solutes_names:Vec<String>, 									
	pub solutes_ch:Vec<i8>,				
	pub solubility:f64,								
	pub pka_values:Vec<f64>,
	pub pka_ions:Vec<String>,						
	pub use_weak:bool,								
	pub salt:bool,									
	pub med_true:bool,
	pub med_min:f64,
	pub med_max:f64,
	pub med_unit:String,
	pub med_conversion:f64,    
}

impl CompoundJson {
	fn to_compound(mut self)->Compound {
		let mut solutes:Vec<(u8,String,i8)> = Vec::new();
		let mut pkas:Vec<(f64,String)> = Vec::new();
		let med_conversion = if self.med_conversion==0.0 {
			None
		}else{
			Some(self.med_conversion)
		};
		
		let meds:(bool,f64,f64,String,Option<f64>) = (self.med_true,
													  self.med_min,
													  self.med_max,
													  self.med_unit,
													  med_conversion);
		
		for (num,(mut name,ch)) in self.solutes_num.iter().zip(
			self.solutes_names.iter_mut().zip(
				self.solutes_ch.iter()
			)
		){
			*name = subscriptise(name);
			solutes.push((*num,name.clone(),*ch))
		};
		
		for (value,ion) in self.pka_values.iter().zip(self.pka_ions.iter_mut()) {
			*ion = subscriptise(ion);
			pkas.push((*value,ion.to_owned()));
		}
		
		for formula in self.formula.iter_mut() {*formula = subscriptise(formula)};
		
		Compound {
			name:self.name,
			formula:self.formula,
			mmass:self.mmass,
			solutes:solutes,
			solubility:self.solubility,
			pka:pkas,
			use_weak:self.use_weak,
			salt:self.salt,
			med:meds,
		}	
	}
}

//Function to generate question based on topic.
//Attempt at a very generic fucntion indeed.
#[cfg(not(target_os = "android"))]
pub fn generate_questions(lib:&Vec<Compound>,questions:Vec<&Fn(&Vec<Compound>)->(String,String)>,lang:u8,mode:u8)->(String,String,String,String) {
	
	let r_ind = rand::thread_rng().gen_range(0,questions.len());
	let (q,a):(String,String) = if mode==SYMBOL {questions[r_ind](lib).sscri(lang)}else{questions[r_ind](lib).sscri_html(lang)};
	
	let (h,mh):(String,String) = match lang {
		EN => {if mode==SYMBOL {enq::helper(&q,lib)}else{enq::helper(&q,lib).sscri_html(lang)}},
		CZ => {if mode==SYMBOL {czq::helper(&q,lib)}else{czq::helper(&q,lib).sscri_html(lang)}},
		_  => (String::new(),String::new()),
	};
	
	(q,a,h,mh)
}

//Function to generate question for equilibrium (REACTION).
//Attempt at a very generic fucntion indeed.
#[cfg(not(target_os = "android"))]
pub fn generate_r_questions(lib:&Vec<Reaction>,questions:Vec<&Fn(&Vec<Reaction>)->(String,String)>,lang:u8,mode:u8)->(String,String,String,String) {
	
	let r_ind = rand::thread_rng().gen_range(0,questions.len());
	let (q,a):(String,String) = if mode==SYMBOL {questions[r_ind](lib).sscri(lang)}else{questions[r_ind](lib).sscri_html(lang)};
	
	let (h,mh):(String,String) = (String::new(),String::new());
	
	(q,a,h,mh)
}
 
//function to convert state symbol to text.
fn state_const_match(con:u8)->String {
	match con {
		GAS => "(g)".to_owned(),
		LIQ => "(l)".to_owned(),
		SOL => "(s)".to_owned(),
		AQU => "(aq)".to_owned(),
		_  => String::new(),
	}
}

//get rid of excessive ones.
fn rem_one(num:u8)->String {
	match num {
		1 => "".to_owned(),
		_ => num.to_string().to_owned(),
	}
}

//4 Sig. fig. function. NB: Rounding is still shaky.
pub fn ff(figs:usize,old:f64)->String{
	//put the original number as a string.
	let old_as_string = old.to_string();
	
	let sign = if old_as_string.chars().nth(0)==Some('-') {'-'}else{'+'};
	
	//make string of insignificant figures.
	let insigs = "0,.+-";
	
	//make a receptacle string for the new number.
	let mut new = String::new();
	//make a signifacant figure counter.
	let mut sig_figs:usize = 0;
	//make an indicaor for whether it's started or not.
	let mut count = false;
	//do the work.
	let mut has_dot = false;
	for x in old_as_string.chars(){
		if !lshash(insigs.chars(),x) {count=true;};
		if (x=='.') | (x==',') {has_dot=true;};
		if count & (x!='.') & (x!=',') {sig_figs+= 1;};
		if (x=='-')||(x=='+'){
		}else if !has_dot {
			new.push(x)
		}else if sig_figs<=(figs+1){
			new.push(x)
		};
	};

	let mut out = String::new();
	let mut lenn = new.chars().count();
	
	//retrieve last character.
	let l = new.chars().nth(lenn-1).unwrap();
	//decide whether to round or return as is.
	let mut round = if sig_figs<=4 {
		if sign=='-' {out.push(sign);};
		out.push_str(&new);
		return out
	}else if (sig_figs>4) & ((l=='5')|(l=='6')|(l=='7')|(l=='8')|(l=='9')) {true}else{false};
	let ln = new.chars().nth(figs);
	round = if !has_dot & ((ln==Some('5'))|(ln==Some('6'))|(ln==Some('7'))|(ln==Some('8'))|(ln==Some('9'))){
		true
	}else if !has_dot {
		false
	}else{
		round
	};
	
	if has_dot & (sig_figs>4) {
		new.pop();
		lenn-= 1;
	};
	
	//construct a final output.
	let mut counter:usize = 0;
	for x in new.chars().rev(){
		counter+= 1;
		if !has_dot & (lenn-counter>figs-1){
			out.push('0')
		}else{
			if !round {
				out.push(x)
			}else{
				let to_do = match_to_round(round,x);
				round = to_do.0;
				if round & (counter==(lenn)){
					out.push(to_do.1);
					out.push('1')
				}else{
					out.push(to_do.1)
				};
			};
		};
	};
	if sign=='-' {out.push(sign);};		

	//Bleh- can't find function.
	if out.chars().rev().last()==Some('.') {
		let mut a:String = out.chars().rev().collect();
		a.pop();
		a
	}else{
		out.chars().rev().collect()
	}
}

//decimal rounding table.
pub fn match_to_round(round:bool,ch:char)->(bool,char){
	if !round {
		return (false,ch)
	}else{
		match ch {
			'0' => (false,'1'),
			'1' => (false,'2'),
			'2' => (false,'3'),
			'3' => (false,'4'),
			'4' => (false,'5'),
			'5' => (false,'6'),
			'6' => (false,'7'),
			'7' => (false,'8'),
			'8' => (false,'9'),
			'9' => (true,'0'),
			_   => (true,ch),
		}
	}
}

pub fn abs(x:i8)->i8{
	let out= if x>=0{x}else{-1*x};
	out
}
pub fn absf64(x:f64)->f64{
	let out= if x>=0.0{x}else{-x};
	out
}

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

//formats value to display by converting to milli, micro, nano, etc.
pub fn dis(value:f64)->String{
	//println!("dis has recieved {};",value);
	if absf64(value)==0.0{
		format!("{}",value)
	}else if absf64(value)>=0.1{
		let pre_out=value;
		format!("{} ",ff(4,pre_out))
	}else if (absf64(value)<0.1) & (absf64(value)>=0.001){
		let pre_out=value*1000.0;
		format!("{} m",ff(4,pre_out))
	}else if (absf64(value)<0.001) & (absf64(value)>=0.000001){
		let pre_out=value*1000000.0;
		format!("{} μ",ff(4,pre_out))
	}else if (absf64(value)<0.0000001) & (absf64(value)>=0.000000001){
		let pre_out=value*1000000000.0;
		format!("{} n",ff(4,pre_out))
	}else{
		format!("{} ",dis_u(value))
	}
}

//formats unitless quantities as N x 10^(z)
pub fn dis_u(value:f64)->String{
	//println!("dis_u has received {}",value);
	if absf64(value)==0.0{
		return
		format!("{}",value)
	}else if (absf64(value)<=1000.0) & (absf64(value)>=0.001) {
		return
		format!("{}",ff(4,value))};
	let mut val=value;
	let mut power:i32=0;
	if absf64(val)>=1.0{
		while absf64(val)>=10.0{
			val=val/10.0;
			power+=1
		}
	}else{
		while absf64(val)<=1.0{
			val=val*10.0;
			power-=1	
		}
	}
	format!("{} x 10^({})",ff(4,val),power)	
	
}

//puts x as xth root.
pub fn form_root(p:String)->String {
	match p.trim().parse(){
		Ok(1)=>{return "".to_owned();},
		Ok(2)=>{return "√".to_owned();},
		 _ =>  {;},
	};
	
	let mut root = String::new();
	for x in p.chars(){root.push(upchar(x).0)};
	root.push('√');
	root
}

pub const TEN:f64=10.0;
pub const R:f64=8.314;
pub const AB_Z:f64=-273.15;


//UTF8 APOCALYPSE STORAGE SPACE
//UTF8 APOCALYPSE STORAGE SPACE
//UTF8 APOCALYPSE STORAGE SPACE..now elsewhere.
pub trait Sscri {
	fn sscri(self,lang:u8)->(String,String);
	fn sscri_html(self,lang:u8)->(String,String);
	fn sscri_android(self,lang:u8)->(String,String);
}

impl Sscri for (String,String) {	
	//Scientific script from ordinary script using rare unicode glyphs.
	fn sscri(self,lang:u8)->Self{		
		(sscri_par(self.0,lang),sscri_par(self.1,lang))
	}
	
	//Scientific script from ordinary script using html sub and superscript.
	fn sscri_html(self,lang:u8)->Self {
		(sscri_par_html(self.0,lang,false),sscri_par_html(self.1,lang,false))
	}
	
	//Scientific script from ordinary script using html sub and superscript.
	//Android version which uses <sup><small></small></sup>
	fn sscri_android(self,lang:u8)->Self {
		(sscri_par_html(self.0,lang,true),sscri_par_html(self.1,lang,true))
	}
}

//Inner function of sscri script.
pub fn sscri_par(a: String,lang:u8)->String{
	//create array of characters to upper. (nb, unicode is stored in unicode storage space)
	let nums = numbers();
	let mut out = String::with_capacity(1000);
	//up->change to superscript immediately
	//inb->superscripting value in brackets.
	let mut up = false;
	let mut inb = false;
	
	//go through string and 
	for x in a.chars(){
		if x=='^'{
			up=true;
			continue
		}else if (up)&(!inb)&(x=='('){
			inb=true;
			continue;
		}else if (up)&(inb)&(x==')'){
			up=false;
			inb=false;
			continue;
		}else if (up) & (upchar(x).1==false){
			up=false;
		};
		let mut found = false;
		if up{
			for y in nums.iter(){
				if y==&x {
					out.push(upchar(x).0);
					found = true;
				};
			};
		};
		if found {continue;};
		out.push(x);
	};
	
	//change dot to comma.
	if lang != EN {
		let a = out;
		out = String::with_capacity(1000);
		for i in 0..a.len() {
			if a.chars().nth(i-1).is_some()
			& a.chars().nth(i).is_some()
			& a.chars().nth(i+1).is_some() {
				if a.chars().nth(i-1).unwrap().is_digit(10)
				& (a.chars().nth(i).unwrap()=='.')
				& a.chars().nth(i+1).unwrap().is_digit(10){
					out.push(',');
				}else{
					out.push(a.chars().nth(i).unwrap());
				};
			}else if a.chars().nth(i).is_some() {
				out.push(a.chars().nth(i).unwrap());
			};	
		};
	}; 
	
	
	up_charge(
		a_to_an(lang,
			futile_ones(
				rem_upsilly(rem_upone(out))
			)	
		)
	)
}

//Inner function of sscri html script.
pub fn sscri_par_html(a: String,lang:u8,android:bool)->String{
	//create array of characters to upper. (nb, unicode is stored in unicode storage space)
	let mut out = String::with_capacity(2000);
	let mut out_a = String::with_capacity(2000);
	//up->change to superscript immediately
	//inb->superscripting value in brackets.
	let mut up = false;
	let mut down = false;
	let mut inb = false;
	let enter_sup = if android {"<sup><small>"}else{"<sup>"};
	let enter_sub = if android {"<sub><small>"}else{"<sub>"};
	let exit_sup = if android {"</sup></small>"}else{"</sup>"};
	let exit_sub = if android {"</sub></small>"}else{"</sub>"};
	
	//go through string and do stuff to make a html out of it.
	//mainly <sup></sup> and <sub></sub> brackets.
	//first pass.
	for x in a.chars(){
		if x=='^' {
			up=true;
			out_a.push_str(enter_sup);
		}else if up & (x=='(') {
			inb = true;
		}else if up & inb & (x==')') {
			inb = false;
			out_a.push_str(exit_sup);
			up=false;
		}else if up & is_supscriptable(x){
			out_a.push(x);
		}else if up {
			up = false;	
			out_a.push_str(exit_sup);
			out_a.push(x);
		}else if !is_subscript(x) | (is_subscript(x) & up) {
			if down {
				down = false;
				out_a.push_str(exit_sub);
			}
			out_a.push(x);
		}else if is_subscript(x) & !down {
			down = true;
			out_a.push_str(enter_sub);
			out_a.push(num_unsub(x));
		}else if is_subscript(x) {
			out_a.push(num_unsub(x));
		}else{
			out_a.push(x);
		};
	};
	
	//change (1-) and (1+) to (+) and (-)
	out_a = discharge_ones(out_a);
	out_a = form_root_unsup(out_a,android);
	
	//second pass for charges. This is probably faster than the other way.
	//also some ridiculous preconditions.
	//println!("count of chars in out_a = {}",out_a.chars().count());
	let mut supping = false;
	for i in 0..out_a.chars().count() {
		
		
		if //if space followed by (#+) skip this char 
		(out_a.chars().nth(i)==Some(' '))
		& (out_a.chars().nth(i+1)==Some('(')) //if start of block, place "<sup>" in place of "("
		& (
			(((out_a.chars().nth(i+2)==Some('+'))|(out_a.chars().nth(i+2)==Some('-')))
				& (out_a.chars().nth(i+3)==Some(')'))
			)
			| (((out_a.chars().nth(i+3)==Some('+'))|(out_a.chars().nth(i+3)==Some('-')))
				& (out_a.chars().nth(i+4)==Some(')'))
			)
		){
		//literally skip
		}else if  //if start of block, place "<sup>" in place of "("
		(out_a.chars().nth(i)==Some('('))
		& (
			(((out_a.chars().nth(i+1)==Some('+'))|(out_a.chars().nth(i+1)==Some('-')))
				& (out_a.chars().nth(i+2)==Some(')'))
			)
			| (((out_a.chars().nth(i+2)==Some('+'))|(out_a.chars().nth(i+2)==Some('-')))
				& (out_a.chars().nth(i+3)==Some(')'))
			)
		) {
			out.push_str(enter_sup);
			supping = true;
		}else if //if end of block, place "</sup>" in place of ")"
		(out_a.chars().nth(i)==Some(')')) 
		& supping {
			out.push_str(exit_sup);
			supping = false;
		}else{
			//Push everything else into the output. A little bit overdefensive.
			match out_a.chars().nth(i) {
				Some(ch) => {out.push(ch);},
				_		 => {},
			};
		};
		
	};
	
	
	//change dot to comma in numbers. for non english stuff.
	if lang != EN {
		let a = out;
		out = String::with_capacity(2000);
		for i in 0..a.len() {
			if a.chars().nth(i-1).is_some()
			& a.chars().nth(i).is_some()
			& a.chars().nth(i+1).is_some() {
				if a.chars().nth(i-1).unwrap().is_digit(10)
				& (a.chars().nth(i).unwrap()=='.')
				& a.chars().nth(i+1).unwrap().is_digit(10){
					out.push(',');
				}else{
					out.push(a.chars().nth(i).unwrap());
				};
			}else if a.chars().nth(i).is_some() {
				out.push(a.chars().nth(i).unwrap());
			};	
		};
	}; 
	
	
	//NB, no need to replace superscript silliness here, as it is all tagged.
	a_to_an(lang,futile_ones(out))
}

//superscripts characters.
pub fn upchar(a:char)->(char,bool){
	let liba = numbers();
	let libb = numbers_sup();
	for i in 0..liba.len(){
		if a==liba[i] {return (libb[i],true);};
	};
	(a,false)
}

//tests for superscript digits.
pub fn is_up(a:char)->bool{
	let libb = numbers_sup();
	for i in 0..libb.len(){
		if a==libb[i] {return true;};
	};
	false
}

//change (1-) and (1+) to (+) and (-)
fn discharge_ones(inp:String)->String {
	let mut out = String::with_capacity(inp.len());
	
	let mut detected = false;
	let mut count = 0;
	for i in 0..inp.chars().count() {
		count+= 1;
		if (inp.chars().nth(i)==Some('('))
		& (inp.chars().nth(i+1)==Some('1'))
		& ((inp.chars().nth(i+2)==Some('+'))|(inp.chars().nth(i+2)==Some('-')))
		& (inp.chars().nth(i+3)==Some(')')) {
			detected = true;
			count = 0;
			match inp.chars().nth(i) {
				Some(ch) => {out.push(ch);},
				_		 => {},
			};
		}else if detected & (count==1) & (inp.chars().nth(i)==Some('1')) {
			detected = false;
		}else{
			match inp.chars().nth(i) {
				Some(ch) => {out.push(ch);},
				_		 => {},
			};
		};

	};
	out
}

//if up to 99th√, put 99th in <sup> instead of superscript.
//NB must be in superscript to start ff with.
fn form_root_unsup(inp:String,android:bool)->String {
	
	let mut output = String::with_capacity(inp.len());
	
	let enter_sup = if android {"<sup><small>"}else{"<sup>"};
	let exit_sup = if android {"</sup></small>"}else{"</sup>"};
	
	let mut in_superscript = false;
	for i in 0..inp.chars().count() {
		if !in_superscript
		& (
			((inp.chars().nth(i+1)==Some('√'))
				& is_sup(inp.chars().nth(i).unwrap_or('!'))
			)|((inp.chars().nth(i+2)==Some('√'))
				& is_sup(inp.chars().nth(i+1).unwrap_or('!'))
				& is_sup(inp.chars().nth(i).unwrap_or('!'))
			)
		){
			in_superscript = true;
			output.push_str(enter_sup); 
			output.push(num_unsup(inp.chars().nth(i).unwrap()));
		}else if in_superscript & is_sup(inp.chars().nth(i).unwrap_or('!')) {
			output.push(num_unsup(inp.chars().nth(i).unwrap()));
		}else if in_superscript & (inp.chars().nth(i)==Some('√')) {
			output.push_str(exit_sup); 
			output.push(inp.chars().nth(i).unwrap());
			in_superscript = false;
		}else{
			match inp.chars().nth(i) {
				Some(ch) => {output.push(ch);},
				_		 => {},
			};
		};
		
	};
	
	output
}

//removes superscript ones which are on their own.
pub fn rem_upone(a:String)->String{
	//get length of string.
	let lenny = a.chars().count();
	//returns unmodified string if length is less than 3.
	if lenny<3 {return a;};
	//Does the work
	let mut b = String::new();
	b.push(a.chars().nth(0).unwrap());
	for i in 1..(lenny-1){
		if (a.chars().nth(i)==Some('\u{00B9}'))
		 & (!is_up(a.chars().nth(i-1).unwrap()))
		 & (!is_up(a.chars().nth(i+1).unwrap())){
			 continue;
		}else{
			 b.push(a.chars().nth(i).unwrap());
		};
	};
	
	if (a.chars().nth(lenny-1)==Some('\u{00B9}')) & (!is_up(a.chars().nth(lenny-2).unwrap())){
	}else{b.push(a.chars().nth(lenny-1).unwrap())};
	b
}

//removes superscript 1/1
pub fn rem_upsilly(a:String)->String{
	//get length of string.
	let d = Some('\u{2e0d}');
	let o = Some('\u{00B9}'); 
	let lenny = a.chars().count();
	//returns unmodified string if length is less than 3.
	if lenny<6 {return a;};
	let mut ind:Vec<usize>=Vec::new();
	
	//find places that need removed.
	for i in 0..(lenny-4){
		if (!is_up(a.chars().nth(i).unwrap())) & (!is_up(a.chars().nth(i+4).unwrap()))
		 & (a.chars().nth(i+1)== o) & (a.chars().nth(i+2)== d) & (a.chars().nth(i+3)== o){
			ind.push(i+1);
			ind.push(i+2);
			ind.push(i+3);
		}else{};
	};
		
	if (!is_up(a.chars().nth(lenny-4).unwrap()))
	 & (a.chars().nth(lenny-3)== o) & (a.chars().nth(lenny-2)== d) & (a.chars().nth(lenny-1)== o){
		ind.push(lenny-3);
		ind.push(lenny-2);
		ind.push(lenny-1);
	}else{};
	//remove them!
	let mut b:Vec<char> = a.chars().collect();
	for i in ind.into_iter().rev(){b.remove(i);};
	let mut c = String::new();
	for x in b.into_iter(){c.push(x)};
	c
}

//replace 1s->s 1X->x 1q->q, but leave 1 x q as it is.
pub fn futile_ones(a:String)->String {
	let mut out = String::new();
	//initialise digits
	let dig = futile_digits();
	let lena = a.chars().count();
	//abandon all hope if the string is too short
	if lena<2 {return a};
	
	//do the work
	for i in 0..(lena-1){
		if (a.chars().nth(i)==Some('1'))
		 & lshash(dig.chars(),a.chars().nth(i+1).unwrap()){
			 out.push(a.chars().nth(i).unwrap());
		}else if a.chars().nth(i).unwrap()!='1'{
			 out.push(a.chars().nth(i).unwrap());
		};
	};
	out.push(a.chars().nth(lena-1).unwrap());
	out	
}

//Replaces a with an at the beginning of words where it is appropriate.
pub fn a_to_an(lang:u8,a:String)->String {
	//This function is for english only.
	if lang != EN {return a};
	
	let mut out = String::new();
	//initialise digits
	let vowels = "aeiouAEIOU".to_owned();
	let lena = a.chars().count();
	//abandon all hope if the string is too short
	if lena<4 {return a};
	
	//do the work
	out.push(a.chars().nth(0).unwrap());
	if ((a.chars().nth(0)==Some('a'))||(a.chars().nth(0)==Some('A')))
	 & (a.chars().nth(1)==Some(' '))
	 & (lshash(vowels.chars(),a.chars().nth(2).unwrap())){
		 out.push('n');
	};
	for i in 1..(lena-3){
		out.push(a.chars().nth(i).unwrap());
		if ((a.chars().nth(i-1)==None)||(a.chars().nth(i-1)==Some(' ')))
		 & ((a.chars().nth(i)==Some('a'))||(a.chars().nth(i)==Some('A')))
		 & (a.chars().nth(i+1)==Some(' '))
		 & (lshash(vowels.chars(),a.chars().nth(i+2).unwrap())){ 
			 out.push('n');
		};
	};
	for i in (lena-3)..lena{
		out.push(a.chars().nth(i).unwrap());
	};
	out	
}

pub fn futile_digits()->String{
	"01234567890-+/=., \n\r)]<:;".to_owned()
}

pub fn numbers()->Vec<char>{
	vec!['0','1','2','3','4','5',
	     '6','7','8','9','-','+',
	     '/','a','b','c','d','e',
	     'f','g','h','i','j','k',
	     'l','m','n','o','p','q',
	     'r','s','t','u','v','w','x',
	     'y','z','A','B','C','D',
	     'E','F','G','H','I','J',
	     'K','L','M','N','O','P',
	     'Q','R','S','T','U','V',
	     'W','X','Y','Z']
}

pub fn numbers_sup()->Vec<char>{
	vec!['\u{2070}','\u{00B9}','\u{00B2}','\u{00B3}','\u{2074}','\u{2075}',
	     '\u{2076}','\u{2077}','\u{2078}','\u{2079}','\u{207B}','\u{207A}',
	     '\u{2e0d}','\u{00AA}','\u{1D47}','\u{1D9C}','\u{1D48}','\u{1D49}',
	     '\u{1DA0}','\u{1D4D}','\u{02B0}','\u{2071}','\u{02B2}','\u{1D4F}',
	     '\u{02E1}','\u{1D50}','\u{207F}','\u{1D52}','\u{1D56}','q',
	     '\u{02B3}','\u{02E2}','\u{1D57}','\u{1D58}','\u{1D5B}','\u{02B7}','\u{02E3}',
	     '\u{02B8}','\u{1DBB}','\u{1D2C}','\u{1D2E}','\u{1D9C}','\u{1D30}',
	     '\u{1D31}','F','\u{1D33}','\u{1D34}','\u{1D35}','\u{1D36}',
	     '\u{1D37}','\u{1D38}','\u{1D39}','\u{1D3A}','\u{1D3C}','\u{1D3E}',
	     '\u{00BA}','\u{1D3F}','\u{2E0F}','\u{1D40}','\u{1D41}','\u{2C7D}',
	     '\u{1D42}','\u{20DF}','\u{02E0}','Z']
}




//Generates a structure for making subscripts.
pub fn num_subs(inp:char)->char {
	match inp {
		'0'=>'\u{2080}',
		'1'=>'\u{2081}',
		'2'=>'\u{2082}',
		'3'=>'\u{2083}',
		'4'=>'\u{2084}',
		'5'=>'\u{2085}',
		'6'=>'\u{2086}',
		'7'=>'\u{2087}',
		'8'=>'\u{2088}',
		'9'=>'\u{2089}',
		_  => inp,
	}
}

//Generates a structure for making subscripts.
pub fn num_unsup(inp:char)->char {
	match inp {
		'\u{2070}'=>'0',
		'\u{00B9}'=>'1',
		'\u{00B2}'=>'2',
		'\u{00B3}'=>'3',
		'\u{2074}'=>'4',
		'\u{2075}'=>'5',
		'\u{2076}'=>'6',
		'\u{2077}'=>'7',
		'\u{2078}'=>'8',
		'\u{2079}'=>'9',
		_  => inp,
	}
}

//Generates a structure for making subscripts.
pub fn num_unsub(inp:char)->char {
	match inp {
		'\u{2080}'=>'0',
		'\u{2081}'=>'1',
		'\u{2082}'=>'2',
		'\u{2083}'=>'3',
		'\u{2084}'=>'4',
		'\u{2085}'=>'5',
		'\u{2086}'=>'6',
		'\u{2087}'=>'7',
		'\u{2088}'=>'8',
		'\u{2089}'=>'9',
		_  => inp,
	}
}

//is number a superscript..
pub fn is_sup(inp:char)->bool {
	match inp {
		'\u{2070}'=>true,
		'\u{00B9}'=>true,
		'\u{00B2}'=>true,
		'\u{00B3}'=>true,
		'\u{2074}'...'\u{2079}'=>true,
		_  => false,
	}
}


//find if number is a subscript.
pub fn is_subscript(inp:char)->bool {
	match inp {
		'\u{2080}'...'\u{2089}' => true,
		_					   => false,
	}
}

//find if number is a subscript.
pub fn is_supscriptable(inp:char)->bool {
	match inp {
		'0'...'9' => true,
		'a'...'z'				=> true,
		'A'...'Z'				=> true,
		'-'|'+'|'/'|'*'			=> true,
		_						=> false,
	}
}

//When Json is parsed, change '&x' to x-subscript.
fn subscriptise(formula:&String)->String {
	
	let mut out = String::with_capacity(20);
	
	let mut replace_next = false;
	for  mut x in formula.chars() {
		if replace_next {
			replace_next = false;
			out.push(num_subs(x));
		}else if x=='&' {
			replace_next = true;
		}else{
			out.push(x);
		}
	};
	out
}

pub fn up_charge(b:String)->String {
	let mut a = b;
	a = a.replace("(+)","\u{207A}");
	a = a.replace("(-)","\u{207B}");
	for i in 1..5{
		let old_mr = format!("({}-)",i);
		let old_pr = format!("({}+)",i);
		a = a.replace(&old_pr,"\u{207A}");
		a = a.replace(&old_mr,"\u{207B}");
	};
	a = a.replace("+)","\u{207A})");
	a = a.replace("-)","\u{207B})");
	a = a.replace("+]","\u{207A}]");
	a = a.replace("-]","\u{207B}]");
	for i in 1..5{
		let old_mr = format!("{}-)",i);
		let old_pr = format!("{}+)",i);
		let old_ms = format!("{}-]",i);
		let old_ps = format!("{}+]",i);
		a = a.replace(&old_pr,"\u{207A}]");
		a = a.replace(&old_mr,"\u{207B}]");
		a = a.replace(&old_ps,"\u{207A}]");
		a = a.replace(&old_ms,"\u{207B}]");
	};
	a
}


#[allow(unused_assignments)]
#[cfg(not(target_os = "android"))]
//Function to open a compound JSON and add its
//content to the library.
pub fn parse_compound_json()->Vec<Compound> {
	
	let mut extra_c = Vec::with_capacity(20);
	
	let mut json = match fs::File::open(JSON) {
		Ok(j) => {j},
		_	  => {return extra_c;},
	};
	
	let mut json_as_string = String::with_capacity(10000);
	json.read_to_string(&mut json_as_string).expect("Could not read json");
	println!("Extra compounds file:\n{}",json_as_string);
	
	for compound in json_as_string.split(":::") {
		println!("Extra compound pre-parsing:\n{:#?}",compound);
		match serde_json::from_str(&compound) {
			Ok(compound) => {
				let compound:CompoundJson = compound;
				println!("Extra compound:\n{:?}",compound);
				extra_c.push(compound.to_compound());},
			Err(x) => {eprintln!("Could not parse Json:\n{}",x);},		
		};
	};
	extra_c
}

#[allow(unused_assignments)]
#[cfg(target_os = "android")]
//Function to open a compound JSON and add its
//content to the library.
//NB, android version of this function is a work in progress.
pub fn parse_compound_json()->Vec<Compound> {
	
	let mut extra_c = Vec::with_capacity(20);
	
	let mut json = match fs::File::open(JSON) {
		Ok(j) => {j},
		_	  => {return extra_c;},
	};
	
	let mut json_as_string = String::with_capacity(10000);
	json.read_to_string(&mut json_as_string);
	
	
	for compound in json_as_string.split(":::") {
		match serde_json::from_str(&compound) {
			Ok(compound) => {
				let compound:CompoundJson = compound;
				extra_c.push(compound.to_compound())
			},
			_			 => {},		
		};
	};
	extra_c
}
