#!python
# -*- encoding: utf-8 -*-

"""
    Backends/AES.py - AES wrapper for java, PyCrypto, pyaes
    
    The MIT License (MIT)

    Copyright (c) 2015 Simon Biewald

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
    THE SOFTWARE.
"""

import os

if os.name == 'java':
    try:
        from Tyche.Backends.Java.AES import *
    except ImportError:
        try:
            from Tyche.Backends.Fallback.AES import *
        except:
            raise SystemError('Your Jython version is outdated. '
                              'Please update or install the Java '
                              'Cryptography Extension (JCE) Unlimited '
                              'Strength Jurisdiction Policy for your '
                              'Java version.')
else:
    try:
        from Crypto.Cipher.AES import *
    except ImportError:
        from Tyche.Backends.Fallback.AES import *
