import subprocess

# git fetch
with open("../../bad_branches.txt") as f:
	l = f.readlines()

l = [m.replace("\n", "") for m in l]

commandList = "git push origin --delete".split(" ")

for branch in l :
	subprocess.run(commandList + [branch])