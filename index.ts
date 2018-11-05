import { foo, use_console_log, use_macro_console_log } from './Cargo.toml';
console.log(foo('abc'));
use_console_log();
use_macro_console_log();
