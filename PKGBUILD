# Maintainer: Woshiluo Luo <woshiluo.luo@outlook.com"
pkgname=wait-online-rs-git
_pkgname=wait-online
pkgver=0.7.ffe4146
pkgrel=1
pkgdesc="tools to wait for Internet (204 response)"
arch=('any')
license=("GPLv3")
depends=()
source=(git+https://git.nas.wsl/Woshiluo/wait-online)

sha1sums=('SKIP')

pkgver() {
	cd ${srcdir}/$_pkgname

	echo 0.$(git rev-list --count HEAD).$(git rev-parse --short HEAD)
}

build() {
	cd ${srcdir}/$_pkgname

	cargo build --release
}

package() {
	cd $_pkgname

	install -Dm755 target/release/wait-online ${pkgdir}/usr/bin/wait-online-rs
  
    install -Dm644 wait-online-rs.service "${pkgdir}/usr/lib/systemd/system/wait-online-rs.service"
    install -Dm644 wait-online-rs-onresume.service "${pkgdir}/usr/lib/systemd/system/wait-online-rs-onresume.service"
    install -Dm644 tmpfiles.conf "${pkgdir}/usr/lib/tmpfiles.d/${pkgname}.conf"
}
