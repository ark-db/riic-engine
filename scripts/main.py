from enum import Enum
import requests
from operator import itemgetter
from pathlib import Path
from PIL import Image
from io import BytesIO
import warnings
import json


class Asset(Enum):
    CHAR = {
        "dir": "chars",
        "base_url": "https://raw.githubusercontent.com/Aceship/Arknight-Images/main/avatars/",
        "quality": 25
    }
    SKILL = {
        "dir": "skills",
        "base_url": "https://raw.githubusercontent.com/Aceship/Arknight-Images/main/ui/infrastructure/skill/",
        "quality": 50
    }


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


def save_image(session, category, name):
    target_path = Path(f"./static/{category.value['dir']}/{name}.webp")

    if target_path.is_file():
        # No need to attempt downloading image if it is already present
        return
    elif (res := session.get(f"{category.value['base_url']}{name}.png")):
        # The Response returned from get() is truthy if the status code is 2xx or 3xx
        Image.open(BytesIO(res.content)) \
             .convert("RGBA") \
             .save(target_path, "webp", quality=category.value["quality"])
    else:
        warnings.warn(
            f"Could not save image of {category.name.lower()} with ID \"{name}\"",
            RuntimeWarning
        )


char_data = []

with requests.Session() as s:
    chars = (
        s.get(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/character_table.json")
        .json()
    )

    cn_char_skills, cn_skill_data = itemgetter("chars", "buffs")(
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
            skills = []
            for skill in cn_char_skills[char_id]["buffChar"]:
                for tier in skill["buffData"]:
                    level_req = tier["cond"]
                    skill_info = en_skill_data.get(
                        tier["buffId"],
                        cn_skill_data[tier["buffId"]]
                    )
                    icon_id = skill_info["skillIcon"]
                    skills.append({
                        "elite": level_req["phase"],
                        "level": level_req["level"],
                        "name": skill_info["buffName"],
                        "iconId": icon_id
                    })
                    save_image(s, Asset.SKILL, icon_id)

            char_data.append({
                "charId": char_id,
                "name": NAME_CHANGES.get(char_id, data["appellation"]),
                "rarity": data["rarity"] + 1,
                "skills": skills
            })
            save_image(s, Asset.CHAR, char_id)

with open("src/lib/data/chars.json", "w") as f:
    json.dump(char_data, f, ensure_ascii=False)
