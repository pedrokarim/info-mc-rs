export interface MotdPreset {
  name: string;
  description: string;
  edition: 'java' | 'bedrock' | 'both';
  motd: string; // legacy § format, \n for line break
}

export const MOTD_PRESETS: MotdPreset[] = [
  {
    name: 'Hypixel Classic',
    description: 'Style reseau populaire avec mini-jeux',
    edition: 'both',
    motd: '§f                §aHypixel Network §c[1.8-1.21]\n§f         §6§lBED WARS§r §7| §b§lSKYWARS§r §7| §e§lBUILD BATTLE',
  },
  {
    name: 'Mineplex Style',
    description: 'Banniere competitive avec bordures barrees',
    edition: 'java',
    motd: '§f            §b§l§m   §r §9§lMINE§f§lPLEX§r §b§l§m   §r\n§f              §e§lNEW UPDATE§r §7- §aJoin Now!',
  },
  {
    name: 'Gradient Sunset',
    description: 'Degrade de couleurs chaudes',
    edition: 'both',
    motd: '§f       §4§l\u00BB §c§lF§6§lI§e§lR§a§lE §2§lC§b§lR§3§lA§9§lF§5§lT §4§l\u00AB\n§f          §7\u27A4 §eSurvival §7\u2022 §6Skyblock §7\u2022 §cPvP',
  },
  {
    name: 'Clean Minimal',
    description: 'Epure et professionnel',
    edition: 'both',
    motd: '§8§l\u2503 §b§lMyServer §8§l\u2503 §7Survival & Creative\n§8§l\u2503 §a\u2714 §7Online §8- §f1.21.4 §8- §d/help §8§l\u2503',
  },
  {
    name: 'PvP Arena',
    description: 'Theme agressif pour serveurs PvP',
    edition: 'java',
    motd: '§4§l\u2694 §c§lBLOOD§f§lCRAFT §4§lPVP §4§l\u2694\n§7§o"No mercy." §8| §c§lKitPvP §8| §4§lFactions §8| §6§lUHC',
  },
  {
    name: 'Survival Paradise',
    description: 'Ambiance nature et aventure',
    edition: 'both',
    motd: '§f    §2§l\u2605 §a§lGreen§2§lValley §aSurvival §2§l\u2605\n§f    §7\u2600 §eTowny §7\u25CF §bMcMMO §7\u25CF §dQuests §7\u25CF §6Economy',
  },
  {
    name: 'Fancy Bordered',
    description: 'Encadre decoratif avec banniere',
    edition: 'both',
    motd: '§b§l\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\n§f    §e§l\u2726 §6§lEPIC§e§lCRAFT §7- §fSeason 4 §e§l\u2726',
  },
  {
    name: 'Obfuscated Hype',
    description: 'Texte anime pour attirer l\'attention',
    edition: 'both',
    motd: '§d§k|§r §5§lNEXUS§d§lMC §f§lNETWORK §d§k|§r §7[1.8-1.21]\n§f     §a§l\u27A4 §e§lDOUBLE XP WEEKEND! §a§l\u25C4',
  },
  {
    name: 'Creative Hub',
    description: 'Colore et accueillant pour serveurs creatifs',
    edition: 'both',
    motd: '§f  §6§l\u273F §e§lCreative§6§lWorld §e§l\u273F §7- §fBuild Anything!\n§f  §b§lPlots §7| §a§lWorldEdit §7| §d§lContests §7| §c§l\u2665 Free',
  },
  {
    name: 'Winter Event',
    description: 'Theme hivernal avec flocons',
    edition: 'both',
    motd: '§f§l  \u2744 §b§lFrost§f§lCraft §b§l\u2744 §7\u2014 §fWinter Event!\n§f     §3\u2603 §7Snow Wars §3\u2022 §7Ice Race §3\u2022 §7Gifts §3\u2603',
  },
  {
    name: 'Bedrock Simple',
    description: 'Compatible Bedrock, couleurs simples',
    edition: 'bedrock',
    motd: '§a§lMyCraft Server §7- §fJava & Bedrock\n§eSurvival §7| §bCreative §7| §cMinigames §7| §d1.21',
  },
  {
    name: 'Arrow Banner',
    description: 'Fleches et degrade bleu-violet',
    edition: 'both',
    motd: '§9§l\u300B §1§l\u300B §5§l\u300B §d§lSTARLIGHT §5§lNETWORK §5§l\u300A §1§l\u300A §9§l\u300A\n§f       §7§oA new adventure awaits... §e§l\u2605',
  },
];
