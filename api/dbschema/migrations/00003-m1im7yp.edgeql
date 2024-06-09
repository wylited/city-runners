CREATE MIGRATION m1im7ypjy4iicfjlm4a66teny6g3b2h5apridlhqykmf2y7tzjn4mq
    ONTO m12e3rcbprasgy7bx6qgdboaquibrtkbyfm63m6336d6xc35ymglva
{
  ALTER TYPE default::Player {
      CREATE PROPERTY jwt: std::str;
  };
};
