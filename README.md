## File Pattern
File names must implement the following pattern:
```
<sheet_name>.<animation_name>.<frame>.(png|jpg|...)
```

## Usage
Generating a root sprite sheet:
```
cli root <sheet_name>
```
Rendering a sprite sheet:
```
cli render <sheet_name>
# Specify an output name
cli render <sheet_name> -o a_sprite_sheet.png
# Specify scale
cli render <sheet_name> -h 512px
# If specifying h and w, how would we handle a change in ratio
```

Rendering a sprite sheet from an overlay:
```
cli render <overlay_name> --overlay <sheet_name>
```
