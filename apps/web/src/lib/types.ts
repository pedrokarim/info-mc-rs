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
  popularity?: Popularity;
}

export interface SkinResponse {
  url: string;
  model: string;
}

export interface CapeResponse {
  url: string;
  active?: boolean;
}

export interface PlayerApiResponse {
  uuid: string;
  username: string;
  skin?: SkinResponse;
  cape?: CapeResponse;
  optifine_cape?: CapeResponse;
  labymod_cape?: CapeResponse;
  retrieved_at: string;
  popularity?: Popularity;
}

export interface Popularity {
  views: number;
  likes: number;
  first_seen_at: string;
  last_seen_at: string;
}

export interface PopularPlayerEntry {
  uuid: string;
  username: string;
  skin_url?: string;
  skin_model?: string;
  views: number;
  likes: number;
  first_seen_at: string;
  last_seen_at: string;
}

export interface PopularServerEntry {
  address: string;
  hostname: string;
  port: number;
  edition: string;
  version_name?: string;
  motd_clean?: string;
  favicon?: string;
  max_players?: number;
  views: number;
  likes: number;
  first_seen_at: string;
  last_seen_at: string;
  last_online_at?: string;
}

export interface LikeStatus {
  liked: boolean;
}

export interface ApiErrorResponse {
  error?: string;
  message?: string;
}
