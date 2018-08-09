#!/bin/sh
#
# Author Radoslaw Cieciwa <radekcieciwa@gmail.com>
#
# Fetching types from file.
# Converting to plantUML format.
#

FILE=$1
METHODS_INPUT_TYPES_QUERY='[recurse(.["key.substructure"][]?) | select(."key.kind" == "source.lang.swift.decl.function.method.instance") | select(."key.name" | tostring | startswith("init(") | not) | ."key.substructure"[] | select(."key.kind" == "source.lang.swift.decl.var.parameter") | ."key.typename"] | unique'
INITIALIZERS_INPUT_TYPES_QUERY='[recurse(.["key.substructure"][]?) | select(."key.kind" == "source.lang.swift.decl.function.method.instance") | select(."key.name" | tostring | startswith("init(")) | ."key.substructure"[] | select(."key.kind" == "source.lang.swift.decl.var.parameter") | ."key.typename"]| unique'

STRUCTURE_JSON=`sourcekitten structure --file "$FILE"`
METHODS_TYPES=`echo $STRUCTURE_JSON | jq -c "$METHODS_INPUT_TYPES_QUERY"`
INITIALIZER_TYPES=`echo $STRUCTURE_JSON | jq -c "$INITIALIZERS_INPUT_TYPES_QUERY"`

FINAL=`echo $STRUCTURE_JSON | jq -c "{ methods: $METHODS_INPUT_TYPES_QUERY, initilizers: $INITIALIZERS_INPUT_TYPES_QUERY }" | sed 's/ /_/g'`
echo $INITIALIZER_TYPES

# python ./class_grabber_converter.py $FINAL
