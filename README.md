# Taller: Manipulación y Monitoreo de Recursos del SO desde el Código

## *Instrucciones*
### **1. Vigilante de Recursos (Monitoreo de RAM y CPU)**

**Concepto**: Gestor de memoria y E/S.

**Objetivo**: Crear un script que monitoree en tiempo real el consumo de recursos y genere una alerta si se supera un umbral, simulando un panel de control del SO.

- **Tarea**: Usar la librería adecuada en Python o en Node.js para obtener el porcentaje de uso de RAM y CPU.

- **Reto**: El script debe guardar un log en un archivo .txt cada vez que la RAM supere el 80%.

### **2. El Simulador de Memoria Caché (L1/L2 vs RAM)**

**Concepto**: Jerarquía de memoria y optimización.

**Objetivo**: Implementar una caché en memoria (usando un Diccionario o Map) para entender cómo el SO evita ir al disco (memoria secundaria) constantemente.

- **Tarea**: Crear un programa que lea archivos grandes. La primera vez que lea un archivo, debe guardarlo en un objeto en RAM. Las siguientes veces debe leerlo desde el objeto, no desde el disco.

- **Reto**: Medir el tiempo de ejecución en ambos casos usando console.time() o time.time() dependiendo del lenguaje node js o python. 

### **3. Estrés de Memoria y Salto a la Virtual (Paginación)**

**Concepto**: Memoria Virtual y Archivo de Paginación.

**Objetivo**: Forzar al sistema operativo a usar la memoria virtual (swap) mediante la creación de objetos masivos.

- **Tarea**: Crear un script que llene una lista/array con millones de strings en un bucle infinito.

- **Reto**: Mientras el script corre, deben abrir el administrador de tareas y observar cómo la virtual sube cuando la RAM física se agota.

- **Advertencia**: Deben implementar un límite de seguridad para no bloquear la máquina.r

### **4. Prioridad de Procesos (Scheduling)**

**Concepto**: Planificación de procesos del Kernel.

**Objetivo**: Aprender que no todos los procesos son iguales para el SO y que podemos influir en su "importancia".

- **Tarea**: En Python o node js, usar una libreria adecuada para cambiar la prioridad de un proceso que esté realizando un cálculo matemático pesado.

- **Reto**: Ejecutar dos instancias del mismo script: una con prioridad "Baja" y otra con prioridad "Tiempo Real". Comparar cuál termina primero el cálculo.
