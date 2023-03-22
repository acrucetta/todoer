# Todoer - do from the CLI

## Intro
The scope of this project will be create a custom CLI tool
for myself that hadnles todo tasks from the command line.

I can add todo tasks, remove them, list them and mark them as done.

## I will be able to add a task with:
```
+ Clean kitchen
Tags: [cleaning, work, research, learning, app, family]
Due: [today,tomorrow,week,weekend,nextweek]
Timestamp:
```

## List tasks with:
```
doer ls 

doer ls --p=p1 --tag=work
doer ls --due=today
```


## Remove tasks with:

This will list the tasks based on when they're due and their priority level. Tasks will have a priority level of P1 to P3.

I can filter the tasks with ls based on their category, either work, priority, or due date. Each one would have a respective flag.

To mark a task as done, we point to the ID and write "done". This will archive the tasks to a tally of tasks that have been finished.

```
doer done 2
```




