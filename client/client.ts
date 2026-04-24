import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

// Configuración base:  En esta sección se establece la conexión entre el cliente (frontend o script)
/// y el programa desplegado en la blockchain de Solana.
const provider = anchor.AnchorProvider.env();// Se obtiene el provider desde el entorno 
//(Anchor CLI / configuración local)
anchor.setProvider(provider); // Se establece como provider global para todas las operaciones

const program = anchor.workspace.Escuela as Program;// Se obtiene el programa compilado desde workspace (IDL)

const wallet = provider.wallet;// Se obtiene la wallet del usuario (quien firma las transacciones)

// Obtener PDA de la escuela: Calcula la direccion PDA de la cuenta de la escuela 
async function getEscuelaPDA() {
  const [escuelaPDA] = await PublicKey.findProgramAddress(
    [
      Buffer.from("escuela"),
      wallet.publicKey.toBuffer()
    ],
    program.programId
  );

  return escuelaPDA;
}

//////////////////////////// CREAR ESCUELA ////////////////////////////
//Convoca la intruccion on-chaim para crear una nueva escuela 
//su flujo es: Calcular el PDA de la escuela- Se llama el método del programa
//se envia la transaccion de la red 
export async function crearEscuela(nombre: string) {

  const escuelaPDA = await getEscuelaPDA();

  const tx = await program.methods
    .crearEscuela(nombre)
    .accounts({
      owner: wallet.publicKey,
      escuela: escuelaPDA,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("Escuela creada:", tx);
}

//////////////////////////// AGREGAR PROFESOR ////////////////////////////
//Permite agregar un profesor a la escuela existente (Se optiene en PDA, 
//invoca el metodo y refirma la transaccion)
export async function agregarProfesor(
  nombre: string,
  especialidad: string,
  experiencia: number
) {

  const escuelaPDA = await getEscuelaPDA();

  const tx = await program.methods
    .agregarProfesor(nombre, especialidad, experiencia)
    .accounts({
      owner: wallet.publicKey,
      escuela: escuelaPDA,
    })
    .rpc();

  console.log("Profesor agregado:", tx);
}

//////////////////////////// VER PROFESORES ////////////////////////////
// Obtiene la información almacenada en la cuenta Escuela.
//En este punto no se usa el RPC, se hace un fech directo de la escuela 
export async function verProfesores() {

  const escuelaPDA = await getEscuelaPDA();

  const cuenta = await program.account.escuela.fetch(escuelaPDA);

  console.log("Profesores:", cuenta.profesores);
}

//////////////////////////// EDITAR PROFESOR (NUEVO) ////////////////////////////
//Modifica los datos del profesor existente, localizando la cuenta 
export async function editarProfesor(
  nombre: string,
  nuevaEspecialidad: string,
  nuevaExperiencia: number
) {

  const escuelaPDA = await getEscuelaPDA();

  const tx = await program.methods
    .editarProfesor(nombre, nuevaEspecialidad, nuevaExperiencia)
    .accounts({
      owner: wallet.publicKey,
      escuela: escuelaPDA,
    })
    .rpc();

  console.log("Profesor editado:", tx);
}

//////////////////////////// ELIMINAR PROFESOR ////////////////////////////
//Elimina un profesor del sistema 
export async function eliminarProfesor(nombre: string) {

  const escuelaPDA = await getEscuelaPDA();

  const tx = await program.methods
    .eliminarProfesor(nombre)
    .accounts({
      owner: wallet.publicKey,
      escuela: escuelaPDA,
    })
    .rpc();

  console.log("Profesor eliminado:", tx);
}

//////////////////////////// ALTERNAR ESTADO ////////////////////////////
//Este permite cambiar el estado del profesor de activo e inactivo y viceversa 
export async function alternarEstado(nombre: string) {

  const escuelaPDA = await getEscuelaPDA();

  const tx = await program.methods
    .alternarEstado(nombre)
    .accounts({
      owner: wallet.publicKey,
      escuela: escuelaPDA,
    })
    .rpc();

  console.log("Estado cambiado:", tx);
}
