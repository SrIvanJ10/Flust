import { useCallback, useState, useRef, type DragEvent } from 'react';
import ReactFlow, {
  addEdge,
  Background,
  Controls,
  type Connection,
  type Node,
  type Edge,
  useNodesState,
  useEdgesState,
  ReactFlowProvider,
  type NodeTypes,
} from 'reactflow';
import 'reactflow/dist/style.css';
import './styles/app.css';
import { compileFlow } from './api';
import Sidebar from './components/Sidebar';
import Toolbar from './components/Toolbar';
import LogsPanel from './components/LogsPanel';
import CustomNode from './components/CustomNode';
import PropertiesPanel from './components/PropertiesPanel';
import { usePlugins } from './hooks/usePlugins';
import type { FlowFile } from './types/plugin';

const nodeTypes: NodeTypes = {
  custom: CustomNode,
};

let nodeId = 0;
const getId = () => `node_${nodeId++}`;

function Flow() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [logs, setLogs] = useState<string[]>([]);
  const [terminalOutput, setTerminalOutput] = useState<string[]>([]);
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [selectedEdge, setSelectedEdge] = useState<Edge | null>(null);
  const [flowName, setFlowName] = useState('my_flow');
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const [reactFlowInstance, setReactFlowInstance] = useState<any>(null);
  const { plugins } = usePlugins();

  const addLog = (message: string) => {
    setLogs((prev) => [...prev, message]);
  };

  const addTerminalOutput = (line: string) => {
    setTerminalOutput((prev) => [...prev, line]);
  };

  const clearLogs = () => {
    setLogs([]);
  };

  const clearTerminal = () => {
    setTerminalOutput([]);
  };

  const onConnect = useCallback(
    (params: Connection) => {
      setEdges((eds) => addEdge(params, eds));
      addLog('Conexión creada');
    },
    [setEdges],
  );

  const onDragOver = useCallback((event: DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'move';
  }, []);

  const onDrop = useCallback(
    (event: DragEvent) => {
      event.preventDefault();

      const pluginId = event.dataTransfer.getData('application/reactflow');
      const label = event.dataTransfer.getData('label');

      if (typeof pluginId === 'undefined' || !pluginId || !reactFlowInstance) {
        return;
      }

      const plugin = plugins.find(p => p.id === pluginId);
      if (!plugin) return;

      const position = reactFlowInstance.screenToFlowPosition({
        x: event.clientX,
        y: event.clientY,
      });

      // Initialize node data with plugin defaults
      const nodeData: any = {
        label,
        pluginId: plugin.id,
        nodeType: plugin.id, // For backward compatibility
        onDelete: handleDeleteNode,
      };

      // Set default values from plugin properties
      plugin.properties.forEach(prop => {
        nodeData[prop.name] = prop.default;
      });

      const newNode: Node = {
        id: getId(),
        type: 'custom',
        position,
        data: nodeData,
      };

      setNodes((nds) => nds.concat(newNode));
      addLog(`Bloque añadido: ${label}`);
    },
    [reactFlowInstance, setNodes, plugins],
  );

  const handleDeleteNode = (nodeId: string) => {
    setNodes((nds) => nds.filter((node) => node.id !== nodeId));
    setEdges((eds) => eds.filter((edge) => edge.source !== nodeId && edge.target !== nodeId));
    if (selectedNode?.id === nodeId) {
      setSelectedNode(null);
    }
    addLog('Bloque eliminado');
  };

  const handleNodeClick = useCallback((_event: React.MouseEvent, node: Node) => {
    setSelectedNode(node);
    setSelectedEdge(null);
  }, []);

  const handleEdgeClick = useCallback((_event: React.MouseEvent, edge: Edge) => {
    setSelectedEdge(edge);
    setSelectedNode(null);
  }, []);

  const handlePaneClick = useCallback(() => {
    setSelectedNode(null);
    setSelectedEdge(null);
  }, []);

  const handleUpdateNode = useCallback((nodeId: string, newData: any) => {
    setNodes((nds) =>
      nds.map((node) => {
        if (node.id === nodeId) {
          return { ...node, data: newData };
        }
        return node;
      })
    );
  }, [setNodes]);

  const handleUpdateEdge = useCallback((edgeId: string, newData: any) => {
    setEdges((eds) =>
      eds.map((edge) => {
        if (edge.id === edgeId) {
          return { ...edge, data: newData };
        }
        return edge;
      })
    );
  }, [setEdges]);

  const handlePlay = async () => {
    addLog('📥 Generando y ejecutando código...');
    clearTerminal();

    try {
      // First, generate the code
      const irNodes = nodes.map((n) => ({
        id: n.id,
        type: n.data.pluginId || n.data.nodeType || 'legacy_code',
        label: n.data.label,
        ...n.data,
      }));

      const irConnections = edges.map((e) => ({
        from: e.source,
        to: e.target,
      }));

      const flow = {
        nodes: irNodes,
        connections: irConnections,
      };

      const compileResult = await compileFlow(flow);
      addLog('✅ Código generado');

      // Show command being executed
      addTerminalOutput(`$ rustc ${flowName}.rs && ./${flowName}`);
      addLog('🔨 Compilando código Rust...');

      const { executeCode } = await import('./api');
      const execResult = await executeCode(compileResult.code, flowName);

      if (execResult.success) {
        addLog('✅ Compilación exitosa');
        addLog('▶️  Ejecutando programa...');

        // Show raw compilation output (if any)
        if (execResult.compile_output && execResult.compile_output !== 'Compilation successful (no warnings)') {
          execResult.compile_output.split('\n').forEach((line: string) => addTerminalOutput(line));
        }

        // Show raw execution output
        if (execResult.execution_output) {
          execResult.execution_output.split('\n').forEach((line: string) => addTerminalOutput(line));
        }

        addLog('✅ Ejecución completada');
        addTerminalOutput('$ ');
      } else {
        addLog('❌ Error en compilación');

        // Show raw compilation error
        if (execResult.compile_output) {
          execResult.compile_output.split('\n').forEach((line: string) => addTerminalOutput(line));
        }

        addTerminalOutput('$ ');
      }

    } catch (error) {
      addLog('❌ Error en ejecución');
      addTerminalOutput(String(error));
      addTerminalOutput('$ ');
      console.error('Execution failed:', error);
    }
  };

  const handleStop = () => {
    addLog('Ejecución detenida');
  };

  const handleSave = () => {
    // Save .flow.json file
    const flowFile: FlowFile = {
      version: '1.0',
      metadata: {
        name: flowName,
        created: new Date().toISOString(),
        modified: new Date().toISOString(),
      },
      nodes: nodes.map(node => ({
        id: node.id,
        pluginId: node.data.pluginId || node.data.nodeType,
        position: node.position,
        data: { ...node.data },
      })),
      edges: edges.map(edge => ({
        id: edge.id,
        source: edge.source,
        target: edge.target,
        data: edge.data,
      })),
    };

    const dataStr = JSON.stringify(flowFile, null, 2);
    const dataUri = 'data:application/json;charset=utf-8,' + encodeURIComponent(dataStr);

    const linkElement = document.createElement('a');
    linkElement.setAttribute('href', dataUri);
    linkElement.setAttribute('download', `${flowName}.flow.json`);
    linkElement.click();

    addLog(`Flujo guardado: ${flowName}.flow.json`);
  };

  const handleDownloadCode = async () => {
    addLog('Generando código Rust...');
    try {
      const irNodes = nodes.map((n) => ({
        id: n.id,
        type: n.data.pluginId || n.data.nodeType || 'legacy_code',
        label: n.data.label,
        ...n.data,
      }));

      const irConnections = edges.map((e) => ({
        from: e.source,
        to: e.target,
      }));

      const flow = {
        nodes: irNodes,
        connections: irConnections,
      };

      const result = await compileFlow(flow);

      // Download the generated Rust code
      const codeBlob = new Blob([result.code], { type: 'text/plain' });
      const codeUrl = URL.createObjectURL(codeBlob);

      const linkElement = document.createElement('a');
      linkElement.setAttribute('href', codeUrl);
      linkElement.setAttribute('download', `${flowName}.rs`);
      linkElement.click();

      URL.revokeObjectURL(codeUrl);

      addLog(`Código descargado: ${flowName}.rs`);
      addLog(`Líneas de código: ${result.code.split('\n').length}`);
    } catch (error) {
      addLog('Error al generar código');
      console.error('Code generation failed:', error);
    }
  };

  const handleLoad = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.flow.json';
    input.onchange = (e: any) => {
      const file = e.target.files[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          try {
            const flowFile: FlowFile = JSON.parse(event.target?.result as string);

            // Restore nodes
            const restoredNodes: Node[] = flowFile.nodes.map(flowNode => ({
              id: flowNode.id,
              type: 'custom',
              position: flowNode.position,
              data: {
                ...flowNode.data,
                onDelete: handleDeleteNode,
              },
            }));

            // Restore edges
            const restoredEdges: Edge[] = flowFile.edges.map(flowEdge => ({
              id: flowEdge.id,
              source: flowEdge.source,
              target: flowEdge.target,
              data: flowEdge.data,
            }));

            setNodes(restoredNodes);
            setEdges(restoredEdges);
            setFlowName(flowFile.metadata.name);
            addLog(`Flujo cargado: ${flowFile.metadata.name}`);
          } catch (error) {
            addLog('Error al cargar flujo');
            console.error('Load failed:', error);
          }
        };
        reader.readAsText(file);
      }
    };
    input.click();
  };

  const handleMenu = () => {
    handleLoad();
  };

  return (
    <div className="app-container">
      <Toolbar
        onPlay={handlePlay}
        onSave={handleSave}
        onMenu={handleMenu}
        onDownloadCode={handleDownloadCode}
        flowName={flowName}
        onFlowNameChange={setFlowName}
      />
      <div className="main-content">
        <Sidebar />
        <div className="canvas-container" ref={reactFlowWrapper}>
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            onInit={setReactFlowInstance}
            onDrop={onDrop}
            onDragOver={onDragOver}
            onNodeClick={handleNodeClick}
            onEdgeClick={handleEdgeClick}
            onPaneClick={handlePaneClick}
            nodeTypes={nodeTypes}
            fitView
          >
            <Controls />
            <Background gap={16} size={1} />
          </ReactFlow>
        </div>
        <PropertiesPanel
          selectedNode={selectedNode}
          selectedEdge={selectedEdge}
          onUpdateNode={handleUpdateNode}
          onUpdateEdge={handleUpdateEdge}
        />
      </div>
      <LogsPanel
        logs={logs}
        terminalOutput={terminalOutput}
        onClear={clearLogs}
        onClearTerminal={clearTerminal}
      />
    </div>
  );
}

export default function App() {
  return (
    <ReactFlowProvider>
      <Flow />
    </ReactFlowProvider>
  );
}
