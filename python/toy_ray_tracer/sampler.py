import math
from typing import Tuple, Union
import taichi as ti
from toy_ray_tracer.vecs import random_float, random_int, vec3f, vec3i


class Disk(object):
    def __init__(self, center: vec3f, radius: float) -> None:
        self.center = center
        self.radius = radius


@ti.data_oriented
class Sampler(object):
    @ti.func
    def random(self):
        raise NotImplementedError


@ti.func
def point_on_disk(theta: float, r: float, center) -> vec3i:
    a = r * ti.sin(theta)
    b = r * ti.cos(theta)
    c = center
    return vec3i([int(a + c[0]), int(b + c[1]), int(c[2])])


BlockSize = Tuple[int, int]


@ti.data_oriented
class DiskRandomSampler(Sampler):
    def __init__(self, disk: Disk) -> None:
        self.disk = disk
        super().__init__()

    @ti.func
    def random(self):
        theta = random_float(0.0, 2.0 * math.pi)
        r = self.disk.radius * ti.sqrt(random_float(0.0, 1.0))
        return point_on_disk(theta, r, self.disk.center)


@ti.data_oriented
class DiskRandomFixedSampler(Sampler):
    def __init__(self, disk: Disk, block_size: BlockSize) -> None:
        super().__init__()
        self.disk = disk
        self.block_size = block_size
        self.n_points = self.block_size[0] * self.block_size[1]
        self.sampled_points = vec3i.field(shape=self.n_points)

        self._init_sampled_points()

    @ti.kernel
    def _init_sampled_points(self):
        for i in range(self.n_points):
            theta = random_float(0.0, 2.0 * math.pi)
            r = self.disk.radius * ti.sqrt(random_float(0.0, 1.0))
            self.sampled_points[i] = point_on_disk(theta, r, self.disk.center)

    @ti.func
    def random(self):
        idx = random_int(0, self.n_points)
        return self.sampled_points[idx]


@ti.data_oriented
class DiskUniformSampler(Sampler):
    def __init__(self, disk: Disk, block_size: BlockSize) -> None:
        super().__init__()
        self.disk = disk
        self.block_size = block_size
        self.n_points: int = self.block_size[0] * self.block_size[1]
        self.sampled_points = vec3i.field(shape=self.n_points)

        self._init_sampled_points()

    @ti.kernel
    def _init_sampled_points(self):
        # [0, 2pi] * [0, 1]
        for idx in range(self.n_points):
            i = idx // self.block_size[1]
            j = idx % self.block_size[1]

            x = i / self.block_size[0]
            y = j / self.block_size[1]
            theta = 2.0 * math.pi * x
            r = self.disk.radius * ti.sqrt(y)

            random_point = point_on_disk(theta, r, self.disk.center)
            self.sampled_points[idx] = random_point

    @ti.func
    def random(self):
        idx = random_int(0, self.n_points)
        return self.sampled_points[idx]


@ti.data_oriented
class DiskBlueNoiseSampler(Sampler):
    def __init__(self, disk: Disk, block_size: BlockSize) -> None:
        self.disk = disk
        self.n_points = block_size[0] * block_size[1]
        self.sampled_points = vec3i.field(shape=self.n_points)
        self._init_sampled_points()

    @ti.kernel
    def _init_sampled_points(self):
        for idx in range(self.n_points):
            xy = halton_sequence_2d(idx)
            theta = 2.0 * math.pi * xy[0]
            r = self.disk.radius * ti.sqrt(xy[1])

            random_point = point_on_disk(theta, r, self.disk.center)
            self.sampled_points[idx] = random_point

    @ti.func
    def random(self):
        idx = random_int(0, self.n_points)
        return self.sampled_points[idx]


@ti.data_oriented
class SamplerEngine:
    def __init__(self, width: int, height: int, sampler: Sampler, color_mode: Union[str, vec3f] = "random") -> None:
        self.size = (width, height)
        self.sampler = sampler
        self.canvas = vec3f.field(shape=(width, height))

    @ti.kernel
    def render(self, count: int):
        for i in range(count):
            pos = self.sampler.random()
            self.canvas[pos[0], pos[1]] = vec3f([1.0, 1.0, 1.0])

    def canvas_to_numpy(self):
        return self.canvas.to_numpy()


@ti.func
def halton_sequence_2d(idx: int):
    return ti.Vector([radical_inverse(idx, 2), radical_inverse(idx, 3)])


@ti.func
def radical_inverse(n: int, base: int) -> float:
    val = 0.0
    inv_base = 1.0 / base
    inv_bi = inv_base
    while n > 0:
        val += inv_bi * (n % base)
        n //= base
        inv_bi *= inv_base
    return val
