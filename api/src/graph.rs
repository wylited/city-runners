use std::collections::{HashMap, HashSet};

use crate::station::{Code, Connection, Station};

pub struct Graph {
    pub stations: HashMap<Code, Station>,
    pub connections: HashSet<Connection>,
}

impl Graph {
    pub fn blank() -> Self {
        Graph {
            stations: HashMap::new(),
            connections: HashSet::new(),
        }
    }

    pub fn from_csv() -> Self {
        let mut lines: Vec<&str> = CSV.lines().collect();
        lines.retain(|line| !line.trim().is_empty());

        let mut stations: HashMap<Code, Station> = HashMap::new();
        let mut connections = HashSet::<Connection>::new();

        let mut prev_station: Option<Station> = None;

        for line in lines {
            let columns: Vec<&str> = line.split(',').collect();

            if columns[0].is_empty() {
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
                name,
                latitude,
                longitude,
                line_code: Code(
                    line_code.chars().next().unwrap(),
                    line_code.chars().nth(1).unwrap(),
                    line_code.chars().nth(2).unwrap(),
                ),
            };

            stations.insert(station.code, station.clone());

            if let Some(prev_station) = prev_station {
                if prev_station.line_code.to_string() == line_code {
                    let minutes = columns[4].parse::<usize>().unwrap_or(0);
                    let connection = Connection(prev_station.code, station.code, minutes);
                    connections.insert(connection);
                }
            }

            prev_station = Some(station);
        }

        Graph {
            stations,
            connections,
        }
    }

    pub fn from_ron() -> Self {
        let stations: HashMap<Code, Station> = ron::de::from_str(STATIONS).unwrap();
        let connections: HashSet<Connection> = ron::de::from_str(CONNECTIONS).unwrap();

        Graph {
            stations,
            connections,
        }
    }
}

const CSV: &str = r#"
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
TIK,Tiu Keng Leng,22.3040231,114.252426,3,TKL
YAT,Yau Tong,22.2977718,114.2370927,4,TKL
QUB,Quarry Bay,22.2882652,114.2090338,5,TKL
NOP,North Point,22.2904837,114.2005605,3,TKL
,,,,,
TKO,Tseung Kwan O,22.3074422,114.2599465,-,TKL
TKO,Lohas Park,22.2956555,114.2687716,7,TKL
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

const STATIONS: &str = r#"
{
    "NAC": (
        code: "NAC",
        id: 0,
        name: "Nam Cheong",
        latitude: 22.3263889,
        longitude: 114.1536861,
        line_code: "TCL",
    ),
    "WHA": (
        code: "WHA",
        id: 0,
        name: "Whampoa",
        latitude: 22.304901,
        longitude: 114.1896461,
        line_code: "KTL",
    ),
    "TIK": (
        code: "TIK",
        id: 0,
        name: "Tiu Keng Leng",
        latitude: 22.3040231,
        longitude: 114.252426,
        line_code: "TKL",
    ),
    "ETS": (
        code: "ETS",
        id: 0,
        name: "East Tsim Sha Tsui",
        latitude: 22.2952166,
        longitude: 114.1746402,
        line_code: "WLK",
    ),
    "HIK": (
        code: "HIK",
        id: 0,
        name: "Hin Keng",
        latitude: 22.3638205,
        longitude: 114.1707186,
        line_code: "TML",
    ),
    "DIH": (
        code: "DIH",
        id: 0,
        name: "Diamond Hill",
        latitude: 22.3396397,
        longitude: 114.2012733,
        line_code: "TML",
    ),
    "WTS": (
        code: "WTS",
        id: 0,
        name: "Wong Tai Sin",
        latitude: 22.3417091,
        longitude: 114.1943553,
        line_code: "KTL",
    ),
    "CEN": (
        code: "CEN",
        id: 0,
        name: "Central",
        latitude: 22.282,
        longitude: 114.1576,
        line_code: "WLK",
    ),
    "YUL": (
        code: "YUL",
        id: 0,
        name: "Yuen Long",
        latitude: 22.4459701,
        longitude: 114.0352445,
        line_code: "TML",
    ),
    "MEF": (
        code: "MEF",
        id: 0,
        name: "Mei Foo",
        latitude: 22.3388904,
        longitude: 114.136522,
        line_code: "TWL",
    ),
    "STW": (
        code: "STW",
        id: 0,
        name: "Sha Tin Wai",
        latitude: 22.3769634,
        longitude: 114.1947694,
        line_code: "TML",
    ),
    "HAH": (
        code: "HAH",
        id: 0,
        name: "Hang Hau",
        latitude: 22.3155885,
        longitude: 114.2644721,
        line_code: "TKL",
    ),
    "KOB": (
        code: "KOB",
        id: 0,
        name: "Kowloon Bay",
        latitude: 22.3235575,
        longitude: 114.2141658,
        line_code: "KTL",
    ),
    "LOW": (
        code: "LOW",
        id: 0,
        name: "Lo Wu",
        latitude: 22.5281624,
        longitude: 114.113146,
        line_code: "EAL",
    ),
    "TUC": (
        code: "TUC",
        id: 0,
        name: "Tung Chung",
        latitude: 22.289278,
        longitude: 113.9413221,
        line_code: "TCL",
    ),
    "TWO": (
        code: "TWO",
        id: 0,
        name: "Tai Wo",
        latitude: 22.4509658,
        longitude: 114.1612012,
        line_code: "EAL",
    ),
    "YMT": (
        code: "YMT",
        id: 0,
        name: "Yau Ma Tei",
        latitude: 22.3129,
        longitude: 114.1707,
        line_code: "TWL",
    ),
    "QUB": (
        code: "QUB",
        id: 0,
        name: "Quarry Bay",
        latitude: 22.2882652,
        longitude: 114.2090338,
        line_code: "TKL",
    ),
    "LAT": (
        code: "LAT",
        id: 0,
        name: "Lam Tin",
        latitude: 22.3066758,
        longitude: 114.2328977,
        line_code: "KTL",
    ),
    "SIH": (
        code: "SIH",
        id: 0,
        name: "Siu Hong",
        latitude: 22.4119019,
        longitude: 113.9790545,
        line_code: "TML",
    ),
    "ADM": (
        code: "ADM",
        id: 0,
        name: "Admiralty",
        latitude: 22.2788,
        longitude: 114.1646,
        line_code: "SIL",
    ),
    "SUN": (
        code: "SUN",
        id: 0,
        name: "Sunny Bay",
        latitude: 22.3318274,
        longitude: 114.0288778,
        line_code: "TCL",
    ),
    "SHS": (
        code: "SHS",
        id: 0,
        name: "Sheung Shui",
        latitude: 22.501124,
        longitude: 114.1280498,
        line_code: "EAL",
    ),
    "TAP": (
        code: "TAP",
        id: 0,
        name: "Tai Po Market",
        latitude: 22.4445147,
        longitude: 114.1707659,
        line_code: "EAL",
    ),
    "TKW": (
        code: "TKW",
        id: 0,
        name: "To Kwa Wan",
        latitude: 22.3169869,
        longitude: 114.1875844,
        line_code: "TML",
    ),
    "HOM": (
        code: "HOM",
        id: 0,
        name: "Ho Man Tin",
        latitude: 22.3093727,
        longitude: 22.3093727,
        line_code: "TML",
    ),
    "SHW": (
        code: "SHW",
        id: 0,
        name: "Sheung Wan",
        latitude: 22.2865033,
        longitude: 114.1521488,
        line_code: "ISL",
    ),
    "HFC": (
        code: "HFC",
        id: 0,
        name: "Heng Fa Chuen",
        latitude: 22.276619,
        longitude: 114.2397239,
        line_code: "ISL",
    ),
    "TSH": (
        code: "TSH",
        id: 0,
        name: "Tai Shui Hang",
        latitude: 22.4084544,
        longitude: 114.222844,
        line_code: "TML",
    ),
    "TKO": (
        code: "TKO",
        id: 0,
        name: "Lohas Park",
        latitude: 22.2956555,
        longitude: 114.2687716,
        line_code: "TKL",
    ),
    "TAW": (
        code: "TAW",
        id: 0,
        name: "Tai Wai",
        latitude: 22.3728022,
        longitude: 114.1787838,
        line_code: "TML",
    ),
    "HKU": (
        code: "HKU",
        id: 0,
        name: "HKU",
        latitude: 22.2839642,
        longitude: 114.1354563,
        line_code: "ISL",
    ),
    "CSW": (
        code: "CSW",
        id: 0,
        name: "Cheung Sha Wan",
        latitude: 22.3354,
        longitude: 114.1563,
        line_code: "TWL",
    ),
    "OCP": (
        code: "OCP",
        id: 0,
        name: "Ocean Park",
        latitude: 22.2486,
        longitude: 114.1743,
        line_code: "SIL",
    ),
    "MOK": (
        code: "MOK",
        id: 0,
        name: "Mong Kok",
        latitude: 22.3191,
        longitude: 114.1694,
        line_code: "TWL",
    ),
    "YAT": (
        code: "YAT",
        id: 0,
        name: "Yau Tong",
        latitude: 22.2977718,
        longitude: 114.2370927,
        line_code: "TKL",
    ),
    "CHH": (
        code: "CHH",
        id: 0,
        name: "Choi Hung",
        latitude: 22.3354008,
        longitude: 114.2086311,
        line_code: "KTL",
    ),
    "CKT": (
        code: "CKT",
        id: 0,
        name: "Che Kung Temple",
        latitude: 22.3747809,
        longitude: 114.1860976,
        line_code: "TML",
    ),
    "TAK": (
        code: "TAK",
        id: 0,
        name: "Tai Koo",
        latitude: 22.2844656,
        longitude: 114.2163547,
        line_code: "ISL",
    ),
    "TIS": (
        code: "TIS",
        id: 0,
        name: "Tin Shui Wai",
        latitude: 22.4480421,
        longitude: 114.004696,
        line_code: "TML",
    ),
    "TIH": (
        code: "TIH",
        id: 0,
        name: "Tin Hau",
        latitude: 22.2821959,
        longitude: 114.1920333,
        line_code: "ISL",
    ),
    "MKK": (
        code: "MKK",
        id: 0,
        name: "Mong Kok East",
        latitude: 22.3217126,
        longitude: 22.3217126,
        line_code: "EAL",
    ),
    "SKW": (
        code: "SKW",
        id: 0,
        name: "Shau Kei Wan",
        latitude: 22.2790272,
        longitude: 114.2286373,
        line_code: "ISL",
    ),
    "KOW": (
        code: "KOW",
        id: 0,
        name: "Kowloon",
        latitude: 22.3048965,
        longitude: 114.1616963,
        line_code: "TCL",
    ),
    "UNI": (
        code: "UNI",
        id: 0,
        name: "University",
        latitude: 22.4133852,
        longitude: 114.2100545,
        line_code: "EAL",
    ),
    "KET": (
        code: "KET",
        id: 0,
        name: "Kennedy Town",
        latitude: 22.2812239,
        longitude: 114.128455,
        line_code: "ISL",
    ),
    "NTK": (
        code: "NTK",
        id: 0,
        name: "Ngau Tau Kok",
        latitude: 22.3154451,
        longitude: 114.2192556,
        line_code: "KTL",
    ),
    "LAK": (
        code: "LAK",
        id: 0,
        name: "Lai King",
        latitude: 22.3484,
        longitude: 114.1261,
        line_code: "TWL",
    ),
    "OLY": (
        code: "OLY",
        id: 0,
        name: "Olympic",
        latitude: 22.317703,
        longitude: 114.1604114,
        line_code: "TCL",
    ),
    "POA": (
        code: "POA",
        id: 0,
        name: "Po Lam",
        latitude: 22.3225369,
        longitude: 114.2579671,
        line_code: "TKL",
    ),
    "TSW": (
        code: "TSW",
        id: 0,
        name: "Tsuen Wan",
        latitude: 22.3683642,
        longitude: 114.1099384,
        line_code: "TWL",
    ),
    "TSY": (
        code: "TSY",
        id: 0,
        name: "Tsing Yi",
        latitude: 22.3583861,
        longitude: 114.1074312,
        line_code: "TCL",
    ),
    "FAN": (
        code: "FAN",
        id: 0,
        name: "Fanling",
        latitude: 22.4920423,
        longitude: 114.1386658,
        line_code: "EAL",
    ),
    "LCK": (
        code: "LCK",
        id: 0,
        name: "Lai Chi Kok",
        latitude: 22.3373,
        longitude: 114.1482,
        line_code: "TWL",
    ),
    "LOF": (
        code: "LOF",
        id: 0,
        name: "Lok Fu",
        latitude: 22.3380634,
        longitude: 114.1869748,
        line_code: "KTL",
    ),
    "HUH": (
        code: "HUH",
        id: 0,
        name: "Hung Hom",
        latitude: 22.3027415,
        longitude: 114.182136,
        line_code: "TML",
    ),
    "SSP": (
        code: "SSP",
        id: 0,
        name: "Sham Shui Po",
        latitude: 22.3307,
        longitude: 114.1623,
        line_code: "TWL",
    ),
    "JOR": (
        code: "JOR",
        id: 0,
        name: "Jordan",
        latitude: 22.3049,
        longitude: 114.1718,
        line_code: "TWL",
    ),
    "TST": (
        code: "TST",
        id: 0,
        name: "Tsim Sha Tsui",
        latitude: 22.2973,
        longitude: 114.1722,
        line_code: "WLK",
    ),
    "WAC": (
        code: "WAC",
        id: 0,
        name: "Wan Chai",
        latitude: 22.2775557,
        longitude: 114.1729643,
        line_code: "ISL",
    ),
    "CAB": (
        code: "CAB",
        id: 0,
        name: "Causeway Bay",
        latitude: 22.2801023,
        longitude: 114.1840143,
        line_code: "ISL",
    ),
    "KWH": (
        code: "KWH",
        id: 0,
        name: "Kwai Hing",
        latitude: 22.3632,
        longitude: 114.1312,
        line_code: "TWL",
    ),
    "FOT": (
        code: "FOT",
        id: 0,
        name: "Fo Tan",
        latitude: 22.3953371,
        longitude: 114.198366,
        line_code: "EAL",
    ),
    "KWF": (
        code: "KWF",
        id: 0,
        name: "Kwai Fong",
        latitude: 22.3569,
        longitude: 114.1279,
        line_code: "TWL",
    ),
    "TWH": (
        code: "TWH",
        id: 0,
        name: "Tai Wo Hau",
        latitude: 22.3708,
        longitude: 114.125,
        line_code: "TWL",
    ),
    "SWH": (
        code: "SWH",
        id: 0,
        name: "Sai Wan Ho",
        latitude: 22.2816237,
        longitude: 114.2222818,
        line_code: "ISL",
    ),
    "DIS": (
        code: "DIS",
        id: 0,
        name: "Disneyland Resort",
        latitude: 22.3155384,
        longitude: 114.0451419,
        line_code: "DRL",
    ),
    "KOT": (
        code: "KOT",
        id: 0,
        name: "Kowloon Tong",
        latitude: 22.336867,
        longitude: 114.177541,
        line_code: "KTL",
    ),
    "KSR": (
        code: "KSR",
        id: 0,
        name: "Kam Sheung Road",
        latitude: 22.4347947,
        longitude: 114.0633108,
        line_code: "TML",
    ),
    "CHW": (
        code: "CHW",
        id: 0,
        name: "Chai Wan",
        latitude: 22.2646033,
        longitude: 114.2370517,
        line_code: "ISL",
    ),
    "FOH": (
        code: "FOH",
        id: 0,
        name: "Fortress Hill",
        latitude: 22.2875389,
        longitude: 114.1933631,
        line_code: "ISL",
    ),
    "LOP": (
        code: "LOP",
        id: 0,
        name: "Long Ping",
        latitude: 22.447583,
        longitude: 114.0254341,
        line_code: "TML",
    ),
    "HEO": (
        code: "HEO",
        id: 0,
        name: "Heng On",
        latitude: 22.4178164,
        longitude: 114.2259505,
        line_code: "TML",
    ),
    "AIR": (
        code: "AIR",
        id: 0,
        name: "Airport",
        latitude: 22.3158106,
        longitude: 113.9365921,
        line_code: "AEL",
    ),
    "SUW": (
        code: "SUW",
        id: 0,
        name: "Sung Wong Toi",
        latitude: 22.32578,
        longitude: 114.1914117,
        line_code: "TML",
    ),
    "SOH": (
        code: "SOH",
        id: 0,
        name: "South Horizons",
        latitude: 22.2425,
        longitude: 114.1491,
        line_code: "SIL",
    ),
    "SKM": (
        code: "SKM",
        id: 0,
        name: "Shek Kip Mei",
        latitude: 22.3318072,
        longitude: 114.1687327,
        line_code: "KTL",
    ),
    "MOS": (
        code: "MOS",
        id: 0,
        name: "Ma On Shan",
        latitude: 22.4249134,
        longitude: 114.2317637,
        line_code: "TML",
    ),
    "HOK": (
        code: "HOK",
        id: 0,
        name: "Hong Kong",
        latitude: 22.2851469,
        longitude: 114.1584546,
        line_code: "WLK",
    ),
    "SYP": (
        code: "SYP",
        id: 0,
        name: "Sai Ying Pun",
        latitude: 22.2854233,
        longitude: 114.1427807,
        line_code: "ISL",
    ),
    "SHT": (
        code: "SHT",
        id: 0,
        name: "Sha Tin",
        latitude: 22.3826796,
        longitude: 114.1877459,
        line_code: "EAL",
    ),
    "EXC": (
        code: "EXC",
        id: 0,
        name: "Exhibition Centre",
        latitude: 22.2816654,
        longitude: 114.1757015,
        line_code: "EAL",
    ),
    "PRE": (
        code: "PRE",
        id: 0,
        name: "Prince Edward",
        latitude: 22.3245,
        longitude: 114.1683,
        line_code: "TWL",
    ),
    "SHM": (
        code: "SHM",
        id: 0,
        name: "Shek Mun",
        latitude: 22.3877892,
        longitude: 114.2085474,
        line_code: "TML",
    ),
    "KWT": (
        code: "KWT",
        id: 0,
        name: "Kwun Tong",
        latitude: 22.3122845,
        longitude: 114.2264543,
        line_code: "KTL",
    ),
    "LET": (
        code: "LET",
        id: 0,
        name: "Lei Tung",
        latitude: 22.2421,
        longitude: 114.1562,
        line_code: "SIL",
    ),
    "WCH": (
        code: "WCH",
        id: 0,
        name: "Wong Chuk Hang",
        latitude: 22.248,
        longitude: 114.1681,
        line_code: "SIL",
    ),
    "WKS": (
        code: "WKS",
        id: 0,
        name: "Wu Kai Sha",
        latitude: 22.4292202,
        longitude: 114.243854,
        line_code: "TML",
    ),
    "AWE": (
        code: "AWE",
        id: 0,
        name: "AsiaWorld-Expo",
        latitude: 22.3219972,
        longitude: 113.9422192,
        line_code: "AEL",
    ),
    "NOP": (
        code: "NOP",
        id: 0,
        name: "North Point",
        latitude: 22.2904837,
        longitude: 114.2005605,
        line_code: "TKL",
    ),
    "AUS": (
        code: "AUS",
        id: 0,
        name: "Austin",
        latitude: 22.3041946,
        longitude: 114.166541,
        line_code: "TML",
    ),
    "CIO": (
        code: "CIO",
        id: 0,
        name: "City One",
        latitude: 22.3830131,
        longitude: 114.203784,
        line_code: "TML",
    ),
    "TUM": (
        code: "TUM",
        id: 0,
        name: "Tuen Mun",
        latitude: 22.3952837,
        longitude: 113.9733214,
        line_code: "TML",
    ),
    "TWW": (
        code: "TWW",
        id: 0,
        name: "Tsuen Wan West",
        latitude: 22.3683642,
        longitude: 114.1099384,
        line_code: "TML",
    ),
    "KAT": (
        code: "KAT",
        id: 0,
        name: "Kai Tak",
        latitude: 22.3303836,
        longitude: 114.1992233,
        line_code: "TML",
    ),
}
"#;

const CONNECTIONS: &str = r#"
[
    ("AUS", "ETS", 4),
    ("LOP", "YUL", 3),
    ("HEO", "MOS", 4),
    ("TSY", "LAK", 5),
    ("ADM", "CEN", 3),
    ("JOR", "TST", 3),
    ("CSW", "SSP", 3),
    ("TIS", "LOP", 4),
    ("CAB", "WAC", 3),
    ("TWW", "MEF", 6),
    ("LOF", "KOT", 3),
    ("WCH", "OCP", 4),
    ("CEN", "SHW", 3),
    ("HAH", "TKO", 4),
    ("CEN", "HOK", 8),
    ("KAT", "DIH", 4),
    ("HOK", "TST", 0),
    ("TAW", "CKT", 3),
    ("STW", "CIO", 4),
    ("SKW", "SWH", 3),
    ("KOB", "CHH", 3),
    ("YAT", "QUB", 5),
    ("SYP", "HKU", 2),
    ("SKM", "PRE", 3),
    ("CIO", "SHM", 3),
    ("TUM", "SIH", 4),
    ("TSH", "HEO", 3),
    ("KWH", "KWF", 3),
    ("SIH", "TIS", 6),
    ("MKK", "HUH", 5),
    ("MEF", "NAC", 4),
    ("YMT", "JOR", 3),
    ("SHS", "FAN", 4),
    ("LAK", "NAC", 6),
    ("OLY", "KOW", 4),
    ("TAP", "UNI", 7),
    ("NTK", "KOB", 3),
    ("SUW", "KAT", 3),
    ("DIH", "WTS", 3),
    ("UNI", "FOT", 5),
    ("TSW", "TWH", 3),
    ("KWF", "LAK", 4),
    ("TWO", "TAP", 4),
    ("KOT", "MKK", 4),
    ("TST", "ADM", 5),
    ("PRE", "MOK", 3),
    ("HOM", "TKW", 4),
    ("TST", "ETS", 6),
    ("POA", "HAH", 3),
    ("EXC", "ADM", 3),
    ("TKO", "TIK", 3),
    ("SWH", "TAK", 3),
    ("YMT", "HOM", 4),
    ("HUH", "EXC", 5),
    ("HIK", "TAW", 4),
    ("LOW", "SHS", 6),
    ("FAN", "TWO", 6),
    ("HUH", "HOM", 4),
    ("KOW", "HOK", 5),
    ("CHW", "HFC", 3),
    ("DIH", "HIK", 6),
    ("HKU", "KET", 2),
    ("MEF", "LCK", 3),
    ("HFC", "SKW", 3),
    ("QUB", "NOP", 3),
    ("NOP", "FOH", 3),
    ("SUN", "TSY", 10),
    ("AWE", "AIR", 2),
    ("TKO", "TKO", 7),
    ("AIR", "TSY", 14),
    ("TIK", "YAT", 4),
    ("KWT", "NTK", 3),
    ("KSR", "TWW", 7),
    ("FOH", "TIH", 3),
    ("LAT", "KWT", 3),
    ("WAC", "ADM", 3),
    ("YAT", "LAT", 3),
    ("TAW", "KOT", 6),
    ("SHT", "TAW", 4),
    ("SUN", "DIS", 6),
    ("SHM", "TSH", 5),
    ("MOS", "WKS", 0),
    ("KOT", "SKM", 3),
    ("NAC", "OLY", 4),
    ("LET", "WCH", 4),
    ("SHW", "SYP", 3),
    ("TSY", "KOW", 13),
    ("HOM", "WHA", 4),
    ("LAK", "MEF", 3),
    ("TAK", "QUB", 3),
    ("CKT", "STW", 3),
    ("TWH", "KWH", 3),
    ("FOT", "SHT", 4),
    ("SSP", "PRE", 3),
    ("OCP", "ADM", 6),
    ("MOK", "YMT", 3),
    ("NAC", "AUS", 4),
    ("YUL", "KSR", 5),
    ("TKW", "SUW", 4),
    ("SOH", "LET", 4),
]
"#;
