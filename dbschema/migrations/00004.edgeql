CREATE MIGRATION m1uyllkkomeiajwtqtpcmch3mvp4ywm2znnk4oxz7e567aggmoajca
    ONTO m1rsdiu3xnjpvpy4gszae3w6ypbpi3gviisdvz4tkdkplkipuxf3ma
{
  ALTER TYPE default::DiscordThread {
      ALTER PROPERTY thread_channel_id {
          RENAME TO thread_id;
      };
  };
};
