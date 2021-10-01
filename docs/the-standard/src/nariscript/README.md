# NariScript

NariScript is the dedicated Domain Specific Language for Nari2D. It is meant to be directly embeddable into
the Nari2D Runtime and enable users to enable very powerful ways of manipulating their models from code if they encounter
limitations with their editor. 

Requirements:
- Embeddable
- Small Runtime
- Resonably Fast
- Simple
- Able to be animated

Thus was created NariScript, a way to create scripts within Nari to manipulate inputs.

Heres an example syntax:
```
import other_fn

fn square_fn(input: Float) -> Float {
    // Do some calculations
    let calculation = input + 2
    // call some other function
    calculation = other_fn calculation
    print "aaaa"
    if calculation % 2 == 0 {
        print "even"
    }
    return calculation
}
```
