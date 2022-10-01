use crate::batch::run_next_app;

//包含任务处理相关的 syscall
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}
