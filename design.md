Factory automation game with ascii/curses interface. Inspired by Factorio, Mindustry, Satisfactory, Infinifactory, and Zachtronics puzzle games.

Does the player control a character that moves around within the world, or have a RTS-like omni-present controller?

Are there hostile NPCs? factorio biters, satisfactory monsters of various sorts, mindustry enemy forces

# Entities
* entities are built mostly from constructed items
* storage can hold multiple of a single item
* pylons
  * extend build range (like zerg creep) and distribute power (like factorio power poles)
  * can be built far away and transmit resources for building slowly, but not transmit power
* directional conveyors
  * `<>^v`
  * move items in the indicated direction
  * can be placed on underground/overhead layers
  * can be configured to transition between layers (underline?)
  * automatically balance multiple inputs in 2->1 and 3->1 configuration
  * filter variant that only pulls/accepts certain items
* balancer does 4 way input/output based on neighbor directions, can do 1->1, 1->2, 1->3, 2->1, 2->2, 3->1
  * adjacent balancers work together? 2 would allow 1->5 .. 3->3 .. 5->1
* conveyor tunnel/bridge
  * `#`
  * teleports items
  * `>#>` allows N/S and E/W paths to cross without interfering
  * `>#    #>` allows crossing space in between
  * `>#^  >#>` not sure what behaviors this should produce
* pipes
  * `-|+` allows for adjacent parallel pipes that don't connect
  * carry fluids from high concentration/density/pressure to low
* computer
  * see Computation
* wires
  * `\/X`
  * carry digital signals
  * connect to entities in diagonal directions
* constructor
  * mindustry-style insertion, belts run directly into constructor, no inserter/grabber entity needed
  * i/o like a balancer, so a one-tile one-machine constructor can handle 1 2 3 ingredients and 3 2 1 output items
  * adjacent constructors work together on larger recipes, 2 can handle 1..5 ingredients and 5..1 output items, etc
  * has an orientation, visible in alt mode
  * orientation determines output item directions/order
    * necessary for blueprint rotation to work
    * overridden by filters on output conveyors

# Computation
* Entities (can?) output data when connected to wires or computers
* Entities can enable / disable / change behavior based on data in connected wire network
* A series of connected wires/computers makes a network, data is broadcast across networks
* Networks don't extend past a block of computers
  * `A\B\CCC\D\E` assume diagonals connect, `C` is computer. network 1 is A+B+C+C+C, network 2 is C+C+C+D+E.
* Signals with the same name on the same network get added together
* Signals are 16-bit signed integers
* Default signal names look like `entity.verb[.item]`
  * pipe.contains.water=100
* Computer entity
  * has 4 I/O variables configured by name and mapped to local registers
    * e.g. A=pipe.contains.water, B=conveyor.powered, ...
    * can output arbitrary signal names
    * can output `signal_name`=`register_value` or `signal_name.register_value`=`1`
  * runs short (16 lines? tis-100 is 15) programs
  * asm-like syntax
  * extra registers
  * can run one line per game tick, or N lines, or to a breakpoint
  * comments start with `#` anywhere except in a signal name where it must be preceded by a space
  * labels are followed by `:`
  * instruction set
    * `brk` breakpoint, pauses execution until next game tick
    * `add` `sub` `mul` `div`
      * arithmetic operations
      * `add A B` A+B -> A
      * `add A B C` A+B -> C
    * `and` `orr` `xor` `not`
      * like arithmetic, except `not` which takes one fewer argument
    * `cmp` sets flags without changing any registers?
    * `jmp` executes instruction at the given label next
    * conditional execution of all instructions or just jmp? (just `jle` or also `addle` `mulle` etc?)
    * `jro X` relative jump X instructions
      * `jro 0` halts the program until the computer is restarted, counts as a breakpoint
      * `jro 1` effectively a nop
    * `mov` copies value from source to destination
    * `nam`
      * changes an I/O signal name, replacing dotted segments
      * `nam 0 -1 n` will change the next to last (-1) dotted segment of the first (0) signal name to 5, e.g. a.b.c.d becomes a.b.n.c
      * `nam 2 0 n` will change the first (0) dotted segment of the third (2) signal name to 5, e.g. a.b.c.d becomes n.b.c.d
      * accept constant string argument as alternative to immediate or register numeric value?

# Items
* science? ore, metal, glass
* magic? gems, gold
* agriculture? seeds, plants

World
* 2d grid
* top down view
* ~1m squares
* layers
  * terrain
    * land, water
  * resource
    * can be mined to produce items
  * main
    * constructor, conveyor, pylon, pipe, computer, wire?
  * underground/overhead
    * conveyor, pipe, wire
  * item
    * mostly on conveyors, maybe optimize that like factorio?
  * creep
    * not a physical layer, calculated from pylon entities, visible in split view

# Interface
* ascii/curses
* playable in monochrome, color adds extra info without having to zoom or inspect
* default view is 1 character per world tile, cycling through entities/items in a tile like dwarf fortress
* split view shows the same area 2-4 times with different layer(s) on each view
* zoom out shows info about chunks (16x16?) but not individual tiles
* zoom in can use 3x3 or 5x5 characters per world tile, show more info like Factorio "alt mode", show entity+items at the same time?
