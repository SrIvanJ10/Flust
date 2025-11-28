/**
 * Property definition for a plugin
 */
export interface PluginProperty {
    name: string;
    type: 'text' | 'code' | 'number' | 'boolean';
    label: string;
    default: string | number | boolean;
    required: boolean;
    multiline?: boolean;
}

/**
 * Plugin definition - describes a block type in the visual editor
 * Follows Interface Segregation Principle with focused interface
 */
export interface Plugin {
    id: string;
    name: string;
    category: string;
    icon: string;
    description: string;
    properties: PluginProperty[];
    template?: string;
}

/**
 * Flow node - represents a block instance in the canvas
 */
export interface FlowNode {
    id: string;
    pluginId: string;
    position: { x: number; y: number };
    data: Record<string, any>;
}

/**
 * Flow edge - represents a connection between blocks
 */
export interface FlowEdge {
    id: string;
    source: string;
    target: string;
    data?: Record<string, any>;
}

/**
 * Flow file format - complete flow definition for save/load
 */
export interface FlowFile {
    version: string;
    metadata: {
        name: string;
        created: string;
        modified: string;
    };
    nodes: FlowNode[];
    edges: FlowEdge[];
}
