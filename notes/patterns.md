RAII means that some resouce (memory, net socket, file handle) is acquired when an object is initialized and
released when an object is deinitialized. Simply saying, objects manage their "heavy" resources.
An initializer is not responsible for acquisition, but an object itself.

```rust
fn process_multiple_items() {
    // 1. We create a span only ONCE
    let processing_span = info_span!("Processing batch");

    for item_id in 0..3 {
        // 2. We enter the span
        let _item_guard = processing_span.enter();

        info!(item_id, "Working on an item");

        // 3. Here `_item_guard` is dead. We're out of the span.
        //    But `processing_span` is still alive and can be REUSEED
    }

    let _final_guard = processing_span.enter();
    info!("Finished processing batch.");
    // Here `_final_guard` is dead

} // The span is also dead
```rust
