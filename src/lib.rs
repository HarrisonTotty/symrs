// {% include 'doc.template' %}

// {% do require('cargo_features') %}

// {% for feature in cargo_features %}
#![feature({{ feature }})]
// {% endfor %}

// {% include 'mod.template' %}
