"""The publisher must preserve both live mmap inodes and lazy asset paths."""
import importlib.machinery
import importlib.util
import mmap
from pathlib import Path
import tempfile
import unittest

loader = importlib.machinery.SourceFileLoader('win10x_denial', str(Path(__file__).with_name('win10x-denial')))
spec = importlib.util.spec_from_loader(loader.name, loader)
helper = importlib.util.module_from_spec(spec)
loader.exec_module(helper)

class BundlePublicationTest(unittest.TestCase):
    def test_rebuild_keeps_running_generation_and_mapped_library(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            old = root / 'bundle'
            (old / 'lib').mkdir(parents=True)
            (old / 'lib/libapp.so').write_bytes(b'old-library')
            (old / 'asset').write_text('old-font')
            with (old / 'lib/libapp.so').open('rb') as stream:
                with mmap.mmap(stream.fileno(), 0, access=mmap.ACCESS_READ) as mapped:
                    first = root / 'first'
                    first.mkdir()
                    (first / 'asset').write_text('first-font')
                    helper.publish_bundle(first, old)
                    pinned = old.resolve()
                    second = root / 'second'
                    second.mkdir()
                    (second / 'asset').write_text('second-font')
                    helper.publish_bundle(second, old)
                    self.assertEqual(mapped[:], b'old-library')
                    self.assertEqual((pinned / 'asset').read_text(), 'first-font')
                    self.assertEqual((old / 'asset').read_text(), 'second-font')
                    self.assertEqual(old.resolve(), second)

if __name__ == '__main__':
    unittest.main()
