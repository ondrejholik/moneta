use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use comfy_table::{Table, Color, Attribute, Cell, Row, presets::UTF8_FULL};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "https://www.perfectcanteen.cz/podniky/moneta-money-bank";
    
    // Fetch HTML content
    let client = Client::new();
    let response = client.get(URL)
        .header("User-Agent", "Mozilla/5.0")
        .send()?;
    let html = response.text()?;
    
    // Parse HTML document
    let document = Html::parse_document(&html);
    
    // Prepare selectors
    let menu_item_selector = Selector::parse("div.menu-item-wrapper").unwrap();
    let category_selector = Selector::parse("div[style='color: #cb8e72;']").unwrap();
    let name_selector = Selector::parse("div.text-body.text-white").unwrap();
    let price_selector = Selector::parse("div.text-body.text-white.menu-item-price").unwrap();
    
    let mut seen_categories = HashSet::new();
    let mut current_category: Option<String> = None;
    
    // Initialize table with styling
    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
        .set_header(Row::from(vec![
            Cell::new("Category").fg(Color::Cyan).add_attribute(Attribute::Bold),
            Cell::new("Dish").fg(Color::Cyan).add_attribute(Attribute::Bold),
            Cell::new("Price").fg(Color::Cyan).add_attribute(Attribute::Bold),
        ]));
    
    // Process each menu item
    for item in document.select(&menu_item_selector) {
        // Update current category if found
        if let Some(category_element) = item.select(&category_selector).next() {
            let category_text = category_element.text().collect::<String>().trim().to_string();
            seen_categories.insert(category_text.clone());
            current_category = Some(category_text);
        }
        
        // Extract name and price
        let name = item.select(&name_selector).next()
            .map(|e| e.text().collect::<String>().trim().to_string());
        let price = item.select(&price_selector).next()
            .map(|e| e.text().collect::<String>().trim().to_string());
        
        if let (Some(name), Some(price), Some(category)) = (name, price, &current_category) {
            if seen_categories.contains(category) {
                // Break condition for special category
                if category == "#perfect chef" {
                    break;
                }
                
                // Determine emoji based on category
                let emoji = if category.to_lowercase().contains("polévka") {
                    "🍜"
                } else {
                    "🥗"
                };
                
                // Add formatted row to table
                table.add_row(vec![
                    Cell::new(format!("{} {}", emoji, category))
                        .fg(Color::Yellow)
                        .add_attribute(Attribute::Bold),
                    Cell::new(name)
                        .fg(Color::White)
                        .add_attribute(Attribute::Bold),
                    Cell::new(price)
                        .fg(Color::Green)
                        .add_attribute(Attribute::Bold),
                ]);
                
                // Prevent duplicate categories
                seen_categories.remove(category);
            }
        }
    }
    
    // Print the formatted table
    println!("{}", table);
    
    Ok(())
}
