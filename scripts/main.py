from enum import Enum
from requests import Session
from pathlib import Path
from PIL import Image
from io import BytesIO
import warnings
from operator import itemgetter
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


# The type hint for char_info is lenient because declaring the entire schema is too painful
def is_operator(char_info: dict) -> bool:
    return char_info["profession"] != "TOKEN" \
        and char_info["profession"] != "TRAP" \
        and not char_info["isNotObtainable"]


def save_image(session: Session, category: Asset, name: str) -> None:
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

with Session() as s:
    CHARS = (
        s.get(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/character_table.json")
        .json()
    )

    CN_CHAR_SKILLS, CN_SKILL_DATA = itemgetter("chars", "buffs")(
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/building_data.json")
        .json()
    )

    EN_SKILL_DATA = (
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/en_US/gamedata/excel/building_data.json")
        .json()
        ["buffs"]
    )

    for char_id, data in CHARS.items():
        if is_operator(data):
            skills = []
            for skill in CN_CHAR_SKILLS[char_id]["buffChar"]:
                for tier in skill["buffData"]:
                    level_req = tier["cond"]
                    skill_info = EN_SKILL_DATA.get(
                        tier["buffId"],
                        CN_SKILL_DATA[tier["buffId"]]
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
