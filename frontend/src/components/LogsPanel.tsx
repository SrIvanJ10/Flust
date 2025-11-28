import React, { useState, useRef, useEffect } from 'react';
import '../styles/logs.css';

interface LogsPanelProps {
    logs: string[];
    terminalOutput: string[];
    onClear: () => void;
    onClearTerminal: () => void;
}

/**
 * LogsPanel - Displays system logs and terminal output in separate tabs
 */
const LogsPanel: React.FC<LogsPanelProps> = ({ logs, terminalOutput, onClear, onClearTerminal }) => {
    const [collapsed, setCollapsed] = useState(false);
    const [activeTab, setActiveTab] = useState<'logs' | 'terminal'>('logs');
    const logsEndRef = useRef<HTMLDivElement>(null);
    const terminalEndRef = useRef<HTMLDivElement>(null);

    // Auto-scroll to bottom when new content is added
    useEffect(() => {
        if (activeTab === 'logs') {
            logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
        } else {
            terminalEndRef.current?.scrollIntoView({ behavior: 'smooth' });
        }
    }, [logs, terminalOutput, activeTab]);

    return (
        <div className={`logs-panel ${collapsed ? 'collapsed' : ''}`}>
            <div className="logs-header">
                <div className="logs-tabs">
                    <button
                        className={`logs-tab ${activeTab === 'logs' ? 'active' : ''}`}
                        onClick={() => setActiveTab('logs')}
                    >
                        📋 Logs del Sistema
                        {logs.length > 0 && <span className="tab-badge">{logs.length}</span>}
                    </button>
                    <button
                        className={`logs-tab ${activeTab === 'terminal' ? 'active' : ''}`}
                        onClick={() => setActiveTab('terminal')}
                    >
                        💻 Terminal
                        {terminalOutput.length > 0 && <span className="tab-badge">{terminalOutput.length}</span>}
                    </button>
                </div>

                <div className="logs-actions">
                    {activeTab === 'logs' ? (
                        <button className="logs-btn" onClick={onClear} title="Limpiar logs">
                            🗑️ Limpiar
                        </button>
                    ) : (
                        <button className="logs-btn" onClick={onClearTerminal} title="Limpiar terminal">
                            🗑️ Limpiar
                        </button>
                    )}
                    <button
                        className="logs-toggle"
                        onClick={() => setCollapsed(!collapsed)}
                        title={collapsed ? 'Expandir' : 'Minimizar'}
                    >
                        {collapsed ? '▲' : '▼'}
                    </button>
                </div>
            </div>

            <div className="logs-content">
                {activeTab === 'logs' ? (
                    <div className="logs-list">
                        {logs.length === 0 ? (
                            <div className="logs-empty">No hay mensajes del sistema</div>
                        ) : (
                            logs.map((log, index) => (
                                <div key={index} className="log-entry">
                                    <span className="log-time">[{new Date().toLocaleTimeString()}]</span>
                                    <span className="log-message">{log}</span>
                                </div>
                            ))
                        )}
                        <div ref={logsEndRef} />
                    </div>
                ) : (
                    <div className="terminal-output">
                        {terminalOutput.length === 0 ? (
                            <div className="terminal-empty">
                                <span className="terminal-prompt">$</span> No hay salida del programa
                            </div>
                        ) : (
                            terminalOutput.map((line, index) => (
                                <div key={index} className="terminal-line">
                                    {line}
                                </div>
                            ))
                        )}
                        <div ref={terminalEndRef} />
                        {terminalOutput.length > 0 && (
                            <div className="terminal-cursor">
                                <span className="terminal-prompt">$</span>
                                <span className="cursor-blink">_</span>
                            </div>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
};

export default LogsPanel;
