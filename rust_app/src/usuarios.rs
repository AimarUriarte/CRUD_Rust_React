use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usuario {
    pub nombre: String,
    pub edad: u8,
}

const ARCHIVO: &str = "usuarios.json";

pub fn agregar_usuario(nombre: &str, edad: u8) -> Vec<Usuario> {
    let mut usuarios = leer_usuarios();

    usuarios.push(Usuario {
        nombre: nombre.to_string(),
        edad,
    });

    guardar_usuarios(&usuarios);
    usuarios
}

pub fn listar_usuarios() -> Vec<Usuario> {
    leer_usuarios()
}

pub fn buscar_usuario(nombre: &str) -> Option<Usuario> {
    let usuarios = leer_usuarios();
    usuarios.into_iter().find(|u| u.nombre.to_lowercase() == nombre.to_lowercase())
}

pub fn eliminar_usuario(nombre: &str) -> Vec<Usuario> {
    let mut usuarios = leer_usuarios();
    usuarios.retain(|u| u.nombre.to_lowercase() != nombre.to_lowercase());
    guardar_usuarios(&usuarios);
    usuarios
}

pub fn modificar_edad(nombre: &str, nueva_edad: u8) -> Vec<Usuario> {
    let mut usuarios = leer_usuarios();

    if let Some(u) = usuarios.iter_mut().find(|u| u.nombre.to_lowercase() == nombre.to_lowercase()) {
        u.edad = nueva_edad;
    }

    guardar_usuarios(&usuarios);
    usuarios
}

fn leer_usuarios() -> Vec<Usuario> {
    match fs::read_to_string(ARCHIVO) {
        Ok(contenido) => serde_json::from_str(&contenido).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn guardar_usuarios(usuarios: &Vec<Usuario>) {
    let json = serde_json::to_string_pretty(usuarios).expect("Error al convertir a JSON");
    fs::write(ARCHIVO, json).expect("No se pudo escribir en el archivo");
}