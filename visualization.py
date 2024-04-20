import graphviz

file_path = './output.txt'

with open(file_path, 'r') as file:
    file_contents = file.read()
    print(file_contents)

lines = file_contents.split("\n")

dot = graphviz.Digraph('round-table', comment = 'The Round Table')
dot.attr(rankdir='LR')  

#graphing the nodes
for i in range(int(lines[0])):
    dot.node(str(i))
    
#graphing the edges
for i in lines[1:]:
    if i!="":
        i = i.split(" ")
        if (i[2] == "\\0"):
            i[2] ="Є"
        dot.edge(i[0], i[1], label = i[2])

dot.render()



    

    