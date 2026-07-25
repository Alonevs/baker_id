// Test script completo para Lexer y Parser
// Fase 39 - Parser Real para scripts .bf

// Variables
var x = 10;
var y = 20;
const z = 30;

// Operadores aritmeticos
print("Suma: " + x + y);
print("Resta: " + x - y);
print("Multiplicacion: " + x * y);
print("Division: " + x / y);
print("Modulo: " + x % y);

// Operadores logicos
var a = true;
var b = false;
print("AND: " + (a && b));
print("OR: " + (a || b));
print("NOT: " + (!a));

// Bitwise NOT
var num = 5;
print("Bitwise NOT de 5: " + ~num);

// Comparaciones
print("Mayor: " + (x > y));
print("Menor o igual: " + (x <= y));
print("Igual: " + (x == y));
print("Diferente: " + (x != y));

// Condicionales
if (x > y) {
    print("x es mayor que y");
} else {
    print("x es menor o igual que y");
}

// Bucles while
var i = 0;
while (i < 5) {
    print("Iteracion " + i);
    i = i + 1;
}

// Bucles for
var j = 0;
for (j = 0; j < 3; j = j + 1) {
    print("For iteracion " + j);
}

// Arrays
var numeros = [1, 2, 3, 4, 5];
print("Array: " + numeros);
print("Primer elemento: " + numeros[0]);

// Objetos
var persona = {
    nombre: "Juan",
    edad: 30,
    ciudad: "Madrid"
};
print("Persona: " + persona);
print("Nombre: " + persona.nombre);

// Funciones
function saludar(nombre) {
    return "Hola, " + nombre + "!";
}

print("Saludo: " + saludar("Mundo"));

// Return
function suma(a, b) {
    return a + b;
}

print("Suma de 5 y 3: " + suma(5, 3));

print("Script completado!");
