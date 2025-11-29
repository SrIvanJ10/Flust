import React, { useState, useCallback, useEffect } from 'react';
import '../styles/properties.css';
import { usePlugins } from '../hooks/usePlugins';
import PropertyField from './PropertyField';
import type { Plugin } from '../types/plugin';

interface PropertiesPanelProps {
    selectedNode: any | null;
    selectedEdge: any | null;
    nodes: any[];
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
    nodes,
    onUpdateNode,
    onUpdateEdge,
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

                        {plugin?.properties.map((property) => {
                            // Custom rendering for target_function in call-function plugin
                            if (property.name === 'target_function' && selectedNode.data.pluginId === 'call-function') {
                                const functionDefinitions = nodes.filter((n: any) => n.data.pluginId === 'function-definition');
                                return (
                                    <div key={property.name} className="property-group">
                                        <label>{property.label}:</label>
                                        <select
                                            value={editingData[property.name] || ''}
                                            onChange={(e) => {
                                                const funcName = e.target.value;
                                                const defNode = functionDefinitions.find((n: any) => n.data.function_name === funcName);
                                                const args = defNode?.data.arguments || [];

                                                // Update both target_function and arguments
                                                setEditingData((prev: any) => ({
                                                    ...prev,
                                                    [property.name]: funcName,
                                                    arguments: args
                                                }));
                                                setHasChanges(true);
                                            }}
                                            className="property-input"
                                        >
                                            <option value="">-- Seleccionar función --</option>
                                            {functionDefinitions.map((def: any) => (
                                                <option key={def.id} value={def.data.function_name}>
                                                    {def.data.function_name}
                                                </option>
                                            ))}
                                        </select>
                                    </div>
                                );
                            }

                            return (
                                <div key={property.name} className="property-group">
                                    <label>{property.label}:</label>
                                    <PropertyField
                                        property={property}
                                        value={editingData[property.name]}
                                        onChange={(value) => handleLocalChange(property.name, value)}
                                    />
                                </div>
                            );
                        })}

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

            {selectedEdge && (() => {
                const sourceNode = nodes.find((n: any) => n.id === selectedEdge.source);
                const targetNode = nodes.find((n: any) => n.id === selectedEdge.target);

                // Check if target is a call-function node
                const isCallFunction = targetNode?.data.pluginId === 'call-function';

                // Get function arguments from target node
                const functionArgs = targetNode?.data.arguments || [];

                // Extract variables from source node (simple heuristic: look for 'let' declarations in code)
                const extractVariables = (node: any): string[] => {
                    const codeProperty = node.data.code || '';
                    const matches = codeProperty.matchAll(/let\s+(mut\s+)?(\w+)/g);
                    return Array.from(matches, (m: any) => m[2]);
                };

                const availableVariables = sourceNode ? extractVariables(sourceNode) : [];

                const connectionType = selectedEdge.connectionType || selectedEdge.data?.connectionType || 'simple';
                const variableMapping = selectedEdge.variableMapping || selectedEdge.data?.variableMapping || {};

                const handleConnectionTypeChange = (newType: string) => {
                    onUpdateEdge(selectedEdge.id, {
                        ...selectedEdge.data,
                        connectionType: newType,
                        variableMapping: newType === 'function_call' ? {} : undefined,
                    });
                };

                const handleVariableMappingChange = (argName: string, varName: string) => {
                    const newMapping = { ...variableMapping, [argName]: varName };
                    onUpdateEdge(selectedEdge.id, {
                        ...selectedEdge.data,
                        connectionType,
                        variableMapping: newMapping,
                    });
                };

                // Show variable mapping if target is call-function OR if connection type is function_call
                const showVariableMapping = isCallFunction || connectionType === 'function_call';

                return (
                    <>
                        <br />
                        <h3>Propiedades de Conexión</h3>
                        <div className="property-group">
                            <label>De:</label>
                            <div className="property-value">{sourceNode?.data.label || selectedEdge.source}</div>
                        </div>
                        <div className="property-group">
                            <label>A:</label>
                            <div className="property-value">{targetNode?.data.label || selectedEdge.target}</div>
                        </div>

                        {!isCallFunction && (
                            <div className="property-group">
                                <label>Tipo de Conexión:</label>
                                <select
                                    value={connectionType}
                                    onChange={(e) => handleConnectionTypeChange(e.target.value)}
                                    className="property-input"
                                >
                                    <option value="simple">Simple (flujo de ejecución)</option>
                                    <option value="function_call">Llamada a función</option>
                                </select>
                            </div>
                        )}

                        {isCallFunction && (
                            <div className="property-group">
                                <label>Tipo de Conexión:</label>
                                <div className="property-value">Llamada a función (automático)</div>
                            </div>
                        )}

                        {showVariableMapping && functionArgs.length > 0 && (
                            <div className="property-group">
                                <label>Mapeo de Variables:</label>
                                <div className="variable-mapping">
                                    {functionArgs.map((arg: any) => (
                                        <div key={arg.name} className="mapping-item">
                                            <span className="arg-name">{arg.name}: {arg.type}</span>
                                            <span className="mapping-arrow">←</span>
                                            <select
                                                value={variableMapping[arg.name] || ''}
                                                onChange={(e) => handleVariableMappingChange(arg.name, e.target.value)}
                                                className="property-input mapping-select"
                                            >
                                                <option value="">-- Seleccionar variable --</option>
                                                {availableVariables.map(varName => (
                                                    <option key={varName} value={varName}>{varName}</option>
                                                ))}
                                                {/* Allow typing literal values */}
                                                <option value="__custom__">-- Valor literal --</option>
                                            </select>
                                            {variableMapping[arg.name] === '__custom__' && (
                                                <input
                                                    type="text"
                                                    placeholder="Ej: 42, 'texto'"
                                                    className="property-input"
                                                    style={{ marginTop: '4px' }}
                                                    onChange={(e) => handleVariableMappingChange(arg.name, e.target.value)}
                                                />
                                            )}
                                        </div>
                                    ))}
                                </div>
                            </div>
                        )}

                        {showVariableMapping && functionArgs.length === 0 && (
                            <p className="property-hint">
                                El nodo destino no define argumentos de función
                            </p>
                        )}

                        {showVariableMapping && availableVariables.length === 0 && (
                            <p className="property-hint" style={{ color: '#ff9800' }}>
                                ⚠️ No se detectaron variables en el nodo origen. Puedes usar valores literales.
                            </p>
                        )}
                    </>
                );
            })()}
        </div>
    );
};

export default PropertiesPanel;
