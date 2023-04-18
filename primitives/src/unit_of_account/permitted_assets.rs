//                              Næ§@@@ÑÉ©
//                        æ@@@@@@@@@@@@@@@@@@
//                    Ñ@@@@?.?@@@@@@@@@@@@@@@@@@@N
//                 ¶@@@@@?^%@@.=@@@@@@@@@@@@@@@@@@@@
//               N@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^@@@».............?@@@@@@@@@É
//              Ñ@@@@@@@@?^@@@@@@@@@@@@@@@@@@'?@@@@@@@@Ñ
//              @@@@@@@@@?^@@@»..............»@@@@@@@@@@
//              @@@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@@@
//              @@@@@@@@@?^ë@@.@@@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^´@@@o.%@@@@@@@@@@@@@@@@@@@@©
//                @@@@@@@?.´@@@@@ë.........*.±@@@@@@@æ
//                 @@@@@@@@?´.I@@@@@@@@@@@@@@.@@@@@N
//                  N@@@@@@@@@@ë.*=????????=?@@@@@Ñ
//                    @@@@@@@@@@@@@@@@@@@@@@@@@@@¶
//                        É@@@@@@@@@@@@@@@@Ñ¶
//                             Næ§@@@ÑÉ©

// Copyright 2023 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Chris D'Costa          email: chris.dcosta@totemaccounting.com

// Totem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Totem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{dispatch::TypeInfo};
/// Asset Main Groupings
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Assets {
    Stocks(Sector),
    Forex(FIAT),
    Crypto(CoinType),
}

/// Market Sector
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Sector {
    HealthCare(HECA),
    ConsumerServices(CONSERV),
    Technology(TECH),
    Industrials(INDS),
    Financials(FINS),
    ConsumerGoods(CONGD),
    BasicMaterials(BASMAT),
    Energy(NRG),
    Telecommunications(TELCOM),
    RealEstate(RELE),
    // AccountingAudit(AUDIT), 
}

/// HealthCare
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum HECA {
    Pharmaceuticals(PHARMA),
    Biotechnology(BIOTEC),
    MedicalDevices(MEDHW),
    HealthCareServices(HCSERV),
}

/// Consumer Services
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CONSERV {
    Restaurants(REST),
    Retail(RETA),
    Media(MEDIA),
    OnlineServices(OSERV),
    Travel(TVL),
}

/// Technology
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum TECH {
    Software(SOFT),
    Internet(NET),
    Semiconductors(SEMICON),
    ComputerHardware(HARD),
}

/// Industrials
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum INDS {
    AerospaceDefense(AIRDEF),
    ConstructionEngineering(CONENG),
    Machinery(MACH),
}

/// Financials
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum FINS {
    Banks(BANK),
    Insurance(INSUR),
    InvestmentBankingAndBrokerage(INVBAN),
}

/// Consumer Goods
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CONGD {
    FoodAndBeverage(FOOBEV),
    HouseholdAndPersonalCare(HSEPER),
    Automobiles(AUTOS),
}

/// Basic Materials
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum BASMAT {
    Mining(MINE),
    Chemicals(CHEMS),
}

/// Energy
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum NRG {
    OilGasExploreProduce(OILGAS),
    Utilities(UTILS),
}

/// Telecommunications
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum TELCOM {
    WirelessWirelineServices(WIRESERV),
    Broadcasting(CAST),
}

/// Real Estate
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum RELE {
    REITs(REITS),
    Homebuilders(HSECON),
    CommerceOffice(COMOFF),
}

/// Pharmaceuticals
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum PHARMA { 
    ///MDC Holdings: NYSE
    MDCO, 
    ///Mirati Therapeutics: Nasdaq
    MRTX, 
    ///Portola Pharmaceuticals: Nasdaq
    PTLA, 
    ///Inovio Pharmaceuticals: Nasdaq
    INO, 
    ///Arrowhead Pharmaceuticals: Nasdaq
    ARWR,
}

/// Biotechnology
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum BIOTEC { 
    ///Celgene Corporation: Nasdaq
    CELG, 
    ///Biogen: Nasdaq
    BIIB, 
    ///Gilead Sciences: Nasdaq
    GILD, 
    ///Bamamrn Pharmaceuticals: Nasdaq
    BMRN, 
    ///Alexion Pharmaceuticals: Nasdaq
    ALXN,
}
/// MedicalDevices
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum MEDHW { 
    ///Bard (C.R.) Inc.: NYSE
    BCR, 
    ///ABIOMED: Nasdaq
    ABMD, 
    ///Intuitive Surgical: Nasdaq
    ISRG, 
    ///Boston Scientific: NYSE
    BSX, 
    ///Align Technology: Nasdaq
    ALGN,
}
/// HealthCareServices
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum HCSERV { 
    ///Universal Health Services: NYSE
    UHS, 
    ///Tenet Healthcare Corporation: NYSE
    THC, 
    ///LHC Group: Nasdaq
    LHCG, 
    ///HCA Healthcare: NYSE
    HCA, 
    ///Community Health Systems: NYSE
    CYH,
}
/// Restaurants
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum REST { 
    ///Buffalo Wild Wings: Nasdaq
    BWLD, 
    ///Dunkin' Brands Group: Nasdaq
    DNKN, 
    ///Jack In The Box: Nasdaq
    JACK, 
    ///Chipotle Mexican Grill: NYSE
    CMG, 
    ///Papa John's International: Nasdaq
    PZZA,
}
/// Retail
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum RETA { 
    ///Macy's: NYSE
    M, 
    ///Nordstrom: NYSE
    JWN, 
    ///Gap: NYSE
    GPS, 
    ///Amazon: Nasdaq
    AMZN, 
    ///Target: NYSE
    TGT,
}
/// 
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum MEDIA { 
    ///The Walt Disney Company: NYSE
    DIS, 
    ///Fox Corporation: Nasdaq
    FOXA, 
    ///News Corp: Nasdaq
    NWS, 
    ///CBS Corporation: NYSE
    CBS, 
    ///Comcast Corporation: Nasdaq
    CMCSA,
}
/// Online Services
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OSERV { 
    ///Google: NASDAQ
    GOOGL, 
    ///eBay: NASDAQ
    EBAY,
}
/// Travel
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum TVL { 
    ///Expedia: NASDAQ
    EXPE, 
    ///TripAdvisor: NASDAQ
    TRIP, 
    ///Priceline: NASDAQ
    PCLN, 
    ///American Airlines: NASDAQ
    AAL, 
    ///United Airlines: NASDAQ
    UAL,
}
/// Software
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum SOFT { 
    ///Microsoft: NASDAQ
    MSFT, 
    ///Oracle: NASDAQ
    ORCL, 
    ///Salesforce: NYSE
    CRM, 
    ///Adobe: NASDAQ
    ADBE, 
    ///Intuit: NASDAQ
    INTU,
}
/// 
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum NET { 
    ///Facebook: NASDAQ
    FB, 
    ///Netflix: NASDAQ
    NFLX, 
    ///Twitter: NYSE
    TWTR,
} 
/// Internet
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum SEMICON { 
    ///Intel: NASDAQ
    INTC, 
    ///NVIDIA: NASDAQ
    NVDA, 
    ///Advanced Micro Devices: NASDAQ
    AMD, 
    ///Micron Technology: NASDAQ
    MU, 
    ///Qualcomm: NASDAQ
    QCOM,
}
/// Hardware
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum HARD {
    /// Hewlett Packard Enterprise : NYSE
    HPE, 
    /// Dell Technologies : NYSE
    DELL, 
    /// Apple : NASDAQ
    AAPL, 
    /// International Business Machines : NYSE
    IBM, 
    /// Cisco Systems : NASDAQ
    CSCO,
} 
/// Aerospace & Defense
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum AIRDEF { 
    /// Lockheed Martin : NYSE
    LMT, 
    /// Raytheon Technologies : NYSE
    RTN, 
    /// Northrop Grumman : NYSE
    NOC, 
    /// Huntington Ingalls Industries : NYSE
    HII, 
    /// General Dynamics : NYSE
    GD,
}
/// Construction & Engineering
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CONENG { 
    /// Fluor Corporation : NYSE
    FLR, 
    /// Weyerhaeuser : NYSE
    WY, 
    /// A.O. Smith Corporation : NYSE
    AOS, 
    /// Parker-Hannifin : NYSE
    PH, 
    /// Hillenbrand : NYSE
    HIL,
}
/// Machinery & Heavy Industry
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum MACH { 
    /// Deere & Company : NYSE
    DE, 
    /// Caterpillar : NYSE
    CAT, 
    /// Manitowoc Company : NYSE
    MTW, 
    /// Cummins : NYSE
    CMI, 
    /// Joy Global : NYSE
    JOY,
}
/// Banks & Credit Unions
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum BANK { 
    /// JPMorgan Chase & Co : NYSE
    JPM, 
    /// Bank of America : NYSE
    BAC, 
    /// Citigroup : NYSE
    C, 
    /// Wells Fargo & Co : NYSE
    WFC, 
    /// PNC Financial Services : NYSE
    PNC,
}
/// Insurance Companies
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum INSUR { 
    /// MetLife : NYSE
    MET, 
    /// American International Group : NYSE
    AIG, 
    /// Prudential Financial : NYSE
    PRU, 
    /// Allstate : NYSE
    ALL, 
    /// Unum Group : NYSE
    UNM,
}
/// Investment Banking & Brokerage
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum INVBAN { 
    /// Goldman Sachs Group : NYSE
    GS, 
    /// Morgan Stanley : NYSE
    MS, 
    /// Credit Suisse Group : NYSE
    CS, 
    /// JPMorgan Chase & Co : NYSE
    JPM, 
    /// Bank of America : NYSE
    BAC,
}
/// Food & Beverage
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum FOOBEV { 
    /// The Coca-Cola Company : NYSE
    KO, 
    /// PepsiCo : NYSE
    PEP, 
    /// Kimberly-Clark : NYSE
    KMB, 
    /// Mondelez International : NASDAQ
    MDLZ, 
    /// General Mills : NYSE
    GIS,
}
/// Household & Personal Care
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum HSEPER { 
    /// Colgate-Palmolive Company : NYSE
    CL, 
    /// Procter & Gamble : NYSE
    PG, 
    /// Kimberly-Clark : NYSE
    KMB, 
    /// Estee Lauder : NYSE
    EL, 
    /// Unilever : NYSE
    UL,
}
/// Automobiles 
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum AUTOS { 
    /// Ford Motor Company : NYSE
    F, 
    /// General Motors : NYSE
    GM, 
    /// Toyota Motor Corporation : NYSE
    TM, 
    /// Honda Motor Company : NYSE
    HMC, 
    /// Fiat Chrysler Automobiles : NYSE
    FCAU,
}
/// Mining & Metals
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum MINE { 
    /// BHP Group : NYSE
    BHP, 
    /// Rio Tinto : NYSE
    RIO, 
    /// Cleveland-Cliffs : NYSE
    CLF, 
    /// Vale : NYSE
    VALE, 
    /// Anglo American : NYSE
    AAL,
}
/// Chemicals
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CHEMS { 
    /// Dow Inc. : NYSE
    DOW, 
    /// DuPont de Nemours : NYSE
    DD, 
    /// PPG Industries : NYSE
    PPG, 
    /// Monsanto Company : NYSE
    MON, 
    /// LyondellBasell Industries : NYSE
    LYB,
}
/// Oil & Gas Exploration & Production
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OILGAS { 
    /// Exxon Mobil Corporation : NYSE
    XOM, 
    /// Chevron Corporation : NYSE
    CVX, 
    /// Royal Dutch Shell : NYSE
    RDSA, 
    /// BP : NYSE
    BP, 
    /// Total : NYSE
    TOT,
}
/// Utilties
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum UTILS { 
    /// Dominion Energy : NYSE
    D, 
    /// Southern Company : NYSE
    SO, 
    /// NextEra Energy : NYSE
    NEE, 
    /// Exelon Corporation : NYSE
    EXC, 
    /// American Electric Power : NYSE
    AEP,
} 
/// Wireless and Wireline Services
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum WIRESERV { 
    /// AT&T : NYSE
    T, 
    /// Verizon Communications : NYSE
    VZ, 
    /// Comcast Corporation : NASDAQ
    CMCSA, 
    /// DISH Network Corporation : NASDAQ
    DISH, 
    /// Charter Communications : NASDAQ
    CHTR,
}
/// Broadcasting & Cable TV
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CAST { 
    /// Discovery Communications : NASDAQ
    DISCA, 
    /// Fox Corporation : NASDAQ
    FOXA, 
    /// Comcast Corporation : NASDAQ
    CMCSA, 
    /// CBS Corporation : NYSE
    CBS, 
    /// News Corporation : NASDAQ
    NWS,
}
/// Real Estate Investment Trusts
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum REITS { 
    /// Vornado Realty Trust : NYSE
    VNO, 
    /// Equinix : NASDAQ
    EQIX, 
    /// Digital Realty Trust : NYSE
    DLR, 
    /// ProLogis : NYSE
    PLD, 
    /// Realty Income Corporation : NYSE
    O,
}
/// Housebuilding
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum HSECON { 
    /// D.R. Horton : NYSE
    DHI, 
    /// Lennar Corporation : NYSE
    LEN, 
    /// PulteGroup : NYSE
    PHM, 
    /// Toll Brothers : NYSE
    TOL, 
    /// NVR : NYSE
    NVR, 
}
/// Commercial and Office
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum COMOFF { 
    /// Boston Properties : NYSE
    BXP, 
    /// American Tower Corporation : NYSE
    AMT, 
    /// Regency Centers Corporation : NYSE
    REG, 
    /// W.P. Carey : NYSE
    WPC, 
    /// Simon Property Group : NYSE
    SPG,
}

/// Accounting & Audit
// #[allow(non_camel_case_types)]
// #[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
// #[scale_info(capture_docs = "always")]
// pub enum AUDIT { 
//     /// TODO
// }

/// Cryptocurrency Types
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CoinType {
    Coin(COIN),
    Token(TOKN),
}

/// Token Types
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum TOKN {
    /// USD Tether
    USDT,
    /// USD Circle
    USDC,
    /// Binance Token
    BNB,
}

/// Cryptocurrency
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum COIN {
    /// Acala
    ACA,
    /// Cardano
    ADA,
    /// Avalanche
    AVA,
    /// Astar
    ASTR,
    /// Cosmos
    ATOM,
    /// Aventus
    AVT,
    /// Ajuna 
    BAJU,
    /// Bitcoin Cash
    BCH,
    /// Bifrost
    BNC,
    /// Bitcoin
    BTC,
    /// Centrifuge
    CFG,
    /// Clover
    CLV,
    /// Crust
    CRU,
    /// Dogecoin
    DOGE,
    /// Polkadot
    DOT,
    /// Efinitity
    EFI,
    /// Equilibrium
    EQ,
    /// Ethereum
    ETH,
    /// Moonbeam
    GLMR,
    /// HydraDX
    HDX,
    /// Interlay
    INTR,
    /// Kilt
    KILT,
    /// Kapex
    KPX,
    /// Kusama
    KSM,
    /// Kylin
    KYL,
    /// Litentry
    LIT,
    /// Litecoin
    LTC,
    /// Nodle
    NODL,
    /// Parallel Finance
    PARA,
    /// Polkadex
    PDEX,
    /// Phala
    PHA,
    /// Darwinia
    RING,
    /// Solana
    SOL,
    /// Integritee
    TEER,
    /// OriginTrail
    TRAC,
    /// Unique
    UNQ,
    /// Stellar
    XLM,
    /// Monero
    XMR,
    /// Ripple
    XRP,
}

/// Fiat Currencies
#[allow(non_camel_case_types)]
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum FIAT {
    /// Chinese Yuan
    CNY,
    /// US Dollar
    USD,
    /// Euro
    EUR,
    /// Japanese Yen
    JPY,
    /// British Pound
    GBP,
    /// South Korean Won
    KRW,
    /// Indian Rupee
    INR,
    /// Hong Kong Dollar
    HKD,
    /// Canadian Dollar
    CAD,
    /// New Taiwan Dollar
    TWD,
    /// Australian Dollar
    AUD,
    /// Brazilian Real
    BRL,
    /// Swiss Franc
    CHF,
    /// Russian Ruble
    RUB,
    /// Thai Baht
    THB,
    /// Saudi Riyal
    SAR,
    /// United Arab Emirates Dirham
    AED,
    /// Singapore Dollar
    SGD,
    /// Malaysian Ringgit
    MYR,
    /// Mexican Peso
    MXN,
    /// Turkish Lira
    TRY,
    /// Vietnamese Dong
    VND,
    /// Swedish Krona
    SEK,
    /// Polish Zloty
    PLN,
    /// Indonesian Rupiah
    IDR,
    /// Israeli New Shekel
    ILS,
    /// Chilean Peso
    CLP,
    /// Egyptian Pound
    EGP,
    /// Philippine Peso
    PHP,
    /// Norwegian Krone
    NOK,
    /// Danish Krone
    DKK,
    /// South African Rand
    ZAR,
    /// New Zealand Dollar
    NZD,
    /// Czech Koruna
    CZK,
    /// Qatari Rial
    QAR,
    /// Colombian Peso
    COP,
    /// Pakistani Rupee
    PKR,
    /// Lebanese Pound
    LBP,
    /// Moroccan Dirham
    MAD,
    /// Kuwaiti Dinar
    KWD,
    /// Romanian Leu
    RON,
    /// Macanese Pataca
    MOP,
    /// Hungarian Forint
    HUF,
    /// Nigerian Naira
    NGN,
    /// Libyan Dinar
    LYD,
    /// Argentine Peso
    ARS,
    /// Peruvian Sol
    PEN,
    /// Bulgarian Lev
    BGN,
    /// Ukrainian Hryvnia
    UAH,
    /// Jordanian Dinar
    JOD,
    /// Kazakhstani Tenge
    KZT,
    /// Omani Rial
    OMR,
    /// Bahraini Dinar
    BHD,
    /// Sri Lankan Rupee
    LKR,
    /// Guatemalan Quetzal
    GTQ,
    /// Kenyan Shilling
    KES,
    /// West African CFA Franc
    XOF,
    /// Dominican Peso
    DOP,
    /// Bolivian Boliviano
    BOB,
    /// Serbian Dinar
    RSD,
    /// Costa Rican Col
    CRC,
    /// Angolan Kwanza
    AOA,
    /// Croatian Kuna
    HRK,
    /// Bangladeshi Taka
    BDT,
    /// Belarusian Ruble
    BYN,
    /// Azerbaijani Manat
    AZN,
    /// Honduran Lempira
    HNL,
    /// Mauritian Rupee
    MUR,
    /// Paraguayan Guarani
    PYG,
    /// Icelandic Kr
    ISK,
    /// Trinidad & Tobago Dollar
    TTD,
    /// Sudanese Pound
    SDG,
    /// Tanzanian Shilling
    TZS,
    /// Albanian Lek
    ALL,
    /// Iraqi Dinar
    IQD,
    /// Brunei Dollar
    BND,
    /// Laotian Kip
    LAK,
    /// Bahamian Dollar
    BSD,
    /// Uruguayan Peso
    UYU,
    /// Zambian Kwacha
    ZMK,
    /// Georgian Lari
    GEL,
    /// Mongolian Tugrik
    MNT,
    /// Bosnia-Herzegovina Convertible Mark
    BAM,
    /// Congolese Franc
    CDF,
    /// Ugandan Shilling
    UGX,
    /// Mozambican Metical
    MZN,
    /// Botswanan Pula
    BWP,
    /// Macedonian Denar
    MKD,
    /// Namibian Dollar
    NAD,
    /// CFP Franc
    XPF,
    /// Moldovan Leu
    MDL,
    /// Jamaican Dollar
    JMD,
    /// Nicaraguan C
    NIO,
    /// Armenian Dram
    AMD,
    /// Malagasy Ariary
    MGA,
    /// Guinean Franc
    GNF,
    /// Rwandan Franc
    RWF,
    /// Maldivian Rufiyaa
    MVR,
    /// Kyrgystani Som
    KGS,
    /// Guyanaese Dollar
    GYD,
    /// Cape Verdean Escudo
    CVE,
    /// Malawian Kwacha
    MWK,
    /// Central African CFA Franc
    XAF,
    /// Bhutanese Ngultrum
    BTN,
    /// Seychellois Rupee
    SCR,
    /// Burundian Franc
    BIF,
    /// Swazi Lilangeni
    SZL,
    /// Sierra Leonean Leone
    SLL,
    /// Gambian Dalasi
    GMD,
    /// Lesotho Loti
    LSL,
    /// Liberian Dollar
    LRD,
    /// Comorian Franc
    KMF,
    /// Tunisian Dinar
    TND,
    /// Tajikistani Somoni
    TJS,
    /// São Tomé and Príncipe Dobra 
    STD,
}