class Localports < Formula
  desc 'List network ports with their associated binaries'
  homepage 'https://github.com/diegoholiveira/localports'
  url 'https://github.com/diegoholiveira/localports/archive/v1.0.0.tar.gz'
  sha256 'REPLACE_WITH_ACTUAL_SHA256'
  license 'MIT'
  head 'https://github.com/diegoholiveira/localports.git', branch: 'main'

  depends_on 'rust' => :build

  def install
    system 'cargo', 'install', *std_cargo_args
  end
end
