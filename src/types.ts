export type Settings = {
  weather_city_id: string;
  atcoder_id: string;
  widget_interval: number;
  using_widgets: string[];
  auto_fullscreen: boolean;
  auto_hide_cursor: boolean;
  using_sports_news: string[];
  nightmode_range: NightmodeRange;
  use_sound_when_disaster: boolean;
};

type NightmodeRange = {
  start: string;
  end: string;
};
