Welcome to ibarrow's documentation!
====================================

ibarrow is a high-performance Python library for converting ODBC data to Apache Arrow format, built with Rust for maximum speed and efficiency.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   api/README
   examples/basic_usage
   examples/performance_benchmarks

Features
========

* 🚀 **High Performance**: Built with Rust for maximum speed
* 🔄 **ODBC Integration**: Direct connection to any ODBC-compatible database
* 📊 **Arrow Format**: Native Apache Arrow support for efficient data processing
* 🐼 **Pandas/Polars Ready**: Seamless integration with popular Python data libraries
* 🛡️ **Type Safe**: Rust-powered reliability with Python convenience
* 🎯 **Two-Level API**: Simple wrappers for common use + raw functions for advanced control

Quick Start
===========

.. code-block:: python

   import ibarrow
   import polars as pl

   # Simple query with Polars DataFrame (recommended)
   df = ibarrow.query_polars(
       dsn="your_dsn",
       user="username",
       password="password",
       sql="SELECT * FROM your_table LIMIT 1000"
   )

   print(f"Retrieved {len(df)} rows")
   print(df.head())

Installation
============

.. code-block:: bash

   pip install ibarrow

Repository
==========

* **GitHub**: https://github.com/thomazyujibaba/ibarrow
* **PyPI**: https://pypi.org/project/ibarrow/
* **Documentation**: https://github.com/thomazyujibaba/ibarrow#readme

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
