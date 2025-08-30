class Localports < Formula
  desc 'List network ports with their associated binaries'
  homepage 'https://github.com/diegoholiveira/localports'
  url 'https://github.com/diegoholiveira/localports/archive/v0.1.0.tar.gz'
  sha256 'b1c96ed149ac41dfbf22d4b168e6ca61993effaf1dcb0a9da47bb30ee050e0a1'
  license 'MIT'
  head 'https://github.com/diegoholiveira/localports.git', branch: 'main'

  depends_on 'rust' => :build

  def install
    system 'cargo', 'install', *std_cargo_args
  end
end
