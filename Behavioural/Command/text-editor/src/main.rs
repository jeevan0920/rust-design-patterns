pub trait Command {
    fn execute(&self, editor: &mut TextEditor);
    fn rollback(&self, editor: &mut TextEditor);
}

pub struct TextEditor {
    text: String,
}

#[derive(Debug, Clone)]
struct AddTextCommand {
    text: String,
}

impl Command for AddTextCommand {
    fn execute(&self, editor: &mut TextEditor) {
        println!("AddTextCommand.execute()");
        editor.text.push_str(&self.text);
    }

    fn rollback(&self, editor: &mut TextEditor) {
        println!("AddTextCommand.rollback()");
        editor.text = editor.text[0..(editor.text.len() - self.text.len())].to_string();
    }
}

struct DeleteTextCommand {
    text: String,
}

impl Command for DeleteTextCommand {
    fn execute(&self, editor: &mut TextEditor) {
        println!("DeleteTextCommand.execute()");
        editor.text = editor.text.replace(&self.text, "");
    }

    fn rollback(&self, editor: &mut TextEditor) {
        println!("DeleteTextCommand.rollback()");
        editor.text.push_str(&self.text);
    }
}

pub struct CommandHistory {
    commands: Vec<Box<dyn Command>>,
}

impl CommandHistory {
    pub fn new() -> CommandHistory {
        CommandHistory {
            commands: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, command: Box<dyn Command>, editor: &mut TextEditor) {
        command.execute(editor);
        self.commands.push(command);
    }

    pub fn undo(&mut self, editor: &mut TextEditor) {
        if let Some(command) = self.commands.pop() {
            command.rollback(editor);
        }
    }
}

impl TextEditor {
    pub fn new() -> TextEditor {
        TextEditor {
            text: String::new(),
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}

fn main() {
    let mut editor = TextEditor::new();
    let mut history = CommandHistory::new();

    let add_hello_command = AddTextCommand {
        text: "Hello, ".to_string(),
    };

    history.execute_command(Box::new(add_hello_command.clone()), &mut editor);

    println!("Text: {}", editor.get_text());

    let delete_command: DeleteTextCommand = DeleteTextCommand {
        text: "Hello, ".to_string(),
    };
    history.execute_command(Box::new(delete_command), &mut editor);

    // add hello
    history.execute_command(Box::new(add_hello_command), &mut editor);

    println!("Text: {}", editor.get_text());

    // add world
    let add_world_command = AddTextCommand {
        text: "World!".to_string(),
    };

    history.execute_command(Box::new(add_world_command), &mut editor);

    println!("Text: {}", editor.get_text());

    history.undo(&mut editor);

    println!("Text: {}", editor.get_text());

    history.undo(&mut editor);

    println!("Text: {}", editor.get_text());
}
