import { useState, useEffect } from 'react';
import type { Plugin } from '../types/plugin';

const PLUGIN_IDS = [
    'legacy-code',
    'debug',
];

export const usePlugins = () => {
    const [plugins, setPlugins] = useState<Plugin[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const loadPlugins = async () => {
            try {
                const loadedPlugins: Plugin[] = [];

                for (const pluginId of PLUGIN_IDS) {
                    try {
                        const response = await fetch(`/plugins/${pluginId}/plugin.json`);
                        if (response.ok) {
                            const pluginData = await response.json();

                            // Load template if exists
                            try {
                                const templateResponse = await fetch(`/plugins/${pluginId}/template.rs`);
                                if (templateResponse.ok) {
                                    pluginData.template = await templateResponse.text();
                                }
                            } catch (e) {
                                console.warn(`No template found for ${pluginId}`);
                            }

                            loadedPlugins.push(pluginData);
                        }
                    } catch (e) {
                        console.warn(`Failed to load plugin ${pluginId}:`, e);
                    }
                }

                setPlugins(loadedPlugins);
                setLoading(false);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'Failed to load plugins');
                setLoading(false);
            }
        };

        loadPlugins();
    }, []);

    return { plugins, loading, error };
};
