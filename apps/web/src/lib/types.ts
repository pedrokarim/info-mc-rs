export type ServerEdition = 'java' | 'bedrock' | 'auto';

export interface MotdRendered {
  raw: string;
  clean: string;
  html: string;
}

export interface AddressInfo {
  hostname: string;
  ip: string;
  port: number;
  srv_record: boolean;
}

export interface VersionInfo {
  name: string;
  protocol: number;
}

export interface PlayerSampleInfo {
  name: string;
  uuid: string;
}

export interface PlayersInfo {
  online: number;
  max: number;
  sample?: PlayerSampleInfo[];
}

export interface ServerApiResponse {
  online: boolean;
  address: AddressInfo;
  version?: VersionInfo;
  players?: PlayersInfo;
  motd?: MotdRendered;
  favicon?: string;
  latency_ms?: number;
  edition: 'java' | 'bedrock';
  error?: string;
  retrieved_at: string;
}

export interface SkinResponse {
  url: string;
  model: string;
}

export interface CapeResponse {
  url: string;
}

export interface PlayerApiResponse {
  uuid: string;
  username: string;
  skin?: SkinResponse;
  cape?: CapeResponse;
  optifine_cape?: CapeResponse;
  retrieved_at: string;
}

export interface ApiErrorResponse {
  error?: string;
  message?: string;
}
