
from typing import TYPE_CHECKING

from astroid import nodes
try:
    from astroid import node_classes
except ImportError:
    from astroid.nodes import node_classes
from pylint.checkers import BaseChecker

if TYPE_CHECKING:

    from pylint.lint import PyLinter

deprecated_typing = ( "List", "Tuple", "Dict", "Set", "Union", "Iterator", "Iterable", "Collection", "Optional", "Generic" )

class TypingChecker(BaseChecker):

    name = "pycon-import"
    msgs = \
    {
        "E9001" : ( "Deprecated import", "pycon-deprecated-typing-import", "Deprecated import" ),
        "E9002" : ( "Deprecated import", "pycon-deprecated-import", "Deprecated import" ),
        "E9003" : ( "Args without type", "pycon-args-without-type", "Args without type" ),
        "E9004" : ( "Function without return definition", "pycon-function-without-return-definition", "Function without return definition" ),
    }

    def visit_importfrom(self, node : nodes.ImportFrom) -> None:

        match node.modname:

            case "typing":

                for name in node.names:

                    if name[0] in deprecated_typing:

                        self.add_message("pycon-deprecated-typing-import", node = node)

            case "project.various":

                for name in node.names:

                    if name[0] == "DeprecatedClass":

                        self.add_message("pycon-deprecated-import", node = node)

    def visit_functiondef(self, node : nodes.FunctionDef) -> None:

        for arg, ann in zip(node.args.args, node.args.annotations):

            if ann is None and arg.name not in ( "self", "cls" ):

                self.add_message("pycon-args-without-type", node = node)

        if node.returns is None and node.name not in ( "__init__", "__del__" ):

            self.add_message("pycon-function-without-return-definition", node = node)

class ExceptionsChecker(BaseChecker):

    name = "pycon-exception"
    msgs = \
    {
        "E9100" : ( "AttributeError", "pycon-deprecated-attribute-error", "AttributeError is a deprecated replacement for cached_property" ),
    }

    def visit_excepthandler(self, node : nodes.ExceptHandler) -> None:
        match node.type:
            case node_classes.Name(name="AttributeError"):
                self.add_message("pycon-deprecated-attribute-error", node=node)

def register(linter : "PyLinter") -> None:

    linter.register_checker(TypingChecker(linter))
    linter.register_checker(ExceptionsChecker(linter))

