import math as m
import numpy as np
import matplotlib.pyplot as plt
try:
    # optional, just used for better default figure style
    import seaborn as sns
    sns.set()
except ImportError:
    sns = None


def quadratic_func(a, b, c, xs):
    return a * xs ** 2 + b * xs + c


def roots(a, b, c):
    disc = (b * b) - (4 * a * c)
    if disc > 0:
        r1 = (-b - m.sqrt(disc)) / (2 * a)
        r2 = (-b + m.sqrt(disc)) / (2 * a)
        return [r1, r2]
    elif disc == 0:
        return [-b / (2*a)]
    else:
        raise ValueError("no real root exist")


def main():
    a, b, c = 1, 1, -3
    xs = np.linspace(-5, 5, 100)
    ys = quadratic_func(a, b, c, xs)

    rs = roots(a, b, c)
    plt.plot(xs, ys)
    # plt.plot(xs, np.zeros_like(xs), 'r')
    plt.hlines(0, xmin=-10, xmax=10, color='g')
    plt.scatter(rs, np.zeros_like(rs))
    plt.show()


if __name__ == '__main__':
    main()
