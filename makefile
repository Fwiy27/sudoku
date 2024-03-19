entrypoint = main

rust:
	rustc src/${entrypoint}.rs
	./${entrypoint}
	rm ${entrypoint}