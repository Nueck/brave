import { ERROR_MSG_DURATION, NO_ERROR_MSG_CODE } from '@/config';

/** 消息栈，防止同一消息同时出现 */
const msgStack = new Map<string | number, string>([]);

function addMsg(msg: Service.RequestError) {
  msgStack.set(msg.code, msg.msg);
}
function removeMsg(msg: Service.RequestError) {
  msgStack.delete(msg.code);
}
function hasMsg(msg: Service.RequestError) {
  return msgStack.has(msg.code);
}

/**
 * 显示错误信息
 * @param error
 */
export function showErrorMsg(error: Service.RequestError) {
  if (!error.msg || NO_ERROR_MSG_CODE.includes(error.code) || hasMsg(error)) return;

  addMsg(error);
  window.console.warn(error.code, error.msg);
  window.$message?.error(error.msg, { duration: ERROR_MSG_DURATION });
  setTimeout(() => {
    removeMsg(error);
  }, ERROR_MSG_DURATION);
}
