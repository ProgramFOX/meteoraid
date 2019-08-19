#[derive(Copy, Clone)]
pub struct Meteor {
    pub shower: Shower,
    pub magnitude: i32,
}

#[derive(Copy, Clone, PartialEq)]
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
}

use Shower::*;
impl Shower {
    pub fn from_imo_code(imo_code: &str) -> Option<Shower> {
        match imo_code {
            "QUA" => Some(Quadrantids),
            "LYR" => Some(Lyrids),
            "ETA" => Some(EtaAquarids),
            "JBO" => Some(JuneBootids),
            "SDA" => Some(DeltaAquariids),
            "CAP" => Some(AlphaCapricornids),
            "PER" => Some(Perseids),
            "KCG" => Some(KappaCygnids),
            "AUR" => Some(AlphaAurigids),
            "SPE" => Some(SeptemberEpsilonPerseids),
            "OCT" => Some(OctoberCameloparalids),
            "DRA" => Some(Draconids),
            "EGE" => Some(EpsilonGeminids),
            "ORI" => Some(Orionids),
            "STA" => Some(SouthernTaurids),
            "NTA" => Some(NorthernTaurids),
            "LEO" => Some(Leonids),
            "DAD" => Some(DecemberAlphaDraconids),
            "MON" => Some(Monocerotids),
            "HYD" => Some(SigmaHydrids),
            "GEM" => Some(Geminids),
            "DLM" => Some(DecemberLeoMinorids),
            "COM" => Some(ComaBerenicids),
            "URS" => Some(Ursids),
            _ => None,
        }
    }

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
        }
    }
}
