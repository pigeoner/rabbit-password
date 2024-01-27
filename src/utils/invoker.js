// import { invoke } from '@tauri-apps/api/tauri';
import db from './db';

export default function invoker(cmd, arg, sucFunc, errFunc) {
  return db[cmd](arg)
    .then(res => {
      sucFunc && sucFunc(res);
    })
    .catch(err => {
      ElMessage.error(err?.message || err);
      errFunc && errFunc(res);
    });
}
