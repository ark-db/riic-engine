from enum import Enum
import re
from requests import Session
from operator import itemgetter
from pathlib import Path
from PIL import Image
from io import BytesIO
import warnings
import json


class Asset(Enum):
    CHAR = {
        "folder": "chars",
        "base_url": "https://raw.githubusercontent.com/astral4/arkdata/main/assets/torappu/dynamicassets/arts/charavatars",
        "quality": 25,
        "upscale": False
    }
    SKILL = {
        "folder": "skills",
        "base_url": "https://raw.githubusercontent.com/astral4/arkdata/main/assets/torappu/dynamicassets/arts/building/skills",
        "quality": 50,
        "upscale": False
    }
    ELITE = {
        "folder": "elite",
        "base_url": "https://raw.githubusercontent.com/Aceship/Arknight-Images/main/ui/elite",
        "quality": 25,
        "upscale": False
    }
    FACILITY = {
        "folder": "facilities",
        "base_url": "https://raw.githubusercontent.com/astral4/arkdata/main/assets/arts/building/buffs",
        "quality": 100,
        "upscale": False
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

HEX_CODE_PATTERN = re.compile(r"#[0-9A-F]{6}")

DISALLOWED_FACILITY_TYPES = {
    "elevator", "corridor"
}

FACILITY_COLORS = {
    "control": "#005752",
    "dormitory": "#21cdcb",
    "hire": "#565656",
    "manufacture": "#ffd800",
    "meeting": "#dd653f",
    "power": "#8fc31f",
    "trading": "#0075a9",
    "training": "#7d0022",
    "workshop": "#e3eb00"
}


# The type hint for char_info is lenient because declaring the entire schema is too painful
def is_operator(char_info: dict[str, object]) -> bool:
    return char_info["profession"] != "TOKEN" \
        and char_info["profession"] != "TRAP" \
        and not char_info["isNotObtainable"]


def save_image(session: Session, category: Asset, name: str) -> None:
    folder, base_url, quality, upscale = itemgetter("folder", "base_url", "quality", "upscale")(
        category.value
    )
    target_path = Path(f"./static/{folder}/{name}.webp")

    if target_path.is_file():
        # No need to attempt downloading image if it is already present
        return
    elif (res := session.get(f"{base_url}/{name}.png")):
        # The Response returned from get() is truthy if the status code is 2xx or 3xx
        image = Image.open(BytesIO(res.content)).convert("RGBA")
        if upscale:
            width, height = image.size
            image = image.resize(
                (width*2, height*2),
                resample=Image.Resampling.LANCZOS
            )
        image.save(target_path, "webp", quality=quality)
    else:
        warnings.warn(
            f"Could not save image of {category.name.lower()} with ID \"{name}\"",
            RuntimeWarning
        )


char_data = []
all_skill_ids: set[str] = set()
all_skills: dict[str, dict[str, str]] = {}
facility_info: dict[str, dict[str, object]] = {}

with Session() as s:
    CHARS = (
        s.get(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/character_table.json")
        .json()
    )

    CHAR_SKILLS, CN_SKILL_DATA = itemgetter("chars", "buffs")(
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/building_data.json")
        .json()
    )

    FACILITY_DATA, EN_SKILL_DATA = itemgetter("rooms", "buffs")(
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/en_US/gamedata/excel/building_data.json")
        .json()
    )

    TEXT_STYLES, CN_SPECIAL_TERMS = itemgetter("richTextStyles", "termDescriptionDict")(
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/gamedata_const.json")
        .json()
    )

    EN_SPECIAL_TERMS = (
        s.get("https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/en_US/gamedata/excel/gamedata_const.json")
        .json()
        ["termDescriptionDict"]
    )

    for char_id, details in CHARS.items():
        if is_operator(details):
            skills = []
            for skill in CHAR_SKILLS[char_id]["buffChar"]:
                for tier in skill["buffData"]:
                    level_req = tier["cond"]
                    skill_id = tier["buffId"]
                    skills.append({
                        "elite": level_req["phase"],
                        "level": level_req["level"],
                        "skillId": skill_id
                    })

                    if skill_id not in all_skill_ids:
                        all_skill_ids.add(skill_id)
                        skill_info = EN_SKILL_DATA.get(
                            skill_id, CN_SKILL_DATA[skill_id]
                        )
                        icon_id = skill_info["skillIcon"]
                        all_skills[skill_id] = {
                            "name": skill_info["buffName"],
                            "desc": skill_info["description"],
                            "iconId": icon_id
                        }
                        save_image(s, Asset.SKILL, icon_id)

            char_data.append({
                "charId": char_id,
                "name": NAME_CHANGES.get(char_id, details["appellation"]),
                "rarity": details["rarity"] + 1,
                "skills": skills
            })
            save_image(s, Asset.CHAR, char_id)

    for facility in FACILITY_DATA.values():
        facility_id = facility["id"].lower()
        if facility_id not in DISALLOWED_FACILITY_TYPES:
            power_cost_by_level: list[int] = []
            capacity_by_level: list[int] = []

            for level in facility["phases"]:
                power_cost_by_level.append(level["electricity"])
                capacity_by_level.append(level["maxStationedNum"])

            facility_info[facility_id] = {
                "name": facility["name"],
                "color": FACILITY_COLORS[facility_id],
                "power": power_cost_by_level,
                "capacity": capacity_by_level
            }

            save_image(s, Asset.FACILITY, facility_id)

    for rank in range(3):
        save_image(s, Asset.ELITE, str(rank))

with open("src/lib/data/chars.json", "w") as f:
    json.dump(char_data, f, ensure_ascii=False)

with open("src/lib/data/skills.json", "w") as f:
    json.dump(all_skills, f, ensure_ascii=False)

with open("src/lib/data/facilities.json", "w") as f:
    json.dump(facility_info, f, ensure_ascii=False)

with open("src/lib/data/text-colors.json", "w") as f:
    STYLES = {
        name: match.group(0)
        for name, style in TEXT_STYLES.items()
        if name.startswith("cc")
        and (match := HEX_CODE_PATTERN.search(style))
    }
    json.dump(STYLES, f)

with open("src/lib/data/terms.json", "w") as f:
    TERMS = {
        name: details["description"]
        for name, details in (CN_SPECIAL_TERMS | EN_SPECIAL_TERMS).items()
        if name.startswith("cc")
    }
    json.dump(TERMS, f, ensure_ascii=False)
