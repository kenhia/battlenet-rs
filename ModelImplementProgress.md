
# Progress of API Wrapper Implementation

> **Last audited against official API docs**: 2026-04-09

## World of Warcraft (Retail) Profile APIs

### Account Profile

| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| *Account Profile Summary* | `profile/user/wow` | `account_profile.rs` | ✅ | ❌ |
| *Protected Character Profile Summary* | `profile/user/wow/protected-character/{realmId}-{characterId}` | `account_profile.rs` | ✅ | ❌ |
| *Account Collections Index* | `profile/user/wow/collections` | `account_profile.rs` | ✅ | ❌ |
| *Account Mount Collections* | `profile/user/wow/collections/mounts` | `account_profile.rs` | ✅ | ❌ |
| *Account Pets Collection* | `profile/user/wow/collections/pets` | `account_profile.rs` | ✅ | ❌ |
| *Account Toys Collection* | `profile/user/wow/collections/toys` | `account_profile.rs` | ✅ | ❌ |
| *Account Heirlooms Collection* | `profile/user/wow/collections/heirlooms` | `account_profile.rs` | ✅ | ❌ |
| *Account Decor Collection Summary* | `profile/user/wow/collections/decor` | `account_profile.rs` | ✅ | ❌ |
| *Account Transmog Collection Summary* | `profile/user/wow/collections/transmogs` | `account_profile.rs` | ✅ | ❌ |

### Character Achievements

| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Achievements Summary | `profile/wow/character/{realmSlug}/{characterName}/achievements` | `character_achievements.rs` | ✅ | ❌ |
| Character Achievements Statistics | `profile/wow/character/{realmSlug}/{characterName}/achievements/statistics` | `character_achievements.rs` | ✅ | ❌ |

### Character Appearance

| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Appearance Summary | `profile/wow/character/{realmSlug}/{characterName}/appearance` | `character_appearance.rs` | ✅ | ❌ |

### Character Collections
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Collections Index | `profile/wow/character/{realmSlug}/{characterName}/collections` | `character_collections.rs` | ✅ | ❌ |
| Character Mounts Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/mounts` | `character_collections.rs` | ✅ | ❌ |
| Character Pets Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/pets` | `character_collections.rs` | ✅ | ❌ |
| Character Toys Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/toys` | `character_collections.rs` | ✅ | ❌ |
| Character Heirlooms Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/heirlooms` | `character_collections.rs` | ✅ | ❌ |


### Character Encounters
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Encounters Summary | `profile/wow/character/{realmSlug}/{characterName}/encounters` | `character_encounters.rs` | ✅ | ❌ |
| Character Dungeons | `profile/wow/character/{realmSlug}/{characterName}/encounters/dungeons` | `character_encounters.rs` | ✅ | ❌ |
| Character Raids | `profile/wow/character/{realmSlug}/{characterName}/encounters/raids` | `character_encounters.rs` | ✅ | ❌ |

### Character Equipment
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Equipment Summary | `profile/wow/character/{realmSlug}/{characterName}/equipment` | `character_equipment.rs` | ✅ | ❌ |

### Character Hunter Pets
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Hunter Pets Summary | `profile/wow/character/{realmSlug}/{characterName}/hunter-pets` | `character_hunter_pets.rs` | ✅ | ❌ |

### Character Media
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Media Summary | `profile/wow/character/{realmSlug}/{characterName}/character-media` | `character_media.rs` | ✅ | ❌ |

### Character Mythic Keystone Profile
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Mythic Keystone Profile Index | `profile/wow/character/{realmSlug}/{characterName}/mythic-keystone-profile` | `character_mythic_keystone.rs` | ✅ | ❌ |
| Character Mythic Keystone Season Details | `profile/wow/character/{realmSlug}/{characterName}/mythic-keystone-profile/season/{seasonId}` | `character_mythic_keystone.rs` | ✅ | ❌ |

### Character Professions
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Professions Summary | `profile/wow/character/{realmSlug}/{characterName}/professions` | `character_professions.rs` | ✅ | ❌ |

### Character Profile
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Profile Summary | `profile/wow/character/{realmSlug}/{characterName}` | `character_profile.rs` | ✅ | ❌ |
| Character Profile Status | `profile/wow/character/{realmSlug}/{characterName}/status` | `character_profile.rs` | ✅ | ❌ |

### Character PvP
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character PvP Bracket Statistics | `profile/wow/character/{realmSlug}/{characterName}/pvp-bracket/{pvpBracket}` | `character_pvp.rs` | ✅ | ❌ |
| Character PvP Summary | `profile/wow/character/{realmSlug}/{characterName}/pvp-summary` | `character_pvp.rs` | ✅ | ❌ |

### Character Quests
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Quests | `profile/wow/character/{realmSlug}/{characterName}/quests` | `character_quests.rs` | ✅ | ❌ |
| Character Completed Quests | `profile/wow/character/{realmSlug}/{characterName}/quests/completed` | `character_quests.rs` | ✅ | ❌ |

### Character Reputations
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Reputations Summary | `profile/wow/character/{realmSlug}/{characterName}/reputations` | `character_reputations.rs` | ✅ | ❌ |

### Character Soulbinds
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Soulbinds | `profile/wow/character/{realmSlug}/{characterName}/soulbinds` | `character_soulbinds.rs` | ✅ | ❌ |

### Character Specializations
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Specializations Summary | `profile/wow/character/{realmSlug}/{characterName}/specializations` | `character_specializations.rs` | ✅ | ❌ |

### Character Statistics
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Statistics Summary | `profile/wow/character/{realmSlug}/{characterName}/statistics` | `character_statistics.rs` | ✅ | ❌ |

### Character Titles
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Titles Summary | `profile/wow/character/{realmSlug}/{characterName}/titles` | `character_titles.rs` | ✅ | ❌ |

### Guild
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Guild | `data/wow/guild/{realmSlug}/{nameSlug}` | `guild.rs` | ✅ | ❌ |
| Guild Activity | `data/wow/guild/{realmSlug}/{nameSlug}/activity` | `guild.rs` | ✅ | ❌ |
| Guild Achievements | `data/wow/guild/{realmSlug}/{nameSlug}/achievements` | `guild.rs` | ✅ | ❌ |
| Guild Roster | `data/wow/guild/{realmSlug}/{nameSlug}/roster` | `guild.rs` | ✅ | ❌ |


## World of Warcraft (Retail) Game Data APIs

### Achievement
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Achievement Categories Index | `data/wow/achievement-category/index` | `achievement.rs` | ✅ | ✅ |
| Achievement Category | `data/wow/achievement-category/{achievementCategoryId}` | `achievement.rs` | ✅ | ✅ |
| Achievement Index | `data/wow/achievement/index` | `achievement.rs` | ✅ | ✅ |
| Achievement | `data/wow/achievement/{achievementId}` | `achievement.rs` | ✅ | ✅ |
| Achievement Media | `data/wow/media/achievement/{achievementId}` | `achievement.rs` | ✅ | ✅ |

### Auction House
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Auctions | `data/wow/connected-realm/{connectedRealmId}/auctions` | `auction_house.rs` | ✅ | ❌ |
| Commodities | `data/wow/auctions/commodities` | `auction_house.rs` | ✅ | ❌ |

### Azerite Essence
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Azerite Essences Index | `data/wow/azerite-essence/index` | `azerite_essence.rs` | ✅ | ❌ |
| Azerite Essence | `data/wow/azerite-essence/{azeriteEssenceId}` | `azerite_essence.rs` | ✅ | ❌ |
| Azerite Essence Search | `data/wow/search/azerite-essence` | `azerite_essence.rs` | ✅ | ❌ |
| Azerite Essence Media | `data/wow/media/azerite-essence/{azeriteEssenceId}` | `azerite_essence.rs` | ✅ | ❌ |

### Connected Realms
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Connected Realms Index | `data/wow/connected-realm/index` | `connected_realm.rs` | ✅ | ✅ |
| Connected Realms | `data/wow/connected-realm/{connectedRealmId}` | `connected_realm.rs` | ✅ | ✅ |
| Connected Realms Search | `data/wow/search/connected-realm` | `connected_realm.rs` | ✅ | ❌ |

### Covenant
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Covenant Index | `data/wow/covenant/index` | `covenant.rs` | ✅ | ❌ |
| Covenant | `data/wow/covenant/{covenantId}` | `covenant.rs` | ✅ | ❌ |
| Covenant Media | `data/wow/media/covenant/{covenantId}` | `covenant.rs` | ✅ | ❌ |
| Soulbind Index | `data/wow/covenant/soulbind/index` | `covenant.rs` | ✅ | ❌ |
| Soulbind | `data/wow/covenant/soulbind/{soulbindId}` | `covenant.rs` | ✅ | ❌ |
| Conduit Index | `data/wow/covenant/conduit/index` | `covenant.rs` | ✅ | ❌ |
| Conduit | `data/wow/covenant/conduit/{conduitId}` | `covenant.rs` | ✅ | ❌ |

### Creature
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Creature Families Index | `data/wow/creature-family/index` | `creature.rs` | ✅ | ❌ |
| Creature Family | `data/wow/creature-family/{creatureFamilyId}` | `creature.rs` | ✅ | ❌ |
| Creature Types Index | `data/wow/creature-type/index` | `creature.rs` | ✅ | ❌ |
| Creature Types | `data/wow/creature-type/{creatureTypeId}` | `creature.rs` | ✅ | ❌ |
| Creature Search | `data/wow/search/creature` | `creature.rs` | ✅ | ❌ |
| Creature Display Media | `data/wow/media/creature-display/{creatureDisplayId}` | `creature.rs` | ✅ | ❌ |
| Creature Family Media | `data/wow/media/creature-family/{creatureFamilyId}` | `creature.rs` | ✅ | ❌ |

### Guild Crest
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Guild Crest Components Index | `data/wow/guild-crest/index` | `guild.rs` | ✅ | ❌ |
| Guild Crest Border Media | `data/wow/guild-crest/border/{borderId}` | `guild.rs` | ✅ | ❌ |
| Guild Crest Emblem Media | `data/wow/guild-crest/border/{emblemId}` | `guild.rs` | ✅ | ❌ |

### Heirloom
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Heirloom Index | `data/wow/heirloom/index` | `heirloom.rs` | ✅ | ❌ |
| Heirloom | `data/wow/heirloom/{heirloomId}` | `heirloom.rs` | ✅ | ❌ |

### Item
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Item Classes Index | `data/wow/item-class/index` | `item.rs` | ✅ | ❌ |
| Item Class | `data/wow/item-class/{itemClassId}` | `item.rs` | ✅ | ❌ |
| Item Sets Index | `data/wow/item-set/index` | `item.rs` | ✅ | ❌ |
| Item Set | `data/wow/item-set/{itemSetId}` | `item.rs` | ✅ | ❌ |
| Item Subclass | `data/wow/item-class/{itemClassId}/item-subclass/{itemSubclassId}` | `item.rs` | ✅ | ❌ |
| Item | `data/wow/item/{itemId}` | `item.rs` | ✅ | ❌ |
| Item Media | `data/wow/media/item/{itemId}` | `item.rs` | ✅ | ❌ |
| Item Search | `data/wow/search/item` | `item.rs` | ✅ | ❌ |

### Journal
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Journal Expansions Index | `data/wow/journal-expansion/index` | `journal.rs` | ✅ | ❌ |
| Journal Expansions | `data/wow/journal-expansion/{journalExpansionId}` | `journal.rs` | ✅ | ❌ |
| Journal Encounters Index | `data/wow/journal-encounters/index` | `journal.rs` | ✅ | ❌ |
| Journal Encounter | `data/wow/journal-encounters/{journalEncounterId}` | `journal.rs` | ✅ | ❌ |
| Journal Encounter Search | `data/wow/search/journal-encounter` | `journal.rs` | ✅ | ❌ |
| Journal Instances Index | `data/wow/journal-instance/index` | `journal.rs` | ✅ | ❌ |
| Journal Instance | `data/wow/journal-instance/{journalInstanceId}` | `journal.rs` | ✅ | ❌ |
| Journal Instance Media | `data/wow/media/journal-instance/{journalInstanceId}` | `journal.rs` | ✅ | ❌ |

### Media Search
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Media Search | `data/wow/search/media` | `media_search.rs` | ✅ | ❌ |

### Modified Crafting
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Modified Crafting Index | `data/wow/modified-crafting/index` | `modified_crafting.rs` | ✅ | ❌ |
| Modified Crafting Category Index | `data/wow/modified-crafting/category/index` | `modified_crafting.rs` | ✅ | ❌ |
| Modified Crafting Category | `data/wow/modified-crafting/category/{categoryId}` | `modified_crafting.rs` | ✅ | ❌ |
| Modified Crafting Reagent Slot Type Index | `data/wow/modified-crafting/reagent-slot-type/index` | `modified_crafting.rs` | ✅ | ❌ |
| Modified Crafting Reagent Slot Type | `data/wow/modified-crafting/reagent-slot-type/{slotTypeId}` | `modified_crafting.rs` | ✅ | ❌ |

### Mount
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mounts Index | `data/wow/mount/index` | `mount.rs` | ✅ | ❌ |
| Mount | `data/wow/mount/{mountId}` | `mount.rs` | ✅ | ❌ |
| Mount Search | `data/wow/search/mount` | `mount.rs` | ✅ | ❌ |

### Mythic Keystone Affix
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Keystone Affixes Index | `data/wow/keystone-affix/index` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Affix | `data/wow/keystone-affix/{keystoneAffixId}` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Affix Media | `data/wow/media/keystone-affix/{keystoneAffixId}` | `mythic_keystone.rs` | ✅ | ❌ |

### Mythic Keystone Dungeon
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Keystone Dungeon Index | `data/wow/mythic-keystone/dungeon/index` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Dungeon | `data/wow/mythic-keystone/dungeon/{dungeonId}` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Index | `data/wow/mythic-keystone/index` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Periods Index | `data/wow/mythic-keystone/period/index` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Period | `data/wow/mythic-keystone/period/{periodId}` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Seasons Index | `data/wow/mythic-keystone/season/index` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Season | `data/wow/mythic-keystone/season/{seasonId}` | `mythic_keystone.rs` | ✅ | ❌ |

### Mythic Keystone Leaderboard
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Keystone Leaderboards Index | `data/wow/connected-realm/{connectedRealmId}/mythic-leaderboard/index` | `mythic_keystone.rs` | ✅ | ❌ |
| Mythic Keystone Leaderboard | `data/wow/connected-realm/{connectedRealmId}/mythic-leaderboard/{dungeonId}/period/{period}` | `mythic_keystone.rs` | ✅ | ❌ |

### Mythic Raid Leaderboard
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Raid Leaderboard | `data/wow/hall-of-fame/{raid}/{faction}` | `mythic_keystone.rs` | ✅ | ❌ |

### Pet
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Pets Index | `data/wow/pet/index` | `pet.rs` | ✅ | ❌ |
| Pet | `data/wow/mount/{petId}` | `pet.rs` | ✅ | ❌ |
| Pet Media | `data/wow/media/{petId}` | `pet.rs` | ✅ | ❌ |
| Pet Abilities Index | `data/wow/pet-ability/index` | `pet.rs` | ✅ | ❌ |
| Pet Ability | `data/wow/pet-ability/{petAbilityId}` | `pet.rs` | ✅ | ❌ |
| Pet Ability | `data/wow/media/pet-ability/{petAbilityId}` | `pet.rs` | ✅ | ❌ |


### Playable Class
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Playable Classes Index | `data/wow/playable-class/index` | `playable_class.rs` | ✅ | ❌ |
| Playable Class | `data/wow/playable-class/{classID}` | `playable_class.rs` | ✅ | ❌ |
| Playable Class Media | `data/wow/media/playable-class/{classID}` | `playable_class.rs` | ✅ | ❌ |
| PvP Talent Slots | `data/wow/media/playable-class/{classID}/pvp-talent-slots` | `playable_class.rs` | ✅ | ❌ |

### Playable Race
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Playable Races Index | `data/wow/playable-race/index` | `playable_race.rs` | ✅ | ❌ |
| Playable Race | `data/wow/playable-race/{playableRaceId}` | `playable_race.rs` | ✅ | ❌ |

### Playable Specialization
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Playable Specializations Index | `data/wow/playable-specialization/index` | `playable_spec.rs` | ✅ | ❌ |
| Playable Specialization | `data/wow/playable-specialization/{specId}` | `playable_spec.rs` | ✅ | ❌ |
| Playable Specialization Media | `data/wow/media/playable-specialization/{specId}` | `playable_spec.rs` | ✅ | ❌ |

### Power Type
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Power Types Index | `data/wow/power-type/index` | `power_type.rs` | ✅ | ❌ |
| Power Type | `data/wow/power-type/{powerTypeId}` | `power_type.rs` | ✅ | ❌ |

### Profession
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Professions Index | `data/wow/profession/index` | `profession.rs` | ✅ | ❌ |
| Profession | `data/wow/profession/{professionId}` | `profession.rs` | ✅ | ❌ |
| Profession Media | `data/wow/media/profession/{professionId}` | `profession.rs` | ✅ | ❌ |
| Profession Skill Tier | `data/wow/profession/{professionId}/skill-tier/{skillTierId}` | `profession.rs` | ✅ | ❌ |
| Recipe | `data/wow/recipe/{recipeId}` | `profession.rs` | ✅ | ❌ |
| Recipe Media | `data/wow/media/recipe/{recipeId}` | `profession.rs` | ✅ | ❌ |

### PvP Season
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| PvP Seasons Index | `data/wow/pvp-season/index` | `pvp.rs` | ✅ | ❌ |
| PvP Season | `data/wow/pvp-season/{pvpSeasonId}` | `pvp.rs` | ✅ | ❌ |
| PvP Leaderboards Index | `data/wow/pvp-season/{pvpSeasonId}/pvp-leaderboard/index` | `pvp.rs` | ✅ | ❌ |
| PvP Leaderboard | `data/wow/pvp-season/{pvpSeasonId}/pvp-leaderboard/{pvpBracket}` | `pvp.rs` | ✅ | ❌ |
| PvP Rewards Index | `data/wow/pvp-season/{pvpSeasonId}/pvp-reward/index` | `pvp.rs` | ✅ | ❌ |

### PvP Tier
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| PvP Tier Media | `data/wow/media/pvp-tier/{pvpTierId}` | `pvp.rs` | ✅ | ❌ |
| PvP Tiers Index | `data/wow/pvp-tier/index` | `pvp.rs` | ✅ | ❌ |
| PvP Tier | `data/wow/pvp-tier/{pvpTierId}` | `pvp.rs` | ✅ | ❌ |

### Quests
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Quests Index | `data/wow/quests/index` | `quest.rs` | ✅ | ❌ |
| Quest | `data/wow/quests/{questId}` | `quest.rs` | ✅ | ❌ |
| Quest Categories Index | `data/wow/quests/category/index` | `quest.rs` | ✅ | ❌ |
| Quest Category | `data/wow/quests/category/{questCategoryId}` | `quest.rs` | ✅ | ❌ |
| Quest Areas Index | `data/wow/quests/area/index` | `quest.rs` | ✅ | ❌ |
| Quest Category | `data/wow/quests/area/{questAreaId}` | `quest.rs` | ✅ | ❌ |
| Quest Types Index | `data/wow/quests/type/index` | `quest.rs` | ✅ | ❌ |
| Quest Type | `data/wow/quests/type/{questTypeId}` | `quest.rs` | ✅ | ❌ |

### Realm
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Realms Index | `data/wow/realm/index` | `realm.rs` | ✅ | ❌ |
| Realm | `data/wow/realm/{realmSlug}` | `realm.rs` | ✅ | ❌ |
| Realm Search | `data/wow/search/realm` | `realm.rs` | ✅ | ❌ |

### Region
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Regions Index | `data/wow/region/index` | `region_api.rs` | ✅ | ❌ |
| Region | `data/wow/region/{regionId}` | `region_api.rs` | ✅ | ❌ |

### Reputations
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Reputation Factions Index | `data/wow/reputation-faction/index` | `reputation.rs` | ✅ | ❌ |
| Reputation Faction | `data/wow/reputation-faction/{reputationFactionId}` | `reputation.rs` | ✅ | ❌ |
| Reputation Tiers Index | `data/wow/reputation-tiers/index` | `reputation.rs` | ✅ | ❌ |
| Reputation Tier | `data/wow/reputation-tiers/{reputationTiersId}` | `reputation.rs` | ✅ | ❌ |

### Spell
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Spell | `data/wow/spell/{spellId}` | `spell.rs` | ✅ | ❌ |
| Spell Media | `data/wow/media/spell/{spellId}` | `spell.rs` | ✅ | ❌ |
| Spell Search | `data/wow/search/spell` | `spell.rs` | ✅ | ❌ |

### Talent
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Talent Tree Index | `data/wow/talent-tree/index` | `talent.rs` | ✅ | ❌ |
| Talent Tree | `data/wow/talent-tree/{talentTreeId}/playable-specialization/{specId}` | `talent.rs` | ✅ | ❌ |
| Talent Tree Nodes | `data/wow/talent-tree/{talentTreeId}` | `talent.rs` | ✅ | ❌ |
| Talents Index | `data/wow/talent/index` | `talent.rs` | ✅ | ❌ |
| Talent | `data/wow/talent/{talentId}` | `talent.rs` | ✅ | ❌ |
| PvP Talents Index | `data/wow/pvp-talent/index` | `talent.rs` | ✅ | ❌ |
| PvP Talent | `data/wow/pvp-talent/{pvpTalentId}` | `talent.rs` | ✅ | ❌ |

### Tech Talent
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Tech Talent Tree Index | `data/wow/tech-talent-tree/index` | `talent.rs` | ✅ | ❌ |
| Tech Talent Tree | `data/wow/tech-talent-tree/{techTalentTreeId}` | `talent.rs` | ✅ | ❌ |
| Tech Talent Index | `data/wow/tech-talent/index` | `talent.rs` | ✅ | ❌ |
| Tech Talent | `data/wow/tech-talent/{techTalentId}` | `talent.rs` | ✅ | ❌ |
| Tech Talent Media | `data/wow/media/tech-talent/{techTalentId}` | `talent.rs` | ✅ | ❌ |

### Title
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Titles Index | `data/wow/title/index` | `title.rs` | ✅ | ❌ |
| Titles | `data/wow/title/{titleId}` | `title.rs` | ✅ | ❌ |

### Toy
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Toys Index | `data/wow/toy/index` | `toy.rs` | ✅ | ❌ |
| Toy | `data/wow/toy/{toyId}` | `toy.rs` | ✅ | ❌ |

### WoW Token
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| WoW Token Index | `data/wow/token/index` | `wow_token.rs` | ✅ | ✅ |

> All regions (US, EU, KR, TW, CN) use the same model and endpoint. Region handling is done by `BattleNetClient`, not per-endpoint logic.

