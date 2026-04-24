import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

// Configuración base
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Escuela as Program;

const wallet = provider.wallet;

// 🔑 Obtener PDA de la escuela
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
export async function verProfesores() {

  const escuelaPDA = await getEscuelaPDA();

  const cuenta = await program.account.escuela.fetch(escuelaPDA);

  console.log("Profesores:", cuenta.profesores);
}

//////////////////////////// EDITAR PROFESOR (NUEVO) ////////////////////////////
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