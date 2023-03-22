import type { Ref } from 'vue';
import type { FormItemRule } from 'naive-ui';
import { REGEXP_CODE_SIX, REGEXP_EMAIL, REGEXP_PWD } from '@/config';
import { REGEXP_USER } from './../../config/regexp';

/** 创建自定义错误信息的必填表单规则 */
export const createRequiredFormRule = (message = '不能为空'): FormItemRule => ({ required: true, message });

export const requiredFormRule = createRequiredFormRule();

/** 表单规则 */
interface CustomFormRules {
  /** 密码 */
  pwd: FormItemRule[];
  /** 验证码 */
  code: FormItemRule[];
  /** 邮箱 */
  email: FormItemRule[];
  /* 用户名 */
  user: FormItemRule[];
}

/** 表单规则 */
export const formRules: CustomFormRules = {
  user: [
    createRequiredFormRule('请输入用户名'),
    {
      key: 'user',
      pattern: REGEXP_USER,
      message: '用户名为2-8位数字和字母的组合',
      trigger: 'input'
    }
  ],
  email: [
    createRequiredFormRule('请输入邮箱地址'),
    { key: 'email', pattern: REGEXP_EMAIL, message: '邮箱地址格式错误', trigger: 'input' }
  ],
  pwd: [
    createRequiredFormRule('请输入密码'),
    { key: 'pwd', pattern: REGEXP_PWD, message: '密码为6-18位数字/字符/符号，至少2种组合', trigger: 'input' }
  ],
  code: [
    createRequiredFormRule('请输入验证码'),
    { key: 'code', pattern: REGEXP_CODE_SIX, message: '验证码格式错误', trigger: 'input' }
  ]
};

/** 是否为空字符串 */
function isBlankString(str: string) {
  return str.trim() === '';
}

/** 获取确认密码的表单规则 */
export function getConfirmPwdRule(pwd: Ref<string>) {
  const confirmPwdRule: FormItemRule[] = [
    { required: true, message: '请输入确认密码' },
    {
      validator: (rule, value) => {
        if (!isBlankString(value) && value !== pwd.value) {
          return Promise.reject(rule.message);
        }
        return Promise.resolve();
      },
      message: '输入的值与密码不一致',
      trigger: 'input'
    }
  ];
  return confirmPwdRule;
}

/** 获取图片验证码的表单规则 */
export function getImgCodeRule(imgCode: Ref<string>) {
  const imgCodeRule: FormItemRule[] = [
    { required: true, message: '请输入验证码' },
    {
      validator: (rule, value) => {
        if (!isBlankString(value) && value !== imgCode.value) {
          return Promise.reject(rule.message);
        }
        return Promise.resolve();
      },
      message: '验证码不正确',
      trigger: 'blur'
    }
  ];
  return imgCodeRule;
}
