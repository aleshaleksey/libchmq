extern crate libchmq;
use libchmq::{enq,czq};
	
	//Test of drawing reactions.
	#[test]
	pub fn draw_reactions() {
		let reactions = enq::create_reaction_lib();
		
		for x in reactions.iter() {
			println!("\nReaction:\n {}",x.draw());
			println!("Reaction with state:\n {}",x.draw_with_state());
			println!("Reaction with enthalpy:\n {}",x.draw_with_heat());
			println!("Reaction with both:\n {}",x.draw_with_hs());
		};
		println!("");
		assert!(true,true);
	}
	
	//Display Czech language compounds.
	#[test]
	pub fn czq_compounds() {
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		
		for x in compounds.iter() {
			println!("\n{}",czq::form_chem(x));
			println!("{}",czq::mini_form_chem(x));
		};
		println!("");
		assert!(true,true);
	}
	
	//Display English langauge compounds.
	#[test]
	pub fn enq_compounds() {
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		
		for x in compounds.iter() {
			println!("\n{}",enq::form_chem(x));
			println!("{}",enq::mini_form_chem(x));
		};
		println!("");
		assert!(true,true);
	}


	#[test]
	pub fn q_5_0a() {
		let reactions = enq::create_reaction_lib();
		let (q,a) = enq::q_5_0_pressure(&reactions);
		
		println!("\nQuestion:\n{}\n\n",q);
		println!("Answer:\n{}\n",a);
		assert!(true,true);
	}
	
	#[test]
	pub fn q_5_0b() {
		let reactions = enq::create_reaction_lib();
		let (q,a) = enq::q_5_0_enthalpy(&reactions);
		
		println!("\nQuestion:\n{}\n\n",q);
		println!("Answer:\n{}\n",a);
		assert!(true,true);
	}
	
	#[test]
	pub fn q_5_1() {
		let reactions = enq::create_reaction_lib();
		let (q,a) = enq::q_5_1(&reactions);
		
		println!("\nQuestion:\n{}\n\n",q);
		println!("Answer:\n{}\n",a);
		assert!(true,true);
	}
	
	#[test]
	pub fn q_5_2() {
		let reactions = enq::create_reaction_lib();
		let (q,a) = enq::q_5_2(&reactions);
		
		println!("\nQuestion:\n{}\n\n",q);
		println!("Answer:\n{}\n",a);
		assert!(true,true);
	}
	
	#[test]
	//test to test every reaction with every question.
	pub fn full_equilibrium_battery() {
		let mut reactions = enq::create_reaction_lib();
		
		for x in reactions.into_iter() {
			let reactions = vec![x];
			
			let (q,a) = enq::q_5_0_pressure(&reactions);
			println!("\nQuestion:\n{}\n\nAnswer:\n{}\n",q,a);
			
			let (q,a) = enq::q_5_0_enthalpy(&reactions);
			println!("\nQuestion:\n{}\n\nAnswer:\n{}\n",q,a);
			
			let (q,a) = enq::q_5_1(&reactions);
			println!("\nQuestion:\n{}\n\nAnswer:\n{}\n",q,a);
			
			let (q,a) = enq::q_5_2(&reactions);
			println!("\nQuestion:\n{}\n\nAnswer:\n{}\n",q,a);
		};
		
		assert!(true,true);
	}
			

	#[test]
	pub fn enq_1_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_0(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_1(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_2(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_2b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_2b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_2c(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_2c(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_3(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_4(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_4(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_4b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_4b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_4c(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_4c(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_1_4d(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_1_4d(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_2_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_2_0(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_2_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_2_1(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_2_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_2_2(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_2_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_2_3(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_2_4(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_2_4(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_2_4s(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_2_4s(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_3_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_3_0(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_3_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_3_1(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_3_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_3_2(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_3_2b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_3_2b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_3_2c(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_3_2c(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_4_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_4_0(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_4_0a(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_4_0a(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_4_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_4_1(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_4_1b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_4_1b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_0(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_0b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_0b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_1(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_1b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_1b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_2a(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_2a(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_2b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_2b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_3(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_6_3b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_6_3b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_0(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_0b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_0b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_0c(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_0c(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_1(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_1b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_1b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_2(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_3(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn enq_7_3b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = enq::create_compound_lib(compounds);
		let (q,a) = enq::q_7_3b(&compounds);
		let (h,mh) = enq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

#[test]
	pub fn czq_1_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_1_0(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_1_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_1_1(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_1_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_1_2(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_1_2b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_1_2b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	//#[test]
	//pub fn czq_1_2c(){
		//let mut compounds = Vec::with_capacity(100);
		//compounds = czq::create_compound_lib(compounds);
		//let (q,a) = czq::q_1_2c(&compounds);
		//let (h,mh) = czq::helper(&q,&compounds);
		
		
		//println!("Question:\n{}\n",q);
		//println!("Answer:\n{}\n",a);
		//println!("Datasheet:\n{}\n",mh);
		//println!("Help:\n{}\n",h);
		//assert!(true,true);
	//}

	#[test]
	pub fn czq_1_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_1_3(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_1_4(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_1_4(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_1_4b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_1_4b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	//#[test]
	//pub fn czq_1_4c(){
		//let mut compounds = Vec::with_capacity(100);
		//compounds = czq::create_compound_lib(compounds);
		//let (q,a) = czq::q_1_4c(&compounds);
		//let (h,mh) = czq::helper(&q,&compounds);
		
		
		//println!("Question:\n{}\n",q);
		//println!("Answer:\n{}\n",a);
		//println!("Datasheet:\n{}\n",mh);
		//println!("Help:\n{}\n",h);
		//assert!(true,true);
	//}

	//#[test]
	//pub fn czq_1_4d(){
		//let mut compounds = Vec::with_capacity(100);
		//compounds = czq::create_compound_lib(compounds);
		//let (q,a) = czq::q_1_4d(&compounds);
		//let (h,mh) = czq::helper(&q,&compounds);
		
		
		//println!("Question:\n{}\n",q);
		//println!("Answer:\n{}\n",a);
		//println!("Datasheet:\n{}\n",mh);
		//println!("Help:\n{}\n",h);
		//assert!(true,true);
	//}

	#[test]
	pub fn czq_2_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_2_0(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_2_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_2_1(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_2_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_2_2(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_2_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_2_3(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_2_4(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_2_4(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_2_4s(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_2_4s(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_3_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_3_0(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_3_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_3_1(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_3_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_3_2(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_3_2b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_3_2b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_3_2c(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_3_2c(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_4_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_4_0(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_4_0a(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_4_0a(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_4_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_4_1(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_4_1b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_4_1b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_0(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_0b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_0b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_1(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_1b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_1b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_2a(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_2a(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_2b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_2b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_3(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_6_3b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_6_3b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_0(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_0(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_0b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_0b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_0c(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_0c(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_1(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_1(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_1b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_1b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_2(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_2(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_3(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_3(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}

	#[test]
	pub fn czq_7_3b(){
		let mut compounds = Vec::with_capacity(100);
		compounds = czq::create_compound_lib(compounds);
		let (q,a) = czq::q_7_3b(&compounds);
		let (h,mh) = czq::helper(&q,&compounds);
		
		
		println!("Question:\n{}\n",q);
		println!("Answer:\n{}\n",a);
		println!("Datasheet:\n{}\n",mh);
		println!("Help:\n{}\n",h);
		assert!(true,true);
	}
