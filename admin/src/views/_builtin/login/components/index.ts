import type { Component } from 'vue';
import { ref } from 'vue';
import { EnumLoginModule } from '@/enum';
import LoginBg from './LoginBg/index.vue';
import PwdLogin from './PwdLogin/index.vue';
import CodeLogin from './CodeLogin/index.vue';
import Register from './Register/index.vue';
import ResetPwd from './ResetPwd/index.vue';

interface LoginModule {
  key: EnumType.LoginModuleKey;
  label: EnumLoginModule;
  component: Component;
}

const modules: LoginModule[] = [
  { key: 'pwd-login', label: EnumLoginModule['pwd-login'], component: PwdLogin },
  { key: 'code-login', label: EnumLoginModule['code-login'], component: CodeLogin },
  { key: 'register', label: EnumLoginModule.register, component: Register },
  { key: 'reset-pwd', label: EnumLoginModule['reset-pwd'], component: ResetPwd }
];

const activeModule = ref(modules[0]);

function toLoginModule(mode: string) {
  const formItem = modules.find(item => item.key === mode) as LoginModule;
  activeModule.value = formItem;
}

export { LoginBg, PwdLogin, CodeLogin, Register, ResetPwd, activeModule, toLoginModule };
