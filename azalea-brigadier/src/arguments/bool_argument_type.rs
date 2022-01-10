use crate::context::command_context::CommandContext;

use super::argument_type::ArgumentType;

struct BoolArgumentType {}

impl ArgumentType for BoolArgumentType {}

impl BoolArgumentType {
    const EXAMPLES: &'static [&'static str] = &["true", "false"];

    fn bool() -> Self {
        Self {}
    }

    fn get_bool<S>(context: CommandContext<S>, name: String) {
        context.get_argument::<bool>(name)
    }
}
