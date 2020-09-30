use crate::cli::Opt as CliOpt;
use code_minimap::Opt;

/// This should be called before calling any cli method or printing any output.
pub(crate) fn reset_signal_pipe_handler() {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;
        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl).unwrap();
        }
    }
}

impl From<CliOpt> for Opt {
    fn from(opt: CliOpt) -> Self {
        Opt {
            file:    opt.file,
            hscale:  opt.hscale,
            vscale:  opt.vscale,
            padding: opt.padding,
        }
    }
}
