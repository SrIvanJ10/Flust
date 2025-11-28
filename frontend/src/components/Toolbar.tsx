import React from 'react';
import '../styles/toolbar.css';

interface ToolbarProps {
    onPlay: () => void;
    onSave: () => void;
    onMenu: () => void;
    onDownloadCode: () => void;
    flowName: string;
    onFlowNameChange: (name: string) => void;
}

const Toolbar: React.FC<ToolbarProps> = ({
    onPlay,
    onSave,
    onMenu,
    onDownloadCode,
    flowName,
    onFlowNameChange
}) => {
    return (
        <div className="toolbar">
            <button className="toolbar-btn" onClick={onMenu} title="Cargar flujo">
                📂
            </button>

            <div className="toolbar-project-name">
                <label>Proyecto:</label>
                <input
                    type="text"
                    value={flowName}
                    onChange={(e) => onFlowNameChange(e.target.value)}
                    className="project-name-input"
                    placeholder="nombre_proyecto"
                />
            </div>

            <button className="toolbar-btn toolbar-btn-play" onClick={onPlay} title="Compilar (Ver código)">
                ▶️ Compilar
            </button>

            <button className="toolbar-btn toolbar-btn-download" onClick={onDownloadCode} title="Descargar código Rust">
                ⬇️ Descargar .rs
            </button>

            <button className="toolbar-btn" onClick={onSave} title="Guardar flujo">
                💾 Guardar .flow
            </button>

            <div className="toolbar-title">Flust - Editor Visual Rust</div>
        </div>
    );
};

export default Toolbar;
