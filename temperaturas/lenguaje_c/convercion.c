/* imprime la tabla fahrenheit-celsius
para fahr = 0, 20, ..., 300 */
#include <stdio.h>

int main () {
	int fahr, celsius;
        int lower, uper, step;
	
	lower = 0;       //limite inferior de la tabla.
	uper = 300;     //limite superior de la tabla. 
	step = 20;       //tamaño del incremento.
	
	fahr = lower;

	while (fahr <= uper) {
		
		celsius = 5 * (fahr-32) /9;
			printf("%d\t%d\n", fahr,celsius); 
			fahr= fahr + step;

			}
// El siguiente programa utiliza la fórmula °C = (5/9) 
// (°F-32) para imprimir la siguiente tabla de temperaturas 
// Fahrenheit y sus equivalentes centígrados o Celsius.


	}	
