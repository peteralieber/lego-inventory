🧱 Lego Organization Architecture White Paper

Purpose

This document outlines a modular system for organizing Lego parts by color and type, optimized for digital implementation in a sorting and inventory app. It includes:

A taxonomy of Lego categories

A logic model for collapsing categories when unnecessary

An isometric storage layout for visual reference

📦 Category Taxonomy

Lego parts are grouped into functional and visual categories. These are organized by Color or Part Type, with optional collapsing when a category is irrelevant for a given color or when the number of parts in the collection do not warrant the full separation.

🔹 By Color

Each color may include a subset of the following categories:

Basic: Standard bricks and plates

Curved: Arcs, circles, and rounded elements

Cutoff: Wedges and angular cut pieces

Slopes: Inclined bricks, including studless variants

Tile+Jump: Flat tiles and jumper plates

Textured: Bricks with surface texture or holes

Clips & Bars: Hinges, clips, rods, and bars

Modified: Unique or altered shapes not fitting other categories

Technic: Gears, axles, pins, and chain-compatible parts

Car: Wheels, fenders, chassis components

Items: Accessories, tools, and miscellaneous objects

Minifig: Figures, dolls, and related gear

Organic: Plants, horns, rocks, and natural shapes

Clear: Transparent or translucent parts

Tracks: Train tracks and rail-compatible pieces

Pillar: Structural supports and columns

Large: Oversized bricks, baseplates, and specialty elements

🔸 By Part Type

 these parts are first separated by part type, and then within each type, separated by color:

Category

Description

Dots

1×1 round tiles

Dotst

Dot-shaped decorative elements

Stud

1×1 round plates

1×1

1×1 square plates

1×1 Tile

1×1 flat tiles

1×1 Slope

Small angled bricks

x2 Slope

1×2 slopes, studless or smooth

SNOT

Studs Not On Top bricks (side-stud parts)

🔄 Category Collapse Logic

When a color lacks parts in a given category, the app should collapse or merge categories to reduce clutter. The following diagram illustrates this logic:

graph TD
    A[Color Category] --> B[Basic]
    A --> C[Curved]
    A --> D[Cutoff]
    A --> E[Slopes]
    A --> F[Tile+Jump]
    A --> G[Textured]
    A --> H[Clips & Bars]
    A --> I[Modified]
    A --> J[Technic]
    A --> K[Car]
    A --> L[Items]
    A --> M[Minifig]
    A --> N[Organic]
    A --> O[Clear]
    A --> P[Tracks]
    A --> Q[Pillar]
    A --> R[Large]

    subgraph Collapse Rules
        S[If category is empty → collapse]
        T[Merge with nearest visual/functional neighbor]
        U[Fallback to Basic or Modified]
    end

    S --> T --> U

Example: If “Slopes” is empty for “Dark Tan”, merge with “Cutoff” or “Basic”.

📐 Isometric Storage Illustration

Below is a conceptual layout for physical or digital storage bins, arranged by color and collapsed categories:

┌────────────┬────────────┬────────────┐
│  Black     │  White     │  Red       │
│ ────────── │ ────────── │ ────────── │
│ Basic      │ Basic      │ Basic      │
│ Slopes     │ Slopes     │ Curved     │
│ Clips/Bars │ Modified   │ Car        │
│ Technic    │ Minifig    │ Organic    │
└────────────┴────────────┴────────────┘

┌────────────┬────────────┬────────────┐
│  Green     │  Blue      │  Clear     │
│ ────────── │ ────────── │ ────────── │
│ Basic      │ Basic      │ Basic      │
│ Organic    │ Car        │ Tile+Jump  │
│ Tracks     │ Technic    │ Modified   │
│ Large      │ Minifig    │ Transparent│
└────────────┴────────────┴────────────┘

Each bin represents a color. Rows represent collapsed categories based on presence. This layout can be used for:

UI grid design

Physical drawer labeling

Inventory database schema

🧠 Implementation Notes for Agents

Use dynamic category generation per color

Allow manual override for merging rules

Support tagging parts with multiple categories

Enable visual previews for each bin

Consider ML-based part recognition for auto-sorting

Let me know if you'd like this converted into a visual diagram or if you want to prototype the app logic next!