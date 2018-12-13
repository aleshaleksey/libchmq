# libchmq[<img src="https://api.travis-ci.org/aleshaleksey/libchmq.svg?branch=master">](https://travis-ci.org/aleshaleksey/libchmq)

This is a library for generating simple chemistry calculation problems.
This library powers the Chmq app on Android, and the Chmq server-client (eg [here]).

NB
--
I have included a copy of the [jni-rs] library here with a modified **Cargo.toml** without backtrace, to simplify compilation on android.

Aims
----

To make a backend library that can generate a large number of questions on various topics in chemistry calculations.

Why Rust
-----------
For no other reason than author's familiarity with the language. Also, does not require additional interpreters to be installed on user's machine.
Lastly is fast enough to be used on the server-side client.


About
-----
***Structure***<br>
The library is currently split into five modules.
*lib*, *enq*, *czq*, *anden*, *andcz*.

*lib* handles core functions, including parsing of additional compounds from json and
formatting output.

*enq* handles the questions and compounds for English module.

*czq* handles the questions and compounds for the Czech module.

*anden* and *andcz* are a makeshift wrapper library exporting functionality to java, for use in Android apps.

***Topics***<br>
Currently covers:
1) Moles, concentrations and dilutions. (topic 1)
2) Osmolarity and osmotic pressure. (topic 2)
3) Ionic strength. (topic 3)
4) Ksp. (topic 4)
6) pH of strong and weak acids/bases (assuming a single pKa), in aqueous solutions. (topic 6)
7) Buffers (assuming a single pKa). (topic 7)

Currently the equilibria chapter is a work in process.

***Use***<br>
All questions are functions, named eg "q_1_0", which return (String,String) and are "pub fn", allowing direct use.<br>

The sscri trait can be used to format the text into a "scientific" script if it is supported by the system fonts.
eg.

```Rust
//Initialise library.
let mut compounds = Vec::with_capacity(200);
let compounds = libchmq::enq::create_compound_lib(compounds);

//Generate text for a question.
let (q,a) = libchmq::enq::q_1_0(&compounds).sscri(libchmq::EN);

//Generate text for a question using html compatible subscript/superscript tags.
let (q_html,a_html) = libchmq::enq::q_1_0(&compounds).sscri_html(libchmq::EN);

//Generate text for a question using android textView compatible format.
//NB Html.fromHtml() needs to be used.
let (q_and,a_and) = libchmq::enq::q_1_0(&compounds).sscri_and(libchmq::EN);

println!("Question:\n{}\n\nAnswer:\n{}",q,a);
println!("Now for html compatible subscripts/superscripts.\nQuestion:\n{}\n\nAnswer:\n{}",q_html,a_html);
println!("Now for android textView compatible format.\nQuestion:\n{}\n\nAnswer:\n{}",q_and,a_and);
```

The text of a question can then be scanned with the "helper" function,
which then generates an abbreviated and complete datasheet for the text scanned.

```Rust
use libchmq::{self,enq};

//Initialise library.
let mut compounds = Vec::with_capacity(200);
let compounds = enq::create_compound_lib(compounds);

//get question and answer.
let (q,a) = enq::q_1_0(&compounds);
let (help,data) = enq::helper(&q,&compounds);

println!("Question:\n{}",q);
println!("Datasheet:\n{}",data);
println!("Answer:\n{}",a);
println!("Help:\n{}",help);
```

The lib module contains a wrapper function "generate_questions",
which takes pointers to a vector of question functions and generates a random question.<br>

```Rust
use libchmq::{self,enq};

//Initialise library.
let mut compounds = Vec::with_capacity(200);
let compounds = enq::create_compound_lib(compounds);

//Generate a complete question, answer and help
//choosing at random from q_3_0, q_3_1 or q_3_2.
//NB SYMBOL uses special chracters. HTML uses <sup> & <sub> tags.
let (q,a,help,data) = generate_questions(
    &compounds,
    vec![&enq::q_3_0,&enq::q_3_1,&enq::q_3_2],
    EN,SYMBOL
);

println!("Question:\n{}",q);
println!("Datasheet:\n{}",data);
println!("Answer:\n{}",a);
println!("Help:\n{}",help);
```

***Comment***<br>
Probably could do with a lot of cleaning up and optimisation. The Czech language section is a work in progress with most of the translation by [zjedna].

[zjedna]:https://github.com/zjedna
[here]:http://biomedicina.upol.cz:7000
[jni-rs]:https://github.com/jni-rs/


