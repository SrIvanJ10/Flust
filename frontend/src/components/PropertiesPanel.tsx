import React, { useState, useCallback, useEffect } from 'react';
import '../styles/properties.css';
import { usePlugins } from '../hooks/usePlugins';
import PropertyField from './PropertyField';
import type { Plugin } from '../types/plugin';

interface PropertiesPanelProps {
    selectedNode: any | null;
    selectedEdge: any | null;
    onUpdateNode: (id: string, data: any) => void;
    onUpdateEdge: (id: string, data: any) => void;
}

/**
 * PropertiesPanel - Displays and manages properties for selected nodes/edges
 * Uses local state for editing with explicit Save/Cancel buttons
 */
const PropertiesPanel: React.FC<PropertiesPanelProps> = ({
    selectedNode,
    selectedEdge,
    onUpdateNode,
}) => {
    const [collapsed, setCollapsed] = useState(false);
    const { plugins } = usePlugins();

    // Local editing state - this prevents input focus loss
    const [editingData, setEditingData] = useState<any>(null);
    const [hasChanges, setHasChanges] = useState(false);

    // Update local state when selectedNode changes
    useEffect(() => {
        if (selectedNode) {
            setEditingData({ ...selectedNode.data });
            setHasChanges(false);
        } else {
            setEditingData(null);
            setHasChanges(false);
        }
    }, [selectedNode?.id]); // Only reset when node ID changes

    const getPlugin = (pluginId: string): Plugin | undefined => {
        return plugins.find(p => p.id === pluginId);
    };

    // Handle local changes without updating React Flow
    const handleLocalChange = useCallback((field: string, value: any) => {
        setEditingData((prev: any) => ({ ...prev, [field]: value }));
        setHasChanges(true);
    }, []);

    // Save changes to React Flow
    const handleSave = useCallback(() => {
        if (selectedNode && editingData) {
            onUpdateNode(selectedNode.id, editingData);
            setHasChanges(false);
        }
    }, [selectedNode, editingData, onUpdateNode]);

    // Cancel changes and revert
    const handleCancel = useCallback(() => {
        if (selectedNode) {
            setEditingData({ ...selectedNode.data });
            setHasChanges(false);
        }
    }, [selectedNode]);

    return (
        <div className={`properties-panel ${collapsed ? 'collapsed' : ''}`}>
            <button
                className="properties-toggle"
                onClick={() => setCollapsed(!collapsed)}
                title={collapsed ? 'Expandir' : 'Colapsar'}
            >
                {collapsed ? '<' : '>'}
            </button>

            {!selectedNode && !selectedEdge && (
                <>
                    <br />
                    <h3>Propiedades</h3>
                    <p className="properties-empty">
                        Selecciona un bloque o conexión para editar sus propiedades
                    </p>
                </>
            )}

            {selectedNode && editingData && (() => {
                const plugin = getPlugin(selectedNode.data.pluginId || selectedNode.data.nodeType);

                return (
                    <>
                        <br />
                        <h3>Propiedades del Bloque</h3>

                        <div className="property-group">
                            <label>Nombre:</label>
                            <input
                                type="text"
                                value={editingData.label || ''}
                                onChange={(e) => handleLocalChange('label', e.target.value)}
                                className="property-input"
                            />
                        </div>

                        <div className="property-group">
                            <label>Tipo:</label>
                            <div className="property-value">{plugin?.name || selectedNode.data.nodeType || 'N/A'}</div>
                        </div>

                        {plugin?.properties.map((property) => (
                            <div key={property.name} className="property-group">
                                <label>{property.label}:</label>
                                <PropertyField
                                    property={property}
                                    value={editingData[property.name]}
                                    onChange={(value) => handleLocalChange(property.name, value)}
                                />
                            </div>
                        ))}

                        {/* Save/Cancel buttons */}
                        <div className="property-actions">
                            <button
                                className="btn-save"
                                onClick={handleSave}
                                disabled={!hasChanges}
                                title="Guardar cambios"
                            >
                                ✓ Guardar
                            </button>
                            <button
                                className="btn-cancel"
                                onClick={handleCancel}
                                disabled={!hasChanges}
                                title="Cancelar cambios"
                            >
                                ✕ Cancelar
                            </button>
                        </div>

                        {hasChanges && (
                            <p className="property-hint" style={{ color: '#ff9800', marginTop: '8px' }}>
                                Hay cambios sin guardar
                            </p>
                        )}
                    </>
                );
            })()}

            {selectedEdge && (
                <>
                    <h3>Propiedades de Conexión</h3>
                    <div className="property-group">
                        <label>De:</label>
                        <div className="property-value">{selectedEdge.source}</div>
                    </div>
                    <div className="property-group">
                        <label>A:</label>
                        <div className="property-value">{selectedEdge.target}</div>
                    </div>
                    <div className="property-group">
                        <label>Mapeo de Variables:</label>
                        <p className="property-hint">
                            (Funcionalidad de mapeo de variables próximamente)
                        </p>
                    </div>
                </>
            )}
        </div>
    );
};

export default PropertiesPanel;
