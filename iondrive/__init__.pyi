import os
from types import ModuleType

import ufoLib2

class IondriveError(Exception): ...

def load(font_objects_module: ModuleType, path: str | os.PathLike[str]) -> ufoLib2.Font: ...
