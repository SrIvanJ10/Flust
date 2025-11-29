import React, { memo } from 'react';
import type { PluginProperty } from '../types/plugin';

interface PropertyFieldProps {
    property: PluginProperty;
    value: any;
    onChange: (value: any) => void;
}

/**
 * PropertyField component - Renders appropriate input field based on property type
 * Follows Single Responsibility Principle by only handling property field rendering
 * Memoized to prevent unnecessary re-renders that cause focus loss
 */
const PropertyField: React.FC<PropertyFieldProps> = ({ property, value, onChange }) => {
    // Code or multiline text
    if (property.type === 'code' || property.multiline) {
        return (
            <textarea
                value={value ?? property.default ?? ''}
                onChange={(e) => onChange(e.target.value)}
                className="property-textarea"
                rows={10}
                placeholder={String(property.default)}
            />
        );
    }

    // Number input
    if (property.type === 'number') {
        return (
            <input
                type="number"
                value={value ?? property.default ?? ''}
                onChange={(e) => onChange(Number(e.target.value))}
                className="property-input"
                placeholder={String(property.default)}
            />
        );
    }

    // Boolean checkbox
    if (property.type === 'boolean') {
        return (
            <input
                type="checkbox"
                checked={value ?? property.default ?? false}
                onChange={(e) => onChange(e.target.checked)}
                className="property-checkbox"
            />
        );
    }

    // Arguments list for function definitions
    if (property.type === 'arguments') {
        const args = value ?? property.default ?? [];

        const handleAddArgument = () => {
            onChange([...args, { name: '', type: '' }]);
        };

        const handleRemoveArgument = (index: number) => {
            onChange(args.filter((_: any, i: number) => i !== index));
        };

        const handleUpdateArgument = (index: number, field: 'name' | 'type', newValue: string) => {
            const updated = [...args];
            updated[index] = { ...updated[index], [field]: newValue };
            onChange(updated);
        };

        return (
            <div className="arguments-list">
                {args.map((arg: any, index: number) => (
                    <div key={index} className="argument-item">
                        <input
                            type="text"
                            value={arg.name || ''}
                            onChange={(e) => handleUpdateArgument(index, 'name', e.target.value)}
                            placeholder="nombre"
                            className="property-input argument-name"
                        />
                        <span>:</span>
                        <input
                            type="text"
                            value={arg.type || ''}
                            onChange={(e) => handleUpdateArgument(index, 'type', e.target.value)}
                            placeholder="tipo (ej: i32)"
                            className="property-input argument-type"
                        />
                        <button
                            onClick={() => handleRemoveArgument(index)}
                            className="btn-remove-arg"
                            title="Eliminar argumento"
                        >
                            ✕
                        </button>
                    </div>
                ))}
                <button onClick={handleAddArgument} className="btn-add-arg">
                    + Añadir argumento
                </button>
            </div>
        );
    }

    // Default text input
    return (
        <input
            type="text"
            value={value ?? property.default ?? ''}
            onChange={(e) => onChange(e.target.value)}
            className="property-input"
            placeholder={String(property.default)}
        />
    );
};

// Memoize the component to prevent re-renders when props haven't changed
export default memo(PropertyField);
