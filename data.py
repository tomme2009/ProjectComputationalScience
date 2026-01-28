parties = [
    'GL PvdA',
    'SP',
    'PvdD',
    'Volt',
    'DENK',
    'CU',
    'NSC',
    'VVD',
    'D66',
    '50Plus',
    'FvD',
    'JA21',
    'PVV',
    'BBB',
    'SGP',
    'CDA',
]

colors = [
    0x96541a, # GL PvdA
    0xff0000, # SP
    0x153921, # PvdD
    0x582c83, # Volt
    0x00b4af, # DENK
    0x009be0, # CU
    0xf0c400, # NSC
    0xf47621, # VVD
    0x00af3f, # D66
    0x933487, # 50Plus
    0x84171a, # FvD
    0xdf201a, # JA21
    0x003f6b, # PVV
    0x95c11f, # BBB
    0xe14400, # SGP
    0x007b5f, # CDA
]

# Data from Wikipedia
# https://nl.wikipedia.org/wiki/Tweede_Kamerverkiezingen_2023
election_2023 = [
    1_643_073, # GL PvdA
    328_225, # SP
    235_148, # PvdD
    178_802, # Volt
    246_765, # DENK
    212_532, # CU
    1_343_287, # NSC
    1_589_519, # VVD
    656_292, # D66
    51_043, # 50Plus
    232_963, # FvD
    71_345, # JA21
    2_450_878, # PVV
    485_551, # BBB
    217_270, # SGP
    345_822, # CDA
]

# Data from Wikipedia
# https://nl.wikipedia.org/wiki/Tweede_Kamerverkiezingen_2025
election_2025 = [
    1_352_163, # GL PvdA
    199_585, # SP
    219_371, # PvdD
    116_468, # Volt
    250_368, # DENK
    201_361, # CU
    39_408, # NSC
    1_505_829, # VVD
    1_790_634, # D66
    151_053, # 50Plus
    480_393, # FvD
    628_517, # JA21
    1_760_966, # PVV
    279_916, # BBB
    238_093, # SGP
    1_246_874, # CDA
]

# Data from KiesKompas
# https://tweedekamer2023.kieskompas.nl/nl/results/compass#
left_right_2023 = [
    2.25, # GL PvdA
    1.15, # SP
    0.25, # PvdD
    4.3, # Volt
    1.6, # DENK
    3.4, # CU
    4.55, # NSC
    7.5, # VVD
    4.75, # D66
    3.85, # 50Plus
    8.2, # FvD
    8.85, # JA21
    5.45, # PVV
    5.45, # BBB
    6.35, # SGP
    6.15, # CDA
]

# Data from KiesKompas
# https://tweedekamer2025.kieskompas.nl/nl/results/compass#
left_right_2025 = [
    1.9, # GL PvdA
    0.95, # SP
    0.2, # PvdD
    2.3, # Volt
    2.3, # DENK
    3.65, # CU
    4.6, # NSC
    8.45, # VVD
    3.65, # D66
    5.2, # 50Plus
    9.4, # FvD
    9.05, # JA21
    6.9, # PVV
    8.25, # BBB
    6.35, # SGP
    5.6, # CDA
]

# Data from Peilingen Nederland
# https://www.peilingennederland.nl/partijtrends.html
# Poll data per month, first month is December 2023,
# last month is September 2025, corresponding to the months
# in between the elections
poll_data = [
    [22, 24, 25, 25, 24, 25, 27, 26, 27, 26, 25, 26, 25, 24, 25, 27, 28, 29, 28, 27, 28, 24], # GL PvdA
    [4, 5, 5, 6, 6, 5, 5, 5, 7, 5, 6, 7, 7, 7, 8, 7, 6, 7, 6, 7, 7, 6], # SP
    [4, 4, 4, 4, 4, 5, 5, 5, 4, 5, 6, 5, 6, 5, 6, 5, 5, 5, 5, 5, 4, 4], # PvdD
    [3, 3, 2, 3, 3, 3, 4, 4, 3, 4, 4, 3, 4, 4, 4, 4, 4, 3, 3, 3, 4, 4], # Volt
    [3, 3, 3, 3, 4, 3, 3, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 3, 3, 4, 4], # DENK
    [3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 3, 3, 4, 4, 4, 3, 3, 3, 4, 3], # CU
    [19, 18, 10, 10, 10, 9, 9, 10, 9, 5, 4, 3, 3, 3, 3, 2, 1, 1, 1, 0, 0, 0], # NSC
    [16, 16, 16, 18, 18, 18, 18, 20, 23, 20, 21, 21, 22, 21, 22, 26, 27, 27, 24, 21, 15, 14], # VVD
    [10, 11, 10, 10, 11, 10, 11, 11, 11, 10, 11, 11, 11, 12, 12, 10, 10, 10, 9, 11, 10, 12], # D66
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], # 50Plus
    [3, 3, 3, 3, 4, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4], # FvD
    [0, 0, 0, 2, 1, 1, 1, 1, 0, 1, 2, 1, 2, 2, 3, 3, 3, 4, 7, 7, 9, 12], # JA21
    [46, 47, 50, 49, 48, 47, 42, 39, 39, 41, 38, 39, 37, 36, 33, 30, 28, 29, 31, 29, 32, 31], # PVV
    [8, 8, 8, 7, 8, 7, 7, 8, 6, 6, 6, 6, 5, 5, 4, 3, 4, 3, 3, 4, 6, 4], # BBB
    [3, 3, 3, 3, 4, 3, 4, 4, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 4, 4, 3], # SGP
    [5, 6, 7, 8, 7, 8, 9, 10, 8, 11, 12, 13, 15, 16, 16, 17, 18, 18, 21, 23, 22, 24], # CDA
]
