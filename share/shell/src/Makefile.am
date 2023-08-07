bin_SCRIPTS = hello_shell
dist_noinst_SCRIPTS = script.sh
CLEANFILES = $(bin_SCRIPTS)
$(bin_SCRIPTS): $(dist_noinst_SCRIPTS)
	cp $< $@
	chmod 775 $@
