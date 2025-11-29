{{#if label}}
println!("{{label}}: {:?}", {{variable}});
{{else}}
println!("{:?}", {{variable}});
{{/if}}
