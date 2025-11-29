fn {{function_name}}<T: std::fmt::Debug>({{#each arguments}}{{name}}: {{type}}{{#unless @last}}, {{/unless}}{{/each}}) {
{{code}}
}
