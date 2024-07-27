// Script to generate stations.json and graph.json from the MTR data
use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::str::FromStr;

// MTR Station
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Station {
    code: Code,
    id: u32,
    name: String,
    latitude: f64,
    longitude: f64,
    line_code: Code,
}

// Compare stations against other stations by their code
impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

// Compare a station against a code
impl PartialEq<Code> for Station {
    fn eq(&self, &other: &Code) -> bool {
        self.code == other
    }
}

// Compare a station against a string (first 3 chars)
impl PartialEq<&str> for Station {
    fn eq(&self, other: &&str) -> bool {
        self.code == *other
    }
}

// A Code, made up of 3 characters.
#[derive(Debug, Deserialize, Copy, Clone, Eq, Hash)]
struct Code(char, char, char);

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Code {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let code = code.chars().collect::<Vec<char>>();
        return if code.len() != 3 {
            Err(())
        } else {
            Ok(Self(code[0], code[1], code[2]))
        };
    }
}

// simple way to add two codes together
impl Add for Code {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let c1 = self.0 as u32 + other.0 as u32;
        let c2 = self.1 as u32 + other.1 as u32;
        let c3 = self.2 as u32 + other.2 as u32;

        Self(
            std::char::from_u32(c1 % 0x110000).unwrap_or('\0'),
            std::char::from_u32(c2 % 0x110000).unwrap_or('\0'),
            std::char::from_u32(c3 % 0x110000).unwrap_or('\0'),
        )
    }
}

// convert a code to a string (datatype)
impl ToString for Code {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.0, self.1, self.2)
    }
}

// trait to compare codes
impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

// trait to compare a code against a string (first 3 chars)
impl PartialEq<&str> for Code {
    fn eq(&self, other: &&str) -> bool {
        self.0 == other.chars().nth(0).unwrap()
            && self.1 == other.chars().nth(1).unwrap()
            && self.2 == other.chars().nth(2).unwrap()
    }
}

// Station code 1, Station code 2, Distance in minutes
#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq)]
struct Connection(Code, Code, usize);

// trait to compare connections
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

// trait to directly compare a code against both codes of the connection
// as it is undirected
impl PartialEq<Code> for Connection {
    fn eq(&self, other: &Code) -> bool {
        self.0 == *other || self.1 == *other
    }
}

// same as above
impl PartialEq<&str> for Connection {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other || self.1 == *other
    }
}

// hash connections by their codes, so we can have a hashset of connections.
impl Hash for Connection {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write((self.0 + self.1).to_string().as_bytes());
    }
}

fn main() {
    let data = r#"
AWE,AsiaWorld-Expo,22.3219972,113.9422192,-,AEL
AIR,Airport,22.3158106,113.9365921,2,AEL
TSY,Tsing Yi,22.3583861,114.1074312,14,AEL
KOW,Kowloon,22.3048965,114.1616963,13,AEL
HOK,Hong Kong,22.2851469,114.1584546,5,AEL
,,,,,
SUN,Sunny Bay,22.3317036,114.0290377,-,DRL
DIS,Disneyland Resort,22.3155384,114.0451419,6,DRL
,,,,,
LOW,Lo Wu,22.5281624,114.113146,-,EAL
SHS,Sheung Shui,22.501124,114.1280498,6,EAL
FAN,Fanling,22.4920423,114.1386658,4,EAL
TWO,Tai Wo,22.4509658,114.1612012,6,EAL
TAP,Tai Po Market,22.4445147,114.1707659,4,EAL
UNI,University,22.4133852,114.2100545,7,EAL
FOT,Fo Tan,22.3953371,114.198366,5,EAL
SHT,Sha Tin,22.3826796,114.1877459,4,EAL
TAW,Tai Wai,22.3728022,114.1787838,4,EAL
KOT,Kowloon Tong,22.3369989,114.1758397,6,EAL
MKK,Mong Kok East,22.3217126,22.3217126,4,EAL
HUH,Hung Hom,22.3027415,114.182136,5,EAL
EXC,Exhibition Centre,22.2816654,114.1757015,5,EAL
ADM,Admiralty,22.2790893,114.1653075,3,EAL
,,,,,
CHW,Chai Wan,22.2646033,114.2370517,-,ISL
HFC,Heng Fa Chuen,22.276619,114.2397239,3,ISL
SKW,Shau Kei Wan,22.2790272,114.2286373,3,ISL
SWH,Sai Wan Ho,22.2816237,114.2222818,3,ISL
TAK,Tai Koo,22.2844656,114.2163547,3,ISL
QUB,Quarry Bay,22.2881703,114.2089006,3,ISL
NOP,North Point,22.2911365,114.2003964,3,ISL
FOH,Fortress Hill,22.2875389,114.1933631,3,ISL
TIH,Tin Hau,22.2821959,114.1920333,3,ISL
CAB,Causeway Bay,22.2801023,114.1840143,3,ISL
WAC,Wan Chai,22.2775557,114.1729643,3,ISL
ADM,Admiralty,22.2790893,114.1653075,3,ISL
CEN,Central,22.281824,114.1578631,3,ISL
SHW,Sheung Wan,22.2865033,114.1521488,3,ISL
SYP,Sai Ying Pun,22.2854233,114.1427807,3,ISL
HKU,HKU,22.2839642,114.1354563,2,ISL
KET,Kennedy Town,22.2812239,114.128455,2,ISL
,,,,,
TIK,Tiu Keng Leng,22.3040231,114.252426,-,KTL
YAT,Yau Tong,22.2978508,114.237233,4,KTL
LAT,Lam Tin,22.3066758,114.2328977,3,KTL
KWT,Kwun Tong,22.3122845,114.2264543,3,KTL
NTK,Ngau Tau Kok,22.3154451,114.2192556,3,KTL
KOB,Kowloon Bay,22.3235575,114.2141658,3,KTL
CHH,Choi Hung,22.3354008,114.2086311,3,KTL
DIH,Diamond Hill,22.3396397,114.2012733,4,KTL
WTS,Wong Tai Sin,22.3417091,114.1943553,3,KTL
LOF,Lok Fu,22.3380634,114.1869748,3,KTL
KOT,Kowloon Tong,22.336867,114.177541,3,KTL
SKM,Shek Kip Mei,22.3318072,114.1687327,3,KTL
PRE,Prince Edward,22.3244648,114.1683614,3,KTL
MOK,Mong Kok,22.319263,114.1694488,3,KTL
YMT,Yau Ma Tei,22.3130692,114.1707484,3,KTL
HOM,Ho Man Tin,22.3093727,22.3093727,4,KTL
WHA,Whampoa,22.304901,114.1896461,4,KTL
,,,,,
TUM,Tuen Mun,22.3952837,113.9733214,-,TML
SIH,Siu Hong,22.4119019,113.9790545,4,TML
TIS,Tin Shui Wai,22.4480421,114.004696,6,TML
LOP,Long Ping,22.447583,114.0254341,4,TML
YUL,Yuen Long,22.4459701,114.0352445,3,TML
KSR,Kam Sheung Road,22.4347947,114.0633108,5,TML
TWW,Tsuen Wan West,22.3683642,114.1099384,7,TML
MEF,Mei Foo,22.3388904,114.136522,6,TML
NAC,Nam Cheong,22.3266391,114.1538155,4,TML
AUS,Austin,22.3041946,114.166541,4,TML
ETS,East Tsim Sha Tsui,22.2952166,114.1746402,4,TML
HUH,Hung Hom,22.3027415,114.182136,4,TML
HOM,Ho Man Tin,22.3093727,22.3093727,4,TML
TKW,To Kwa Wan,22.3169869,114.1875844,4,TML
SUW,Sung Wong Toi,22.32578,114.1914117,4,TML
KAT,Kai Tak,22.3303836,114.1992233,3,TML
DIH,Diamond Hill,22.3396397,114.2012733,4,TML
HIK,Hin Keng,22.3638205,114.1707186,6,TML
TAW,Tai Wai,22.3728022,114.1787838,4,TML
CKT,Che Kung Temple,22.3747809,114.1860976,3,TML
STW,Sha Tin Wai,22.3769634,114.1947694,3,TML
CIO,City One,22.3830131,114.203784,4,TML
SHM,Shek Mun,22.3877892,114.2085474,3,TML
TSH,Tai Shui Hang,22.4084544,114.222844,5,TML
HEO,Heng On,22.4178164,114.2259505,3,TML
MOS,Ma On Shan,22.4249134,114.2317637,4,TML
WKS,Wu Kai Sha,22.4292202,114.243854,,TML
,,,,,
TUC,Tung Chung,22.289278,113.9413221,-,TCL
SUN,Sunny Bay,22.3318274,114.0288778,10,TCL
TSY,Tsing Yi,22.3583861,114.1074312,10,TCL
LAK,Lai King,22.3484137,114.12604,5,TCL
NAC,Nam Cheong,22.3263889,114.1536861,6,TCL
OLY,Olympic,22.317703,114.1604114,4,TCL
KOW,Kowloon,22.3048965,114.1616963,4,TCL
HOK,Hong Kong,22.2851469,114.1584546,5,TCL
,,,,,
POA,Po Lam,22.3225369,114.2579671,-,TKL
HAH,Hang Hau,22.3155885,114.2644721,3,TKL
TKO,Tseung Kwan O,22.3074422,114.2599465,4,TKL
TKO,Lohas Park,22.2956555,114.2687716,7,TKL
TIK,Tiu Keng Leng,22.3040231,114.252426,3,TKL
YAT,Yau Tong,22.2977718,114.2370927,4,TKL
QUB,Quarry Bay,22.2882652,114.2090338,5,TKL
NOP,North Point,22.2904837,114.2005605,3,TKL
,,,,,
TSW,Tsuen Wan,22.3683642,114.1099384,-,TWL
TWH,Tai Wo Hau,22.3708,114.125,3,TWL
KWH,Kwai Hing,22.3632,114.1312,3,TWL
KWF,Kwai Fong,22.3569,114.1279,3,TWL
LAK,Lai King,22.3484,114.1261,4,TWL
MEF,Mei Foo,22.3388904,114.136522,3,TWL
LCK,Lai Chi Kok,22.3373,114.1482,3,TWL
CSW,Cheung Sha Wan,22.3354,114.1563,3,TWL
SSP,Sham Shui Po,22.3307,114.1623,3,TWL
PRE,Prince Edward,22.3245,114.1683,3,TWL
MOK,Mong Kok,22.3191,114.1694,3,TWL
YMT,Yau Ma Tei,22.3129,114.1707,3,TWL
JOR,Jordan,22.3049,114.1718,3,TWL
TST,Tsim Sha Tsui,22.2973,114.1722,3,TWL
ADM,Admiralty,22.2788,114.1646,5,TWL
CEN,Central,22.282,114.1576,3,TWL
,,,,,
SOH,South Horizons,22.2425,114.1491,-,SIL
LET,Lei Tung,22.2421,114.1562,4,SIL
WCH,Wong Chuk Hang,22.248,114.1681,4,SIL
OCP,Ocean Park,22.2486,114.1743,4,SIL
ADM,Admiralty,22.2788,114.1646,6,SIL
,,,,,
CEN,Central,22.282,114.1576,-,WLK
HOK,Hong Kong,22.2851469,114.1584546,8,WLK
TST,Tsim Sha Tsui,22.2973,114.1722,-,WLK
ETS,East Tsim Sha Tsui,22.2952166,114.1746402,6,WLK
"#;

    let mut lines: Vec<&str> = data.lines().collect();
    lines.retain(|line| !line.trim().is_empty()); // remove empty lines

    let mut station_list: HashMap<Code, Station> = HashMap::new();
    let mut graph = HashSet::<Connection>::new();

    let mut prev_station: Option<Station> = None;
    let mut prev_line_code = "".to_string();

    for line in lines {
        let columns: Vec<&str> = line.split(',').collect();

        if columns[0] == "" {
            prev_station = None;
            continue;
        }

        let code = Code(
            columns[0].chars().nth(0).unwrap(),
            columns[0].chars().nth(1).unwrap(),
            columns[0].chars().nth(2).unwrap(),
        );
        let name = columns[1].to_string();
        let latitude = columns[2].parse::<f64>().unwrap();
        let longitude = columns[3].parse::<f64>().unwrap();
        let line_code = columns[5].to_string();

        let station = Station {
            code,
            id: 0, // assign a default id for now
            name,
            latitude,
            longitude,
            line_code: Code(
                line_code.chars().next().unwrap(),
                line_code.chars().nth(1).unwrap(),
                line_code.chars().nth(2).unwrap(),
            ),
        };

        station_list.insert(station.code, station.clone());

        if let Some(prev_station) = prev_station {
            if prev_station.line_code.to_string() == line_code {
                let minutes = columns[4].parse::<usize>().unwrap_or(0);
                let connection = Connection(prev_station.code, station.code, minutes);
                graph.insert(connection);
            }
        }

        prev_station = Some(station);
    }

    // write variables as json
    let station_list_json = serde_json::to_string_pretty(&station_list).unwrap();
    let graph_json = serde_json::to_string_pretty(&graph).unwrap();

    std::fs::write("stations.json", station_list_json).expect("Failed to write to file");
    std::fs::write("graph.json", graph_json).expect("Failed to write to file");
}
