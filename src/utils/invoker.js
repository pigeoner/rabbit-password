import { invoke } from '@tauri-apps/api/tauri';

export default function invoker(cmd, args, sucFunc, errFunc) {
  return invoke(cmd, args)
    .then(res => {
      if (res.code === 0) {
        sucFunc(res);
      } else {
        return Promise.reject(res.msg);
      }
    })
    .catch(err => {
      ElMessage.error(err.message || err);
      errFunc ? errFunc(res) : null;
    });
}
