import dataclasses
import customtkinter

@dataclasses.dataclass(frozen=True)
class Settings:
    appareance_mode : str = "dark"
    default_color_theme : str = "green"
    
    width : int = 1000
    height : int = 1000

    cell_size : int = 100

    color_black : str = "green"
    color_white : str = "beige"

    @classmethod
    def apply_settings(cls):
        customtkinter.set_appearance_mode(cls.appearence_mode)
        customtkinter.set_default_color_theme(cls.default_color_theme)

    @classmethod 
    def get_geometry_string(cls) -> str:
        return f"{cls.width}x{cls.height}"
    