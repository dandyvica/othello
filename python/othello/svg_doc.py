""" A minimalistic class to create SVG documents """
class SVGDoc:
    # define hash of SVG tags
    # svg_tags = {
    #     'rect'   : """<rect x="{}" y="{}" width="{}" height="{}" style="{}"/>""",
    #     'circle' : """<circle cx="{}" cy="{}" r="{}" style="{}" />""",
    #     'text'   : """<text x="{}" y="{}" style="{}">{}</text>""",
    #     'text_anchor'   : """<text x="{}" y="{}" text-anchor="{}" style="{}">{}</text>""",
    #     'line'   : """<line x1="{}" y1="{}" x2="{}" y2="{}" style="{}" />"""
    # }

    """ begin SVG doc with passed canvas dimensions """
    def __init__(self, **kwargs) -> None:
        xml_prologue = """<?xml version="1.0" encoding="UTF-8"?>"""

        # format arguments as SVG attibutes
        attributes = self.__format_attributes(**kwargs)

        self.tags = xml_prologue + """<svg xmlns="http://www.w3.org/2000/svg" {}>""".format(attributes)

    """ generic tag generation """
    def draw(self, tag_name: str, **kwargs) -> None:
        # is it a self enclosing tag or not?
        if "content" in kwargs.keys():
            # get content and delete that key
            content: str = kwargs['content']
            del kwargs['content']

            # build tag+attributes
            begin_tag: str = f"<{tag_name} {self.__format_attributes(**kwargs)}>"

            # build non self-enclosing tag
            self.tags += f"{begin_tag}{content}</{tag_name}>"
        # self enclosing tags are easier
        else:
            self.tags += f"<{tag_name} {self.__format_attributes(**kwargs)} />"


    """ end-up SVG doc """
    def close(self) -> None:
        self.tags += "</svg>"

    """ private method for format attributes """
    def __format_attributes(self, **kwargs) -> str:
        return ' '.join([f'{key}="{value}"' for key,value in kwargs.items()])
