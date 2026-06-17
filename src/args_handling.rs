use std::path::PathBuf;

pub struct ParsedArgs
{
    pub filename: Option<PathBuf>,
    pub config_file: PathBuf,
    pub config_dir: PathBuf,
}

pub fn parse_arguments(args: Vec<String>) -> Result<ParsedArgs, String>
{
    let mut filename: Option<PathBuf> = None;
    let mut config_path: Option<String> = None;

    let mut i = 1;
    while i < args.len()
    {
        if args[i] == "--config" || args[i] == "-c"
        {
            i += 1;
            if i >= args.len()
            {
                return Err(format!("missing argument for {}", args[i - 1]));
            }
            config_path = Some(args[i].clone());
        }
        else if filename.is_none()
        {
            filename = Some(PathBuf::from(&args[i]));
        }

        i += 1;
    }

    let config_file = match config_path
    {
        Some(p) => PathBuf::from(p),
        None    => PathBuf::from("config/init.lua"),
    };

    let config_file = std::fs::canonicalize(&config_file)
        .unwrap_or(config_file);

    let parent = config_file.parent();
    
    let config_dir = match parent
    {
        Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
        _ => PathBuf::from("."),
    };
    
    return Ok(ParsedArgs { filename, config_file, config_dir });
}