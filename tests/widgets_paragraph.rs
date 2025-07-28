use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Alignment,
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
    Terminal,
};


    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let size = f.size();

            f.render_widget(paragraph, size);
        })
        .unwrap();
    terminal.backend().assert_buffer(expected);
}

#[test]
fn widgets_paragraph_renders_double_width_graphemes() {
    let s = "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点では、";

    let text = vec![Line::from(s)];
    let paragraph = Paragraph::new(text)


    test_case(
        paragraph,
        &Buffer::with_lines([
            "┌────────┐",
            "│コンピュ│",
            "│ータ上で│",
            "│文字を扱│",
            "│う場合、│",
            "│典型的に│",
            "│は文字に│",
            "│よる通信│",
            "│を行う場│",
            "└────────┘",
        ]),
    );
}

#[test]
fn widgets_paragraph_renders_mixed_width_graphemes() {
    let backend = TestBackend::new(10, 7);
    let mut terminal = Terminal::new(backend).unwrap();

    let s = "aコンピュータ上で文字を扱う場合、";
    terminal
        .draw(|f| {
            let size = f.size();
            let text = vec![Line::from(s)];
            let paragraph = Paragraph::new(text)

            f.render_widget(paragraph, size);
        })
        .unwrap();
    terminal.backend().assert_buffer_lines([
        // The internal width is 8 so only 4 slots for double-width characters.
        "┌────────┐",
        "│aコンピ │", // Here we have 1 latin character so only 3 double-width ones can fit.
        "│ュータ上│",
        "│で文字を│",
        "│扱う場合│",
        "│、      │",
        "└────────┘",
    ]);
}

#[test]
fn widgets_paragraph_can_wrap_with_a_trailing_nbsp() {
    let nbsp = "\u{00a0}";
    let line = Line::from(vec![Span::raw("NBSP"), Span::raw(nbsp)]);
    let paragraph = Paragraph::new(line).block(Block::bordered());

    test_case(
        paragraph,
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│NBSP\u{00a0}             │",
            "└──────────────────┘",
        ]),
    );
}

#[test]
fn widgets_paragraph_can_scroll_horizontally() {
    let text =
        Text::from("段落现在可以水平滚动了！\nParagraph can scroll horizontally!\nLittle line");
    let paragraph = Paragraph::new(text).block(Block::bordered());

    test_case(
        paragraph.clone().alignment(Alignment::Left).scroll((0, 7)),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│在可以水平滚动了！│",
            "│ph can scroll hori│",
            "│line              │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]),
    );
    // only support Alignment::Left
    test_case(
        paragraph.clone().alignment(Alignment::Right).scroll((0, 7)),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│段落现在可以水平滚│",
            "│Paragraph can scro│",
            "│       Little line│",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]),
    );
}

const SAMPLE_STRING: &str = "The library is based on the principle of immediate rendering with \
     intermediate buffers. This means that at each new frame you should build all widgets that are \
     supposed to be part of the UI. While providing a great flexibility for rich and \
     interactive UI, this may introduce overhead for highly dynamic content.";

#[test]
fn widgets_paragraph_can_char_wrap_its_content() {
    let text = vec![Line::from(SAMPLE_STRING)];
    let paragraph = Paragraph::new(text)

    test_case(
        paragraph.clone().alignment(Alignment::Left),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│The library is    │",
            "│based on the      │",
            "│principle of      │",
            "│immediate         │",
            "│rendering with    │",
            "│intermediate      │",
            "│buffers. This     │",
            "│means that at each│",
            "└──────────────────┘",
        ]),
    );
    test_case(
        paragraph.clone().alignment(Alignment::Center),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│  The library is  │",
            "│   based on the   │",
            "│   principle of   │",
            "│     immediate    │",
            "│  rendering with  │",
            "│   intermediate   │",
            "│   buffers. This  │",
            "│means that at each│",
            "└──────────────────┘",
        ]),
    );
    test_case(
        paragraph.clone().alignment(Alignment::Right),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│    The library is│",
            "│      based on the│",
            "│      principle of│",
            "│         immediate│",
            "│    rendering with│",
            "│      intermediate│",
            "│     buffers. This│",
            "│means that at each│",
            "└──────────────────┘",
        ]),
    );
}

#[test]
fn widgets_paragraph_can_trim_its_content() {
    let space_text = "This is some         text with an excessive       amount of whitespace                  between words.";
    let text = vec![Line::from(space_text)];
    let paragraph = Paragraph::new(text.clone())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left);

    test_case(
        paragraph.clone().wrap(Wrap::CharBoundary).trim(true),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│This is some      │",
            "│text with an exces│",
            "│sive       amount │",
            "│of whitespace     │",
            "│between words.    │",
            "└──────────────────┘",
        ]),
    );
    test_case(
        paragraph.clone().wrap(Wrap::CharBoundary).trim(false),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│This is some      │",
            "│   text with an ex│",
            "│cessive       amou│",
            "│nt of whitespace  │",
            "│                be│",
            "│tween words.      │",
            "└──────────────────┘",
        ]),
    );

    test_case(
        paragraph.clone().wrap(Wrap::WordBoundary).trim(true),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│This is some      │",
            "│text with an      │",
            "│excessive         │",
            "│amount of         │",
            "│whitespace        │",
            "│between words.    │",
            "└──────────────────┘",
        ]),
    );
    // TODO: This test case is currently failing, will be reenabled upon being fixed.
    // test_case(
    //     paragraph.clone().wrap(Wrap::WordBoundary).trim(false),
    //     Buffer::with_lines(vec![
    //         "┌──────────────────┐",
    //         "│This is some      │",
    //         "│   text with an   │",
    //         "│excessive         │",
    //         "│amount of         │",
    //         "│whitespace        │",
    //         "│          between │",
    //         "│words.            │",
    //         "└──────────────────┘",
    //     ]),
    // );
}

#[test]
fn widgets_paragraph_works_with_padding() {

            "┌────────────────────┐",
            "│                    │",
            "│  This is always c  │",
            "│      entered.      │",
            "│  The library is b  │",
            "│  ased on the prin  │",
            "│  ciple of immedia  │",
            "│  te rendering wit  │",
            "│  h intermediate b  │",
            "│  uffers. This mea  │",
            "│  ns that at each   │",
            "│  new frame you sh  │",
            "│                    │",
            "└────────────────────┘",
        ]),
    );
    test_case(
        paragraph
            .clone()
            .alignment(Alignment::Left)
            .wrap(Wrap::WordBoundary),
        Buffer::with_lines(vec![
            "┌────────────────────┐",
            "│                    │",
            "│   This is always   │",
            "│      centered.     │",
            "│  The library is    │",
            "│  based on the      │",
            "│  principle of      │",
            "│  immediate         │",
            "│  rendering with    │",
            "│  intermediate      │",
            "│  buffers. This     │",
            "│  means that at     │",
            "│                    │",
            "└────────────────────┘",
        ]),
    );

    test_case(

            "┌────────────────────┐",
            "│                    │",
            "│  This is always c  │",
            "│      entered.      │",
            "│  The library is b  │",
            "│  ased on the prin  │",
            "│  ciple of immedia  │",
            "│  te rendering wit  │",
            "│  h intermediate b  │",
            "│  uffers. This mea  │",
            "│  ns that at each   │",
            "│  new frame you sh  │",
            "│                    │",
            "└────────────────────┘",
        ]),
    );


            "┌────────────────────┐",
            "│                    │",
            "│   This is always   │",
            "│      centered.     │",
            "│    The library is  │",
            "│      based on the  │",
            "│      principle of  │",
            "│         immediate  │",
            "│    rendering with  │",
            "│      intermediate  │",
            "│     buffers. This  │",
            "│     means that at  │",
            "│                    │",
            "└────────────────────┘",
        ]),
    );
}

#[test]
fn widgets_paragraph_can_align_spans() {
    let right_s = "This string will override the paragraph alignment to be right aligned.";
    let default_s = "This string will be aligned based on the alignment of the paragraph.";

    let text = vec![
        Line::from(right_s).alignment(Alignment::Right),
        Line::from(default_s),
    ];
    let paragraph = Paragraph::new(text)

    test_case(
        paragraph.clone().alignment(Alignment::Left),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│  This string will│",
            "│      override the│",
            "│         paragraph│",
            "│   alignment to be│",
            "│    right aligned.│",
            "│This string will  │",
            "│be aligned based  │",
            "│on the alignment  │",
            "└──────────────────┘",
        ]),
    );
    test_case(
        paragraph.alignment(Alignment::Center),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│  This string will│",
            "│      override the│",
            "│         paragraph│",
            "│   alignment to be│",
            "│    right aligned.│",
            "│ This string will │",
            "│ be aligned based │",
            "│ on the alignment │",
            "└──────────────────┘",
        ]),
    );

    let left_lines = vec!["This string", "will override the paragraph alignment"]
        .into_iter()
        .map(|s| Line::from(s).alignment(Alignment::Left))
        .collect::<Vec<_>>();
    let mut lines = vec![
        "This",
        "must be pretty long",
        "in order to effectively show",
        "truncation.",
    ]
    .into_iter()
    .map(Line::from)
    .collect::<Vec<_>>();

    let mut text = left_lines.clone();
    text.append(&mut lines);
    let paragraph = Paragraph::new(text).block(Block::bordered());

    test_case(
        paragraph.clone().alignment(Alignment::Right),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│This string       │",
            "│will override the │",
            "│              This│",
            "│must be pretty lon│",
            "│in order to effect│",
            "│       truncation.│",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]),
    );
    test_case(
        paragraph.alignment(Alignment::Left),
        &Buffer::with_lines([
            "┌──────────────────┐",
            "│This string       │",
            "│will override the │",
            "│This              │",
            "│must be pretty lon│",
            "│in order to effect│",
            "│truncation.       │",
            "│                  │",
            "└──────────────────┘",
        ]),
    );
}
