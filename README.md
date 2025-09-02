# CV Language Specification

**CV** - A systems programming language with immutability by default, expression-based syntax, and functional programming principles.

**File Extension**: `.cv`

## Naming Conventions

- **Types**: camelCase (`string`, `i32`, `arrayList<T>`, `result<T,E>`, `userProfile`)
- **Functions**: camelCase (`processInput`, `validateEmail`, `calculateScore`)
- **Variables**: snake_case (`user_name`, `processed_data`, `error_count`)
- **Mutability**: `@` prefix for mutable variables (`@counter`, `@buffer`)
- **References**: `&` for immutable refs, `&@` for mutable refs

## Keywords

```
and      break    else     false    fn       for      if       in       loop
not      or       patch    record   return   true     union    when
```

## Operators

### Assignment & Mutation

- `=` - Immutable binding
- `@variable = value` - Mutable assignment

### Arithmetic

- `+` `-` `*` `/` `%` - Standard arithmetic
- `+=` `-=` `*=` `/=` `%=` - Compound assignment (for mutable variables)

### Comparison

- `==` `!=` `<` `>` `<=` `>=` - Comparisons
- `and` `or` `not` - Logical operators

### Other

- `::` - Type annotation separator
- `->` - Function return type
- `..` - Range operator
- `.` - Method call / field access
- `&` - Reference operator
- `*` - Dereference operator

## Punctuation

- `;` - Statement terminator
- `:` - Pattern matching case separator
- `|` - Union variant separator
- `{}` - Block delimiters
- `[]` - Array literals
- `()` - Function calls, grouping, tuple-like data
- `<>` - Generic type parameters

## Comments

```cv
// Single line comment
/* Multi-line comment */
```

## Variable Declarations

### Immutable (default)

```cv
// With explicit type
string user_name = "john";
i32 count = 42;
arrayList<string> items = ["a", "b", "c"];

// With type inference
user_name = "john";
count = 42;
items = ["a", "b", "c"];
```

### Mutable

```cv
// With explicit type
string @buffer = "initial";
i32 @counter = 0;
arrayList<i32> @numbers = [1, 2, 3];

// With type inference
@buffer = "initial";
@counter = 0;
@numbers = [1, 2, 3];
```

### References

```cv
// Immutable references
i32 value = 42;
i32& value_ref = &value;
print(*value_ref);  // prints 42

// Mutable references to mutable data
string @buffer = "hello";
string& @buffer_ref = &@buffer;  // mutable reference variable to mutable buffer
*@buffer_ref = "world";          // modifies original buffer

// Function parameters
void processData(arrayList<i32>& data) {
    // data is an immutable reference to the original array
    print(data.length());
}

void modifyData(arrayList<i32>& @data) {
    // @data is a mutable reference variable - can modify the original
    *@data.push(42);
}
```

## Function Declarations

```cv
// Basic function
returnType functionName(paramType param_name, paramType param_name) {
    // body - final expression is returned
}

// Examples
i32 add(i32 a, i32 b) {
    a + b
}

void logMessage(string message) {
    print(message);
}

string processData(string& input, i32 @processed_count) {
    @processed_count = @processed_count + 1;
    (*input).trimWhitespace().toLowerCase()
}
```

## Control Flow

### Conditionals

```cv
// If expressions
result = if condition {
    "true_value"
} else {
    "false_value"
};

// If statements
if condition {
    doSomething();
};
```

### Pattern Matching

```cv
// When expressions
result = when value {
    pattern1: expression1;
    pattern2: expression2;
    else: default_expression;
};

// When statements
when response {
    ok(data): processData(data);
    err(msg): logError(msg);
};
```

### Loops

```cv
// For loops
for item in collection {
    processItem(item);
}

// While loops
while condition {
    doWork();
}

// Loop expressions (can return values)
result = loop {
    // ... work
    if done {
        break final_value;
    }
};
```

## Data Types

### Primitives

- `i8`, `i16`, `i32`, `i64` - Signed integers
- `u8`, `u16`, `u32`, `u64` - Unsigned integers
- `isize`, `usize` - Platform-dependent signed/unsigned integers
- `f32`, `f64` - Floating point
- `bool` - Boolean (`true`, `false`)
- `string` - String literal
- `char` - Single character

### References

- `T&` - Immutable reference to type T
- `T&` with `@variable` - Mutable reference variable (can reassign reference and modify referenced value)

### Collections

- `arrayList<T>` - Dynamic array
- `fixedArray<T, N>` - Fixed-size array
- `hashMap<K, V>` - Hash map
- `linkedList<T>` - Linked list

### Custom Types

#### Records (Structs)

```cv
record typeName {
    field_name: fieldType;
    other_field: otherType;
}

// Example
record userProfile {
    displayName: string;
    age: u8;
    emailAddress: string;
}
```

#### Unions (Enums)

```cv
union typeName = variant1 | variant2(dataType) | variant3;

// Examples
union option<T> = some(T) | none;
union result<T, E> = ok(T) | err(E);
union colorEnum = red | green | blue;
union shapeType = circle(f64) | rectangle(f64, f64);
```

## Method Extensions

Use the `patch` keyword to add methods to existing types:

```cv
patch userProfile {
    string getDisplayInfo() {
        self.displayName + " (" + self.age.toString() + ")"
    }

    bool isAdult() {
        self.age >= 18
    }
}

patch result<T, E> {
    bool isOk() {
        when self {
            ok(_): true;
            err(_): false;
        }
    }

    T unwrapOr(T default_value) {
        when self {
            ok(value): value;
            err(_): default_value;
        }
    }
}

// Usage
user = userProfile {
    displayName: "John",
    age: 25,
    emailAddress: "john@example.com"
};

info = user.getDisplayInfo();  // "John (25)"
adult = user.isAdult();        // true
```

## Reference Rules & Memory Safety

### Borrowing Rules

1. **Multiple immutable references** OR **one mutable reference** - never both
2. **References cannot outlive** the data they point to
3. **Automatic dereferencing** for method calls and field access

```cv
// Valid: multiple immutable references
i32 value = 42;
i32& ref1 = &value;
i32& ref2 = &value;
print(*ref1 + *ref2);  // OK

// Valid: one mutable reference
arrayList<i32> @numbers = [1, 2, 3];
@mut_ref = &@numbers;  // mutable reference to mutable data
*mut_ref.push(4);      // OK

// Invalid: cannot have both immutable and mutable refs
i32 @counter = 0;
i32& immut_ref = &@counter;
@mut_ref2 = &@counter;  // Compile error!

// Automatic dereferencing for convenience
record point {
    x: i32;
    y: i32;
}

point p = point { x: 10, y: 20 };
point& p_ref = &p;
print(p_ref.x);  // Automatically dereferences, no need for (*p_ref).x
```

## Method Chaining

```cv
// Chain methods together naturally
result = input
    .trimWhitespace()
    .toLowerCase()
    .replace(" ", "_")
    .addPrefix("processed_");

// Complex chaining with intermediate values
formatted_data = raw_data
    .parseJson()
    .extractField("name")
    .validateNotEmpty()
    .normalizeCase();
```

## Error Handling

```cv
// Using result<T, E> union
result<string, string> readFile(string path) {
    // ... implementation
    ok(file_contents) // or err(error_message)
}

// Method chaining with error handling
processed_result = readFile("data.txt")
    .unwrapOr("default content")
    .trimWhitespace()
    .toLowerCase();
```

## Variable Shadowing

```cv
// Clean transformation chains with shadowing
string processInput(string raw_data) {
    // Each step shadows the previous, maintaining immutability
    data = raw_data.trimWhitespace();     // shadows parameter
    data = data.toLowerCase();            // shadows previous data
    data = data.replace(" ", "_");        // shadows again
    data = data.addPrefix("processed_");  // final transformation

    data  // return final result
}
```

## Expression vs Statement Rules

### Expressions (no semicolon, return values)

- Function calls: `result = calculateValue(x, y)`
- Method chains: `result = input.transform().process()`
- Blocks: `value = { doComputation(); final_result }`
- Conditionals: `x = if condition { a } else { b }`
- Pattern matching: `y = when value { pattern: result; }`

### Statements (require semicolon)

- Variable bindings: `x = 42;`
- Function calls not used as values: `print("hello");`
- Control flow not used as expressions: `if condition { doAction(); };`

## Operator Precedence (highest to lowest)

1. Method calls, field access, array access: `.`, `[]`
2. Function calls: `f()`
3. Unary: `not`, `-`, `&`, `*`
4. Multiplication/Division: `*`, `/`, `%`
5. Addition/Subtraction: `+`, `-`
6. Comparisons: `<`, `>`, `<=`, `>=`
7. Equality: `==`, `!=`
8. Logical AND: `and`
9. Logical OR: `or`
10. Assignment: `=`

## Example CV Program

```cv
// File: userManager.cv

union result<T, E> = ok(T) | err(E);

record userProfile {
    displayName: string;
    age: u8;
    emailAddress: string;
}

patch userProfile {
    string summary() {
        self.displayName + " <" + self.emailAddress + ">, age " + self.age.toString()
    }
}

patch result<T, E> {
    T unwrapOr(T default_value) {
        when self {
            ok(value): value;
            err(_): default_value;
        }
    }
}

result<userProfile, string> createUser(string name_input, u8 age_input, string email_input) {
    // Method chaining for data cleaning
    clean_name = name_input
        .trimWhitespace()
        .toLowerCase()
        .capitalizeFirst();

    clean_email = email_input
        .trimWhitespace()
        .toLowerCase();

    // Validation with pattern matching
    when validateAge(age_input) {
        ok(valid_age): {
            when validateEmail(clean_email) {
                ok(_): {
                    new_user = userProfile {
                        displayName: clean_name,
                        age: valid_age,
                        emailAddress: clean_email
                    };
                    ok(new_user)
                };
                err(email_error): err("Invalid email: " + email_error);
            }
        };
        err(age_error): err("Invalid age: " + age_error);
    }
}

void main() {
    user_result = createUser("  john DOE  ", 25, "  JOHN@EXAMPLE.COM  ");

    when user_result {
        ok(user_data): print("Created user: " + user_data.summary());
        err(error_msg): print("Error: " + error_msg);
    };
}
```

---

**CV Language** - Control Voltage for Code: Patch together immutable transformations with expressive, functional programming.
