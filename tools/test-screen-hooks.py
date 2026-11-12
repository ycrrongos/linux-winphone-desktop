#!/usr/bin/env python3
"""Root helper contract tests: isolated files, mocked elevation and execution."""
import importlib.machinery, importlib.util, json, os, pathlib, stat, subprocess, tempfile, unittest
from unittest.mock import patch
loader=importlib.machinery.SourceFileLoader('hooks',str(pathlib.Path(__file__).resolve().parents[1]/'packaging/win10x-screen-hooks'))
spec=importlib.util.spec_from_loader('hooks',loader);module=importlib.util.module_from_spec(spec);loader.exec_module(module)
class HookAuthorization(unittest.TestCase):
 def test_exact_snapshot_and_changes_require_reauthorization(self):
  with tempfile.TemporaryDirectory() as temp:
   folder=pathlib.Path(temp);config=folder/'config.json';uid=os.getuid()
   if uid==0: self.skipTest('Run without root so owner checks exercise a real user')
   document={'screen_hooks_root':True,'screen_off_command':'echo off','screen_on_command':'echo on'};config.write_text(json.dumps(document))
   with patch.object(module,'ROOT',folder/'protected'),patch.object(module,'SUDOERS',folder/'rules'),patch.object(module.os,'geteuid',return_value=0),patch.dict(os.environ,{'PKEXEC_UID':str(uid)},clear=False),patch.object(module.subprocess,'run',return_value=subprocess.CompletedProcess([],0)) as execute:
    with patch.object(module.sys,'argv',['helper','install',str(config)]):module.main()
    self.assertEqual(stat.S_IMODE((folder/'protected'/f'{uid}.json').stat().st_mode),0o600)
    self.assertEqual(stat.S_IMODE((folder/'rules'/f'win10x-screen-{uid}').stat().st_mode),0o440)
    with patch.object(module.sys,'argv',['helper','off']),self.assertRaises(SystemExit) as result:module.main()
    self.assertEqual(result.exception.code,0)
    self.assertEqual(execute.call_args.args[0],['/bin/sh','-c','echo off'])
    document['screen_off_command']='echo changed';config.write_text(json.dumps(document));execute.reset_mock()
    with patch.object(module.sys,'argv',['helper','off']),self.assertRaisesRegex(SystemExit,'authorize again'):module.main()
    execute.assert_not_called()
    link=folder/'symlink.json';link.symlink_to(config)
    with patch.object(module.sys,'argv',['helper','install',str(link)]),self.assertRaises(OSError):module.main()
 def test_non_root_cannot_install(self):
  with patch.object(module.os,'geteuid',return_value=1000),self.assertRaisesRegex(SystemExit,'Administrator'):module.main()
if __name__=='__main__':unittest.main()
