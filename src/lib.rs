use anchor_lang::prelude::*;

declare_id!("5jdZrShR18Eh93DW4zF5VgkV6Y3v6VPapMAESv6BBsDb");

#[program]
pub mod escuela {
    use super::*;

    //////////////////////////// Crear Escuela /////////////////////////////////////
    pub fn crear_escuela(context: Context<NuevaEscuela>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let profesores: Vec<Profesor> = Vec::new();

        context.accounts.escuela.set_inner(Escuela {
            owner: owner_id,
            nombre,
            profesores,
        });

        Ok(())
    }

    //////////////////////////// Agregar Profesor /////////////////////////////////////
    pub fn agregar_profesor(
        context: Context<NuevoProfesor>,
        nombre: String,
        especialidad: String,
        experiencia: u16
    ) -> Result<()> {

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let profesor = Profesor {
            nombre,
            especialidad,
            experiencia,
            activo: true,
        };

        context.accounts.escuela.profesores.push(profesor);

        Ok(())
    }

    //////////////////////////// EDITAR PROFESOR (NUEVO) /////////////////////////////////////
  pub fn editar_profesor(
        context: Context<NuevoProfesor>,
        nombre: String,
        nueva_especialidad: String,
        nueva_experiencia: u16
    ) -> Result<()> {

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let profesores = &mut context.accounts.escuela.profesores;

        msg!("Buscando profesor: {}", nombre);

        for profesor in profesores.iter_mut() {

            if profesor.nombre == nombre {

                profesor.especialidad = nueva_especialidad.clone();
                profesor.experiencia = nueva_experiencia;

                msg!("Profesor actualizado correctamente");

                return Ok(());
            }
        }

        msg!("Profesor no encontrado");
        Err(Errores::ProfesorNoExiste.into())
}

    //////////////////////////// Eliminar Profesor /////////////////////////////////////
    pub fn eliminar_profesor(context: Context<NuevoProfesor>, nombre: String) -> Result<()> {

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let profesores = &mut context.accounts.escuela.profesores;

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
    pub fn ver_profesores(context: Context<NuevoProfesor>) -> Result<()> {

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de profesores: {:#?}", context.accounts.escuela.profesores);

        Ok(())
    }

    //////////////////////////// Alternar Estado /////////////////////////////////////
    pub fn alternar_estado(context: Context<NuevoProfesor>, nombre: String) -> Result<()> {

        require!(
            context.accounts.escuela.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let profesores = &mut context.accounts.escuela.profesores;

        for i in 0..profesores.len() {
            if profesores[i].nombre == nombre {

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

#[derive(Accounts)]
pub struct NuevoProfesor<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub escuela: Account<'info, Escuela>,
}