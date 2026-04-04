### Features
* Triangle Primitive
* Renderable Rotation
* Audio
* Animation
* UI
* Remove components
* Debug Tools (Draw colliders)
* Logging

* ~~Collision System~~
* ~~Assets Manager~~
* ~~Transform Component~~
* ~~Game Object / Entity~~
* ~~Remove entities~~
* ~~Camera~~
* ~~Component Handling~~
* ~~Mouse Input~~
* ~~Smart entity id assigning~~
* ~~Texture Rendering~~
* ~~Show FPS~~
* ~~Entity counter~~
* ~~Stress Test System~~
* ~~Velocity System~~

### Problems
* Find a way to fix the empty spaces in vectors of systems/components
* Sub-pixel distortion eg. "fat pixel"
* Components of deleted entities are not deleted
* When using texture it copies the image data multiple times
* Systems may crash if no entities with components is found
* Refactor render system and component store

* ~~Colors are not converted from 255 to 1.0 when rendered~~
* ~~Texture loader will load the same texture 100 times if it's used 100 times~~
* ~~Blurry textures~~
* ~~Fix Primitive Circle~~
* ~~Memory leak (doesn't clean old buffers)~~
* ~~Recreates buffers every frame~~