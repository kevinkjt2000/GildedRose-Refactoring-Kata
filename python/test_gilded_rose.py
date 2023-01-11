# -*- coding: utf-8 -*-
import unittest

from gilded_rose import Item, GildedRose, clamp


class GildedRoseTest(unittest.TestCase):
    def test_clamp_works(self):
        self.assertEqual(1, clamp(1, 0, 2))
        self.assertEqual(0, clamp(-1, 0, 2))
        self.assertEqual(2, clamp(5, 0, 2))

    def test_backstage_passes_gain_then_drop_to_zero_after_concert(self):
        items = [Item("Backstage passes to a TAFKAL80ETC concert", 15, 0)]
        gilded_rose = GildedRose(items)

        for expected in [5, 15, 30]:
            gilded_rose.update_quality()
            gilded_rose.update_quality()
            gilded_rose.update_quality()
            gilded_rose.update_quality()
            gilded_rose.update_quality()
            self.assertEqual(expected, items[0].quality)
        gilded_rose.update_quality()
        self.assertEqual(0, items[0].quality)

    def test_normal_items_lose_quality_slowly_then_quickly_then_stops(self):
        items = [
            Item("+5 Dexterity Vest", 2, 6),
            Item("Elixir of the Mongoose", 5, 7),
        ]
        gilded_rose = GildedRose(items)
        expectations = [
            (5, 6),
            (4, 5),
            (2, 4),
            (0, 3),
            (0, 2),
            (0, 0),
            (0, 0),
        ]

        for a, b in expectations:
            gilded_rose.update_quality()
            self.assertEqual(a, items[0].quality)
            self.assertEqual(b, items[1].quality)

    def test_aged_brie_gains_quality_and_eventually_maxes(self):
        items = [Item("Aged Brie", 2, 0)]
        gilded_rose = GildedRose(items)

        for expected in [1, 2, 4, 6]:
            gilded_rose.update_quality()
            self.assertEqual(expected, items[0].quality)

        while items[0].quality < 50:
            gilded_rose.update_quality()
        gilded_rose.update_quality()
        gilded_rose.update_quality()
        self.assertEqual(50, items[0].quality)

    def test_legendary_sulfuras_has_unwavering_quality(self):
        items = [
            Item("Sulfuras, Hand of Ragnaros", -1, 20),
            Item("Sulfuras, Hand of Ragnaros", 0, 80),
        ]
        gilded_rose = GildedRose(items)
        gilded_rose.update_quality()
        gilded_rose.update_quality()
        gilded_rose.update_quality()
        gilded_rose.update_quality()
        gilded_rose.update_quality()
        gilded_rose.update_quality()
        self.assertEqual(20, items[0].quality)
        self.assertEqual(80, items[1].quality)


if __name__ == '__main__':
    unittest.main()
