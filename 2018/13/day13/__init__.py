from dataclasses import dataclass
from os import mkdir
from shutil import rmtree
from typing import Dict, List, Optional, Tuple
import sys

from .direction import Direction, parse_direction
from .rotation import Rotation, next_rotation, rotate_direction
from .track import Segment, Track, TrackBuilder


COLLISION_CHAR = 'X'


@dataclass
class Cart:
    direction: Direction
    last_turn: Rotation


Coordinate = Tuple[int, int]
CartDict = Dict[Coordinate, Cart]


class SnapshotPrinter:

    def __init__(self, dirname: str, track: Track) -> None:
        self.dirname = dirname
        self.time = 0
        self.track = track
        rmtree(dirname)
        mkdir(dirname)

    def print(self, carts: CartDict) -> None:
        print('\rsnapshot time', self.time, end='', file=sys.stderr)
        with open('snapshots/{:05d}'.format(self.time), 'w') as file:
            for row in render_state(carts, self.track):
                print(row, file=file)
        self.time += 1

    def close(self):
        print(file=sys.stderr)


def advance(
        carts: CartDict,
        track: Track) -> Tuple[CartDict, Optional[Coordinate]]:
    carts, collision = move_carts(carts, track)
    carts = rotate_carts(carts, track)
    return carts, collision


def find_final_cart(carts: CartDict, track: Track) -> Optional[Coordinate]:
    collision: Optional[Coordinate] = None
    snapshots = SnapshotPrinter('snapshots', track)
    snapshots.print(carts)
    while 1 < len(carts):
        carts, collision = advance(carts, track)
        snapshots.print(carts)
    snapshots.close()
    return tuple(carts.keys())[0] if carts else collision


def find_first_collision(carts: CartDict, track: Track) -> Coordinate:
    collision: Optional[Coordinate] = None
    snapshots = SnapshotPrinter('snapshots', track)
    snapshots.print(carts)
    while not collision:
        carts, collision = advance(carts, track)
        snapshots.print(carts)
    snapshots.close()
    return collision


def load_carts(file_name: str) -> CartDict:
    carts: CartDict = {}
    with open(file_name) as lines:
        i = 0
        for line in lines:
            j = 0
            for char in line:
                direction = parse_direction(char)
                if direction:
                    carts[(i, j)] = Cart(direction, Rotation.RIGHT)
                j += 1
            i += 1
    return carts


def load_track(file_name: str) -> Track:
    track = TrackBuilder()
    with open(file_name) as lines:
        for line in lines:
            for char in line:
                track.append(char)
    return track.build()


def move_carts(
        carts: CartDict,
        track: Track) -> Tuple[CartDict, Optional[Coordinate]]:
    new_carts: CartDict = {}
    first_collision: Optional[Coordinate] = None
    queue: List[Coordinate] = sorted(carts.keys(), reverse=True)
    while queue:
        coord = queue.pop()
        cart = carts[coord]
        i, j = coord
        new_coord = (i + 1, j) if cart.direction == Direction.DOWN \
            else (i, j - 1) if cart.direction == Direction.LEFT \
            else (i, j + 1) if cart.direction == Direction.RIGHT \
            else (i - 1, j)
        if new_coord in queue:
            # This cart bumped into one that hadn't moved yet.
            queue.remove(new_coord)
            first_collision = first_collision or new_coord
        elif new_coord in new_carts:
            # This cart bumped into one that already moved.
            del new_carts[new_coord]
            first_collision = first_collision or new_coord
        else:
            new_carts[new_coord] = cart
    return new_carts, first_collision


def part1(file_name: str) -> None:
    print('\nPart 1:')
    carts = load_carts(file_name)
    track = load_track(file_name)
    i, j = find_first_collision(carts, track)
    print('{},{}'.format(j, i))


def part2(file_name: str) -> None:
    print('\nPart 2:')
    carts = load_carts(file_name)
    track = load_track(file_name)
    coord = find_final_cart(carts, track)
    if coord:
        i, j = coord
        print('{},{}'.format(j, i))
    else:
        print('None')


def render_state(carts: CartDict, track: Track) -> Tuple[str, ...]:
    lines: List[str] = []
    i = 0
    for row in track:
        chars: List[str] = []
        j = 0
        for segment in row:
            cart = carts.get((i, j))
            char = cart.direction.value if cart else segment.value
            chars.append(char)
            j += 1
        lines.append(''.join(chars))
        i += 1
    return tuple(lines)


def rotate_cart(cart: Cart, segment: Segment) -> Cart:
    d, l, r, u = tuple(Direction)
    if segment == Segment.CURVE_LEFT:
        direction = {d: r, l: u, r: d, u: l}[cart.direction]
        return Cart(direction, cart.last_turn)
    if segment == Segment.CURVE_RIGHT:
        direction = {d: l, l: d, r: u, u: r}[cart.direction]
        return Cart(direction, cart.last_turn)
    if segment == Segment.INTERSECTION:
        turn = next_rotation(cart.last_turn)
        direction = rotate_direction(cart.direction, turn)
        return Cart(direction, turn)
    if segment == Segment.HORIZONTAL and cart.direction in (l, r):
        return cart
    if segment == Segment.VERTICAL and cart.direction in (d, u):
        return cart
    if segment == Segment.SPACE:
        raise Exception('cart flew off the track')
    raise Exception('bad cart or segment')


def rotate_carts(carts: CartDict, track: Track) -> CartDict:
    result: CartDict = {}
    for coord in carts.keys():
        i, j = coord
        cart = carts[coord]
        segment = track[i][j]
        result[coord] = rotate_cart(cart, segment)
    return result
