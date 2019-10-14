// Copyright (c) 2019, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::util::AlignedArray;

#[rustfmt::skip]
#[no_mangle]
pub static rav1e_mc_subpel_filters: AlignedArray<[i8; 600]> = AlignedArray::new([
   0,   1,  -3,  63,   4,  -1,   0,   0, // REGULAR
   0,   1,  -5,  61,   9,  -2,   0,   0,
   0,   1,  -6,  58,  14,  -4,   1,   0,
   0,   1,  -7,  55,  19,  -5,   1,   0,
   0,   1,  -7,  51,  24,  -6,   1,   0,
   0,   1,  -8,  47,  29,  -6,   1,   0,
   0,   1,  -7,  42,  33,  -6,   1,   0,
   0,   1,  -7,  38,  38,  -7,   1,   0,
   0,   1,  -6,  33,  42,  -7,   1,   0,
   0,   1,  -6,  29,  47,  -8,   1,   0,
   0,   1,  -6,  24,  51,  -7,   1,   0,
   0,   1,  -5,  19,  55,  -7,   1,   0,
   0,   1,  -4,  14,  58,  -6,   1,   0,
   0,   0,  -2,   9,  61,  -5,   1,   0,
   0,   0,  -1,   4,  63,  -3,   1,   0,
   0,   1,  14,  31,  17,   1,   0,   0, // SMOOTH
   0,   0,  13,  31,  18,   2,   0,   0,
   0,   0,  11,  31,  20,   2,   0,   0,
   0,   0,  10,  30,  21,   3,   0,   0,
   0,   0,   9,  29,  22,   4,   0,   0,
   0,   0,   8,  28,  23,   5,   0,   0,
   0,  -1,   8,  27,  24,   6,   0,   0,
   0,  -1,   7,  26,  26,   7,  -1,   0,
   0,   0,   6,  24,  27,   8,  -1,   0,
   0,   0,   5,  23,  28,   8,   0,   0,
   0,   0,   4,  22,  29,   9,   0,   0,
   0,   0,   3,  21,  30,  10,   0,   0,
   0,   0,   2,  20,  31,  11,   0,   0,
   0,   0,   2,  18,  31,  13,   0,   0,
   0,   0,   1,  17,  31,  14,   1,   0,
  -1,   1,  -3,  63,   4,  -1,   1,   0, // SHARP
  -1,   3,  -6,  62,   8,  -3,   2,  -1,
  -1,   4,  -9,  60,  13,  -5,   3,  -1,
  -2,   5, -11,  58,  19,  -7,   3,  -1,
  -2,   5, -11,  54,  24,  -9,   4,  -1,
  -2,   5, -12,  50,  30, -10,   4,  -1,
  -2,   5, -12,  45,  35, -11,   5,  -1,
  -2,   6, -12,  40,  40, -12,   6,  -2,
  -1,   5, -11,  35,  45, -12,   5,  -2,
  -1,   4, -10,  30,  50, -12,   5,  -2,
  -1,   4,  -9,  24,  54, -11,   5,  -2,
  -1,   3,  -7,  19,  58, -11,   5,  -2,
  -1,   3,  -5,  13,  60,  -9,   4,  -1,
  -1,   2,  -3,   8,  62,  -6,   3,  -1,
   0,   1,  -1,   4,  63,  -3,   1,  -1,
   0,   0,  -2,  63,   4,  -1,   0,   0, // REGULAR 4
   0,   0,  -4,  61,   9,  -2,   0,   0,
   0,   0,  -5,  58,  14,  -3,   0,   0,
   0,   0,  -6,  55,  19,  -4,   0,   0,
   0,   0,  -6,  51,  24,  -5,   0,   0,
   0,   0,  -7,  47,  29,  -5,   0,   0,
   0,   0,  -6,  42,  33,  -5,   0,   0,
   0,   0,  -6,  38,  38,  -6,   0,   0,
   0,   0,  -5,  33,  42,  -6,   0,   0,
   0,   0,  -5,  29,  47,  -7,   0,   0,
   0,   0,  -5,  24,  51,  -6,   0,   0,
   0,   0,  -4,  19,  55,  -6,   0,   0,
   0,   0,  -3,  14,  58,  -5,   0,   0,
   0,   0,  -2,   9,  61,  -4,   0,   0,
   0,   0,  -1,   4,  63,  -2,   0,   0,
   0,   0,  15,  31,  17,   1,   0,   0, // SMOOTH 4
   0,   0,  13,  31,  18,   2,   0,   0,
   0,   0,  11,  31,  20,   2,   0,   0,
   0,   0,  10,  30,  21,   3,   0,   0,
   0,   0,   9,  29,  22,   4,   0,   0,
   0,   0,   8,  28,  23,   5,   0,   0,
   0,   0,   7,  27,  24,   6,   0,   0,
   0,   0,   6,  26,  26,   6,   0,   0,
   0,   0,   6,  24,  27,   7,   0,   0,
   0,   0,   5,  23,  28,   8,   0,   0,
   0,   0,   4,  22,  29,   9,   0,   0,
   0,   0,   3,  21,  30,  10,   0,   0,
   0,   0,   2,  20,  31,  11,   0,   0,
   0,   0,   2,  18,  31,  13,   0,   0,
   0,   0,   1,  17,  31,  15,   0,   0
]);

#[rustfmt::skip]
#[no_mangle]
pub static rav1e_filter_intra_taps: AlignedArray<[[i8; 64]; 5]> = AlignedArray::new([
  [ -6,  10,  -5,   2,  -3,   1,  -3,   1,
    -4,   6,  -3,   2,  -3,   2,  -3,   1,
     0,   0,  10,   0,   1,  10,   1,   2,
     0,   0,   6,   0,   2,   6,   2,   2,
     0,  12,   0,   9,   0,   7,  10,   5,
     0,   2,   0,   2,   0,   2,   6,   3,
     0,   0,   0,   0,   0,   0,   0,   0,
    12,   0,   9,   0,   7,   0,   5,   0],
  [-10,  16,  -6,   0,  -4,   0,  -2,   0,
   -10,  16,  -6,   0,  -4,   0,  -2,   0,
     0,   0,  16,   0,   0,  16,   0,   0,
     0,   0,  16,   0,   0,  16,   0,   0,
     0,  10,   0,   6,   0,   4,  16,   2,
     0,   0,   0,   0,   0,   0,  16,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
    10,   0,   6,   0,   4,   0,   2,   0],
  [ -8,   8,  -8,   0,  -8,   0,  -8,   0,
    -4,   4,  -4,   0,  -4,   0,  -4,   0,
     0,   0,   8,   0,   0,   8,   0,   0,
     0,   0,   4,   0,   0,   4,   0,   0,
     0,  16,   0,  16,   0,  16,   8,  16,
     0,   0,   0,   0,   0,   0,   4,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
    16,   0,  16,   0,  16,   0,  16,   0],
  [ -2,   8,  -1,   3,  -1,   2,   0,   1,
    -1,   4,  -1,   3,  -1,   2,  -1,   2,
     0,   0,   8,   0,   3,   8,   2,   3,
     0,   0,   4,   0,   3,   4,   2,   3,
     0,  10,   0,   6,   0,   4,   8,   2,
     0,   3,   0,   4,   0,   4,   4,   3,
     0,   0,   0,   0,   0,   0,   0,   0,
    10,   0,   6,   0,   4,   0,   3,   0],
  [-12,  14, -10,   0,  -9,   0,  -8,   0,
   -10,  12,  -9,   1,  -8,   0,  -7,   0,
     0,   0,  14,   0,   0,  14,   0,   0,
     0,   0,  12,   0,   0,  12,   0,   1,
     0,  14,   0,  12,   0,  11,  14,  10,
     0,   0,   0,   0,   0,   1,  12,   1,
     0,   0,   0,   0,   0,   0,   0,   0,
    14,   0,  12,   0,  11,   0,   9,   0]
]);