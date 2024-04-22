#!/bin/python3.11

import argparse
from subprocess import run
from pathlib import Path
from time import sleep
import shutil
import os
from contextlib import suppress


TF2_DIR = Path.home() / '.local'/'share'/'Steam' / \
    'steamapps'/'common'/'Team Fortress 2'

loaded_lib = Path("/tmp")/"liboxide.so"


def inject(pid, lib):

    shutil.copy(lib.__str__(), "/tmp")

    command = ['sudo', 'gdb', '-n', '-q', '-batch',
               '-ex', 'attach ' + pid,
               '-ex', 'set $dlopen = (void* (*)(char*, int))dlopen',
               '-ex', 'set $dlerror = (char* (*)(void))dlerror',
               '-ex', 'call $dlopen("' + loaded_lib.__str__() + '", 2)',
               '-ex', 'call $dlerror()',
               '-ex', 'detach',
               '-ex', 'quit']

    result = run(command).returncode

    if not result == 0:
        print("failed to inject")
        exit(result)


def unload(pid):

    loaded_lib = Path("/tmp")/"liboxide.so"

    command = ['sudo', 'gdb', '-n', '-q', '-batch',
               '-ex', 'attach ' + pid,
               '-ex', 'set $dlopen = (void* (*)(char*, int))dlopen',
               '-ex', 'set $dlclose = (int (*)(void*))dlclose',
               '-ex', 'set $dlerror = (char* (*)(void))dlerror',
               '-ex', 'set $self = $dlopen("'+loaded_lib.__str__()+'", 6)',
               '-ex', 'call $dlerror()',
               '-ex', 'call $dlclose($self)',
               '-ex', 'call $dlerror()',
               '-ex', 'call $dlclose($self)',
               '-ex', 'call $dlerror()',
               '-ex', 'backtrace',
               '-ex', 'detach',
               '-ex', 'quit']

    result = run(command).returncode

    os.remove(loaded_lib)
    if not result == 0:
        print("failed to unload")
        exit(result)


def get_pid():
    pid = run(
        ['pidof', 'tf_linux64'], capture_output=True).stdout.decode('utf-8')\
        .strip()

    if pid == '':
        print('tf2 not runnig')
        exit(0)
    return pid


def get_lib(debug=False):

    lib = Path(os.path.dirname(os.path.realpath(__file__))) / 'target' / ('debug' if debug else 'release') \
        / 'liboxide.so'
    if not lib.exists():
        build(debug)
    return lib


def build(dev=False):
    command = ["cargo", "build"]
    if not dev:
        command.append("-r")

    result = run(command).returncode
    if result != 0:
        print("failed to build oxide")
        exit(0)


def start_tf2():
    with suppress(KeyboardInterrupt):
        run(["./tf_linux64", "-game", "tf", "-steam", "-novid", "-nojoy",
             "-nosteamcontroller", "-nohltv", "-particles", "1",
             "-precachefontchars", "-noquicktime", "-nobreakpad" , "-gl"],
            cwd=TF2_DIR,
            env={**os.environ, "RUST_BACKTRACE": "FULL", "LD_LIBRARY_PATH": "bin/linux64"})
    print("[oxide] killing tf2");
    pid = get_pid()
    run(["kill","-9",pid])
    os.remove("/tmp/source_engine_2925226592.lock")



parser = argparse.ArgumentParser(
    prog='oxide toolbox')

parser.add_argument('action', choices=[
                    'inject',
                    'unload',
                    'build',
                    'start_tf2',
                    'reload',
                    ], default=inject)
parser.add_argument(
    '-d', '--debug', help='build for debug ', action='store_true')
args = parser.parse_args()

print(args)
match args.action:
    case 'inject':
        pid = get_pid()
        lib = get_lib(args.debug)

        inject(pid, lib, )
    case 'unload':
        pid = get_pid()

        unload(pid)
    case 'reload':
        pid = get_pid()
        lib = get_lib(args.debug)

        unload(pid)
        sleep(1)
        inject(pid, lib)
    case 'build':
        build(args.debug)
    case 'start_tf2':
        start_tf2()
