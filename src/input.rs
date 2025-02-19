use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct Opts {
    /// H1 username
    #[structopt(short="u",long="name")]
    pub username: String,


    /// H1 apikey
    #[structopt(short="k",long="key")]
    pub key : String,

}

#[warn(unused_mut)]
impl Opts {
    pub fn read() -> Self {
        let opts = Opts::from_args();

        opts
    }


}
