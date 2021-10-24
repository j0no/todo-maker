# Todo Maker

Command Line Tool that creates new todo file in `~/Todo` for the day unless it already exits.

# Why? lol

Yes. This literally only makes writes a file to `~/Todo`. I like to use [typora](https://typora.io/) as my offline note taking/todo app because I can always export my todo to Markdown. This will save me time everyday.

# My Mac Setup

I have [xbar](https://github.com/matryer/xbar) setup to run a [todo script](xbar_script/todo.sh) that calls on this tool.

![screenshot](https://raw.githubusercontent.com/j0no/todo-maker/main/.github/xbar_example.png)

 # Install

 First build the tool

```bash
make build
```

Then install the cli

```bash
make install
```







