use clap::{Parser, Subcommand};
use std::path::PathBuf;
use vikas_runtime::VikasRuntime;

#[derive(Parser)]
#[command(name = "vikas")]
#[command(about = "Vikas.js Runtime", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a JavaScript file
    Run {
        #[arg(required = true)]
        file: PathBuf,
    },

    /// Evaluate JavaScript code
    Eval {
        #[arg(required = true)]
        code: String,
    },

    /// Start an HTTP server
    Serve {
        #[arg(short, long, default_value = "3000")]
        port: u16,
        #[arg(required = true)]
        file: PathBuf,
    },

    /// Create a new project
    Create {
        #[arg(required = true)]
        name: String,
    },

    /// Show version information
    Version,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let mut runtime = VikasRuntime::new();
            let code = std::fs::read_to_string(&file)?;
            match runtime.execute_script(&code) {
                Ok(result) => {
                    println!("Result: {}", result);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Eval { code } => {
            let mut runtime = VikasRuntime::new();
            match runtime.execute_script(&code) {
                Ok(result) => {
                    println!("Result: {}", result);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Serve { port, file } => {
            let mut runtime = VikasRuntime::new();
            let code = std::fs::read_to_string(&file)?;

            if let Err(e) = runtime.execute_script(&code) {
                eprintln!("Error executing file: {}", e);
                std::process::exit(1);
            }

            let server = runtime.create_http_server();
            println!("Starting server on port {}", port);
            tokio::runtime::Runtime::new()?.block_on(async {
                server.listen(port).await?;
                Ok::<_, Box<dyn std::error::Error>>(())
            })?;

            Ok(())
        }
        Commands::Create { name } => {
            create_project(&name)?;
            Ok(())
        }
        Commands::Version => {
            println!("Vikas.js v0.1.0");
            Ok(())
        }
    }
}

fn create_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = PathBuf::from(name);
    if project_path.exists() {
        eprintln!("Project '{}' already exists", name);
        std::process::exit(1);
    }

    std::fs::create_dir_all(&project_path)?;
    std::fs::create_dir_all(project_path.join("src"))?;

    let package_json = format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "type": "module",
  "scripts": {{
    "start": "vikas serve src/index.js",
    "dev": "vikas serve src/index.js --port 3000"
  }},
  "dependencies": {{
    "vikas": "0.1.0"
  }}
}}
"#,
        name
    );

    let index_js = r#"
// Vikas.js HTTP Server
console.log("🚀 Vikas.js server starting...");

// Routes are defined here
const routes = {
    "/": (req) => {
        return {
            status: 200,
            headers: { "Content-Type": "text/html" },
            body: "<h1>Welcome to Vikas.js!</h1>"
        };
    },
    "/api/hello": (req) => {
        return {
            status: 200,
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ message: "Hello from Vikas.js!" })
        };
    }
};

// Export routes to be used by the server
export default routes;
"#;

    std::fs::write(project_path.join("package.json"), package_json)?;
    std::fs::write(project_path.join("src").join("index.js"), index_js)?;

    println!("✅ Project '{}' created successfully!", name);
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  npm install");
    println!("  npm start");
    println!("  or");
    println!("  vikas serve src/index.js");

    Ok(())
}
