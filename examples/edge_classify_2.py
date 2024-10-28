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

def classify_iter(g):
    edges = {}
    visited = set()
    t = 0
    start_time = {}
    finish_time = {}

    for u in g.vertices():
        if u in visited:
            continue

        continuations = [
            ('node:start', u, None),
        ]

        while len(continuations) > 0:
            state, u, more = continuations.pop()

            if state == 'node:start':
                continuations.append(('node:end', u, None))

                parent = more

                visited.add(u)
                t += 1
                start_time[u] = t

                if parent is not None:
                    edges[(parent, u)] = 'tree'

                continuations.append(('node:neighbors', u, 0))
            elif state == 'node:neighbors':
                i = more
                
                neighbors = g.neighbors(u)[i:]
                for i in range(len(neighbors)):
                    v = neighbors[i]

                    if v not in visited:
                        continuations.append(('node:neighbors', u, i + 1))
                        continuations.append(('node:start', v, u))
                        break
                    elif v not in finish_time:
                        edges[(u, v)] = 'back'
                    elif start_time[u] < start_time[v]:
                        edges[(u, v)] = 'forward'
                    else:
                        edges[(u, v)] = 'cross'

            elif state == 'node:end':
                t += 1
                finish_time[u] = t

    return edges

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
# g.add_edge(0, 1)
# g.add_edge(1, 2)
# g.add_edge(2, 3)
# g.add_edge(3, 0)
# g.add_edge(3, 4)
# g.add_edge(4, 5)
# g.add_edge(5, 0)
# g.add_edge(4, 2)

# g.add_edge(0, 1)
# g.add_edge(1, 2)
# g.add_edge(0, 2)

g.add_edge("u", "v")
g.add_edge("u", "x")
g.add_edge("v", "y")
g.add_edge("y", "x")
g.add_edge("x", "v")
g.add_edge("w", "y")
g.add_edge("w", "z")

# Running DFS
# results = dfs(g)
# print("Parent Map:", results.parent)
# print("Start Times:", results.start_time)
# print("Finish Times:", results.finish_time)
# print("Edge Classifications:", results.edges)
# print("DFS Order:", results.order)

# Running Iterative DFS
edges = classify_iter(g)
print("Edge Classifications:", edges)

