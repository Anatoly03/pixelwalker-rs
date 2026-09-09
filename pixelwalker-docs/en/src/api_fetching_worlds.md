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
