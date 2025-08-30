class Localports < Formula
  desc 'List network ports with their associated binaries'
  homepage 'https://github.com/diegoholiveira/localports'
  version '0.1.0'
  license 'MIT'

  if Hardware::CPU.arm?
    url 'https://github.com/diegoholiveira/localports/releases/download/v0.1.0/localports-aarch64-apple-darwin.tar.gz'
    sha256 'fa794f5ae392da360379ee709e355baddff1d758bd43fdb74bcb944c1cdafa4c'
  else
    url 'https://github.com/diegoholiveira/localports/releases/download/v0.1.0/localports-x86_64-apple-darwin.tar.gz'
    sha256 '5feff9a9c7af2001a214b381347962be53c8810d1b5364ae2e81d1e80cc09511'
  end

  def install
    bin.install 'localports'
  end
end
