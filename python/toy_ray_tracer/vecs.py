import taichi as ti

from typing import Type as __Type
from taichi.types.compound_types import vector as __vector
from taichi.lang.matrix import Matrix as __Matrix, MatrixField as __MatrixField

float_inf = float("inf")
vec2f: __Type[__Matrix] = __vector(2, dtype=float)
vec3f: __Type[__Matrix] = __vector(3, dtype=float)

vec3i: __Type[__Matrix] = __vector(3, dtype=int)

color3 = vec3f
point3 = vec3f

vec2f_field = __MatrixField
vec3f_field = __MatrixField


@ti.func
def random_float(min: float = 0, max: float = 1.0) -> float:
    return ti.random(float) * (max - min) + min


@ti.func
def random_int(min: int = 0, max: int = 100) -> int:
    return int(ti.floor(ti.random(float) * (max - min) + min))


@ti.func
def random_color3() -> color3:
    return vec3f(random_float(), random_float(), random_float())
