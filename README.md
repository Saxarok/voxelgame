# unnamed voxel game
## Build Instructions
Setup a rust toolchain

## Temporary todo list:
* Come up with a nice shader/uniform abstraction
* Refactor code to use a reference to `queue` instead of `Rc`
* Think of a proc derive macro for `Bindable`
* Introduce a concept of a resource store
* Specify all `Bindable`s in a pipeline and automatically bind them
* Decouple rendering code from data

## Special Thanks
* [Vilkillian](https://github.com/orgs/OpenGames/people/ZecosMAX) - helped with UV coordinates