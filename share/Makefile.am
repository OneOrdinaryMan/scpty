dirs_to_include = bash python shell
files_to_distribute = bash.tar.gz python.tar.gz shell.tar.gz
maindir = $(datadir)/scpty
EXTRA_DIST = $(dirs_to_include)
main_DATA = $(files_to_distribute)
CLEANFILES = $(files_to_distribute)
bash.tar.gz: bash
	tar -czf $@ -C $< .
python.tar.gz: python
	tar -czf $@ -C $< .
shell.tar.gz: shell
	tar -czf $@ -C $< .
