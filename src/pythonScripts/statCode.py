for stat in ["globes", "mercs", "kda", "mmr", "lengths"]:
    print ("""\nlet {0} = &hero_stats[h].{0};\nlet {0}_mean = mean({0});\nsummary_stats.push({0}_mean);\nsummary_stats.push(sigma({0},{0}_mean));""".format(stat))
