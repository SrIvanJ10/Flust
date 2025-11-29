import React from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import '../styles/customnode.css';

const nodeIcons: Record<string, string> = {
    input: '📥',
    output: '📤',
    process: '⚙️',
    decision: '🔀',
};

interface CustomNodeProps {
    id: string;
    data: any;
}

const CustomNode: React.FC<CustomNodeProps> = ({ id, data }) => {
    const handleDelete = () => {
        if (data.onDelete) {
            data.onDelete(id);
        }
    };

    const nodeType = data.nodeType || 'input';
    const icon = nodeIcons[nodeType] || '';
    const isContainer = data.pluginId === 'function-definition';

    if (isContainer) {
        return (
            <div className="custom-node container-node" style={{ width: '100%', height: '100%' }}>
                <NodeResizer
                    minWidth={400}
                    minHeight={300}
                    isVisible={true}
                    lineClassName="resize-line"
                    handleClassName="resize-handle"
                />
                <div className="container-header">
                    <div className="node-icon">{data.icon || '📦'}</div>
                    <div className="node-label">{data.label}</div>
                    <button className="node-settings" onClick={handleDelete} title="Eliminar">
                        x
                    </button>
                </div>
                <div className="container-body">
                    {/* Content area for dropping nodes */}
                </div>
            </div>
        );
    }

    const isStartNode = data.pluginId === 'start-node';

    return (
        <div className="custom-node">
            {!isStartNode && <Handle type="target" position={Position.Left} className="handle-input" />}

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
