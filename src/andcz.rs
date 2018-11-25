//use Compound;
//use Sscri;
//use czq;

//use rand;
//use rand::Rng;

//use czq::WARNING;
//use czq::TITLE;
//use czq::ABOUT;
//use CZ;

#[cfg(target_os = "android")]use jni::JNIEnv;
#[cfg(target_os = "android")]use jni::objects::{JClass};
#[cfg(target_os = "android")]use jni::sys::jstring;

#[allow(non_snake_case)]
#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_infoCz_infoCz (env: JNIEnv, class: JClass)->jstring {
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",TITLE,ABOUT,WARNING,WARNING))
	   .expect("Couldn't create java string!")
	   .into_inner()
	
}

#[allow(non_snake_case)]
#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_molesQCz_molesQuestionsCz<'a> (env: JNIEnv, class: JClass)->jstring {
	println!("molesQuestions has been called from Java...");
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=czq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,7);
	let mut q_a_text=match r_ind{
		0=>czq::q_1_0(&compounds),
		1=>czq::q_1_1(&compounds),
		2=>czq::q_1_2(&compounds),
		3=>czq::q_1_2b(&compounds),
		4=>czq::q_1_3(&compounds),
		5=>czq::q_1_4(&compounds),
		_=>czq::q_1_4b(&compounds),
	};
	q_a_text = q_a_text.sscri(CZ);
	let (help,minihelp)=czq::helper(&q_a_text.0,&compounds);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp);
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_osmoticQCz_osmoticQuestionsCz (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=czq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));

	let r_ind=rand::thread_rng().gen_range(0,6);
	let mut q_a_text=match r_ind{
		0=>czq::q_2_0(&compounds),
		1=>czq::q_2_1(&compounds),
		2=>czq::q_2_3(&compounds),
		3=>czq::q_2_2(&compounds),
		4=>czq::q_2_4(&compounds),
		_=>czq::q_2_4s(&compounds),
	};
	q_a_text = q_a_text.sscri(CZ);
	let (help,minihelp) = czq::helper(&q_a_text.0,&compounds);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp);
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_ionicQCz_ionicQuestionsCz (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=czq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,4);
	let mut q_a_text=match r_ind{
		0=>czq::q_3_0(&compounds),
		1=>czq::q_3_2(&compounds),
		2=>czq::q_3_2b(&compounds),
		3=>czq::q_3_2c(&compounds),
		_=>czq::q_3_1(&compounds),
	};
	q_a_text = q_a_text.sscri(CZ);
	let (help,minihelp) = czq::helper(&q_a_text.0,&compounds);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp);
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_kspQCz_kspQuestionsCz (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=czq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,4);
	let mut q_a_text=match r_ind{
		1=>czq::q_4_0a(&compounds),
		2=>czq::q_4_1(&compounds),
		3=>czq::q_4_0(&compounds),
		_=>czq::q_4_1b(&compounds),
	};
	q_a_text = q_a_text.sscri(CZ);
	let (help,minihelp) = czq::helper(&q_a_text.0,&compounds);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp);
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}			

#[allow(unused_variables)]
#[no_mangle]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_phQCz_pHQuestionsCz (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=czq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,6);
	let mut q_a_text=match r_ind{
		1=>czq::q_6_0b(&compounds),
		2=>czq::q_6_0(&compounds),
		3=>czq::q_6_1(&compounds),
		4=>czq::q_6_1b(&compounds),
		5=>czq::q_6_3(&compounds),
		_=>czq::q_6_3b(&compounds),
	};
	q_a_text = q_a_text.sscri(CZ);
	let (help,minihelp) = czq::helper(&q_a_text.0,&compounds);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp);
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_bufferQCz_bufferQuestionsCz (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=czq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,10);
	let mut q_a_text=match r_ind{
		0=>czq::q_7_0(&compounds),
		1=>czq::q_7_0b(&compounds),
		2=>czq::q_6_2a(&compounds),
		3=>czq::q_6_2b(&compounds),
		4=>czq::q_7_0c(&compounds),
		5=>czq::q_7_1(&compounds),
		6=>czq::q_7_1b(&compounds),
		7=>czq::q_7_2(&compounds),
		8=>czq::q_7_3(&compounds),
		_=>czq::q_7_3b(&compounds),
	};
	q_a_text = q_a_text.sscri(CZ);
	let (help,minihelp) = czq::helper(&q_a_text.0,&compounds);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp);
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()		
}
