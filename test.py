maze = [
    [0,1,2,3],
    [1,6,5,4],
    [1,7,8,9],
    [9,8,7,6],
    [2,3,4,5],
    [1,0,1,1]
]

directions = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
]

def dfs(point, visited, maze, count):
    print(f"count: {count}")
    points = [(direction[0] + point[0], direction[1] + point[1]) for direction in directions]
    walk = [
            (maze[x], maze[y]) for x, y in points
    ]
    print(points, walk)
    return count

print(dfs((0, 0), [(0, 0)], maze, 0))
        # if next_dir[0] <= 3 and next_dir[1] >= 0 and next_dir[0] >= 0 and next_dir[1] <= 5: 
        #     next_point = maze[next_dir[0]][next_dir[1]]
        #     if next_point == maze[point[0]][point[1]] + 1 and next_dir not in visited:
        #         visited.append(next_dir)
        #         return dfs(next_dir, visited, maze, count)
