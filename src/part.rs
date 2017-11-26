use item::Item;
use item_stream::ItemStream;

pub struct Part
{
    pub size: u64,
    pub items: Vec<Item>,
}

impl Part
{
    pub fn new() -> Part
    {
        Part
        {
            size: 0,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Item)
    {
        self.size += item.size();
        self.items.push(item);
    }

    pub fn partition<'a>(stream: ItemStream<'a>, size: u64) -> Vec<Part>
    {
        let mut current = Part::new();
        let mut parts = Vec::new();

        for item in stream
        {
            if item.size() > size
            {
                let mut part = Part::new();
                part.add(item);
                parts.push(part);
            }
            else if item.size() + current.size > size
            {
                parts.push(current);
                current = Part::new();
                current.add(item);
            }
            else
            {
                current.add(item);
            }
        }

        parts.push(current);
        parts
    }
}
