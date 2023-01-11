# -*- coding: utf-8 -*-


def clamp(num, lower, upper):
    return max(lower, min(num, upper))


class GildedRose(object):
    def __init__(self, items):
        self.items = items

    def update_quality(self):
        def backstage_passes(item):
            if item.sell_in > 10:
                return 1
            elif item.sell_in > 5:
                return 2
            elif item.sell_in > 0:
                return 3
            else:
                return -item.quality

        for item in self.items:
            if "Sulfuras" in item.name:
                continue

            if "Backstage passes" in item.name:
                change_in_quality = backstage_passes(item)
            elif "Aged Brie" == item.name:
                change_in_quality = 1 if item.sell_in > 0 else 2
            else:
                change_in_quality = -1 if item.sell_in > 0 else -2

            item.sell_in -= 1
            item.quality = clamp(item.quality + change_in_quality, 0, 50)


class Item:
    def __init__(self, name, sell_in, quality):
        self.name = name
        self.sell_in = sell_in
        self.quality = quality

    def __repr__(self):
        return "%s, %s, %s" % (self.name, self.sell_in, self.quality)
