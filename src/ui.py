import tkinter as tk
from tkinter import ttk
import sys
import os

root_w = tk.Tk()
root_w.geometry("800x800")
root_w.title = "Image 2 Ascii"

file_path = os.getcwd() + "\\src\\ascii.txt"

i = 0
max = 0
min = sys.maxsize
with open(file_path, 'r') as text:
    for line in text:
        i += 1
        if max < len(line.strip()):
            max = len(line.strip())
        if len(line.strip()) < min:
            min = len(line.strip())

frame = ttk.Frame(root_w)
frame.grid()
ttk.Label(frame, text="Max Width: " + str(max) + ", Min Width: " + str(min) + ", Height: " + str(i)).grid(column=0, row=0)

root_w.mainloop()