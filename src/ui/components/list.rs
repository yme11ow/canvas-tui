/*  use ratatui::widgets::{List, ListItem};

 pub fn create_list<'a, T, F>(
     items: &[T],
     get_text: F,
 ) -> List<'a>
 where
     F: Fn(&T) -> String,
 {
     let items: Vec<ListItem> = items
         .iter()
         .map(|item| ListItem::new(get_text(item)))
         .collect();

     List::new(items)
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Cyan))
}
*/