# v3alpha1 developer notes

Alpha APIs may change without notice. `ReleaseChannel` filters and pipeline
status enums are synthetic — do not ship production clients against this
fixture.

Cross-version `$ref` to v2 pagination (`PageResult`, `ListOptions`) exercises
populate resolution when the contract module spans 3.0.3 and 3.1.0 entry roots.

Pipeline LRO handles returned from `POST /pipelines` are stable for the lifetime
of a documentation run only.
