from enum import Enum

class Direction(Enum):
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4

class Walk:
    def __init__(self, n):
        self.n = n
        self.world = ["."] * (n * n)
        self.world[0] = '*'
        self.ant = [0, 0]

    def has_fallen(self):
        return not all((0 <= c < self.n for c in self.ant))

    def walk(self, direction):
        if direction == Direction.UP:
            self.ant[1] -= 1
        elif direction == Direction.DOWN:
            self.ant[1] += 1
        elif direction == Direction.LEFT:
            self.ant[0] -= 1
        elif direction == Direction.RIGHT:
            self.ant[0] += 1
        fallen = self.has_fallen()
        if not fallen:
            self.world[self.ant[0] + (self.ant[1] * self.n)] = "*"
        return not fallen

    def __repr__(self):
        buf = ""
        for i in range(self.n):
            lptr =  i * self.n
            rptr =  (i+1) * self.n
            world_row = self.world[lptr:rptr]
            if i == self.ant[1] and not self.has_fallen():
                world_row[self.ant[0]] = 'x'
            buf += "".join(world_row) + "\n"
        return buf

def antwalk():
    print("Welcome to AntWalk!")
    walk = Walk(10)
    print(walk)
    while True:
        print("\n[U]p, [D]own, [L]eft, or [R]ight")
        try:
            command = input(">>> ").upper()
        except KeyboardInterrupt:
            return

        direction = None
        if command == "U":
            direction = Direction.UP
        elif command == "D":
            direction = Direction.DOWN
        elif command == "L":
            direction = Direction.LEFT
        elif command == "R":
            direction = Direction.RIGHT

        if direction is None:
            print("Command not recognised")
            continue

        result = walk.walk(direction)
        print(walk)

        if not result:
            print("Walk has concluded")
            print("Enter a filename to save your walk:")
            try:
                filename = input(">>> ")
            except KeyboardInterrupt:
                return
            try:
                with open(filename, "w") as outf:
                    print(walk, file=outf, end="")
            except (OSError, IOError) as file_err:
                print(file_err)
                print("Could not write walk to {}".format(filename))
            else:
                print("Walk successfully written to {}".format(filename))
            return

if __name__ == "__main__":
    antwalk()
