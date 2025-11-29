import axios from 'axios';

const API_BASE_URL = 'http://localhost:3000/api';

export interface FlowIR {
    nodes: Array<{
        id: string;
        plugin_type: string;
        label?: string | null;
        properties: Record<string, any>;
    }>;
    connections: Array<{
        from: string;
        to: string;
        connection_type?: string;
        variable_mapping?: Record<string, string>;
    }>;
}

export interface CompileResponse {
    code: string;
}

export interface ExecuteResponse {
    success: boolean;
    compile_output: string;
    execution_output: string;
    error?: string;
}

export const compileFlow = async (flow: FlowIR): Promise<CompileResponse> => {
    const response = await axios.post(`${API_BASE_URL}/compile`, flow);
    return response.data;
};

export const executeCode = async (code: string, filename: string): Promise<ExecuteResponse> => {
    const response = await axios.post(`${API_BASE_URL}/execute`, {
        code,
        filename,
    });
    return response.data;
};
