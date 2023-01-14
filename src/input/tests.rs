#[cfg(feature = "search")]
use crate::SearchMode;
use crate::{input::InputEvent, LineNumbers, PagerState};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

// Just a transparent function to fix incompatiblity issues between
// versions
// TODO: Remove this later in favour of how handle_event should actually be called
fn handle_input(ev: Event, p: &PagerState) -> Option<InputEvent> {
    p.input_classifier.classify_input(ev, p)
}

// Keyboard navigation
#[test]
#[allow(clippy::too_many_lines)]
fn test_kb_nav() {
    let mut pager = PagerState::new().unwrap();
    pager.upper_mark = 12;
    pager.line_numbers = LineNumbers::Enabled;
    pager.rows = 5;

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark + 1)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark - 1)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(0)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE));
        assert_eq!(
            // rows is 5, therefore upper_mark = upper_mark - rows -1
            Some(InputEvent::UpdateUpperMark(8)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::SHIFT));
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX - 1)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE));
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX - 1)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT));
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX - 1)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE));
        assert_eq!(
            // rows is 5, therefore upper_mark = upper_mark - rows -1
            Some(InputEvent::UpdateUpperMark(16)),
            handle_input(ev, &pager)
        );
    }

    {
        // Half page down
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
        // Rows is 5 and upper_mark is at 12 so result should be 14
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(14)),
            handle_input(ev, &pager)
        );
    }

    {
        // Half page up
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
        // Rows is 5 and upper_mark is at 12 so result should be 10
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(10)),
            handle_input(ev, &pager)
        );
    }
    {
        // Space for page down
        let ev = Event::Key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
        // rows is 5, therefore upper_mark = upper_mark - rows -1
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(16)),
            handle_input(ev, &pager)
        );
    }
    {
        // Enter key for one line down when no message on prompt
        let ev = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        // therefore upper_mark += 1
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(13)),
            handle_input(ev, &pager)
        );
    }
}

#[test]
fn test_restore_prompt() {
    let mut pager = PagerState::new().unwrap();
    pager.message = Some("Prompt message".to_string());
    {
        // Enter key for one line down when no message on prompt
        let ev = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        // therefore upper_mark += 1
        assert_eq!(
            Some(InputEvent::RestorePrompt),
            pager.input_classifier.classify_input(ev, &pager)
        );
    }
}

#[test]
fn test_mouse_nav() {
    let mut pager = PagerState::new().unwrap();
    pager.upper_mark = 12;
    pager.line_numbers = LineNumbers::Enabled;
    pager.rows = 5;
    {
        let ev = Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            row: 0,
            column: 0,
            modifiers: KeyModifiers::NONE,
        });

        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark + 5)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            row: 0,
            column: 0,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark - 5)),
            handle_input(ev, &pager)
        );
    }
}

#[test]
fn test_saturation() {
    let mut pager = PagerState::new().unwrap();
    pager.upper_mark = 12;
    pager.line_numbers = LineNumbers::Enabled;
    pager.rows = 5;

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        // PagerState for local use
        let mut pager = PagerState::new().unwrap();
        pager.upper_mark = usize::MAX;
        pager.line_numbers = LineNumbers::Enabled;
        pager.rows = 5;
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        // PagerState for local use
        let mut pager = PagerState::new().unwrap();
        pager.upper_mark = usize::MIN;
        pager.line_numbers = LineNumbers::Enabled;
        pager.rows = 5;
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MIN)),
            handle_input(ev, &pager)
        );
    }
}

#[test]
fn test_misc_events() {
    let mut pager = PagerState::new().unwrap();
    pager.upper_mark = 12;
    pager.line_numbers = LineNumbers::Enabled;
    pager.rows = 5;

    {
        let ev = Event::Resize(42, 35);
        assert_eq!(
            Some(InputEvent::UpdateTermArea(42, 35)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL));
        assert_eq!(
            Some(InputEvent::UpdateLineNumber(!pager.line_numbers)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        assert_eq!(Some(InputEvent::Exit), handle_input(ev, &pager));
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert_eq!(Some(InputEvent::Exit), handle_input(ev, &pager));
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert_eq!(None, handle_input(ev, &pager));
    }
}

#[test]
#[allow(clippy::too_many_lines)]
#[cfg(feature = "search")]
fn test_search_bindings() {
    let mut pager = PagerState::new().unwrap();
    pager.upper_mark = 12;
    pager.line_numbers = LineNumbers::Enabled;
    pager.rows = 5;

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));
        assert_eq!(
            Some(InputEvent::Search(SearchMode::Forward)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
        assert_eq!(
            Some(InputEvent::Search(SearchMode::Reverse)),
            handle_input(ev, &pager)
        );
    }
    {
        // NextMatch and PrevMatch forward search
        let next_event = Event::Key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
        let prev_event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));

        assert_eq!(
            pager.input_classifier.classify_input(next_event, &pager),
            Some(InputEvent::MoveToNextMatch(1))
        );
        assert_eq!(
            pager.input_classifier.classify_input(prev_event, &pager),
            Some(InputEvent::MoveToPrevMatch(1))
        );
    }

    {
        pager.search_mode = SearchMode::Reverse;
        // NextMatch and PrevMatch reverse search
        let next_event = Event::Key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
        let prev_event = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));

        assert_eq!(
            pager.input_classifier.classify_input(next_event, &pager),
            Some(InputEvent::MoveToPrevMatch(1))
        );
        assert_eq!(
            pager.input_classifier.classify_input(prev_event, &pager),
            Some(InputEvent::MoveToNextMatch(1))
        );
    }
}
