#!/usr/bin/env python
from pathlib import Path

import pyqtgraph as pg
from pyqtgraph.Qt import QtGui, QtWidgets, QtCore
import numpy as np
import sys
import imageio


def imageHoverEvent(event):
    """Show the position, pixel, and value under the mouse cursor."""
    if event.isExit():
        p1.setTitle("")
        return
    pos = event.pos()
    i, j = pos.y(), pos.x()
    i = int(np.clip(i, 0, data.shape[0] - 1))
    j = int(np.clip(j, 0, data.shape[1] - 1))
    val = data[i, j]
    ppos = img.mapToParent(pos)
    x, y = ppos.x(), ppos.y()
    if isinstance(val, np.ndarray):
        p1.setTitle(
            "pos: (%0.1f, %0.1f)  pixel: (%d, %d)  value: [%.3g %.3g %.3g]"
            % (x, y, i, j, val[0], val[1], val[2])
        )
    else:
        p1.setTitle(
            "pos: (%0.1f, %0.1f)  pixel: (%d, %d)  value: %.3g" % (x, y, i, j, val)
        )


THIS_DIR = Path(__file__).parent
IM_DIR = THIS_DIR / ".." / "out"


def update_image(fpath):
    global data
    print(f"reloading file: {fpath}")
    data = imageio.v3.imread(fpath)
    img.setImage(image=np.flipud(data))


if __name__ == "__main__":
    try:
        fpath = sys.argv[1]
    except IndexError:
        # print("Usage: imview file.ext")
        # fpath = sorted(IM_DIR.iterdir())[-1]
        fpath = IM_DIR / "latest.ppm"

    print(f"loading : {fpath}")
    data = imageio.v3.imread(fpath)

    fswatch = QtCore.QFileSystemWatcher()
    fswatch.addPath(str(fpath))

    fswatch.fileChanged.connect(update_image)

    # Interpret image data as row-major instead of col-major
    pg.setConfigOptions(imageAxisOrder="row-major")

    pg.mkQApp()

    win = pg.GraphicsLayoutWidget(size=(800, 450))

    win.setWindowTitle("simple image viewer")
    win.setWindowIcon(QtGui.QIcon(str(THIS_DIR / "icon.ico")))
    win.show()

    # A plot area (ViewBox + axes) for displaying the image
    p1 = win.addPlot(title="")

    # Item for displaying image data
    img = pg.ImageItem()
    p1.addItem(img)
    img.setImage(image=np.flipud(data))

    # Contrast/color control
    hist = pg.HistogramLUTItem()
    hist.setImageItem(img)
    hist.setLevelMode("rgba")
    win.addItem(hist)

    # Draggable line for setting isocurve level
    isoLine = pg.InfiniteLine(angle=0, movable=True, pen="g")
    hist.vb.addItem(isoLine)
    hist.vb.setMouseEnabled(y=False)  # makes user interaction a little easier
    isoLine.setValue(0.8)
    isoLine.setZValue(1000)  # bring iso line above contrast controls

    img.hoverEvent = imageHoverEvent

    pg.exec()
