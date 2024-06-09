CREATE MIGRATION m12e3rcbprasgy7bx6qgdboaquibrtkbyfm63m6336d6xc35ymglva
    ONTO m1kzievmpbwb2ohf6wtgbwsdf3s7pkrra6oc2pyqb3f6kn5qc7mmha
{
  ALTER TYPE default::Player {
      CREATE REQUIRED PROPERTY password: std::str {
          SET REQUIRED USING (<std::str>{});
      };
      ALTER PROPERTY username {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
