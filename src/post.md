Implementing a Text-File Parser with nom

This a an overview of what I'm doing.

Nom is confusing, because it's full of macros. I saw a RustLatam talk that said macros are improving, so I'm hoping it gets better.
Also, some of the names of macros changed a bit between versions.

But parser combinators are user-friendly; parsing is a hard problem; there's no right answer like ISO 8609 or serializable mode.

I had seen parser combinators in a functional programming class once, but since mostly used PEG grammars. There'se a million different
ways to define parsers, and eveyrbody has their favorite. Rust seems to only have nom (or only talk about nom).

The one thing that I like about nom/PC, is how testable they are! Combined with Rusts' inline testing it makes it really easy to build up.

So, what we'll be parsing are TSPLIB problem definitions, which mostly look like this:
```
NAME: berlin52
TYPE: TSP
COMMENT: 52 locations in Berlin (Groetschel)
DIMENSION: 52
EDGE_WEIGHT_TYPE: EUC_2D
NODE_COORD_SECTION
1 565.0 575.0
2 25.0 185.0
3 345.0 750.0
4 945.0 685.0
5 845.0 655.0
6 880.0 660.0
7 25.0 230.0
... (edges omitted for brevity)
EOF
```

The program I was copying used regexes to get the DIMENSION, and then the edge x/ys, but adapting the regexes to rust and the way the source
program was fragile (rustc was complaining) I decided to just write a small parser.

There was a big learning curve to get onto nom (which is why I'm writing this). It has this 3-valued Result type, (with an extra Incomplete option for the rest of the stream).

you define a parser with
Here is a simple pares
```
named!(name<&str, &str>, 
tag_s!("NAME")
);
```
Will make a function called `name`, which will return the atom "NAME".
The type signature is <input_type (which is string), your function's output type>.
`tag_s!` will be your building block. (The non _s function `tag` takes bytes)

Ok, but what if I don't care about the `NAME` token, but the actual string that comes after?
```
named!(name<&str, &str>, 
preceded!("NAME:")
);
```
Gets you whatever comes after `NAME:`. 
But wait, I think there's optional whitespace after the colon! Or at least I want to support it.
This gets us to our next bread&butter function, `do_parse!` (used to be called chain! I think), which lets you chain things, and also map your result to a type.

```
named!(name<&str,&str>,
   do_parse!(
        tag_s!("NAME:) >>
        space0 >>
        value: rest >>
        (value)
	)
);
```
So with `do_parse` you can chain parsers to parse, say, a whole line, and you can give names to some of the parsers' outputs, and return it.
Here I use the `space0` parser which is 0 or more whitespace characters, and the `rest` parser, which gives you the rest of an input.  (This works on a single line)
From chaining these parsers together, we return the `value`.

Nice! But my TSPLIB format is mostly specified as `KEY: VALUE` pairs, so I'd like to make a thing that can work for any KEY and give me the VALUE.

Enter `named_args`:

```
named_args!(kv<'a>(key: &'a str)<&'a str, &'a str>,
   do_parse!(
        tag_s!(key) >>
        tag_s!(":") >>
        space0 >>
        value: rest >>
        (value)
	)
);
```
This makes a function called `kv` which takes both `input :str`, and a key:str. Unfortunately you have to start giving lifetimes here, but it's not too bad.

Now my test looks like
```
#[test]
fn name_test() {
    assert_eq!(kv("NAME: some_name45", "NAME"), Ok(("", "some_name45")))
}
```
Other parsers can call the `kv` function with the `call!` macro. 
Let's use it again!

###The TYPE line.

I like this example because the difficulty (after an abrupt jump into the nom-pool) is ramping up real slow.
Next is the `TYPE` property, which is an ENUM of 
1.1.2 TYPE : <string>
Specifies the type of the data. Possible types are
TSP Data for a symmetric traveling salesman problem
ATSP Data for an asymmetric traveling salesman problem
SOP Data for a sequential ordering problem
HCP Hamiltonian cycle problem data
CVRP Capacitated vehicle routing problem data
TOUR A collection of tours

So this is a straight up rust enum:
```
#[derive(Debug, PartialEq, Eq, EnumString)]
enum ProblemType {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR,
}
```
I wanted to have some automatically derived `FromString` function. Turns out this is called the `FromStr` trait, and you usually write it
yourself, but I found a crate called `strum` which gives us a mderive macro we can use.
So now we could do 

```
let problem_type = "TSP".parse().unwrap()
```

Let's go implement our TYPE parser!

What I like about parser combinators is that once you get them right, they're really elegant! This is what the TYPE parser looks like:

```
named!(get_type<&str, ProblemType>,
    map_res!(call!(kv, "TYPE"), str::parse)
);
``
Oh yes, this introduces another combinator, `map_res!`. It takes the output from a combinator and applies a function to it, I believe exactly like 
the regular Result.map.
And that's all we have to do!
I made this quick and dirty unit test with a vim macro:
```
#[test]
fn test_type() {
    assert_eq!(get_type("TYPE: TSP"), Ok(("", ProblemType::TSP)));
    assert_eq!(get_type("TYPE: ATSP"), Ok(("", ProblemType::ATSP)));
    assert_eq!(get_type("TYPE: SOP"), Ok(("", ProblemType::SOP)));
    assert_eq!(get_type("TYPE: HCP"), Ok(("", ProblemType::HCP)));
    assert_eq!(get_type("TYPE: CVRP"), Ok(("", ProblemType::CVRP)));
    assert_eq!(get_type("TYPE: TOUR"), Ok(("", ProblemType::TOUR)));
}
```
If this enum was to change a lot, our test should problably look different. It would probably enumerate every value in the Enum, convert it
to a string, then make sure the `get_type` parser could parse it. However, since the parser is so small, this is basically unit testing
```str::parse``` and our macro derived from_str function.

###The DIMENSION property
1.1.4 DIMENSION : <integer>
For a TSP or ATSP, the dimension is the number of its nodes. For a CVRP, it is the total
number of nodes and depots. For a TOUR file it is the dimension of the corresponding
problem.

So here we're parsing an integer. We can almost exactly copy the `get_type` parser, except we tell `str::parse` that we want an integer
back:
```
named!(get_dimension<&str, i64>,
    map_res!(call!(kv, "DIMENSION"), str::parse::<i64>)
);
#[test]
fn test_dimension() {
    let dimension = 8;
    assert_eq!(
        get_dimension(&format!("DIMENSION: {}", dimension)),
        Ok(("", dimension))
    );
}
```

And actually, since we specified the return type of our function as i64, Rust can infer what return type we want from parse, and we don't
even need to tell it!!!!:

```
named!(get_dimension<&str, i64>,
    map_res!(call!(kv, "DIMENSION"), str::parse)
);
```

It's almost like we could make our `kv` function call parse for us, and have the generics machinery make us not have to call map_res everytime.
But as a Rust noobie I'm going to stop here because I don't want to get passing around types and going higher order.
Also, I'm not sure if the Nom macros would work with a trait bound parameter.

Already, I feel like leaving off the `str::parse::<i64>` for `str::parse` is getting a little "implicit magic-ey`, and I'm going to leave the
explicit type call to make it easier to read.

####Abstracting out the tests
JK JK; I just ended up doing a bunch of trait bound type magic. I got tired of writing out the tests the exact same way, so I told myself;
surely I could write a function that calls my parser with a value, and then checks that it is correctly parsed?
I ended up with this:
```
fn test_kv<G: Display + Debug + PartialEq>(
    kvfunc: fn(&str) -> Result<(&str, G), Err<&str>>,
    key: &str,
    value: G,
) {
    assert_eq!(kvfunc(&format!("{}: {}", key, value)), Ok(("", value)));
}
```
So this takes a parser that works with KV's, then a key, and a value to try parsing with.
The Trait bounds are for

* Display -> that the type can be put into a string
* PartialEq -> Makes the values comparable
* Debug -> assert_eq! was complaining


Now I can write my tests as
```
#[test]
fn test_capacity_kv() {
    test_kv(get_capacity, "CAPACITY", 8)
}
```
Which is nicer, and leaves open the door for test_kv to maybe even generate some of the 
valid values itself, in a kind of Property Based Testing scenario (Maybe I would just add the `+ Generate` Trait bound)? Nice.

NB: It didn't work with the name_test, because somehow the lifetimes are mismatched.
A combination of my experience level, macros making this opaque, lifetimes being hard to understand, and functions being generally
hard to pass around in Rust made lose a few hours trying to fix it, but I'll probably leave it.
I think it has to do with the `kv` function making the `key` variable have the same lifetime as its results. I wasn't able to set
the key lifetime to a different lifetime 

###A first attempt at parsing the metadata into a coherent object.

The TSPLIB spec states that these metadata rows could be in any order (as long as their dependencies are fulfilled; 
eg listing out the coordinates requires knowing how many there are).

For this first stab I'll fix the order of the 

at this point I have all the pieces to put the metadata into a TSPLIB Problem object.
Here's what I've got:

```
#[derive(Debug, PartialEq, Eq, Clone)]
struct TSPLProblem {
    dimension: i64,
    coords: Vec<Coord>,
    name: String,
    comment: String,
    problem_type: ProblemType,
    edge_weight_type: EdgeWeightType,
}
```
And to parse it, I did a big `do_parse` with some `opt`'s around each line to make them all optional.
```
named!(parse_problem<&str, TSPLProblem>,
    do_parse!(
        name: opt!(get_name) >>
        ptype: opt!(get_type) >>
        comment: opt!(get_comment) >>
        dimension: opt!(get_dimension) >>
        ewt: opt!(get_edge_weight_type) >>
        (TSPLProblem {
            name:name.unwrap_or("").to_string(),
            problem_type:ptype.unwrap(),
            comment: comment.unwrap_or("").to_string(),
            dimension:dimension.unwrap(),
            edge_weight_type:ewt.unwrap_or(EdgeWeightType::EUC_2D),
            coords: Vec::new(),
        })
    )
);
```
The constructor doesn't take `Options` however, so I either set a default value, or panic'ed with `unwrap`. Will revisit later. 
I'm not sure which ones are actually required at the moment.
Then I put in an empty list of `coordinates` because they are in a different section and haven't parsed them yet.

###Fixing the KV parser
As this parser stood, it wouldn't work on multiline files. I wasn't sure how to do it when starting so I had punted on it.
I took me some fiddling (one dead-end with the take_while function), but I found how to do it in the nom tests. I modified `kv` to look like:

```
named_args!(kv<'a>(key: &'a str)<&'a str, &'a str>,
   do_parse!(
        tag_s!(key) >>
        tag_s!(":") >>
        space0 >>
        value: not_line_ending >>
        line_ending >>
        (value)
    )
);
```
I replaced the `rest` parser with a `not_line_ending` one chained to a `line_ending`, while still only returning the `value`.
Sadly this broke the easy unit tests that didn't have line endings in them. I looked at nom's solution for this, wrapping 
`line_ending` with `opt!(complete!())`, but it didn't work for me and also didn't matter much so I gave up. Nom is changing
versions to 5.0 as I write this, and the new version has a different opinion on end-of-stream characters, so I'll wait.

So now my first version of the `test_parse_problem` looked like this:
```
#[test]
fn test_parse_problem() {
    let header = "NAME: berlin52
TYPE: TSP
COMMENT: 52 locations in Berlin (Groetschel)
DIMENSION: 52
EDGE_WEIGHT_TYPE: EUC_2D
";
    let parsed = TSPLProblem {
        name: "berlin52".to_string(),
        problem_type: ProblemType::TSP,
        comment: "52 locations in Berlin (Groetschel)".to_string(),
        dimension: 52,
        edge_weight_type: EdgeWeightType::EUC_2D,
        coords: Vec::new(),
    };
    assert_eq!(parse_problem(header), Ok(("", parsed)))
}
```
And to my surprise, after the small multi-line modification, it passed immediately! Yay parser combinators and incrementally building up
confidence in bigger and bigger pieces.

Now, what's left is the "meat" of the format, to get the city list, which I had put off because it's not in the super easy `KEY: VALUE`
format.

###Getting the cities
There are a few ways of specifying TSPLIB problems. In the small samples I had seen, there would be
a line called 
```
NODE_COORD_SECTION
 1 38.24 20.42
 2 39.57 26.15
 ...
```

I was a little apprehensive when first starting, but now am ready to go!
We'll need:
* a tag_s("NODE_COORD_SECTION"), 
* a newline
* a Coord2D parser that gets the i, x, y values into a Coord
then a `many1!` combinator that gives us a list of Coords

The coord2d parser looked like this:

```
named!(get_2d_coord<&str, Coord>,
    do_parse!(
        opt!(multispace) >>
         i: digit >>
            space >>
         x: float >>
            space >>
         y: float >>
         line_ending >>
         (Coord( i.parse().unwrap(), n32(x), n32(y))))
);

#[test]
fn test_2d_coords() {
    let input = " 1 1.0 3.0\n";
    assert_eq!(get_2d_coord(input), Ok(("", Coord(1, n32(1.0), n32(3.0)))));
}
```

And the full node_coords_section like this:
```
named!(node_data_section<&str, Vec<Coord> >,
    do_parse!(
        tag_s!("NODE_COORD_SECTION") >>
        line_ending >>
        coords: many1!(get_2d_coord) >>
        opt!(complete!(tag_s!("EOF\n"))) >>
        (coords)
    )
);
```

I still haven't figured out the EOF/end of input business, but it passes the current unit test.
I use the n32 type because Rust zealously implements IEEE754 and doesn't let you compare NaNs, which are technically not a number so "incomparable",
though I've never met anyone who actually used that property.
So I use the `noisy_floats` crate that panics instead of holding NaNs, and has the `N32` (and other sizes) type that holds a non-NaN f32.

