fn main() {
   
    println!("Hello, world!");
    // VARIABLES  Y CONSTANTES**************************************************************************************
    /*
    Rust es un lenguaje con tipado estatico
    */ 
    let x = 3 ;
    println!("el numero x es {}" , x);

    // si reasignamos la variable nos dará error
    /* 
    let x = 4; 
    Esta instruccion no funcionara pero si queremos que reasignar debemos usar mut
    */

    let mut y = 3;
    println!("el numero y es {}" , y);
    y = 4; 
    println!("el numero y es {}" , y);

    // rust tambien admite constantes con la palabra const
    // se debe declarar con const NOMBRECONSTANTE : tipoconstante  =  123123.123
    
    const PI : f32 = 3.1416;

    // no podemos ponerle mut a las constantes y deben llevar el tipo si o si 
    println!("El valor de PI es {}" , PI);


    

}
