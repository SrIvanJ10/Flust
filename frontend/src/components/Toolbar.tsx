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
            <button className="toolbar-btn" onClick={onMenu} title="Load .flow">
                Load .flow
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

            <div className="toolbar-title">Flust - Rust Visual Editor</div>

            <button className="toolbar-btn toolbar-btn-play" onClick={onPlay} title="Compile and run">
                Compile and run
            </button>

            <button className="toolbar-btn toolbar-btn-download" onClick={onDownloadCode} title="Download .rs">
                Download rust code
            </button>

            <button className="toolbar-btn" onClick={onSave} title="Save .flow">
                Download flow
            </button>
        </div>
    );
};

export default Toolbar;
