//use Compound;
//use Sscri;
//use enq;

//use rand;
//use rand::Rng;
//use EN;

//use enq::WARNING;
//use enq::TITLE;
//use enq::ABOUT;

#[cfg(target_os = "android")]use jni::JNIEnv;
#[cfg(target_os = "android")]use jni::objects::{JClass};
#[cfg(target_os = "android")]use jni::sys::jstring;

#[allow(non_snake_case)]
#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_infoEn_infoEn (env: JNIEnv, class: JClass)->jstring {
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",TITLE,ABOUT,WARNING,WARNING))
	   .expect("Couldn't create java string!")
	   .into_inner()
	
}

#[allow(non_snake_case)]
#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_molesQEn_molesQuestionsEn<'a> (env: JNIEnv, class: JClass)->jstring {
	println!("molesQuestions has been called from Java...");
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=enq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,9);
	let mut q_a_text=match r_ind{
		0=>enq::q_1_0(&compounds),
		1=>enq::q_1_1(&compounds),
		2=>enq::q_1_2(&compounds),
		3=>enq::q_1_2b(&compounds),
		4=>enq::q_1_3(&compounds),
		5=>enq::q_1_4(&compounds),
		6=>enq::q_1_4c(&compounds),
		7=>enq::q_1_4d(&compounds),
		_=>enq::q_1_4b(&compounds),
	};
	q_a_text = q_a_text.sscri_html_android(EN,"style=\"white-space:pre-wrap;\"");
	let (help,minihelp)=enq::helper(&q_a_text.0,&compounds).sscri_android(EN);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp).sscri_html_body(EN,"style=\"white-space:pre-wrap;\"");
	
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_osmoticQEn_osmoticQuestionsEn (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=enq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));

	let r_ind=rand::thread_rng().gen_range(0,6);
	let mut q_a_text=match r_ind{
		0=>enq::q_2_0(&compounds),
		1=>enq::q_2_1(&compounds),
		2=>enq::q_2_3(&compounds),
		3=>enq::q_2_2(&compounds),
		4=>enq::q_2_4(&compounds),
		_=>enq::q_2_4s(&compounds),
	};
	q_a_text = q_a_text.sscri_html_android(EN,"style=\"white-space:pre-wrap;\"");
	let (help,minihelp)=enq::helper(&q_a_text.0,&compounds).sscri_android(EN);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp).sscri_html_body(EN,"style=\"white-space:pre-wrap;\"");
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_ionicQEn_ionicQuestionsEn (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=enq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,5);
	let mut q_a_text=match r_ind{
		0=>enq::q_3_0(&compounds),
		1=>enq::q_3_2(&compounds),
		2=>enq::q_3_2b(&compounds),
		3=>enq::q_3_2c(&compounds),
		_=>enq::q_3_1(&compounds),
	};
	q_a_text = q_a_text.sscri_html_android(EN,"style=\"white-space:pre-wrap;\"");
	let (help,minihelp)=enq::helper(&q_a_text.0,&compounds).sscri_android(EN);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp).sscri_html_body(EN,"style=\"white-space:pre-wrap;\"");
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_kspQEn_kspQuestionsEn (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=enq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,4);
	let mut q_a_text=match r_ind{
		1=>enq::q_4_0a(&compounds),
		2=>enq::q_4_1(&compounds),
		3=>enq::q_4_0(&compounds),
		_=>enq::q_4_1b(&compounds),
	};
	q_a_text = q_a_text.sscri_html_android(EN,"style=\"white-space:pre-wrap;\"");
	let (help,minihelp)=enq::helper(&q_a_text.0,&compounds).sscri_android(EN);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp).sscri_html_body(EN,"style=\"white-space:pre-wrap;\"");
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}			

#[allow(unused_variables)]
#[no_mangle]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_phQEn_pHQuestionsEn (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=enq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,6);
	let mut q_a_text=match r_ind{
		1=>enq::q_6_0b(&compounds),
		2=>enq::q_6_0(&compounds),
		3=>enq::q_6_1(&compounds),
		4=>enq::q_6_1b(&compounds),
		5=>enq::q_6_3(&compounds),
		_=>enq::q_6_3b(&compounds),
	};
	q_a_text = q_a_text.sscri_html_android(EN,"style=\"white-space:pre-wrap;\"");
	let (help,minihelp)=enq::helper(&q_a_text.0,&compounds);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp).sscri_html_body(EN,"style=\"white-space:pre-wrap;\"");
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()
}

#[no_mangle]
#[allow(unused_variables)]
#[cfg(target_os = "android")]
pub extern fn Java_chmq_example_owl_chmq_bufferQEn_bufferQuestionsEn (env: JNIEnv, class: JClass)->jstring {
	
	let mut compounds:Vec<Compound>=Vec::new();
	compounds=enq::create_compound_lib(compounds);
	//Insert functions here. Or are they already in scope?	
	let q_num_text=format!("{}\n",format!("\nQuestion ID: {}",rand::thread_rng().gen_range(0,99999)));
	
	let r_ind=rand::thread_rng().gen_range(0,10);
	let mut q_a_text=match r_ind{
		0=>enq::q_7_0(&compounds),
		1=>enq::q_7_0b(&compounds),
		2=>enq::q_6_2a(&compounds),
		3=>enq::q_6_2b(&compounds),
		4=>enq::q_7_0c(&compounds),
		5=>enq::q_7_1(&compounds),
		6=>enq::q_7_1b(&compounds),
		7=>enq::q_7_2(&compounds),
		8=>enq::q_7_3(&compounds),
		_=>enq::q_7_3b(&compounds),
	};
	q_a_text = q_a_text.sscri_html_android(EN,"style=\"white-space:pre-wrap;\"");
	let (help,minihelp)=enq::helper(&q_a_text.0,&compounds).sscri_android(EN);
	q_a_text.0 = format!("{}\n\n{}",q_a_text.0,minihelp).sscri_html_body(EN,"style=\"white-space:pre-wrap;\"");
	
	env.new_string(format!("{}ZQ_QZ{}ZQ_QZ{}ZQ_QZ{}",q_num_text,q_a_text.0,q_a_text.1,help))
	   .expect("Couldn't create java string!")
	   .into_inner()		
}
