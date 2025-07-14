import os
import pathlib
import re

def main():
    Workspace([
        "crate/archive"
    ]).sync()

class Workspace:
    def __init__(self, exclude: list[str]) -> None:
        self.exclude = exclude
    
    def sync(self) -> None:
        toml_path = self._toml()
        with open(toml_path, "r", encoding="utf-8") as f:
            content: str = f.read()
        new: str = re.sub(
            r'\[workspace\][^\[]*',
            self._replace_toml_members_section,
            content,
            flags=re.DOTALL
        )
        with open(toml_path, "w", encoding="utf-8") as f:
            f.write(new)
        print("✅ Updated `Cargo.toml` with new workspace members.")

    def _replace_toml_members_section(self, found: re.Match[str]) -> str:
        workspace_block: str = found.group(0)
        new_members_block = self._members_block()
        lines = workspace_block.splitlines()
        out: list[str] = []
        inside_members = False
        for line in lines:
            stripped = line.strip()
            if not inside_members and stripped.startswith("members") and "[" in stripped:
                out.append(new_members_block)
                inside_members = True
                continue
            if inside_members and "]" in stripped:
                inside_members = False
                continue
            if not inside_members:
                out.append(line)
        return "\n".join(out)

    def _toml(self) -> pathlib.Path:
        return self._root() / "Cargo.toml"

    def _root(self) -> pathlib.Path:
        return pathlib.Path.cwd()

    def _members_block(self) -> str:
        lines: list[str] = [
            "members = ["
        ]
        for member in self._members():
            lines.append(f'"{member}",')
        lines.append("]")
        return "\n".join(lines)

    def _members(self) -> list[str]:
        members: list[str] = []
        root: pathlib.Path = self._root()
        for directory_path, _, file_names in os.walk(root):
            if "Cargo.toml" in file_names:
                rel_path: pathlib.Path = pathlib.Path.cwd()
                rel_path_0: str = os.path.relpath(directory_path, rel_path)
                rel_path_1 = rel_path_0.replace("\\", "/")
                if not any(rel_path_1.startswith(e) for e in self.exclude):
                    members.append(rel_path_1)
        sorted_members: list[str] = sorted(members)
        sorted_members.remove(".")
        return sorted_members
    
main()