#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;  //this will store values on run time
extern crate rocket_cors;
use std::sync::{Arc,Mutex}; //for global variables  this is used to capture data from lazystaotic
use std::collections::HashMap; //data stored in HASHMAP
use rocket_contrib::json::{Json,JsonValue};
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions
};
use rocket::State;

type ID = usize; //type se global variable define krte hn 

#[derive(Debug, PartialEq, Eq, Deserialize)] //Jb data ko in the form of bytes bhjna hota ha tou Deserialize use krte hn struct ko use krte waqt lagate hn isse
// Header se yeh pata chlta ha k request ai kaha se ha
struct Message{
    id:ID,
    content:String
}


fn make_cors() -> Cors{     //yeh function sirf un origins ko allow krega jsko hm allow krwana chah rhe hn
    let allowed_origins = AllowedOrigins::some_exact(&[

        "http://192.168.0.108"          //cross origin mien hm front end se bhi request krwa skte hn ya kisi aur server se bhi krwa skte hn

    ]) ; //jaha se koi cheez generate hoti ha usse origin khte hn
    CorsOptions{
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        // .into_iter se yeh keh rhe hn k method get bh hoskta ha post bh .map iteration perform krega jo values peche se
        // iterate hokr aenge 
        allowed_headers: AllowedHeaders::some(&[   //headers k andar yeh functionality hoti ha jo apka meta deta lekr arhe 
            "Authorization",                       //hote hn mtlb req k andar ka cheez defined ha jese string ha ya json ha etc
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error while building CORS")
}


#[get("/")]
fn hello() -> JsonValue {
    json!([
        {       
        "id" : "01",
        "name" : "Ezan"
        },
        {
        "id" : "02",
        "name" : "Asif"
        },
        {
        "id" : "03",
        "name" : "Ilyas"
        }
    ]) // returning a json object
}

// jo bhi value arhi hoti ha usse real time mien save krwane k lia hm MUTEX use krte hn.

type MessageMap = Mutex<HashMap<ID, String>>;  //ID jo upar struct mien dec ki ha woh ha

#[post("/add", data="<user_input>")]
fn helloPost(user_input : Json<Message>, map:State<'_, MessageMap>)  //'_ is lifetome
{                                   //map iteration krwane k lia
    println!("{:?}", user_input.0.content); //contents woh ha jo client side se arha ha
}

fn rocket() -> rocket::Rocket{
    rocket::ignite()
    .mount("/", routes![hello,helloPost]).attach(make_cors())
    .manage(Mutex::new(HashMap::<ID,String>::new()))
}

fn main() {
    rocket().launch();
}


//lazy statics hamare paas values ko run time pe hold krwane k lia mutex is he ka ha
//  dotenv environment ko hold krwane k lia 
