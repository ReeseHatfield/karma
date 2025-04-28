import sys
import re
import networkx as nx
import matplotlib.pyplot as plt


datafile = sys.argv[1]

# parse digusting regex parse from rust + grep output
topic_re = re.compile(r'from_subject: "(.*?)", via_predicate: "(.*?)", to_object_label: "(.*?)"')


G = nx.DiGraph()

# digraph stuff yoinked from https://www.geeksforgeeks.org/directed-graphs-multigraphs-and-visualization-in-networkx/
with open(datafile, 'r', encoding='utf-8') as f:
    for line in f:
        match = topic_re.search(line)
        if match:
            subject, predicate, obj = match.groups()
            # this feels so nice that it actually works that simply
            G.add_edge(subject, obj, label=predicate)

# id played around with these params, these look fine
pos = nx.spring_layout(G, k=0.8, iterations=100)
plt.figure(figsize=(14, 10))
nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=1500)
nx.draw_networkx_edges(G, pos, arrowstyle='->', arrowsize=20, edge_color='gray')
nx.draw_networkx_labels(G, pos, font_size=10, font_weight='bold')

# also https://stackoverflow.com/questions/20133479/how-to-draw-directed-graphs-using-networkx-in-python
edge_labels = {(u, v): d['label'] for u, v, d in G.edges(data=True)}
nx.draw_networkx_edge_labels(G, pos, edge_labels=edge_labels, font_color='darkgreen', font_size=8)

plt.title("LLM Crawl Graph", fontsize=16)
plt.axis('off')
plt.tight_layout()
output_file = "graph_output.png"
plt.savefig(output_file, format='png', dpi=300)
print(f"Graph saved to {output_file}")
