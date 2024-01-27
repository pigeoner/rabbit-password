import Dexie from 'dexie';
/**
 * addAccount(account) 添加账号
 * updateAccount(account) 更新账号
 * deleteAccount(id) 删除账号
 * getAccount(id) 获取账号
 * getAccounts() 获取所有账号
 * searchAccounts(keyword) 搜索账号
 */
class DB {
  constructor() {
    this.db = new Dexie('password-data');
    /**
     *  id: Number,                 // 数据库自增id，插入时不需要
     *  name: String,               // 网站或服务名称
     *  url: String | null,         // 链接，如果服务不是网站则为null
     *  description: String | null, // 描述信息
     *  username: String,           // 登录用户名
     *  email: String | null,       // 邮箱
     *  phone: String | null,       // 手机号
     *  pwd: String,                // 密码
     *  last_update: String | null, // 最后更新时间
     */
    this.db.version(1).stores({
      accounts: '++id, name, url, description, username, email, phone, pwd, last_update'
    });
  }

  addAccount(account) {
    return this.db.accounts.add(account);
  }

  updateAccount(account) {
    return this.db.accounts.update(account.id, account);
  }

  updateAccounts(accounts) {
    return this.db.accounts.bulkUpdate(accounts);
  }

  deleteAccount(id) {
    return this.db.accounts.delete(id);
  }

  getAccount(id) {
    return this.db.accounts.get(id);
  }

  getAccounts() {
    return this.db.accounts.toArray();
  }

  searchAccounts(keyword) {
    return this.db.accounts
      .where('name')
      .startsWithIgnoreCase(keyword)
      .or('username')
      .startsWithIgnoreCase(keyword)
      .or('url')
      .startsWithIgnoreCase(keyword)
      .or('remark')
      .startsWithIgnoreCase(keyword)
      .toArray();
  }
}

export default new DB();
