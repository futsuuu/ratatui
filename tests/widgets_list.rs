use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState},
    Terminal,
};

#[test]
fn list_should_shows_the_length() {
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];
    let list = List::new(items);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());

    let empty_list = List::default();
    assert_eq!(empty_list.len(), 0);
    assert!(empty_list.is_empty());
}

#[test]
fn widgets_list_should_highlight_the_selected_item() {
    let backend = TestBackend::new(10, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut state = ListState::default();
    state.select(Some(1));
    terminal
        .draw(|f| {
            let size = f.size();
            let items = vec![
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
            ];
            let list = List::new(items)
                .highlight_style(Style::default().bg(Color::Yellow))
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut state);
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec!["   Item 1 ", ">> Item 2 ", "   Item 3 "]);
    for x in 0..10 {
        expected.get_mut(x, 1).set_bg(Color::Yellow);
    }
    terminal.backend().buffer().assert_eq(&expected);
}

#[test]
fn widgets_list_should_highlight_the_selected_item_wide_symbol() {
    let backend = TestBackend::new(10, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut state = ListState::default();

    let wide_symbol = "▶  ";

    state.select(Some(1));
    terminal
        .draw(|f| {
            let size = f.size();
            let items = vec![
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
            ];
            let list = List::new(items)
                .highlight_style(Style::default().bg(Color::Yellow))
                .highlight_symbol(wide_symbol);
            f.render_stateful_widget(list, size, &mut state);
        })
        .unwrap();

    let mut expected = Buffer::with_lines(vec!["   Item 1 ", "▶  Item 2 ", "   Item 3 "]);

    for x in 0..10 {
        expected.get_mut(x, 1).set_bg(Color::Yellow);
    }
    terminal.backend().buffer().assert_eq(&expected);
}

#[test]
fn widgets_list_should_truncate_items() {
    struct TruncateTestCase<'a> {
        selected: Option<usize>,
        items: Vec<ListItem<'a>>,
        expected: Buffer,
    }

    let backend = TestBackend::new(10, 2);
    let mut terminal = Terminal::new(backend).unwrap();

    let cases = [
        // An item is selected
        TruncateTestCase {
            selected: Some(0),
            items: vec![
                ListItem::new("A very long line"),
                ListItem::new("A very long line"),
            ],
            expected: Buffer::with_lines(vec![
                format!(">> A ve{}  ", symbols::line::VERTICAL),
                format!("   A ve{}  ", symbols::line::VERTICAL),
            ]),
        },
        // No item is selected
        TruncateTestCase {
            selected: None,
            items: vec![
                ListItem::new("A very long line"),
                ListItem::new("A very long line"),
            ],
            expected: Buffer::with_lines(vec![
                format!("A very {}  ", symbols::line::VERTICAL),
                format!("A very {}  ", symbols::line::VERTICAL),
            ]),
        },
    ];
    for case in cases {
        let mut state = ListState::default();
        state.select(case.selected);
        terminal
            .draw(|f| {
                let list = List::new(case.items.clone())
                    .block(Block::default().borders(Borders::RIGHT))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(list, Rect::new(0, 0, 8, 2), &mut state);
            })
            .unwrap();
        terminal.backend().buffer().assert_eq(&case.expected);
    }
}

#[test]
fn widgets_list_should_clamp_offset_if_items_are_removed() {
    let backend = TestBackend::new(10, 4);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut state = ListState::default();

    // render with 6 items => offset will be at 2
    state.select(Some(5));
    terminal
        .draw(|f| {
            let size = f.size();
            let items = vec![
                ListItem::new("Item 0"),
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
                ListItem::new("Item 4"),
                ListItem::new("Item 5"),
            ];
            let list = List::new(items).highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut state);
        })
        .unwrap();
    terminal
        .backend()
        .buffer()
        .assert_eq(&Buffer::with_lines(vec![
            "   Item 2 ",
            "   Item 3 ",
            "   Item 4 ",
            ">> Item 5 ",
        ]));

    // render again with 1 items => check offset is clamped to 1
    state.select(Some(1));
    terminal
        .draw(|f| {
            let size = f.size();
            let items = vec![ListItem::new("Item 3")];
            let list = List::new(items).highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut state);
        })
        .unwrap();
    terminal
        .backend()
        .buffer()
        .assert_eq(&Buffer::with_lines(vec![
            "   Item 3 ",
            "          ",
            "          ",
            "          ",
        ]));
}

#[test]
fn widgets_list_should_display_multiline_items() {
    let backend = TestBackend::new(10, 6);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut state = ListState::default();
    state.select(Some(1));
    terminal
        .draw(|f| {
            let size = f.size();
            let items = vec![
                ListItem::new(vec![Line::from("Item 1"), Line::from("Item 1a")]),
                ListItem::new(vec![Line::from("Item 2"), Line::from("Item 2b")]),
                ListItem::new(vec![Line::from("Item 3"), Line::from("Item 3c")]),
            ];
            let list = List::new(items)
                .highlight_style(Style::default().bg(Color::Yellow))
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut state);
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "   Item 1 ",
        "   Item 1a",
        ">> Item 2 ",
        "   Item 2b",
        "   Item 3 ",
        "   Item 3c",
    ]);
    for x in 0..10 {
        expected.get_mut(x, 2).set_bg(Color::Yellow);
        expected.get_mut(x, 3).set_bg(Color::Yellow);
    }
    terminal.backend().buffer().assert_eq(&expected);
}

#[test]
fn widgets_list_should_repeat_highlight_symbol() {
    let backend = TestBackend::new(10, 6);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut state = ListState::default();
    state.select(Some(1));
    terminal
        .draw(|f| {
            let size = f.size();
            let items = vec![
                ListItem::new(vec![Line::from("Item 1"), Line::from("Item 1a")]),
                ListItem::new(vec![Line::from("Item 2"), Line::from("Item 2b")]),
                ListItem::new(vec![Line::from("Item 3"), Line::from("Item 3c")]),
            ];
            let list = List::new(items)
                .highlight_style(Style::default().bg(Color::Yellow))
                .highlight_symbol(">> ")
                .repeat_highlight_symbol(true);
            f.render_stateful_widget(list, size, &mut state);
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "   Item 1 ",
        "   Item 1a",
        ">> Item 2 ",
        ">> Item 2b",
        "   Item 3 ",
        "   Item 3c",
    ]);
    for x in 0..10 {
        expected.get_mut(x, 2).set_bg(Color::Yellow);
        expected.get_mut(x, 3).set_bg(Color::Yellow);
    }
    terminal.backend().buffer().assert_eq(&expected);
}

#[test]
fn widget_list_should_not_ignore_empty_string_items() {
    let backend = TestBackend::new(6, 4);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let items = vec![
                ListItem::new("Item 1"),
                ListItem::new(""),
                ListItem::new(""),
                ListItem::new("Item 4"),
            ];

            let list = List::new(items)
                .style(Style::default())
                .highlight_style(Style::default());

            f.render_widget(list, f.size());
        })
        .unwrap();
    terminal
        .backend()
        .buffer()
        .assert_eq(&Buffer::with_lines(vec!["Item 1", "", "", "Item 4"]));
}

#[allow(clippy::too_many_lines)]
#[test]
fn widgets_list_enable_always_highlight_spacing() {
    #[track_caller]
    fn test_case(state: &mut ListState, space: HighlightSpacing, expected: &Buffer) {
        let backend = TestBackend::new(30, 8);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let size = f.size();
                let table = List::new(vec![
                    ListItem::new(vec![Line::from("Item 1"), Line::from("Item 1a")]),
                    ListItem::new(vec![Line::from("Item 2"), Line::from("Item 2b")]),
                    ListItem::new(vec![Line::from("Item 3"), Line::from("Item 3c")]),
                ])
                .block(Block::default().borders(Borders::ALL))
                .highlight_symbol(">> ")
                .highlight_spacing(space);
                f.render_stateful_widget(table, size, state);
            })
            .unwrap();
        terminal.backend().buffer().assert_eq(expected);
    }

    assert_eq!(HighlightSpacing::default(), HighlightSpacing::WhenSelected);

    let mut state = ListState::default();
    // no selection, "WhenSelected" should only allocate if selected
    test_case(
        &mut state,
        HighlightSpacing::default(),
        &Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Item 1                      │",
            "│Item 1a                     │",
            "│Item 2                      │",
            "│Item 2b                     │",
            "│Item 3                      │",
            "│Item 3c                     │",
            "└────────────────────────────┘",
        ]),
    );

    // no selection, "Always" should allocate regardless if selected or not
    test_case(
        &mut state,
        HighlightSpacing::Always,
        &Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│   Item 1                   │",
            "│   Item 1a                  │",
            "│   Item 2                   │",
            "│   Item 2b                  │",
            "│   Item 3                   │",
            "│   Item 3c                  │",
            "└────────────────────────────┘",
        ]),
    );

    // no selection, "Never" should never allocate regadless if selected or not
    test_case(
        &mut state,
        HighlightSpacing::Never,
        &Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Item 1                      │",
            "│Item 1a                     │",
            "│Item 2                      │",
            "│Item 2b                     │",
            "│Item 3                      │",
            "│Item 3c                     │",
            "└────────────────────────────┘",
        ]),
    );

    // select first, "WhenSelected" should only allocate if selected
    state.select(Some(0));
    test_case(
        &mut state,
        HighlightSpacing::default(),
        &Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│>> Item 1                   │",
            "│   Item 1a                  │",
            "│   Item 2                   │",
            "│   Item 2b                  │",
            "│   Item 3                   │",
            "│   Item 3c                  │",
            "└────────────────────────────┘",
        ]),
    );

    // select first, "Always" should allocate regardless if selected or not
    state.select(Some(0));
    test_case(
        &mut state,
        HighlightSpacing::Always,
        &Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│>> Item 1                   │",
            "│   Item 1a                  │",
            "│   Item 2                   │",
            "│   Item 2b                  │",
            "│   Item 3                   │",
            "│   Item 3c                  │",
            "└────────────────────────────┘",
        ]),
    );

    // select first, "Never" should never allocate regadless if selected or not
    state.select(Some(0));
    test_case(
        &mut state,
        HighlightSpacing::Never,
        &Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Item 1                      │",
            "│Item 1a                     │",
            "│Item 2                      │",
            "│Item 2b                     │",
            "│Item 3                      │",
            "│Item 3c                     │",
            "└────────────────────────────┘",
        ]),
    );
}
