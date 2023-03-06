# copurchased-item

This project is to find the most copurchased item on Amazon.
The dataset is Amazon product co-purchasing network, March 02 2003 from Stanford Large Network Dataset Collection.
The dataset is a node and edge graph. I analyzed this dataset by measuring the centrality and the node degree of each node.

First, I looked into the degree distribution of the graph. So, I calculated P(k) which is the probability that a randomly selected node would have a degree of k. <br>
Next, I measured the eigen centrality, measurement of the node degree by counting how many links its connections have, to find which node has the highest degree. Then, I will analyze it if it is biased or flawed by looking at the graph to avoid hubs bias.
