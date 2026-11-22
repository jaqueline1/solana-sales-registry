use anchor_lang::prelude::*;

declare_id!("AJDNp5ZYCdA7XRYZKNa2v5hcdkprKybSFTD6aoD2yRPm");

#[program]
pub mod registro {
    use super::*;

    pub fn crear_registro(ctx: Context<NuevoRegistro>, nombre_negocio: String) -> Result<()> {
        require!(nombre_negocio.as_bytes().len() <= 30, Errores::TextoMuyLargo);
        ctx.accounts.registro.set_inner(Registro {
            owner: ctx.accounts.owner.key(),
            nombre_negocio: nombre_negocio.clone(),
            clientes: Vec::new(),
            id_counter: 0,
        });
        msg!("Registro {} creado", nombre_negocio);
        Ok(())
    }

    pub fn crear_cliente(ctx: Context<NuevoCliente>, nombre: String) -> Result<()> {
        require!(nombre.as_bytes().len() <= 30, Errores::TextoMuyLargo);
        let registro = &mut ctx.accounts.registro;
        require!(registro.clientes.len() < 10, Errores::CapacidadAlcanzada);
        let id = registro.id_counter.checked_add(1).ok_or(Errores::SinIds)?;
        let cliente_key = ctx.accounts.cliente.key();
        registro.id_counter = id;
        registro.clientes.push(cliente_key);
        ctx.accounts.cliente.set_inner(Cliente {
            id_cliente: id,
            nombre,
            compras: 0,
            historial: Vec::new(),
        });
        Ok(())
    }

    pub fn registrar_venta(
        ctx: Context<NuevaVenta>,
        id_cliente: u16,
        sku: String,
        descripcion: String,
        monto: u32,
    ) -> Result<()> {
        require!(monto > 0, Errores::MontoNoPositivo);
        require!(sku.as_bytes().len() <= 5 && !sku.is_empty(), Errores::TextoMuyLargo);
        require!(descripcion.as_bytes().len() <= 50, Errores::TextoMuyLargo);
        let registro = &ctx.accounts.registro;
        let cliente_key = ctx.accounts.cliente.key();
        require!(registro.clientes.contains(&cliente_key), Errores::ClienteNoPertenece);
        let cliente = &mut ctx.accounts.cliente;
        require!(cliente.id_cliente == id_cliente, Errores::ClienteNoPertenece);
        require!(cliente.historial.len() < 10, Errores::CapacidadAlcanzada);
        let compras = cliente.compras.checked_add(1).ok_or(Errores::CapacidadAlcanzada)?;
        ctx.accounts.venta.set_inner(Venta { sku, descripcion, monto });
        cliente.historial.push(ctx.accounts.venta.key());
        cliente.compras = compras;
        Ok(())
    }

    pub fn cambiar_monto(
        ctx: Context<ModificarVenta>,
        id_cliente: u16,
        sku: String,
        nuevo_monto: u32,
    ) -> Result<()> {
        let _ = sku;
        require!(nuevo_monto > 0, Errores::MontoNoPositivo);
        require!(ctx.accounts.cliente.id_cliente == id_cliente, Errores::ClienteNoPertenece);
        require!(
            ctx.accounts.cliente.historial.contains(&ctx.accounts.venta.key()),
            Errores::NoSKU
        );
        ctx.accounts.venta.monto = nuevo_monto;
        Ok(())
    }

    pub fn hacer_devolucion(
        ctx: Context<EliminarVenta>,
        id_cliente: u16,
        sku: String,
    ) -> Result<()> {
        require!(ctx.accounts.cliente.id_cliente == id_cliente, Errores::ClienteNoPertenece);
        let _ = sku;
        let venta_key = ctx.accounts.venta.key();
        let cliente = &mut ctx.accounts.cliente;
        require!(cliente.historial.contains(&venta_key), Errores::NoSKU);
        cliente.historial.retain(|&key| key != venta_key);
        cliente.compras = cliente.compras.checked_sub(1).ok_or(Errores::EstadoInvalido)?;
        Ok(())
    }

    pub fn eliminar_cliente(ctx: Context<EliminarCliente>, id_cliente: u16) -> Result<()> {
        let cliente_key = ctx.accounts.cliente.key();
        require!(ctx.accounts.cliente.id_cliente == id_cliente, Errores::ClienteNoPertenece);
        require!(
            ctx.accounts.registro.clientes.contains(&cliente_key),
            Errores::ClienteNoPertenece
        );
        require!(
            ctx.accounts.cliente.historial.is_empty() && ctx.accounts.cliente.compras == 0,
            Errores::ClienteConVentas
        );

        ctx.accounts.registro.clientes.retain(|&key| key != cliente_key);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nombre_negocio: String)]
pub struct NuevoRegistro<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Registro::INIT_SPACE,
        seeds = [b"registro", nombre_negocio.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub registro: Account<'info, Registro>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoCliente<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,
    #[account(
        init,
        payer = owner,
        space = 8 + Cliente::INIT_SPACE,
        seeds = [
            b"cliente",
            registro.key().as_ref(),
            (registro.id_counter + 1).to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub cliente: Account<'info, Cliente>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16, sku: String)]
pub struct NuevaVenta<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,
    #[account(
        mut,
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,
    #[account(
        init,
        payer = owner,
        space = 8 + Venta::INIT_SPACE,
        seeds = [b"venta", cliente.key().as_ref(), sku.as_bytes()],
        bump
    )]
    pub venta: Account<'info, Venta>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16, sku: String)]
pub struct ModificarVenta<'info> {
    pub owner: Signer<'info>,
    #[account(has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,
    #[account(
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,
    #[account(
        mut,
        seeds = [b"venta", cliente.key().as_ref(), sku.as_bytes()],
        bump
    )]
    pub venta: Account<'info, Venta>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16, sku: String)]
pub struct EliminarVenta<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,
    #[account(
        mut,
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,
    #[account(
        mut,
        close = owner,
        seeds = [b"venta", cliente.key().as_ref(), sku.as_bytes()],
        bump
    )]
    pub venta: Account<'info, Venta>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16)]
pub struct EliminarCliente<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,
    #[account(
        mut,
        close = owner,
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,
}

#[account]
#[derive(InitSpace)]
pub struct Registro {
    pub owner: Pubkey,
    #[max_len(30)]
    pub nombre_negocio: String,
    #[max_len(10)]
    pub clientes: Vec<Pubkey>,
    pub id_counter: u16,
}

#[account]
#[derive(InitSpace)]
pub struct Cliente {
    pub id_cliente: u16,
    #[max_len(30)]
    pub nombre: String,
    pub compras: u16,
    #[max_len(10)]
    pub historial: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Venta {
    #[max_len(5)]
    pub sku: String,
    #[max_len(50)]
    pub descripcion: String,
    pub monto: u32,
}

#[error_code]
pub enum Errores {
    #[msg("La wallet firmante no es la autoridad del registro")]
    NoOwner,
    #[msg("La venta no aparece en el historial del cliente")]
    NoSKU,
    #[msg("Un cliente con ventas activas no puede eliminarse")]
    ClienteConVentas,
    #[msg("La cuenta Cliente no pertenece a este registro")]
    ClienteNoPertenece,
    #[msg("El monto de venta debe ser mayor que cero")]
    MontoNoPositivo,
    #[msg("Se alcanzo la capacidad del registro")]
    CapacidadAlcanzada,
    #[msg("El valor de texto supera la capacidad prevista")]
    TextoMuyLargo,
    #[msg("Se agotaron los identificadores")]
    SinIds,
    #[msg("El estado o contador del cliente es inconsistente")]
    EstadoInvalido,
}
