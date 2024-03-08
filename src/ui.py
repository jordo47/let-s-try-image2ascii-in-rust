import tkinter as tk
from tkinter import ttk
from tkinter import filedialog
import shutil
import sys
import os
import keyboard
import time
import threading


class App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.geometry("250x250")
        self.title("JPEG -> Ascii")

        self.button = ttk.Button(self, text="Upload JPEG File", command=self.button_click)
        self.button.grid(column=0, row=0, padx=15, pady=15)

    def openAscii(self):
        os.system("notepad ./src/ascii.txt")
    
    def button_click(self):
        src_img_path = filedialog.askopenfilename(title="Open JPEG File", initialdir="/",
            filetypes=[("Open JPEG File", "*.jpg")])
        
        dst_path = os.getcwd() + "\\src\\image.jpg"
        shutil.copy(src_img_path, dst_path)
        os.system("cargo run")

        thread = threading.Thread(target=self.openAscii)
        thread.start()

        time.sleep(1)

        keyboard.send('windows+up')

        for i in range(10):
            keyboard.send('ctrl + -')



root_w = App()
root_w.mainloop()