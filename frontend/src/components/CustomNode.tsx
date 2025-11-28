import React from 'react';
import { Handle, Position, type NodeProps } from 'reactflow';
import '../styles/customnode.css';

const nodeIcons: Record<string, string> = {
    input: '📥',
    output: '📤',
    add: '➕',
    subtract: '➖',
    multiply: '✖️',
    divide: '➗',
    greater_than: '>',
    less_than: '<',
    equals: '=',
};

const CustomNode: React.FC<NodeProps> = ({ data, id }) => {
    const handleDelete = () => {
        if (data.onDelete) {
            data.onDelete(id);
        }
    };

    const nodeType = data.nodeType || 'input';
    const icon = nodeIcons[nodeType] || '';

    return (
        <div className="custom-node">
            <Handle type="target" position={Position.Left} className="handle-input" />

            <div className="node-content">
                <div className="node-icon">{icon}</div>
                <div className="node-label">{data.label}</div>
                <button className="node-settings" onClick={handleDelete} title="Eliminar">
                    x
                </button>
            </div>

            <Handle type="source" position={Position.Right} className="handle-output" />
        </div>
    );
};

export default CustomNode;
