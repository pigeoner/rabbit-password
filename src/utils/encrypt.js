import CryptoJS from 'crypto-js';

export default {
  encryptSHA512(str) {
    return CryptoJS.SHA512(str).toString();
  },

  encrypt(str, key) {
    const encryptedPassword = CryptoJS.AES.encrypt(str, key).toString();
    return encryptedPassword;
  },

  decrypt(str, key) {
    const decryptedPassword = CryptoJS.AES.decrypt(str, key).toString(CryptoJS.enc.Utf8);
    return decryptedPassword;
  }
};
