import sys
import turtle
from turtle import *

class State:
    x = 0
    y = 0
    theta = 0

    def __init__(self, x, y, angle):
        self.x = x
        self.y = y
        self.theta = angle

class DrawSystem:
    todo = None
    len = None
    theta = None
    Stack = []

    def __init__(self, s, l, t):
        self.todo = s
        self.len = l
        self.theta = t

    def render(self):
        pen1 = Pen()
        pen1.hideturtle()
        pen1.speed(0)
        pen1.pensize(2)
        pen1.left(90)
        pen1.penup()
        pen1.goto(0, -400)
        pen1.pendown()
        pen1.screen.bgcolor("#ffffff")
        pen1.color("#000000")
        turtle.tracer(0.0)
        turtle.resizemode("auto")
        for i in range (0, len(self.todo)):
            c = self.todo[i]
            if c == "F":
                pen1.forward(self.len)
            elif c == "+":
                pen1.right(self.theta)
            elif c == "-":
                pen1.left(self.theta)
            elif c == "[":
                self.Stack.append(State(pen1.xcor(), pen1.ycor(), pen1.heading()))
                self.len *= .9
            elif c == "]":
                pen1.penup()

                last = len(self.Stack) - 1
                pen1.goto(self.Stack[last].x, self.Stack[last].y)
                pen1.setheading(self.Stack[last].theta)
                self.Stack.pop()
                pen1.pendown()
                self.len *= 1/.9
        turtle.mainloop()

def main():
    drawer = DrawSystem(sys.argv[1], 2, 25)
    drawer.render()

if __name__ == "__main__":
    main()
