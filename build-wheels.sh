#!/bin/bash
set -ex

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
export PATH="$HOME/.cargo/bin:$PATH"

cd /io

for PYBIN in /opt/python/{cp36-cp36m,cp37-cp37m,cp38-cp38,cp39-cp39}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"

    "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
    "${PYBIN}/python" setup.py bdist_wheel --plat-name=manylinux2014_x86_64
done

for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done
