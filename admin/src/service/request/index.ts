import { getServiceEnvConfig } from '~/.env-config';
import { createRequest } from './request';

const { url } = getServiceEnvConfig(import.meta.env);

export const basicRequest = createRequest({
  baseURL: url
});

export const mockRequest = createRequest({
  baseURL: '/mock'
});
