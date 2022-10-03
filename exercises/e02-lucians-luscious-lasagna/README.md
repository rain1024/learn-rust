## Lucians Luscious Lasagna

```
This exercise is coppied from 
https://exercism.org/tracks/rust/exercises/lucians-luscious-lasagna/
```

In this exercise you're going to write some code to help you cook a brilliant lasagna from your favorite cooking book.

You have four tasks, all related to the time spent cooking the lasagna.

### Task 1

Define the expected oven time in minutes

Define the expected_minutes_in_oven binding to check how many minutes the lasagna should be in the oven. According to the cooking book, the expected oven time in minutes is 40:

```rs
expected_minutes_in_oven()
// Returns: 40
```

### Task 2

Calculate the remaining oven time in minutes

Define the remaining_minutes_in_oven function that takes the actual minutes the lasagna has been in the oven as a parameter and returns how many minutes the lasagna still has to remain in the oven, based on the expected oven time in minutes from the previous task.

```rs
remaining_minutes_in_oven(30)
// Returns: 10
```

### Task 3

Define the preparation_time_in_minutes function that takes the number of layers you added to the lasagna as a parameter and returns how many minutes you spent preparing the lasagna, assuming each layer takes you 2 minutes to prepare.

```rs
preparation_time_in_minutes(2)
// Returns: 4
```

### Task 4

```rs
Define the elapsed_time_in_minutes function that takes two parameters: the first parameter is the number of layers you added to the lasagna, and the second parameter is the number of minutes the lasagna has been in the oven. The function should return how many minutes you've worked on cooking the lasagna, which is the sum of the preparation time in minutes, and the time in minutes the lasagna has spent in the oven at the moment.
```

```rs
elapsed_time_in_minutes(3, 20)
// Returns: 26
```






