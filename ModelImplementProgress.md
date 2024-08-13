
# Progress of API Wrapper Implementation

## World of Warcraft (Retail) Profile APIs

### Account Profile

| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| *Account Profile Summary* | `profile/user/wow` | TBD | TBD | TBD |
| *Protected Character Profile Summary* | `profile/user/wow/protected-character/{realmId}-{characterId}` | TBD | TBD | TBD |
| *Account Collections Index* | `profile/user/wow/collections` | TBD | TBD | TBD |
| *Account Mount Collections* | `profile/user/wow/collections/mounts` | TBD | TBD | TBD |
| *Account Pets Collection* | `profile/user/wow/collections/pets` | TBD | TBD | TBD |
| *Account Toys Collection* | `profile/user/wow/collections/toys` | TBD | TBD | TBD |
| *Account Heirlooms Collection* | `profile/user/wow/collections/heirlooms` | TBD | TBD | TBD |

### Character Achievements

| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Achievements Summary | `profile/wow/character/{realmSlug}/{characterName}/achievements` | TBD | TBD | TBD |
| Character Achievements Statistics | `profile/wow/character/{realmSlug}/{characterName}/achievements/statistics` | TBD | TBD | TBD |

### Character Appearance

| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Appearance Statistics | `profile/wow/character/{realmSlug}/{characterName}/appearance` | TBD | TBD | TBD |

### Character Collections
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Collections Index | `profile/wow/character/{realmSlug}/{characterName}/collections` | TBD | TBD | TBD |
| Character Mounts Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/mounts` | TBD | TBD | TBD |
| Character Pets Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/pets` | TBD | TBD | TBD |
| Character Toys Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/toys` | TBD | TBD | TBD |
| Character Heirlooms Summary | `profile/wow/character/{realmSlug}/{characterName}/collections/heirlooms` | TBD | TBD | TBD |

### Character Encounters
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Encounters | Character Encounters Summary | `profile/wow/character/{realmSlug}/{characterName}/encounters` | TBD | TBD | TBD |
| Character Encounters | Character Dungeons | `profile/wow/character/{realmSlug}/{characterName}/dungeons` | TBD | TBD | TBD |
| Character Encounters | Character Raids | `profile/wow/character/{realmSlug}/{characterName}/raids` | TBD | TBD | TBD |

### Character Equipment
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Equipment | Character Equipment Summary | `profile/wow/character/{realmSlug}/{characterName}/equipment` | TBD | TBD | TBD |

### Character Hunter Pets
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Hunter Pets | Character Hunter Pets Summary | `profile/wow/character/{realmSlug}/{characterName}/hunter-pets` | TBD | TBD | TBD |

### Character Media
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Media | Character Media Summary | `profile/wow/character/{realmSlug}/{characterName}/character-media` | TBD | TBD | TBD |

### Character Mythic Keystone Profile
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Mythic Keystone Profile Index | `profile/wow/character/{realmSlug}/{characterName}/mythic-keystone-profile` | TBD | TBD | TBD |
| Character Mythic Keystone Season Details | `profile/wow/character/{realmSlug}/{characterName}/mythic-keystone-profile/season/{seasonId}` | TBD | TBD | TBD |

### Character Professions
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Professions Summary | `profile/wow/character/{realmSlug}/{characterName}/professions` | TBD | TBD | TBD |

### Character Profile
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Profile Summary | `profile/wow/character/{realmSlug}/{characterName}` | `character_profile.rs` | ✅ | ❌ |
| Character Profile Status | `profile/wow/character/{realmSlug}/{characterName}/status` | `character_profile.rs` | ✅ | ✅ |

### Character PvP
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character PvP Bracket Statistics | `profile/wow/character/{realmSlug}/{characterName}/pvp-bracket/{pvpBracket}` | TBD | TBD | TBD |
| Character PvP Summary | `profile/wow/character/{realmSlug}/{characterName}/pvp-summary` | TBD | TBD | TBD |

### Character Quests
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Quests | `profile/wow/character/{realmSlug}/{characterName}/quests` | TBD | TBD | TBD |
| Character Completed Quests | `profile/wow/character/{realmSlug}/{characterName}/quests/completed` | TBD | TBD | TBD |

### Character Reputations
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Reputations Summary | `profile/wow/character/{realmSlug}/{characterName}/reputations` | TBD | TBD | TBD |

### Character Soulbinds
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Soulbinds | `profile/wow/character/{realmSlug}/{characterName}/quests/soulbinds` | TBD | TBD | TBD |

### Character Specializations
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Specializations Summary | `profile/wow/character/{realmSlug}/{characterName}/quests/specializations` | TBD | TBD | TBD |

### Character Statistics
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Statistics Summary | `profile/wow/character/{realmSlug}/{characterName}/quests/statistics` | TBD | TBD | TBD |

### Character Titles
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Character Titles Summary | `profile/wow/character/{realmSlug}/{characterName}/quests/titles` | TBD | TBD | TBD |

### Guild
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Guild | `data/wow/guild/{realmSlug}/{nameSlug}` | TBD | TBD | TBD |
| Guild Activity | `data/wow/guild/{realmSlug}/{nameSlug}/activity` | TBD | TBD | TBD |
| Guild Achievements | `data/wow/guild/{realmSlug}/{nameSlug}/achievements` | TBD | TBD | TBD |
| Guild Roster | `data/wow/guild/{realmSlug}/{nameSlug}/roster` | TBD | TBD | TBD |


## World of Warcraft (Retail) Game Data APIs

### Achievement
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Achievement Categories Index | `data/wow/achievement-category/index` | `achievment.rs` | ✅ | ✅ |
| Achievement Category | `data/wow/achievement-category/{achievementCategoryId}` | `achievment.rs` | ✅ | ✅ |
| Achievement Index | `data/wow/achievement/index` | `achievment.rs` | ✅ | ✅ |
| Achievement | `data/wow/achievement/{achievementId}` | `achievment.rs` | ✅ | ✅ |
| Achievement Media | `data/wow/media/achievement/{achievementId}` | `achievment.rs` | ✅ | ✅ |

### Auction House
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Auctions | `data/wow/connected-realm/{connectedRealmId}/auctions` | TBD | TBD | TBD |
| Commodities | `data/wow/auctions/commodities` | TBD | TBD | TBD |

### Azerite Essence
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Azerite Essences Index | `data/wow/azerite-essences/index` | TBD | TBD | TBD |
| Azerite Essence | `data/wow/azerite-essences/{azeriteEssenceId}` | TBD | TBD | TBD |
| Azerite Essence Search | `data/wow/search/azerite-essences` | TBD | TBD | TBD |
| Azerite Essence Media | `data/wow/media/azerite-essences/{azeriteEssenceId}` | TBD | TBD | TBD |

### Connected Realms
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Connected Realms Index | `data/wow/connected-realm/index` | `connected_realm.rs` | ✅ | ✅ |
| Connected Realms | `data/wow/connected-realm/{connectedRealmId}` | `connected_realm.rs` | ✅ | ✅ |
| Connected Realms Search | `data/wow/search/connected-realm` | TBD | TBD | TBD |

### Covenant
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Covenant Index | `data/wow/covenant/index` | TBD | TBD | TBD |
| Covenant | `data/wow/covenant/{covenantId}` | TBD | TBD | TBD |
| Covenant Media | `data/wow/media/covenant/{covenantId}` | TBD | TBD | TBD |
| Soulbind Index | `data/wow/covenant/soulbind/index` | TBD | TBD | TBD |
| Soulbind | `data/wow/covenant/soulbind/{soulbindId}` | TBD | TBD | TBD |
| Conduit Index | `data/wow/covenant/conduit/index` | TBD | TBD | TBD |
| Conduit | `data/wow/covenant/conduit/{conduitId}` | TBD | TBD | TBD |

### Creature
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Creature Families Index | `data/wow/creature-family/index` | TBD | TBD | TBD |
| Creature Family | `data/wow/creature-family/{creatureFamilyId}` | TBD | TBD | TBD |
| Creature Types Index | `data/wow/creature-type/index` | TBD | TBD | TBD |
| Creature Types | `data/wow/creature-type/{creatureTypeId}` | TBD | TBD | TBD |
| Creature Search | `data/wow/search/creature` | TBD | TBD | TBD |
| Creature Display Media | `data/wow/media/creature-display/{creatureDisplayId}` | TBD | TBD | TBD |
| Creature Family Media | `data/wow/media/creature-family/{creatureFamilyId}` | TBD | TBD | TBD |

### Guild Crest
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Guild Crest Components Index | `data/wow/guild-crest/index` | TBD | TBD | TBD |
| Guild Crest Border Media | `data/wow/guild-crest/border/{borderId}` | TBD | TBD | TBD |
| Guild Crest Emblem Media | `data/wow/guild-crest/border/{emblemId}` | TBD | TBD | TBD |

### Heirloom
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Heirloom Index | `data/wow/heirloom/index` | TBD | TBD | TBD |
| Heirloom | `data/wow/heirloom/{heirloomId}` | TBD | TBD | TBD |

### Item
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Item Classes Index | `data/wow/item-class/index` | TBD | TBD | TBD |
| Item Class | `data/wow/item-class/{itemClassId}` | TBD | TBD | TBD |
| Item Sets Index | `data/wow/item-set/index` | TBD | TBD | TBD |
| Item Set | `data/wow/item-set/{itemSetId}` | TBD | TBD | TBD |
| Item Subclass | `data/wow/item-class/{itemClassId}/item-subclass/{itemSubclassId}` | TBD | TBD | TBD |
| Item | `data/wow/item/{itemId}` | TBD | TBD | TBD |
| Item Media | `data/wow/media/item/{itemId}` | TBD | TBD | TBD |
| Item Search | `data/wow/search/item` | TBD | TBD | TBD |

### Journal
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Journal Expansions Index | `data/wow/journal-expansion/index` | TBD | TBD | TBD |
| Journal Expansions | `data/wow/journal-expansion/{journalExpansionId}` | TBD | TBD | TBD |
| Journal Encounters Index | `data/wow/journal-encounters/index` | TBD | TBD | TBD |
| Journal Encounter | `data/wow/journal-encounters/{journalEncounterId}` | TBD | TBD | TBD |
| Journal Encounter Search | `data/wow/search/journal-encounter` | TBD | TBD | TBD |
| Journal Instances Index | `data/wow/journal-instance/index` | TBD | TBD | TBD |
| Journal Instance | `data/wow/journal-instance/{journalInstanceId}` | TBD | TBD | TBD |
| Journal Instance Media | `data/wow/media/journal-instance/{journalInstanceId}` | TBD | TBD | TBD |

### Media Search
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Media Search | `data/wow/search/media` | TBD | TBD | TBD |

### Modified Crafting
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Modified Crafting Index | `data/wow/modified-crafting/index` | TBD | TBD | TBD |
| Modified Crafting Category Index | `data/wow/modified-crafting/category/index` | TBD | TBD | TBD |
| Modified Crafting Category | `data/wow/modified-crafting/category/{categoryId}` | TBD | TBD | TBD |
| Modified Crafting Reagent Slot Type Index | `data/wow/modified-crafting/reagent-slot-type/index` | TBD | TBD | TBD |
| Modified Crafting Reagent Slot Type | `data/wow/modified-crafting/reagent-slot-type/{slotTypeId}` | TBD | TBD | TBD |

### Mount
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mounts Index | `data/wow/mount/index` | TBD | TBD | TBD |
| Mount | `data/wow/mount/{mountId}` | TBD | TBD | TBD |
| Mount Search | `data/wow/search/mount` | TBD | TBD | TBD |

### Mythic Keystone Affix
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Keystone Affixes Index | `data/wow/keystone-affix/index` | TBD | TBD | TBD |
| Mythic Keystone Affix | `data/wow/keystone-affix/{keystoneAffixId}` | TBD | TBD | TBD |
| Mythic Keystone Affix Media | `data/wow/media/keystone-affix/{keystoneAffixId}` | TBD | TBD | TBD |

### Mythic Keystone Dungeon
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Keystone Dungeon Index | `data/wow/mythic-keystone/dungeon/index` | TBD | TBD | TBD |
| Mythic Keystone Dungeon | `data/wow/mythic-keystone/dungeon/{dungeonId}` | TBD | TBD | TBD |
| Mythic Keystone Index | `data/wow/mythic-keystone/index` | TBD | TBD | TBD |
| Mythic Keystone Periods Index | `data/wow/mythic-keystone/period/index` | TBD | TBD | TBD |
| Mythic Keystone Period | `data/wow/mythic-keystone/period/{periodId}` | TBD | TBD | TBD |
| Mythic Keystone Seasons Index | `data/wow/mythic-keystone/season/index` | TBD | TBD | TBD |
| Mythic Keystone Season | `data/wow/mythic-keystone/season/{seasonId}` | TBD | TBD | TBD |

### Mythic Keystone Leaderboard
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Keystone Leaderboards Index | `data/wow/connected-realm/{connectedRealmId}/mythic-leaderboard/index` | TBD | TBD | TBD |
| Mythic Keystone Leaderboard | `data/wow/connected-realm/{connectedRealmId}/mythic-leaderboard/{dungeonId}/period/{period}` | TBD | TBD | TBD |

### Mythic Raid Leaderboard
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Mythic Raid Leaderboard | `data/wow/hall-of-fame/{raid}/{faction}` | TBD | TBD | TBD |

### Pet
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Pets Index | `data/wow/pet/index` | TBD | TBD | TBD |
| Pet | `data/wow/mount/{petId}` | TBD | TBD | TBD |
| Pet Media | `data/wow/media/{petId}` | TBD | TBD | TBD |
| Pet Abilities Index | `data/wow/pet-ability/index` | TBD | TBD | TBD |
| Pet Ability | `data/wow/pet-ability/{petAbilityId}` | TBD | TBD | TBD |
| Pet Ability | `data/wow/media/pet-ability/{petAbilityId}` | TBD | TBD | TBD |


### Playable Class
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Playable Classes Index | `data/wow/playable-class/index` | TBD | TBD | TBD |
| Playable Class | `data/wow/playable-class/{classID}` | TBD | TBD | TBD |
| Playable Class Media | `data/wow/media/playable-class/{classID}` | TBD | TBD | TBD |
| PvP Talent Slots | `data/wow/media/playable-class/{classID}/pvp-talent-slots` | TBD | TBD | TBD |

### Playable Race
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Playable Races Index | `data/wow/playable-race/index` | TBD | TBD | TBD |
| Playable Race | `data/wow/playable-race/{playableRaceId}` | TBD | TBD | TBD |

### Playable Specialization
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Playable Specializations Index | `data/wow/playable-specialization/index` | TBD | TBD | TBD |
| Playable Specialization | `data/wow/playable-specialization/{specId}` | TBD | TBD | TBD |
| Playable Specialization Media | `data/wow/media/playable-specialization/{specId}` | TBD | TBD | TBD |

### Power Type
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Power Types Index | `data/wow/power-type/index` | TBD | TBD | TBD |
| Power Type | `data/wow/power-type/{powerTypeId}` | TBD | TBD | TBD |

### Profession
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Professions Index | `data/wow/profession/index` | TBD | TBD | TBD |
| Profession | `data/wow/profession/{professionId}` | TBD | TBD | TBD |
| Profession Media | `data/wow/media/profession/{professionId}` | TBD | TBD | TBD |
| Profession Skill Tier | `data/wow/profession/{professionId}/skill-tier/{skillTierId}` | TBD | TBD | TBD |
| Recipe | `data/wow/recipe/{recipeId}` | TBD | TBD | TBD |
| Recipe Media | `data/wow/media/recipe/{recipeId}` | TBD | TBD | TBD |

### PvP Season
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| PvP Seasons Index | `data/wow/pvp-season/index` | TBD | TBD | TBD |
| PvP Season | `data/wow/pvp-season/{pvpSeasonId}` | TBD | TBD | TBD |
| PvP Leaderboards Index | `data/wow/pvp-season/{pvpSeasonId}/pvp-leaderboard/index` | TBD | TBD | TBD |
| PvP Leaderboard | `data/wow/pvp-season/{pvpSeasonId}/pvp-leaderboard/{pvpBracket}` | TBD | TBD | TBD |
| PvP Rewards Index | `data/wow/pvp-season/{pvpSeasonId}/pvp-reward/index` | TBD | TBD | TBD |

### PvP Tier
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| PvP Tier Media | `data/wow/media/pvp-tier/{pvpTierId}` | TBD | TBD | TBD |
| PvP Tiers Index | `data/wow/pvp-tier/index` | TBD | TBD | TBD |
| PvP Tier | `data/wow/pvp-tier/{pvpTierId}` | TBD | TBD | TBD |

### Quests
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Quests Index | `data/wow/quests/index` | TBD | TBD | TBD |
| Quest | `data/wow/quests/{questId}` | TBD | TBD | TBD |
| Quest Categories Index | `data/wow/quests/category/index` | TBD | TBD | TBD |
| Quest Category | `data/wow/quests/category/{questCategoryId}` | TBD | TBD | TBD |
| Quest Areas Index | `data/wow/quests/area/index` | TBD | TBD | TBD |
| Quest Category | `data/wow/quests/area/{questAreaId}` | TBD | TBD | TBD |
| Quest Types Index | `data/wow/quests/type/index` | TBD | TBD | TBD |
| Quest Type | `data/wow/quests/type/{questTypeId}` | TBD | TBD | TBD |

### Realm
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Realms Index | `data/wow/realm/index` | TBD | TBD | TBD |
| Realm | `data/wow/realm/{realmSlug}` | TBD | TBD | TBD |
| Realm Search | `data/wow/search/realm` | TBD | TBD | TBD |

### Region
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Regions Index | `data/wow/region/index` | TBD | TBD | TBD |
| Region | `data/wow/region/{regionId}` | TBD | TBD | TBD |

### Reputations
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Reputation Factions Index | `data/wow/reputation-faction/index` | TBD | TBD | TBD |
| Reputation Faction | `data/wow/reputation-faction/{reputationFactionId}` | TBD | TBD | TBD |
| Reputation Tiers Index | `data/wow/reputation-tiers/index` | TBD | TBD | TBD |
| Reputation Tier | `data/wow/reputation-tiers/{reputationTiersId}` | TBD | TBD | TBD |

### Spell
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Spell | `data/wow/spell/{spellId}` | TBD | TBD | TBD |
| Spell Media | `data/wow/media/spell/{spellId}` | TBD | TBD | TBD |
| Spell Search | `data/wow/search/spell` | TBD | TBD | TBD |

### Talent
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Talent Tree Index | `data/wow/talent-tree/index` | TBD | TBD | TBD |
| Talent Tree | `data/wow/talent-tree/{talentTreeId}/playable-specialization/{specId}` | TBD | TBD | TBD |
| Talent Tree Nodes | `data/wow/talent-tree/{talentTreeId}` | TBD | TBD | TBD |
| Talents Index | `data/wow/talent/index` | TBD | TBD | TBD |
| Talent | `data/wow/talent/{talentId}` | TBD | TBD | TBD |
| PvP Talents Index | `data/wow/pvp-talent/index` | TBD | TBD | TBD |
| PvP Talent | `data/wow/pvp-talent/{pvpTalentId}` | TBD | TBD | TBD |

### Tech Talent
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Tech Talent Tree Index | `data/wow/tech-talent-tree/index` | TBD | TBD | TBD |
| Tech Talent Tree | `data/wow/tech-talent-tree/{techTalentTreeId}` | TBD | TBD | TBD |
| Tech Talent Index | `data/wow/tech-talent/index` | TBD | TBD | TBD |
| Tech Talent | `data/wow/tech-talent/{techTalentId}` | TBD | TBD | TBD |
| Tech Talent Media | `data/wow/media/tech-talent/{techTalentId}` | TBD | TBD | TBD |

### Title
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Titles Index | `data/wow/title/index` | TBD | TBD | TBD |
| Titles | `data/wow/title/{titleId}` | TBD | TBD | TBD |

### Toy
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| Toys Index | `data/wow/toy/index` | TBD | TBD | TBD |
| Toy | `data/wow/toy/{toyId}` | TBD | TBD | TBD |

### WoW Token
| API | Endpoint | Model | Impl. | Unit Test |
|-----|----------|-------|-------|-----------|
| WoW Token Index (US, EU, KR, TW) | `data/wow/token/index` | `wow_token.rs` | ✅ | ✅ |
| WoW Token Index (CN) | `data/wow/token/index` | `wow_token.rs` | ✅ | ✅ |

