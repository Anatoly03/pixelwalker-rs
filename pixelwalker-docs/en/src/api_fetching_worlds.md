# Fetching Worlds

When you are logged in, you can communicate with the database on the API server. For example, you can answer questions like: How many registered users are there? Who has the most wooted worlds? Or you can fetch the list of all world titles and descriptions and then analyse the most frequent word yourself.

The client exposes a `collection::<T>()` method which creates a records query builder for a collection. `T` has the implement the trait `PWCollection`.

### Total Worlds

```rust
// don't forget: `use pixelwalker::api::World;`
let client = Client::new().auth_with_email_password()?;
let total_worlds = client.collection::<World>().len()?
```

To get the total items in a collection the `len()` method is provided on the query builder.

### Most Wooted Worlds

```rust
let client = Client::new().auth_with_email_password()?;

for (idx, world) in client
    .collection::<World>()
    .sort("-woots")
    .sort("-plays")
    .take(10)?
    .iter()
    .enumerate()
{
    println!("{}. ({} woots) \t{}", idx + 1, world.woots, world.title);
}
```

To get the TOP-10 most wooted worlds we sort the world by woots in descending order and take the first 10 elements.

The `sort()` method takes a string of the sort configuration, a field and optionally preceeding the sorting order. The default is ascending order. To sort by woots in descending order, we prefix the sort configuration with a minus.

Since we are only interested in the first ten results, we can invoke the `take()` method which will return us a vector of at most 10 worlds.

### Most Played Worlds

Below is a table generated on Wednesday, 9th September 2026. The code is equivalent.

|         | Plays | Title                          | Owner                |
|:-------:|:-----:|:-------------------------------|:---------------------|
|  **1.** |  5451 | [REALMS] * Lobby & Hub  *      |               MARTEN |
|  **2.** |  1545 | vhisarion wip                  |                 JOEY |
|  **3.** |  1142 | Update World                   |              PRIDDLE |
|  **4.** |  1105 | Absent Pain                    |            THEEGGLET |
|  **5.** |   808 | bfgenthrtghb                   |                 FISH |
|  **6.** |   703 | Trials of Everybody            |               LICTOR |
|  **7.** |   681 | Statsu 418                     |              ANATOLY |
|  **8.** |   579 | Hora de foc                    |             EDILIGHT |
|  **9.** |   549 | World of pixel art             |               ALIYRA |
| **10.** |   535 | ~Natures Haven~                |               ASRIEL |
| **11.** |   530 | 256 demo worlds                |     THEREAL265993303 |
| **12.** |   515 | Salad ruins                    |                 FISH |
| **13.** |   505 | Tea Land 3                     |            TURTLECAT |
| **14.** |   503 | The Core                       |                KALEB |
| **15.** |   451 | Diminishing Free Edit 2        |              THEBIGH |
| **16.** |   449 | Basic Block Cavern             |            THEEGGLET |
| **17.** |   440 | [OFF] Treasure Hunt            |               MARTEN |
| **18.** |   425 | The Line [Maintenance]         |              ANATOLY |
| **19.** |   412 | Half-Pie Quest                 |                  BOO |
| **20.** |   411 | CLIMB : : KALNS                |                 OSHN |
