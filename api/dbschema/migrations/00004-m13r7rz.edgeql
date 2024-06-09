CREATE MIGRATION m13r7rzmlxeut3nvvyx6vnh3pmufucbfysdsetn7zo274t42wgwnha
    ONTO m1im7ypjy4iicfjlm4a66teny6g3b2h5apridlhqykmf2y7tzjn4mq
{
  ALTER TYPE default::Player {
      DROP PROPERTY jwt;
  };
};
