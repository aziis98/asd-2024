# Algorithm from GeeksforGeeks (wrong)
# https://www.geeksforgeeks.org/tree-back-edge-and-cross-edges-in-dfs-of-graph/

import random

class Graph:
	# instance variables
	def __init__(self, v):
		# v is the number of nodes/vertices
		self.time = 0
		self.traversal_array = []
		self.v = v
		# e is the number of edge (randomly chosen between 9 to 45)
		self.e = random.randint(9, 45)
		# adj. list for graph
		self.graph_list = [[] for _ in range(v)]
		# adj. matrix for graph
		self.graph_matrix = [[0 for _ in range(v)] for _ in range(v)]

	# function to create random graph
	def create_random_graph(self):
		# add edges upto e
		for i in range(self.e):
			# choose src and dest of each edge randomly
			src = random.randrange(0, self.v)
			dest = random.randrange(0, self.v)
			# re-choose if src and dest are same or src and dest already has an edge
			while src == dest and self.graph_matrix[src][dest] == 1:
				src = random.randrange(0, self.v)
				dest = random.randrange(0, self.v)
			# add the edge to graph
			self.add_edge(src, dest)

	def add_edge(self, src, dest):
		self.graph_list[src].append(dest)
		self.graph_matrix[src][dest] = 1

	# function to print adj list
	def print_graph_list(self):
		print("Adjacency List Representation:")
		for i in range(self.v):
			print(i, "-->", *self.graph_list[i])
		print()

	# function to print adj matrix
	def print_graph_matrix(self):
		print("Adjacency Matrix Representation:")
		for i in self.graph_matrix:
			print(i)
		print()

	# function the get number of edges
	def number_of_edges(self):
		return self.e

	# function for dfs
	def dfs(self):
		self.visited = [False]*self.v
		self.start_time = [0]*self.v
		self.end_time = [0]*self.v

		for node in range(self.v):
			if not self.visited[node]:
				self.traverse_dfs(node)
		print()
		print("DFS Traversal: ", self.traversal_array)
		print()

	def traverse_dfs(self, node):
		self.visited[node] = True
		self.traversal_array.append(node)
		self.start_time[node] = self.time
		self.time += 1
		for neighbour in self.graph_list[node]:
			print('Edge:', str(node)+'-->'+str(neighbour))

			if not self.visited[neighbour]:
				print(' => Tree Edge')
				self.traverse_dfs(neighbour)
			else:
				print(f"Times: ({self.start_time[node]}, {self.end_time[node]}) ({self.start_time[neighbour]}, {self.end_time[neighbour]})")

				if self.start_time[node] > self.start_time[neighbour] and self.end_time[node] < self.end_time[neighbour]:
					print(' => Back Edge')
				elif self.start_time[node] < self.start_time[neighbour] and self.end_time[node] > self.end_time[neighbour]:
					print(' => Forward Edge')
				else:
					print(' => Cross Edge')
			self.end_time[node] = self.time
			self.time += 1


if __name__ == "__main__":
	g = Graph(4)
	# g.create_random_graph()

	g.add_edge(0, 1)
	g.add_edge(1, 2)
	g.add_edge(2, 3)
	g.add_edge(3, 1)

	g.print_graph_list()
	g.print_graph_matrix()
	g.dfs()
