# Sonic Store

> A high-performance storage engine built for experimentation, exploration, and future specialization.

## Overview

Sonic Store is an ongoing project focused on building a fast, lightweight, and efficient storage core from the ground up.

At the moment, the goal is simple:

* Build a solid storage engine foundation
* Focus on performance and correctness
* Implement core storage primitives
* Explore different indexing and retrieval strategies

The long-term direction is intentionally undefined.

This project may evolve into:

* A geographical data store
* A vector database
* A time-series engine
* A search-focused storage layer
* Or something entirely different

The destination is not fixed yet.

The current focus is understanding the fundamental building blocks required for a modern storage engine and identifying gaps that existing solutions may not address efficiently.

---

## Current Status

🚧 Active Development

The project is currently in the foundational phase.

Implemented and planned work includes:

* Core storage abstractions
* Read/Write operations
* Memory management
* Indexing structures
* Persistence mechanisms
* Performance benchmarking
* Internal APIs

Many components are experimental and subject to change.

---

## Philosophy

Instead of starting with a specific product category, Sonic Store starts with the storage layer itself.

The idea is to build a flexible and performant core first, then allow real-world requirements and discovered limitations to guide the eventual direction of the project.

In other words:

> Build the engine first. Discover the vehicle later.

---

## Why?

Modern systems increasingly rely on specialized storage solutions:

* Vector databases for AI applications
* Geospatial engines for location intelligence
* Search engines for retrieval systems
* Analytical stores for large-scale processing

Rather than immediately choosing one domain, Sonic Store explores the common foundations shared across them.

---

## Roadmap

### Phase 1 — Core Foundation

* [ ] Storage engine architecture
* [ ] File management layer
* [ ] Basic indexing
* [ ] CRUD operations
* [ ] Benchmark framework

### Phase 2 — Performance

* [ ] Memory optimization
* [ ] Caching strategies
* [ ] Concurrent access
* [ ] Compression experiments

### Phase 3 — Discovery

* [ ] Evaluate real-world use cases
* [ ] Identify underserved storage problems
* [ ] Determine specialization direction

### Phase 4 — Pivot

* [ ] Geospatial storage
* [ ] Vector search
* [ ] Time-series workloads
* [ ] Search engine capabilities
* [ ] Whatever proves most valuable

---

## Project Structure

```text
sonic-store/
├── src/
├── tests/
├── benchmarks/
├── docs/
└── README.md
```

---

## Vision

Sonic Store is currently a research and engineering project.

There is no fixed product vision today.

The goal is to keep building the core, learn from the process, discover gaps in existing systems, and allow those findings to determine where the project ultimately lands.

Until then, the focus remains on performance, simplicity, and strong fundamentals.

---

## License

MIT License
