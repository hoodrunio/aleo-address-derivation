from setuptools import setup, find_packages

setup(
    name="aleo_address_derivation",
    version="0.1.0",
    packages=find_packages(where="python"),
    package_dir={"": "python"},
    install_requires=[
        "aleo_address_derivation",
    ],
)