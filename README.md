# Solana Sales Registry

**Sistema descentralizado de registro de negocios, clientes y ventas en Solana**

Proyecto desarrollado como parte del **Solana Engineering Track: Advanced**, organizado por WayLearn, utilizando Rust, Anchor, React y TypeScript.

## 1. Descripción del proyecto

Solana Sales Registry es una aplicación educativa basada en blockchain que permite administrar información de negocios, clientes y operaciones comerciales mediante un programa desarrollado en Rust con el framework Anchor.

El sistema utiliza cuentas de Solana y direcciones derivadas de programa (PDA) para organizar los registros y establecer relaciones entre las entidades.

El frontend proporciona una interfaz para interactuar con el programa desplegado en Solana Devnet.

Este proyecto está destinado a fines educativos y de desarrollo. No procesa pagos comerciales ni sustituye un sistema de facturación fiscal.

## 2. Objetivo

Desarrollar e integrar una aplicación descentralizada que permita registrar y consultar información comercial, administrar clientes y ventas, y aplicar reglas de validación mediante un programa de Solana.

Los objetivos específicos son:

- Implementar instrucciones mediante Rust y Anchor.
- Utilizar cuentas relacionadas para representar negocios, clientes y ventas.
- Incorporar validaciones y restricciones en las operaciones comerciales.
- Generar un cliente TypeScript compatible con el IDL del programa.
- Integrar una interfaz web desarrollada con React.
- Verificar el funcionamiento mediante pruebas en Solana Devnet.

## 3. Tecnologías utilizadas

| Tecnología | Aplicación |
|---|---|
| Solana | Red blockchain utilizada |
| Rust | Lenguaje del programa |
| Anchor | Framework para desarrollar instrucciones y cuentas |
| TypeScript | Integración entre frontend y blockchain |
| React | Interfaz de usuario |
| Vite | Servidor de desarrollo |
| Codama | Generación del cliente TypeScript |
| Solflare | Conexión y firma mediante wallet |
| GitHub Codespaces | Entorno de desarrollo |
| Solana Playground | Compilación y despliegue inicial |

## 4. Funcionalidades del programa

El programa incluye instrucciones para las siguientes operaciones:

**Crear registro:** inicializa el registro de un negocio y almacena la dirección pública de su autoridad.

**Crear cliente:** registra un cliente asociado al negocio y le asigna un identificador incremental.

**Registrar venta:** crea una venta relacionada con un cliente, incluyendo identificador del producto, descripción e importe.

**Cambiar monto:** permite modificar el importe registrado de una venta, sujeto a las validaciones del programa.

**Hacer devolución:** procesa la devolución de una venta y actualiza las referencias relacionadas.

**Eliminar cliente:** permite retirar un cliente cuando se cumplen las condiciones definidas por el programa.

Estas operaciones están implementadas en el código Rust. Sus resultados desde el frontend requieren las pruebas funcionales correspondientes.

## 5. Estructura de datos

El programa utiliza tres tipos principales de cuentas:

- **Registro:** contiene la autoridad del negocio, su nombre, las referencias a clientes y el contador de identificadores.
- **Cliente:** almacena el identificador, el nombre, el número de compras y las referencias a su historial.
- **Venta:** contiene el SKU, la descripción y el monto de una operación comercial.

Las relaciones entre estas cuentas permiten organizar y consultar la información del negocio.

## 6. Validaciones y reglas de negocio

El código contiene restricciones relacionadas con:

- Longitud máxima de los campos de texto.
- Capacidad de registros y referencias.
- Generación de identificadores incrementales.
- Relación entre clientes, registros y ventas.
- Validación de montos mayores que cero.
- Condiciones para modificar o eliminar registros.

**Aporte funcional en revisión:** validaciones de datos comerciales en el programa Rust. La comprobación del rechazo de importes no positivos y la identificación de una mejora adicional respecto al proyecto guiado quedan pendientes de documentar.

## 7. Despliegue en Solana Devnet

**Red:** Solana Devnet

**Program ID:**

`AJDNp5ZYCdA7XRYZKNa2v5hcdkprKybSFTD6aoD2yRPm`

[Consultar programa en Solana Explorer](https://explorer.solana.com/address/AJDNp5ZYCdA7XRYZKNa2v5hcdkprKybSFTD6aoD2yRPm?cluster=devnet)

El programa se compiló y desplegó mediante Solana Playground.

La terminal confirmó:

- `Build successful`
- `Deployment successful`

El archivo `idl.json` se exportó desde Playground y se incorporó al repositorio.

## 8. Repositorios y archivos

**Repositorio del programa y la documentación:**

https://github.com/jaqueline1/solana-sales-registry

**Repositorio de trabajo del frontend:**

https://github.com/jaqueline1/Registro-Ventas-Solana-App

El frontend parte del proyecto educativo de WayLearn y fue configurado en GitHub Codespaces para utilizar el IDL y el Program ID del despliegue.

La integración de todos los archivos modificados del frontend y del cliente generado en el repositorio de entrega continúa pendiente.

## 9. Instalación y ejecución del frontend

El frontend puede ejecutarse desde GitHub Codespaces, utilizando la copia del repositorio de WayLearn.

Para reproducir la configuración, se necesita:

1. Abrir el repositorio del frontend en Codespaces.
2. Instalar las dependencias mediante `npm install`.
3. Obtener el archivo `idl.json` exportado desde Solana Playground.
4. Importar el IDL y configurar el Program ID utilizando el comando del proyecto:

```bash
npm run configurar:idl -- ./tmp-idl/registro.json --program-id AJDNp5ZYCdA7XRYZKNa2v5hcdkprKybSFTD6aoD2yRPm
```

El archivo `./tmp-idl/registro.json` debe corresponder al IDL descargado del programa desplegado.

5. Iniciar el frontend:

```bash
npm run dev
```

6. Abrir la aplicación mediante el puerto 5173 de Codespaces.

Para realizar operaciones que modifiquen cuentas en Devnet se necesita una wallet de pruebas, como Solflare, con SOL de Devnet.

No es necesario utilizar fondos reales.

## 10. Pruebas realizadas

| Comprobación | Resultado observado |
|---|---|
| Compilación del programa Rust | Exitosa |
| Despliegue en Solana Devnet | Exitoso |
| Exportación del IDL | Completada |
| Generación del cliente TypeScript | Completada |
| Coherencia entre IDL, cliente y Program ID | Validada por el script de comprobación |
| Ejecución de la aplicación React | Exitosa |
| Conexión de Solflare | Exitosa |
| Consulta del saldo de la wallet | 0 SOL de prueba |
| Creación de negocio y clientes desde el frontend | Completada |
| Registro, corrección y devolución de ventas | Completada |
| Pruebas de rechazo de operaciones inválidas | Completada |

Estas comprobaciones se realizaron durante el desarrollo del proyecto en Solana Playground y GitHub Codespaces.

## 11. Evidencias del proyecto

Las capturas de compilación, despliegue, generación del cliente y ejecución del frontend se conservaron durante el desarrollo y están pendientes de incorporarse a la carpeta `evidencias/` del repositorio.

Las pruebas funcionales deberán incluir el estado inicial, la operación realizada, el resultado esperado, el resultado observado y, cuando corresponda, el enlace a la transacción de Solana Explorer.

No se reportan transacciones comerciales de prueba como exitosas mientras no hayan sido ejecutadas y verificadas.

## 12. Estado del proyecto

El prototipo cuenta con un programa compilado y desplegado en Devnet y un frontend que se ejecuta con la configuración del IDL correspondiente.

Las siguientes actividades están pendientes para completar la entrega:

- Publicar los archivos actualizados del frontend y del cliente generado.
- Integrar las evidencias al repositorio.
- Comprobar las operaciones comerciales en Devnet.
- Documentar el aporte funcional y su caso de prueba.
- Verificar que el repositorio permita reproducir el proyecto.

## 13. Referencias

- [Solana — Documentación oficial](https://solana.com/docs)
- [Anchor — Documentación oficial](https://www.anchor-lang.com/docs)
- [WayLearn — Solana Engineering Track: Advanced](https://waylearn.gitbook.io/solana-engineering-track-advanced)
- [Solana Explorer — Devnet](https://explorer.solana.com/?cluster=devnet)

---

**Proyecto académico desarrollado para el Solana Engineering Track: Advanced.**

*Red de desarrollo: Solana Devnet.*
