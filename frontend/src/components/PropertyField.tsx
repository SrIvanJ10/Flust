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
