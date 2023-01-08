use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name == "Sulfuras, Hand of Ragnaros" {
                item.quality = 80;
                continue;
            }
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            } else {
                if item.quality < 50 {
                    item.quality = item.quality + 1;

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.quality = item.quality - 1;
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn backstage_passes_gain_then_drop_to_zero_after_concert() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 0)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        assert_eq!(5, rose.items[0].quality);
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        assert_eq!(15, rose.items[0].quality);
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        rose.update_quality();
        assert_eq!(30, rose.items[0].quality);
        rose.update_quality();
        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn normal_item_loses_quality_slowly_then_quickly_then_stops() {
        let items = vec![Item::new("+5 Dexterity Vest", 2, 6)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        assert_eq!(5, rose.items[0].quality);
        rose.update_quality();
        assert_eq!(4, rose.items[0].quality);
        // after 2 days, double decay
        rose.update_quality();
        assert_eq!(2, rose.items[0].quality);
        rose.update_quality();
        assert_eq!(0, rose.items[0].quality);
        // no qualty left to lose
        rose.update_quality();
        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn aged_brie_gains_quality_and_eventually_maxes() {
        let items = vec![Item::new("Aged Brie", 2, 0)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        assert_eq!(1, rose.items[0].quality);
        rose.update_quality();
        assert_eq!(2, rose.items[0].quality);
        // after 2 days, double gain
        rose.update_quality();
        assert_eq!(4, rose.items[0].quality);
        rose.update_quality();
        assert_eq!(6, rose.items[0].quality);
        
        while rose.items[0].quality < 50 {
            rose.update_quality();
        }
        // after many days, max at 50
        rose.update_quality();
        rose.update_quality();
        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn legendary_sulfuras_has_unwavering_quality() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 0, 0)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        assert_eq!(80, rose.items[0].quality);
        rose.update_quality();
        assert_eq!(80, rose.items[0].quality);
    }
}
