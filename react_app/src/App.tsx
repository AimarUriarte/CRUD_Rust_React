import React, { useEffect, useState } from "react";
import { Usuario } from "./types";

const API_URL = "http://127.0.0.1:3001";

function App() {
  const [usuarios, setUsuarios] = useState<Usuario[]>([]);
  const [nombre, setNombre] = useState("");
  const [edad, setEdad] = useState<number>(0);
  const [editando, setEditando] = useState<string | null>(null);

  // Cargar usuarios al iniciar
  useEffect(() => {
    listarUsuarios();
  }, []);

  const listarUsuarios = async () => {
    const res = await fetch(`${API_URL}/usuarios`);
    const data = await res.json();
    setUsuarios(data);
  };

  const agregarUsuario = async () => {
    if (!nombre.trim() || edad <= 0) return alert("Datos inválidos");
    await fetch(`${API_URL}/usuarios`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ nombre, edad }),
    });
    limpiarFormulario();
    listarUsuarios();
  };

  const eliminarUsuario = async (nombre: string) => {
    await fetch(`${API_URL}/usuarios/${nombre}`, { method: "DELETE" });
    listarUsuarios();
  };

  const iniciarEdicion = (usuario: Usuario) => {
    setEditando(usuario.nombre);
    setNombre(usuario.nombre);
    setEdad(usuario.edad);
  };

  const guardarEdicion = async () => {
    if (!editando) return;
    await fetch(`${API_URL}/usuarios/${editando}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ nombre, edad }),
    });
    limpiarFormulario();
    listarUsuarios();
  };

  const limpiarFormulario = () => {
    setEditando(null);
    setNombre("");
    setEdad(0);
  };

  return (
    <div style={{ maxWidth: 600, margin: "2rem auto", fontFamily: "sans-serif" }}>
      <h1>CRUD de Usuarios</h1>

      <div style={{ marginBottom: "1rem" }}>
        <input
          type="text"
          placeholder="Nombre"
          value={nombre}
          onChange={(e) => setNombre(e.target.value)}
          style={{ marginRight: "0.5rem" }}
        />
        <input
          type="number"
          placeholder="Edad"
          value={edad}
          onChange={(e) => setEdad(parseInt(e.target.value))}
          style={{ marginRight: "0.5rem", width: "5rem" }}
        />
        {editando ? (
          <button onClick={guardarEdicion}>Guardar</button>
        ) : (
          <button onClick={agregarUsuario}>Agregar</button>
        )}
        {editando && (
          <button onClick={limpiarFormulario} style={{ marginLeft: "0.5rem" }}>
            Cancelar
          </button>
        )}
      </div>

      <ul style={{ listStyle: "none", padding: 0 }}>
        {usuarios.map((u) => (
          <li
            key={u.nombre}
            style={{
              marginBottom: "0.5rem",
              padding: "0.5rem",
              border: "1px solid #ddd",
              borderRadius: "5px",
            }}
          >
            <b>{u.nombre}</b> ({u.edad} años)
            <button
              onClick={() => iniciarEdicion(u)}
              style={{ marginLeft: "0.5rem" }}
            >
              Editar
            </button>
            <button
              onClick={() => eliminarUsuario(u.nombre)}
              style={{ marginLeft: "0.5rem", color: "red" }}
            >
              Eliminar
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
