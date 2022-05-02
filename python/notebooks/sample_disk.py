from argparse import ArgumentParser
from dataclasses import dataclass, field
from typing import Tuple
import taichi as ti

import _hacked  # noqa
from toy_ray_tracer.sampler import DiskUniformSampler, Sampler, DiskBlueNoiseSampler, DiskRandomSampler, DiskRandomFixedSampler, SamplerEngine, Disk  # noqa

ti.init(ti.gpu)


@dataclass
class Config:
    sampler: str
    width: int
    height: int
    count: int
    block_size: Tuple[int, int]
    radius: int = field(default=0)

    def __post_init__(self):
        if self.radius == 0:
            self.radius = self.width // 2
        print(self)


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("--sampler", type=str, default="random")
    parser.add_argument("--width", type=int, default=200)
    parser.add_argument("--height", type=int, default=200)
    parser.add_argument("--count", type=int, default=10000)
    parser.add_argument("--block-size", type=int, nargs=2, default=[8, 8])
    return parser.parse_args()


def create_sampler(cfg: Config) -> Sampler:
    disk = Disk(center=(cfg.width // 2, cfg.height // 2, 0),
                radius=cfg.radius)
    sampler_type = cfg.sampler.lower()
    if sampler_type == "uniform":
        sampler = DiskUniformSampler(disk=disk, block_size=cfg.block_size)
    elif sampler_type == "random":
        sampler = DiskRandomSampler(disk=disk)
    elif sampler_type == "random_fixed":
        sampler = DiskRandomFixedSampler(disk=disk, block_size=cfg.block_size)
    elif sampler_type == "blue_noise":
        sampler = DiskBlueNoiseSampler(disk=disk, block_size=cfg.block_size)
    else:
        raise RuntimeError(f"unknown sampler type {sampler_type}")
    return sampler


def run_sampler(cfg: Config):
    sampler = create_sampler(cfg)
    engine = SamplerEngine(
        width=cfg.width, height=cfg.height, sampler=sampler)
    engine.render(count=cfg.count)
    return engine.canvas_to_numpy()


if __name__ == "__main__":
    args = parse_args()
    cfg = Config(sampler=args.sampler, width=args.width,
                 height=args.height, count=args.count, block_size=tuple(args.block_size))

    img = run_sampler(cfg=cfg)
    ti.tools.imshow(img)
