CREATE MIGRATION m1kutfpt7nhuh2qiep4lcwtqxal5wff6baitexlaixahgekdzvccoa
    ONTO m13r7rzmlxeut3nvvyx6vnh3pmufucbfysdsetn7zo274t42wgwnha
{
  ALTER TYPE default::Player {
      CREATE REQUIRED PROPERTY admin: std::bool {
          SET REQUIRED USING (false);
      };
  };
};
