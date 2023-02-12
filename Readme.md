# Priority-expiry-cache

## Intro

This problem is one of the famous questions asked in companies interviews,
the like of tesla, amazon, bp, etc.

[Tesla Phone Screen Cache Problem](https://medium.com/double-pointer/tesla-google-facebook-phone-screen-cache-problem-4e24f5b886f8)


I personally found this question a bit too much for an hour interview,
to go from ideation to writing a full code implementation.
This crate it's an attempt to make asking this question obsolete
and spare good candidates from a rejection for a problem which 
is more based on intuition than on algo and data structures skills.

For all of those interested in just having a good priority, expiry cache in place 
feel free to look at the documentation of this crate send prs 
and tickets at the official github repo.

All code is released under and informal
[Beerware](https://en.wikipedia.org/wiki/Beerware) licence.

## Problem Statement

The problem statement requires us to design a cache with the following methods:

- get(String key)
- set(String key, String value, int priority, int expiry)
- evictItem(int currentTime)

The rules by which the cache operates is are follows:

1. If an expired item is available. Remove it. If multiple items have the same expiry, removing any one suffices.
2. If condition #1 can’t be satisfied, remove an item with the least priority.
3. If more than one item satisfies condition #2, remove the least recently used one.
4. Multiple items can have the same priority and expiry.

Untold rules:
 - All of those operations should be O(1) time and space complexity.

## 1 Min Solution summary

It's an extension of the [LRU Cache Wikipedia](https://en.wikipedia.org/wiki/Cache_replacement_policies#Least_recently_used_(LRU))
as explained in this implementation [LRU Cache Interview Cake](https://www.interviewcake.com/concept/java/lru-cache)
the difference it's the addition of a binary tree to keep track of the min and max priority and expiry.

## Solution

Assumptions:
 - all the parameters do have fixed length e.g. String= len 1024; Int = u32

Data structure used:
 - [Doubly linked list](https://en.wikipedia.org/wiki/Doubly_linked_list)
 - [Binary Tree](https://en.wikipedia.org/wiki/Binary_tree)
 - [Hash map](https://en.wikipedia.org/wiki/Hash_table)

### Set O(1) time and space
Let's start from set, to reduce the Lookup time we are going to use hashmap to store a reference
to the object that will encapsulate the "value" parameters at cost O(K) time and space
assuming the map its pre-initialized, now K is the length of the String because it will be the input of our
hash function, given our assumption is value it's a fixed length then O(1) will be our cost.

Now we can make an assumption that the int it's a finite number say u32, this means we can construct
a binary tree with a depth of 32 with access time of O(32) therefore O(1) time and space.

This way we can build 2 binary trees that will give us the min and max priority and expiry in O(1) cost.
to satisfy rule 4 we need as well to use a double linked list as leaf level of the binary trees
so we can have multiple items with the same expiry or same priority and to satisfy rule 3 about
the removal of the least recently used.

The complexity is O(1) for the insertion + O(1) for the insertion binary tree of priority and expiry O(K)x2 
assuming K its constant then O(1)+O(1)x2 = O(1) time and space. 

### Get O(1) time and space

The get is simpler because we only have to access the hashmap to get the reference to the object which
we do already know happens in O(1) time and space.

And

Keep the expiry least used doubly linked list on both expiry and priority consistent
we are going to move the item to the head of the list, this way we can keep track of the 
least recently used O(1).

The complexity is O(1) for the lookup + O(1) for the insertion binary tree of priority and expiry O(K)x2
assuming K its constant then O(1)+O(1)x2 = O(1) time and space.


### EvictItem O(1) time and space

Following rule number 1 we are going to get the min expiry time in O(1) thanks to the binary tree mentioned earlier,
and delete the first item, as policy we are using the least recently used for both expiry and priority.

As requested if the min it's still not expired we are going to get in O(1) the min priority,
and we can remove the least recently used thanks to the doubly linked list.

The complexity is O(1)x2 for the find of the min expiry time and the min in priority and O(1)
to remove the tail of the doubly linked list = O(1) time and space.


## Extras

As suggested here [Rust Performance Book](https://nnethercote.github.io/perf-book/) 
we have used ahash to reduce the hashing time.