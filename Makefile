install:
	sh install.sh

uninstall:
	sh uninstall.sh

# not recommended
install-root:
	sh install.sh

	#system-wide install
	sudo cp target/release/ascii_display /bin/
	sudo echo "ascii_display" >> /etc/bash.bashrc