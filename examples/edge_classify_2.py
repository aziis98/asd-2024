# Algorithm from MIT OpenCourseWare (seems correct)
# https://courses.csail.mit.edu/6.006/fall11/rec/rec14.pdf

class DFSResult:
    def __init__(self):
        self.parent = {}
        self.start_time = {}
        self.finish_time = {}
        self.edges = {}  # Edge classification for directed graph.
        self.order = []
        self.t = 0

def dfs(g):
    results = DFSResult()
    for vertex in g.vertices():
        if vertex not in results.parent:
            dfs_visit(g, vertex, results)
    return results

def dfs_visit(g, v, results, parent=None):
    results.parent[v] = parent
    results.t += 1
    results.start_time[v] = results.t
    if parent is not None:
        results.edges[(parent, v)] = 'tree'

    for n in g.neighbors(v):
        if n not in results.parent:  # n is not visited
            dfs_visit(g, n, results, v)
        elif n not in results.finish_time:
            results.edges[(v, n)] = 'back'
        elif results.start_time[v] < results.start_time[n]:
            results.edges[(v, n)] = 'forward'
        else:
            results.edges[(v, n)] = 'cross'

    results.t += 1
    results.finish_time[v] = results.t
    results.order.append(v)

# Graph structure
class Graph:
    def __init__(self):
        self.adjacency_list = {}

    def add_edge(self, u, v):
        if u not in self.adjacency_list:
            self.adjacency_list[u] = []
        self.adjacency_list[u].append(v)

    def vertices(self):
        return self.adjacency_list.keys()

    def neighbors(self, v):
        return self.adjacency_list.get(v, [])

# Example usage:
g = Graph()
g.add_edge(0, 1)
g.add_edge(1, 2)
g.add_edge(2, 3)
g.add_edge(3, 0)  # Creating the cycle 0 -> 1 -> 2 -> 3 -> 0

# Running DFS
results = dfs(g)

print("Parent Map:", results.parent)
print("Start Times:", results.start_time)
print("Finish Times:", results.finish_time)
print("Edge Classifications:", results.edges)
print("DFS Order:", results.order)
