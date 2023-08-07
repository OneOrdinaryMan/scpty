bin_SCRIPTS = hello_python
dist_noinst_SCRIPTS = script.py
CLEANFILES = hello
$(bin_SCRIPTS): $(dist_noinst_SCRIPTS)
	cp $< $@
	chmod 775 $@
