from zipfile import ZipFile, ZIP_BZIP2
import tarfile
import shutil
from os import path, chdir
from subprocess import call

currentdir = path.dirname(path.realpath(__file__))
chdir(currentdir)
call(['python3', 'build_single_patch.py'])

chdir('gui')
call(['cargo', 'build', '--release', '--target', 'x86_64-unknown-linux-gnu'])
call(['cargo', 'build', '--release', '--target', 'x86_64-pc-windows-gnu'])

chdir(currentdir)
with ZipFile(path.join('release', 'AllStatTabs_Ips+Patch.zip'), 'w', ZIP_BZIP2) as ipszip:
    ipszip.write('patch.asm')
    ipszip.write('settings.conf')
    ipszip.write('README.md')
    ipszip.write('LICENSE')
    ipszip.write(path.join('ips', 'CT_AllStatTabs_By2.ips'), 'CT_AllStatTabs.ips')
    ipszip.write(path.join('ips', 'CT_AllStatTabs_By2_PowHit.ips'),
                 path.join('options', 'CT_AllStatTabs_By2_PowHit.ips'))
    ipszip.write(path.join('ips', 'CT_AllStatTabs_By1.ips'),
                 path.join('options', 'CT_AllStatTabs_By1.ips'))
    ipszip.write(path.join('ips', 'CT_AllStatTabs_By1_PowHit.ips'),
                 path.join('options', 'CT_AllStatTabs_By1_PowHit.ips'))

gui_windows_zip_filename = path.join('release', 'AllStatTabsSettings-x86_64-Windows.zip')
with ZipFile(gui_windows_zip_filename, 'w', ZIP_BZIP2) as gui_windows_zip:
    gui_windows_zip.write(
        path.join(
            'gui',
            'target',
            'x86_64-pc-windows-gnu',
            'release',
            'AllStatTabsSettings.exe'
        ),
        'AllStatTabsSettings.exe'
    )
    gui_windows_zip.write('patch.asm', path.join('src', 'patch.asm'))

gui_linux_tar_filename = path.join('release', 'AllStatTabsSettings-x86_64-Linux.tar.gz')
with tarfile.open(gui_linux_tar_filename, 'w:gz') as gui_linux_tar:
    gui_linux_tar.add(
        path.join(
            'gui',
            'target',
            'x86_64-unknown-linux-gnu',
            'release',
            'AllStatTabsSettings'
        ),
        'AllStatTabsSettings'
    )
    gui_linux_tar.add('patch.asm', path.join('src', 'patch.asm'))

