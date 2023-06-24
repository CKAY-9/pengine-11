# Physics Engine 11
This is a physics engine based on the physics I learned in grade 11.

# Features
## Capabilities
- 2D Kinematics
- Forces
- Energy

## Engine
- Custom scripting language and parser (more later)
- Written in Rust

</br>

# PEScript
PEScript is a custom scripting language to interact with PEngine 11.

## Simple Example
Let's create a simple square with a x velocity of 3m/s
```
scripts/my_script.pes

pes {
    create obj sq1;
    modify sq1 velocity x 3;
}
```
Let's understand what we just did. We created a file called `my_script.pes` in the scripts folder of PEngine 11.
`pes {}` defines where our simulation begins; anything outside of it will be regared as a comment. We then define
a `obj` and gave it the name `sq1` (Syntax: `create [type] [name]`). 

Once we have our object created, we `modify` it's `velocity` in the `x` direction and change it to 3m/s
(Syntax: `modify [name] [component] [variable] [value]`).

To create comments within pes, we can just use `// comment`.

## Documentation

### Global
PEScript requires all statements to end in a semicolon;</br>

### Creating
To create a new thing, we use the `create` command. </br>
Syntax: `create [obj, let, const] [name] [value?]`

Names must be one word and cannot be named `simulation`

#### Example

```
pes {
    create obj myobj;
    create var myvar 5;
    create const myconst 9.81;
}
```

### Modyfing
Modify a property or variable.
Syntax: `modify [name] [component] [variable] [value]`

**YOU CANNOT MODIFY CONSTANTS**

#### Example
```
pes {
    ...

    modify myvar self value 10; // set the self value of myvar to 10
    modify myobj velocity x myvar; // set the myobj's x veolcity to 5m/s
    modify myconst self value 15; // no error, but this won't do anything...

    // we can also modify the simulation itself
    modify simulation gravity acceleration 20; // set the acceleration due to gravity to 20m/s^2
}
```

### Information
Get the data associated with a certain property
Syntax: `print [name] [component] [variable]`

#### Example
```
pes {
    ...
    print myobj velocity x; // prints "myobj.velocity.x: 10m/s" to the window
    modify myobj velocity x 5;
    print myobj velocity x; // prints "myobj.velocity.x: 5m/s" to the window
}
```

### Misc
PEScript also has some one off things you can use.

#### Example
```
the initial simulation time is 10 seconds, and maybe you want to change it.

pes {
    init simulation time 5; // runs the simulation for 5 seconds initially
    ... // do stuff
    continue simulation time 10; // run the simulation for an additional 10 seconds
    ... // do stuff

    // what if we want to stop the object at certain height, basically have a ground
    modify myobj constraint minY -30; // the displacement will only ever be allowed to be -30m in the -y direction

    modify myobj constraint maxX 50; // basically have a wall

    create obj myobj2;
    pause simulation 5; // will not continue for 5 seconds, this still simulates the already creates objects however

    halt; // stop the simulation
}
```

### Errors
If PEScript's interpretor cannot understand the script. It will exit out of the script and print an error. 

### All Together

```
this text will be ignored by PEScript
pes {
    // set gravity to -10m/s^2 instead of -9.81m/s^2
    create const custom_gravity -10;
    modify simulation gravity acceleration custom_gravity;

    create obj myCoolObject1;
    modify myCoolObject1 force x 50; // give the object 50J of force in the positive x direction
    modify myCoolObject1 acceleration y 10; // set the acceleration of the object to 10m/s^2 in the positive y direction

    print myobj acceleration y; // this should print "myobj.acceleration.y: 0m/s^2" as gravity is -10m/s^2
}
```

If you need more examples to learn from, check the `/scripts/examples` folders.
