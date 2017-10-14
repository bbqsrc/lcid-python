from setuptools import setup

setup(
        name="lcid",
        author="Brendan Molloy",
        author_email="brendan@bbqsrc.net",
        description="LCID lookup tool",
        version="1.0.0",
        license="CC0-1.0",
        packages=["lcid"],
        package_data={'': ["lcids.json"]},
        url="https://github.com/bbqsrc/lcid-python",
        long_description=open("README.md").read()
)
