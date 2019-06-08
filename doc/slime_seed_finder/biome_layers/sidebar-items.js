initSidebarItems({"constant":[["NUM_LAYERS",""],["UNKNOWN_BIOME_ID",""]],"fn":[["area_from_coords",""],["biome_to_color",""],["candidate_river_map",""],["candidate_river_map_generator",""],["draw_map",""],["draw_map_image",""],["generate",""],["generate_image",""],["generate_image_up_to_layer",""],["generate_up_to_layer",""],["generate_up_to_layer_extra",""],["generate_up_to_layer_extra_2",""],["map_with_river_at",""],["pretty_biome_map_hills",""],["reduce_id","Helper function to classify an input into [0, 1, 2, 3] Used by MapRiver"],["reverse_map_river",""],["reverse_map_river_mix","This returns the biome parent of MapRiverMix. Since the rivers actually overwrite some of the info, it is incomplete. The unknown biomes are represented as 0xFF"],["reverse_map_smooth","Works at least 9/16*100 % of the time"],["reverse_map_voronoi_zoom","Works 99.9 % of the time* p = 0.9992 for each tile The probability of having at least one error in a 30x30 area is 50%"],["reverse_map_zoom","Actually, this works 100% of the time"],["river_seed_finder","River Seed Finder"],["river_seed_finder_range","River Seed Finder"],["segregate_coords_prevoronoi_hd","Detect which points are being used for the last layer (hd) and which are used for the reverse_voronoi (prevoronoi)"]],"mod":[["biome_id",""]],"static":[["BIOME_COLORS",""],["BIOME_INFO",""]],"struct":[["Area",""],["Biome",""],["CachedMap",""],["HelperMapRiverAll","Like MapRiver, but will generate all the possible rivers for this 26-bit seed"],["HelperMapZoomAllEdges","Unlike the regular MapZoom, this one makes sure that v11 is different from any of its neighbours. This way we can generate all the possible edges (and therefore rivers) for this 25-bit seed."],["Map",""],["MapAddIsland","This layer uses 64 bits but only affects shores (regions near ocean). Deep ocean is not affected. This makes continental biome borders a good candidate for getting the seed. Ocean islands also seem unaffected, but they are generated in layer 25."],["MapAddMushroomIsland",""],["MapAddSnow",""],["MapBiome",""],["MapBiomeEdge",""],["MapCoolWarm",""],["MapDeepOcean",""],["MapHeatIce",""],["MapHills","This layer uses 64 bits"],["MapIsland",""],["MapMap",""],["MapRareBiome",""],["MapRemoveTooMuchOcean",""],["MapRiver",""],["MapRiverInit",""],["MapRiverMix",""],["MapShore",""],["MapSkip",""],["MapSmooth",""],["MapSpecial",""],["MapVoronoiZoom",""],["MapZoom",""],["SparseMap",""],["TestMapCheckers",""],["TestMapXhz",""],["TestMapZero",""]],"trait":[["GetMap",""]]});