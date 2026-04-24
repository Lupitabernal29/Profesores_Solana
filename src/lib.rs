///Programa Géstion de Escuela Solana (CRUD de profesores)
//Este programa, desarrollado sobre la blockchain de Solana utilizando el
/// framework Anchor, tiene como propósito gestionar la información de una
/// escuela y su conjunto de profesores mediante operaciones tipo CRUD
/// (Crear, Leer, Actualizar y Eliminar).

use anchor_lang::prelude::*; //Esta línea de codigo tiene como funcionalidad
//importar el preludio del framework  Anchor

declare_id!("");

#[program]
pub mod escuela {
    use super::*;

    //////////////////////////// Crear Escuela /////////////////////////////////////
    //Esta funcion inicializa una nueva cuenta determinada "Escuela " en la blockchain.
    //Se ejecuta una sola vez en owner y establece los datos iniciales.
    pub fn crear_escuela(context: Context<NuevaEscuela>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();// Esta linea obtiene clave pública del 
        //usuario que afirma la transaccion.
        msg!("Owner id: {}", owner_id);

        let profesores: Vec<Profesor> = Vec::new(); // Permite inicializar el vector de Profesores
        //vacio

        context.accounts.escuela.set_inner(Escuela { //Se asignan los valores iniciales
        //a la  cuenta Escuela 
            owner: owner_id,// identifica el propietario
            nombre,///Nombre de la escuela proporcionado 
            profesores,//lista vacia 
        });

        Ok(())
    }

    //////////////////////////// Agregar Profesor /////////////////////////////////////
    //FUNCIÓN: agregar_profesor (CREATE dentro de la escuela)
    /// Permite agregar un nuevo profesor al vector dentro de la cuenta Escuela.
    /// Es importante mencionar que solo el owner puede ejecutar esta acción.
    pub fn agregar_profesor(
        context: Context<NuevoProfesor>,
        nombre: String,
        especialidad: String,
        experiencia: u16
    ) -> Result<()> {
//Esta validacion es de seguridad: solo el owner puede modificar la escuela 
        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );
        //Se crea un anueva instancia, es decir agrega un nuevo profesor 
        let profesor = Profesor {
            nombre,
            especialidad,
            experiencia,
            activo: true, //esta linea indica que por defecto el profesor inicia activo
        };

        context.accounts.escuela.profesores.push(profesor);

        Ok(())
    }

    //////////////////////////// EDITAR PROFESOR (NUEVO) /////////////////////////////////////
    //Esta función permite modificar la especialidad y experiencia de un profesor existente 
    //la búsqueda se realiza por el nombre, es importante mencionar que si  no se encuentra 
    //tal búsqueda se lanza en error, por eso lo cual es importante verificar la correcta escritura 
    //del nombre.
  pub fn editar_profesor(
        context: Context<NuevoProfesor>,
        nombre: String,
        nueva_especialidad: String,
        nueva_experiencia: u16
    ) -> Result<()> {
        //Validación de acceso 
        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );
      //Referencia  mutable para modificar los datos
        let profesores = &mut context.accounts.escuela.profesores;

        msg!("Buscando profesor: {}", nombre);
//Permite recorrer cada profesor 
        for profesor in profesores.iter_mut() { //Comparación con el nombre 

            if profesor.nombre == nombre { /realiza la comparación con el nombre 
 //Se actualiza unicamente los campos necesarios (especialidad y experiencia)

                profesor.especialidad = nueva_especialidad.clone();
                profesor.experiencia = nueva_experiencia;

                msg!("Profesor actualizado correctamente");

                return Ok(());
            }
        }
//Caso en el que no se encuentre el profesor se genera un error 
        msg!("Profesor no encontrado");
        Err(Errores::ProfesorNoExiste.into())
}

    //////////////////////////// Eliminar Profesor /////////////////////////////////////
    ////Esta función Elimina completamnete a un profesor del sistema , se utiliza una eliminacion
    //física del sistema, esto cambia lo indices internos del arreglo
    pub fn eliminar_profesor(context: Context<NuevoProfesor>, nombre: String) -> Result<()> {
    //validacion de acceso

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let profesores = &mut context.accounts.escuela.profesores;
    //Se recorre usando indices para poder eleiminar 
        for i in 0..profesores.len() {
            if profesores[i].nombre == nombre {
                profesores.remove(i);
                msg!("Profesor {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::ProfesorNoExiste.into())
    }

    //////////////////////////// Ver Profesores /////////////////////////////////////
    //Permite visualizar todos los profesores registrados, como dato importante:
    //solana no retorna listas directamente una logs (msg) para inspeccionar
    pub fn ver_profesores(context: Context<NuevoProfesor>) -> Result<()> {

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de profesores: {:#?}", context.accounts.escuela.profesores);

        Ok(())
    }

    //////////////////////////// Alternar Estado /////////////////////////////////////
    //Esta funcion de alternar estado es adicional, y cambia el estado de un profesor 
    //sin eliminarlo es decir es un soft delete que lleva del control de dispobilidad 
    //Activo=true
    //Inactivo=false 
    pub fn alternar_estado(context: Context<NuevoProfesor>, nombre: String) -> Result<()> {

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let profesores = &mut context.accounts.escuela.profesores;

        for i in 0..profesores.len() {
            if profesores[i].nombre == nombre {
//Se invierte el valor actual 
                profesores[i].activo = !profesores[i].activo;

                msg!(
                    "El profesor {} ahora tiene estado activo: {}",
                    nombre,
                    profesores[i].activo
                );

                return Ok(());
            }
        }

        Err(Errores::ProfesorNoExiste.into())
    }
}

//////////////////////////// ERRORES /////////////////////////////////////
#[error_code]
pub enum Errores {
    #[msg("No eres el propietario")]
    NoEresElOwner,

    #[msg("El profesor no existe")]
    ProfesorNoExiste,
}

//////////////////////////// CUENTA PRINCIPAL /////////////////////////////////////
#[account]
#[derive(InitSpace)]
pub struct Escuela {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    profesores: Vec<Profesor>,
}

//////////////////////////// STRUCT PROFESOR /////////////////////////////////////
//Modelo que representa a cada profesor dentro de la escuela 
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Profesor {
    #[max_len(60)]
    nombre: String,

    #[max_len(60)]
    especialidad: String,

    experiencia: u16,

    activo: bool,
}

//////////////////////////// CONTEXTOS /////////////////////////////////////
//Define las cuentas necesarias para crear una escuela, incluye la inicializacion con PDA
#[derive(Accounts)]
pub struct NuevaEscuela<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Escuela::INIT_SPACE + 8,
        seeds = [b"escuela", owner.key().as_ref()],
        bump
    )]
    pub escuela: Account<'info, Escuela>,

    pub system_program: Program<'info, System>,
}
//Define las cuentas necesarias para interactuar con la escuela existente
//se utiliza para todas las operaciones del CRUD 
#[derive(Accounts)]
pub struct NuevoProfesor<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub escuela: Account<'info, Escuela>,
}
