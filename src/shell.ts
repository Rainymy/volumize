import { exec, execFile, type ExecException } from "node:child_process";

export function spawnExec(cmd: string): Promise<string | ExecException> {
  return new Promise((resolve, reject) => {
    exec(cmd, (error, stdout, stderr) => {
      if (error) reject(error);
      else resolve(stdout);
    });
  })
}

export function spawnExecFile(file: string, args: string[]): Promise<string | ExecException> {
  return new Promise((resolve, reject) => {
    execFile(file, args, (error, stdout, stderr) => {
      if (error) reject(error);
      else resolve(stdout);
    });
  })
}