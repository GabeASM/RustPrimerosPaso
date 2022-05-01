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

    // tipos de datos ***************************************
    

    //Tipos de entero 
    //Los numeros se deben definir por bits y por signo 
    let mut i :u32 = 4 ; // u significa unsigned (solo podemos usar positivos)
    let mut j :i32 = 4;  //i significa signed y podemos usar negativos y positivos
    j = -4; 

    //Tipo entero
    let mut decimal1 = 4.0; // solo podemos asignarles float a decimal1 ya habiendo dicho cuanto valia
    let decimal2 : f32 = 4.0;


    //Rust nos permite operaciones basicas matematicas

    let suma = 10+10; 
    let resta = 10-5; 
    let multiplicacion = 4*2 ; 
    let division = 34/4;
    let resto = 34%4 ;

    let suma1 : f32; 
    let a = 4.5; 
    let b = 4.5 ; 
    suma1 = a+b; 

    //tipo booleano 
    let verdadero = true; 
    let falso : bool ; 
    falso = false; 

    //Tipo Caracter
    let c = "c";

    //tipos compuesto 
    /*las tuplas es una forma general de agrupar varios valores, que pueden ser de diferentes tipo
    las tuplas tienen una longitud fija (no pueden encoger ni agrandar tamaño) */

    let tupla :(f32 ,i32,i32) = (500.32 , 4, -123 );
    let (r,t,u) = tupla;
    println!("el segundo valor de la tupla es: {}" , t);
    let primero = tupla.0;

    //Tipo array 
    let matriz = [23,43,21,1];
    let matriz2 :[i32;5] = [23,12,3,1,2];

    let matrizrapida = [3;5];
    //[3,5] = [3,3,3,3,3];
    





    
    
    


    

}
