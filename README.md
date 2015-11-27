WARN: This project is under development. Not ready to use and not registered to creats.io

# file_logger
A logger backend of files.

Provides

* [ ] configuration via file (maybe toml will be used)
* [x] configuration as code
* [x] flexible log format
* [x] log filtering

DOES NOT provide

* log lotation -- It should be independent from loggers and done by `logrotate` or lotating file writers.
* binary log -- Its very different from text logger.
* multiple output files -- KISS. It should be done by some sort of logger aggrigator.
* plug-ins -- KISS. This is meant to be a leaf library.
* configuration via environment variables -- Envs are too poor to configure file logger.
