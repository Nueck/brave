import type { Component } from 'vue';
import { ref } from 'vue';
import { EnumInitModule } from '@/enum';
import Bg from './Bg/index.vue';
import User from './User/index.vue';

interface InitModule {
  key: EnumType.InitModuleKey;
  label: EnumInitModule;
  component: Component;
}

const modules: InitModule[] = [{ key: 'user', label: EnumInitModule.user, component: User }];

const activeModule = ref(modules[0]);

function toInitModule(mode: string) {
  const formItem = modules.find(item => item.key === mode) as InitModule;
  activeModule.value = formItem;
}

export { Bg, activeModule, toInitModule };
