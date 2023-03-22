import { computed, ref } from 'vue';
import { REGEXP_EMAIL } from '@/config';
import { fetchSendEmail } from '@/service';
import { useLoading } from '../common';
import useCountDown from './useCountDown';

export default function useSmsCode() {
  const { loading, startLoading, endLoading } = useLoading();
  const { counts, start, isCounting } = useCountDown(60);

  const tokenCode = ref('');
  const initLabel = '获取验证码';
  const countingLabel = (second: number) => `${second}秒后重新获取`;
  const label = computed(() => {
    let text = initLabel;
    if (loading.value) {
      text = '';
    }
    if (isCounting.value) {
      text = countingLabel(counts.value);
    }
    return text;
  });

  /** 判断邮箱地址是否正确 */
  function isEmailValid(email: string) {
    let valid = true;
    if (email.trim() === '') {
      window.$message?.error('邮箱地址不能为空！');
      valid = false;
    } else if (!REGEXP_EMAIL.test(email)) {
      window.$message?.error('邮箱地址格式错误！');
      valid = false;
    }
    return valid;
  }

  /**
   * 获取邮箱验证码
   */
  async function getEmailCode(email: string) {
    const valid = isEmailValid(email);
    if (!valid || loading.value) return;

    startLoading();
    const { data } = await fetchSendEmail(email);
    if (data) {
      window.$message?.success('验证码发送成功！');
      tokenCode.value = data.code;
      start();
    } else {
      tokenCode.value = '';
      window.$message?.error('验证码发送失败！');
    }
    endLoading();
  }

  return {
    label,
    start,
    isCounting,
    getEmailCode,
    loading,
    tokenCode
  };
}
