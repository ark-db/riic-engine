import requests
import functools
import operator
import json

NAME_CHANGES = {
    "char_118_yuki": "Shirayuki",
    "char_196_sunbr": "Gummy",
    "char_115_headbr": "Zima",
    "char_195_glassb": "Istina",
    "char_197_poca": "Rosa",
    "char_1001_amiya2": "Amiya (Guard)",
    "char_4055_bgsnow": "Pozyomka",
    "char_4064_mlynar": "Mlynar",
}


def is_operator(char_info):
    return char_info["profession"] != "TOKEN" \
        and char_info["profession"] != "TRAP" \
        and not char_info["isNotObtainable"]


def flatten(arr):
    # Source: https://stackoverflow.com/questions/952914/how-do-i-make-a-flat-list-out-of-a-list-of-lists/45323085#45323085
    return functools.reduce(operator.iconcat, arr, [])


def get_skill_info(skill):
    level_req = skill["cond"]
    skill_info = en_skill_data.get(
        skill["buffId"], cn_skill_data[skill["buffId"]])
    return {
        "elite": level_req["phase"],
        "level": level_req["level"],
        "name": skill_info["buffName"],
        "iconId": skill_info["skillIcon"]
    }


char_data = []

with requests.Session() as s:
    chars = (
        s.get(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/character_table.json")
        .json()
    )

    cn_char_skills, cn_skill_data = operator.itemgetter("chars", "buffs")(
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/building_data.json")
        .json()
    )

    en_skill_data = (
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/en_US/gamedata/excel/building_data.json")
        .json()
        ["buffs"]
    )

    for char_id, data in chars.items():
        if is_operator(data):
            char_data.append({
                "charId": char_id,
                "name": NAME_CHANGES.get(char_id, data["appellation"]),
                "rarity": data["rarity"] + 1,
                "skills": flatten(
                    [
                        get_skill_info(tier)
                        for tier in skill["buffData"]
                    ]
                    for skill in cn_char_skills[char_id]["buffChar"]
                    if skill["buffData"]
                )
            })

with open("src/lib/data/chars.json", "w") as f:
    json.dump(char_data, f, ensure_ascii=False)
