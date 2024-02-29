# wgpu-text-rendering
boop bop

#include "core/base.h"
#include "core/arithmetics.hpp" x
#include "core/Vector2.hpp" x
#include "core/Projection.h" x
#include "core/Scanline.h" x
#include "core/Shape.h"
    - Contour.h
        - EdgeHolder.h
            - edge-segments.h
                - SignedDistance.hpp x
                - EdgeColor.h x
#include "core/BitmapRef.hpp"
#include "core/Bitmap.h"
#include "core/bitmap-interpolation.hpp"
#include "core/pixel-conversion.hpp"
#include "core/edge-coloring.h"
#include "core/generator-config.h"
#include "core/msdf-error-correction.h"
#include "core/render-sdf.h"
#include "core/rasterization.h"
#include "core/sdf-error-estimation.h"
#include "core/save-bmp.h"
#include "core/save-tiff.h"
#include "core/shape-description.h"
