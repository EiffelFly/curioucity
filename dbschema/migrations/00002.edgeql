CREATE MIGRATION m1jgmqhuzr634rnjocqthacnbminfdh3vba3ccx7sxdf4ij3qmkcjq
    ONTO m17f5q5dvzwvetpnjnjn2k75zob7pb4653fxbyvm2fuwy64xvhdmpq
{
  ALTER TYPE default::Tag {
      ALTER PROPERTY name {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
