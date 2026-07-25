// Script de prueba para Lexer y Parser
// Fase 39 - Parser Real para scripts .bf

var x = 10;
var y = 20;

print("Suma: " + x + y);
print("Resta: " + x - y);
print("Multiplicación: " + x * y);
print("División: " + x / y);
print("Módulo: " + x % y);

// Operadores lógicos
var a = true;
var b = false;
print("AND: " + (a && b));
print("OR: " + (a || b));
print("NOT: " + (!a));

// Comparaciones
print("Mayor: " + (x > y));
print("Menor: " + (x < y));
print("Igual: " + (x == y));

// Bitwise NOT
var z = 5;
print("Bitwise NOT de 5: " + ~z);

// Condicionales
if (x > y) {
    print("x es mayor que y");
} else {
    print("x es menor o igual que y");
}

// Bucles
var i = 0;
while (i < 5) {
    print("Iteración: " + i);
    i = i + 1;
}

// Arrays
var numeros = [1, 2, 3, 4, 5];
print("Array: " + numeros);

// Objetos
var persona = {
    nombre: "Juan",
    edad: 30,
    ciudad: "Madrid"
};
print("Persona: " + persona);

print("¡Script completado!");
