CREATE MIGRATION m1kzievmpbwb2ohf6wtgbwsdf3s7pkrra6oc2pyqb3f6kn5qc7mmha
    ONTO initial
{
  CREATE EXTENSION pgcrypto VERSION '1.3';
  CREATE EXTENSION auth VERSION '1.0';
  CREATE TYPE default::Player {
      CREATE REQUIRED PROPERTY username: std::str;
  };
};
