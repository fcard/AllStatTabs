import os
import re

incsrc = re.compile('^\s*incsrc\s*"?(.*?)"?\s*$')

class PatchMaker:
    def __init__(self, output):
        self.output_filename = output
        self.dirlist = []
    #
    def __enter__(self):
        self.output = open(self.output_filename, 'w')
        return self
    #
    def __exit__(self, *rest):
        self.output.close()
    #
    def read_file(self, filename):
        os.chdir(os.path.dirname(filename))
        self.dirlist.append(os.getcwd())
        file = open(os.path.basename(filename), 'r')
        lines = file.readlines()
        file.close()
        for line in lines:
            if line.strip().startswith('incsrc'):
                self.read_file(incsrc.match(line).groups(0)[0])
                self.dirlist.pop()
                os.chdir(self.dirlist[-1])
            else:
                self.output.write(line)

os.chdir(os.path.dirname(os.path.realpath(__file__)))
with PatchMaker("patch.asm") as patch_maker:
    patch_maker.read_file(os.path.join("src","patch.asm"))

