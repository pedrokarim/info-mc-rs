/**
 * Structure presets organized as a file tree.
 * Auto-generated from /static/structures/ directory.
 */

export interface StructureFolder {
  name: string;
  label: string;
  icon: string;
  files: { name: string; path: string }[];
}

export const STRUCTURE_PRESETS: StructureFolder[] = [
  {
    name: 'igloo',
    label: 'Igloo',
    icon: '🧊',
    files: [
      { name: 'igloo_bottom', path: 'igloo/igloo_bottom.nbt' },
      { name: 'igloo_middle', path: 'igloo/igloo_middle.nbt' },
      { name: 'igloo_top_trapdoor', path: 'igloo/igloo_top_trapdoor.nbt' },
      { name: 'igloo_top_no_trapdoor', path: 'igloo/igloo_top_no_trapdoor.nbt' },
    ],
  },
  {
    name: 'pillageroutpost',
    label: 'Pillager Outpost',
    icon: '⚔️',
    files: [
      { name: 'watchtower', path: 'pillageroutpost/watchtower.nbt' },
      { name: 'watchtower_overgrown', path: 'pillageroutpost/watchtower_overgrown.nbt' },
      { name: 'cage1', path: 'pillageroutpost/feature_cage1.nbt' },
      { name: 'cage2', path: 'pillageroutpost/feature_cage2.nbt' },
      { name: 'cage_with_allays', path: 'pillageroutpost/feature_cage_with_allays.nbt' },
      { name: 'logs', path: 'pillageroutpost/feature_logs.nbt' },
      { name: 'targets', path: 'pillageroutpost/feature_targets.nbt' },
      { name: 'tent1', path: 'pillageroutpost/feature_tent1.nbt' },
      { name: 'tent2', path: 'pillageroutpost/feature_tent2.nbt' },
    ],
  },
  {
    name: 'shipwreck',
    label: 'Shipwreck',
    icon: '🚢',
    files: [
      { name: 'full', path: 'shipwreck/swrightsideupfull.nbt' },
      { name: 'full_degraded', path: 'shipwreck/swrightsideupfulldegraded.nbt' },
      { name: 'with_mast', path: 'shipwreck/swwithmast.nbt' },
      { name: 'with_mast_degraded', path: 'shipwreck/swwithmastdegraded.nbt' },
      { name: 'front_half', path: 'shipwreck/swrightsideupfronthalf.nbt' },
      { name: 'back_half', path: 'shipwreck/swrightsideupbackhalf.nbt' },
      { name: 'sideways_full', path: 'shipwreck/swsidewaysfull.nbt' },
      { name: 'upside_down_full', path: 'shipwreck/swupsidedownfull.nbt' },
    ],
  },
  {
    name: 'ruined_portal',
    label: 'Ruined Portal',
    icon: '🟣',
    files: [
      { name: 'giant_portal_1', path: 'ruined_portal/giant_portal_1.nbt' },
      { name: 'giant_portal_2', path: 'ruined_portal/giant_portal_2.nbt' },
      { name: 'giant_portal_3', path: 'ruined_portal/giant_portal_3.nbt' },
      { name: 'portal_1', path: 'ruined_portal/portal_1.nbt' },
      { name: 'portal_2', path: 'ruined_portal/portal_2.nbt' },
      { name: 'portal_3', path: 'ruined_portal/portal_3.nbt' },
      { name: 'portal_4', path: 'ruined_portal/portal_4.nbt' },
      { name: 'portal_5', path: 'ruined_portal/portal_5.nbt' },
    ],
  },
  {
    name: 'fossils',
    label: 'Fossils',
    icon: '🦴',
    files: [
      { name: 'skull_01', path: 'fossils/fossil_skull_01.nbt' },
      { name: 'skull_02', path: 'fossils/fossil_skull_02.nbt' },
      { name: 'skull_03', path: 'fossils/fossil_skull_03.nbt' },
      { name: 'skull_04', path: 'fossils/fossil_skull_04.nbt' },
      { name: 'spine_01', path: 'fossils/fossil_spine_01.nbt' },
      { name: 'spine_02', path: 'fossils/fossil_spine_02.nbt' },
      { name: 'spine_03', path: 'fossils/fossil_spine_03.nbt' },
      { name: 'spine_04', path: 'fossils/fossil_spine_04.nbt' },
    ],
  },
  {
    name: 'nether_fossils',
    label: 'Nether Fossils',
    icon: '🔥',
    files: Array.from({ length: 14 }, (_, i) => ({
      name: `fossil_${i + 1}`,
      path: `nether_fossils/fossil_${i + 1}.nbt`,
    })),
  },
  {
    name: 'ruin',
    label: 'Ocean Ruins',
    icon: '🏛️',
    files: [
      { name: 'big_ruin1_brick', path: 'ruin/big_ruin1_brick.nbt' },
      { name: 'big_ruin2_mossy', path: 'ruin/big_ruin2_mossy.nbt' },
      { name: 'big_ruin3_cracked', path: 'ruin/big_ruin3_cracked.nbt' },
      { name: 'warm4', path: 'ruin/big_ruin_warm4.nbt' },
      { name: 'warm5', path: 'ruin/big_ruin_warm5.nbt' },
      { name: 'ruin1_brick', path: 'ruin/ruin1_brick.nbt' },
      { name: 'ruin_warm1', path: 'ruin/ruin_warm1.nbt' },
      { name: 'ruin_warm2', path: 'ruin/ruin_warm2.nbt' },
    ],
  },
  {
    name: 'coralcrust',
    label: 'Coral Crust',
    icon: '🪸',
    files: [
      { name: 'crust1', path: 'coralcrust/crust1.nbt' },
      { name: 'crust2', path: 'coralcrust/crust2.nbt' },
      { name: 'crust3', path: 'coralcrust/crust3.nbt' },
      { name: 'outcropping1', path: 'coralcrust/outcropping1.nbt' },
      { name: 'outcropping2', path: 'coralcrust/outcropping2.nbt' },
      { name: 'outcropping3', path: 'coralcrust/outcropping3.nbt' },
    ],
  },
];
