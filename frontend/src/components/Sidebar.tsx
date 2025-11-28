import React, { useState } from 'react';
import '../styles/sidebar.css';
import { usePlugins } from '../hooks/usePlugins';

const Sidebar: React.FC = () => {
    const [collapsed, setCollapsed] = useState(false);
    const { plugins, loading, error } = usePlugins();

    const onDragStart = (event: React.DragEvent, pluginId: string, pluginName: string) => {
        event.dataTransfer.setData('application/reactflow', pluginId);
        event.dataTransfer.setData('label', pluginName);
        event.dataTransfer.effectAllowed = 'move';
    };

    const categories = Array.from(new Set(plugins.map(p => p.category)));

    return (
        <div className={`sidebar ${collapsed ? 'collapsed' : ''}`}>
            <button
                className="sidebar-toggle"
                onClick={() => setCollapsed(!collapsed)}
                title={collapsed ? 'Expandir' : 'Colapsar'}
            >
                {collapsed ? '>' : '<'}
            </button>
            <h3 className="sidebar-title">Bloques</h3>

            {loading && <div className="sidebar-loading">Cargando plugins...</div>}
            {error && <div className="sidebar-error">Error: {error}</div>}

            {categories.map(category => (
                <div key={category} className="category">
                    <h4 className="category-title">{category}</h4>
                    <div className="blocks">
                        {plugins
                            .filter(plugin => plugin.category === category)
                            .map(plugin => (
                                <div
                                    key={plugin.id}
                                    className="block-item"
                                    draggable
                                    onDragStart={(e) => onDragStart(e, plugin.id, plugin.name)}
                                    title={plugin.description}
                                >
                                    <span className="block-icon">{plugin.icon}</span>
                                    <span className="block-label">{plugin.name}</span>
                                </div>
                            ))}
                    </div>
                </div>
            ))}
        </div>
    );
};

export default Sidebar;
