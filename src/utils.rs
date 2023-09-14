use super::*;
use std::str::FromStr;


/// Parses the server address from a list of arguments.
pub fn get_bind_address(args: &Vec<String>) -> Result<SocketAddr> {
    let arg = args
        .get(ARG_POS_ADDRESS)
        .ok_or_else(|| anyhow!("Missing address argument on pos [{}]", ARG_POS_ADDRESS))?;

    SocketAddr::from_str(arg)
        .map_err(|err| {
            anyhow!("Invalid address: {}", err)
        })
}
