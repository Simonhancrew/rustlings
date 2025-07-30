#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

impl Order {
    fn clone(&self) -> Self {
        Order {
            name: self.name.clone(),
            year: self.year,
            made_by_phone: self.made_by_phone,
            made_by_mobile: self.made_by_mobile,
            made_by_email: self.made_by_email,
            item_number: self.item_number,
            count: self.count,
        }
    }
    fn update(&self, name: String, count: u32) -> Self {
        let mut res = self.clone();
        res.name = name;
        res.count = count;
        res
    }
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
        let your_order = order_template.update(String::from("Hacker in Rust"), 1);
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
