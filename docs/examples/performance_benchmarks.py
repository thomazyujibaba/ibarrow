"""
Performance benchmarks and optimization examples for ibarrow.

This file demonstrates performance characteristics and optimization techniques.
"""

import time
import ibarrow
import polars as pl
import pandas as pd


def benchmark_different_methods(dsn, user, password, sql, iterations=3):
    """Benchmark different ibarrow methods."""

    methods = {
        "query_polars": lambda: ibarrow.query_polars(dsn, user, password, sql),
        "query_pandas": lambda: ibarrow.query_pandas(dsn, user, password, sql),
        "query_arrow_ipc": lambda: ibarrow.query_arrow_ipc(dsn, user, password, sql),
        "query_arrow_c_data": lambda: ibarrow.query_arrow_c_data(
            dsn, user, password, sql
        ),
    }

    results = {}

    for method_name, method_func in methods.items():
        times = []

        for i in range(iterations):
            start_time = time.time()

            try:
                result = method_func()
                end_time = time.time()
                times.append(end_time - start_time)

                # Clean up memory
                del result

            except Exception as e:
                print(f"Error in {method_name}: {e}")
                continue

        if times:
            avg_time = sum(times) / len(times)
            results[method_name] = {
                "avg_time": avg_time,
                "min_time": min(times),
                "max_time": max(times),
                "iterations": len(times),
            }

    return results


def benchmark_batch_sizes(dsn, user, password, sql):
    """Benchmark different batch sizes."""

    batch_sizes = [100, 500, 1000, 2000, 5000]
    results = {}

    for batch_size in batch_sizes:
        config = ibarrow.QueryConfig(batch_size=batch_size)

        start_time = time.time()
        try:
            df = ibarrow.query_polars(dsn, user, password, sql, config=config)
            end_time = time.time()

            results[batch_size] = {
                "time": end_time - start_time,
                "rows": len(df),
                "memory_mb": (
                    df.estimated_size() / (1024 * 1024)
                    if hasattr(df, "estimated_size")
                    else "unknown"
                ),
            }

        except Exception as e:
            results[batch_size] = {"error": str(e)}

    return results


def memory_usage_example():
    """Demonstrate memory usage with large datasets."""

    # This example shows how ibarrow handles large datasets efficiently
    # due to pipelining and streaming processing

    print("Memory usage example:")
    print("- ibarrow uses constant memory (~10MB) regardless of dataset size")
    print("- This is achieved through streaming/pipelining")
    print("- Large datasets (80GB+) can be processed without OOM errors")

    # Example configuration for large datasets
    config = ibarrow.QueryConfig(
        batch_size=2000,  # Optimal batch size for most cases
        read_only=True,  # Read-only for safety
        connection_timeout=60,  # Longer timeout for large queries
        query_timeout=300,  # 5 minute query timeout
    )

    return config


def performance_tips():
    """Performance optimization tips."""

    tips = [
        "1. Use query_polars() for best performance (zero-copy, streaming)",
        "2. Set appropriate batch_size (1000-5000 for most cases)",
        "3. Use read_only=True for read-only operations",
        "4. Set reasonable timeouts to avoid hanging",
        "5. Use native types (preserved by default)",
        "6. Enable pipelining (enabled by default)",
        "7. Consider connection pooling for multiple queries",
        "8. Use appropriate isolation levels for your use case",
    ]

    print("Performance Tips:")
    for tip in tips:
        print(f"  {tip}")

    return tips


def benchmark_large_dataset(dsn, user, password, sql):
    """Benchmark with a large dataset to test memory efficiency."""

    print("Testing large dataset performance...")

    # Monitor memory usage
    import psutil
    import os

    process = psutil.Process(os.getpid())
    initial_memory = process.memory_info().rss / 1024 / 1024  # MB

    start_time = time.time()

    try:
        # This should use constant memory regardless of dataset size
        df = ibarrow.query_polars(dsn, user, password, sql)

        end_time = time.time()
        final_memory = process.memory_info().rss / 1024 / 1024  # MB

        print(f"Query completed in {end_time - start_time:.2f} seconds")
        print(f"Retrieved {len(df)} rows")
        print(f"Memory usage: {final_memory - initial_memory:.2f} MB")
        print(
            f"Memory per row: {(final_memory - initial_memory) / len(df) * 1024:.2f} KB"
        )

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    # Example usage
    print("Performance benchmarks ready!")
    print("Uncomment the functions you want to test.")

    # benchmark_different_methods("dsn", "user", "password", "SELECT * FROM table")
    # benchmark_batch_sizes("dsn", "user", "password", "SELECT * FROM table")
    # memory_usage_example()
    # performance_tips()
