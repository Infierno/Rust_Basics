#include <stdio.h>
#include <stdlib.h>

int main () {
	printf("Adivina un Numero");
	// Genera un numero al random... tabun...
	// int i32_random = rand();
	
	int i32_range = (rand() %100) +1; // Generates a number between 1 and 100
	printf("El numero secreto es: %d\n", i32_range);
	printf("Por favor pon tu apuesta (un numero guey...)\n");

	int i32_apuesta = 0;
	scanf("%d", &i32_apuesta);
	printf("Apostaste: %d\n", i32_apuesta);

}
