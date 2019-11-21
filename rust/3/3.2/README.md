# Dining Philosophers

For our second project, let’s look at a classic concurrency problem. It’s called ‘the dining philosophers’. It was originally conceived by Dijkstra in 1965, but we’ll use a lightly adapted version from this paper by Tony Hoare in 1985.

````
In ancient times, a wealthy philanthropist endowed a College to accommodate five eminent philosophers.
Each philosopher had a room in which they could engage in their professional activity of thinking; 
there was also a common dining room, furnished with a circular table, surrounded by five chairs,
 each labelled by the name of the philosopher who was to sit in it. 

They sat anticlockwise around the table. To the left of each philosopher there was laid a golden fork,
 and in the centre stood a large bowl of spaghetti, which was constantly replenished. 

A philosopher was expected to spend most of their time thinking; but when they felt hungry, 
they went to the dining room, sat down in their own chair, picked up their own fork on their left,
 and plunged it into the spaghetti.

But such is the tangled nature of spaghetti that a second fork is required to carry it to the mouth.
The philosopher therefore had also to pick up the fork on their right. 
When they were finished they would put down both their forks, get up from their chair, and continue thinking.
Of course, a fork can be used by only one philosopher at a time.
If the other philosopher wants it, they just have to wait until the fork is available again.
````

This classic problem shows off a few different elements of concurrency. The reason is that it's actually slightly tricky to implement: a simple implementation can deadlock. For example, let's consider a simple algorithm that would solve this problem:

A philosopher picks up the fork on their left.
They then pick up the fork on their right.
They eat.
They return the forks.

Now, let’s imagine this sequence of events:

Philosopher 1 begins the algorithm, picking up the fork on their left.

Philosopher 2 begins the algorithm, picking up the fork on their left.

Philosopher 3 begins the algorithm, picking up the fork on their left.

Philosopher 4 begins the algorithm, picking up the fork on their left.

Philosopher 5 begins the algorithm, picking up the fork on their left.

... ? All the forks are taken, but nobody can eat!

