#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Meteor {
    pub shower: Shower,
    pub magnitude: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Shower {
    Quadrantids,
    Lyrids,
    EtaAquarids,
    JuneBootids,
    DeltaAquariids,
    AlphaCapricornids,
    Perseids,
    KappaCygnids,
    AlphaAurigids,
    SeptemberEpsilonPerseids,
    OctoberCameloparalids,
    Draconids,
    EpsilonGeminids,
    Orionids,
    SouthernTaurids,
    NorthernTaurids,
    Leonids,
    DecemberAlphaDraconids,
    Monocerotids,
    SigmaHydrids,
    Geminids,
    DecemberLeoMinorids,
    ComaBerenicids,
    Ursids,
    Antihelion,
    Sporadic,
}

use Shower::*;
impl Shower {
    pub fn to_imo_code(&self) -> &str {
        match self {
            Quadrantids => "QUA",
            Lyrids => "LYR",
            EtaAquarids => "ETA",
            JuneBootids => "JBO",
            DeltaAquariids => "SDA",
            AlphaCapricornids => "CAP",
            Perseids => "PER",
            KappaCygnids => "KCG",
            AlphaAurigids => "AUR",
            SeptemberEpsilonPerseids => "SPE",
            OctoberCameloparalids => "OCT",
            Draconids => "DRA",
            EpsilonGeminids => "EGE",
            Orionids => "ORI",
            SouthernTaurids => "STA",
            NorthernTaurids => "NTA",
            Leonids => "LEO",
            DecemberAlphaDraconids => "DAD",
            Monocerotids => "MON",
            SigmaHydrids => "HYD",
            Geminids => "GEM",
            DecemberLeoMinorids => "DLM",
            ComaBerenicids => "COM",
            Ursids => "URS",
            Antihelion => "ANT",
            Sporadic => "SPO",
        }
    }
}
