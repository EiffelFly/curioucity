CREATE MIGRATION m1s7ynu4odlu2iftaufaa7g6yinvsfonbnlamtotbukn77nkkobozq
    ONTO m1lbs4mm7rrws6wq725g2owyv4x45rxs2xdc4go4jbqupal5egusyq
{
  ALTER TYPE default::Url {
      CREATE LINK resource := (.<url);
  };
};
