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

// Copyright 2020 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Félix Daudré-Vignier   email: felix@totemaccounting.com
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

use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum ChangesInInventories {
    /// P50_0001_D000,
    RawMaterials,
    /// P50_0001_D001,
    WorkInProgress,
    /// P50_0001_D002,
    FinishedGoods,
}
/// Cost Of Goods Sold
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Cogs {
    ChangesInInventories(ChangesInInventories),
    /// P50_0001_D003,
    PurchasesDirectMaterialCosts,
    /// P50_0001_D004,
    DirectLabourCosts,
    /// P50_0001_D005,
    ManufacturingFactoryOverhead,
    /// P50_0001_D006,
    PurchasesReturnsAndAllowances,
    /// P50_0001_D007,
    PurchasedIscount,
    /// P50_0001_D008,
    /// P50_0001_D009,
    InventoryWriteDown,
    CryptoBurnWriteDown,
}
/// Charges In Out
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0002_ {
    /// P50_0002_D000,
    InterEntityChargeIn,
    /// P50_0002_D001,
    InterEntityChargeOut,
    /// P50_0002_D002,
    ITDistribution,
    /// P50_0002_D003,
    DistributionToCapital,
    /// P50_0002_D004,
    Allocations,
    /// P50_0002_D005,
    AllocationByCountry,
    /// P50_0002_D006,
    FacilitiesAllocation,
    /// P50_0002_D007,
    AssessResearchDevelopmentRecharges,
}
/// PropertyPlantEquipment
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum PropertyPlantEquipment {
    /// P50_0003_D001,
    /// B12_1001_D000,
    /// B12_1002_C000,
    Land,
    /// P50_0003_D002,
    /// B12_1001_D001,
    /// B12_1002_C001,
    Buildings,
    /// P50_0003_D003,
    /// B12_1001_D002,
    /// B12_1002_C002,
    FurnitureFixturesFittings,
    /// P50_0003_D004,
    /// B12_1001_D003,
    /// B12_1002_C003,
    PlantAndEquipment,
    /// P50_0003_D005,
    /// B12_1001_D004,
    /// B12_1002_C004,
    MotorVehicles,
    /// P50_0003_D006,
    /// B12_1001_D005,
    /// B12_1002_C005,
    Supplies,
    /// P50_0003_D007,
    /// B12_1001_D006,
    /// B12_1002_C006,
    ComputerAndITEquipment,
    /// P50_0003_D008,
    /// B12_1001_D007,
    /// B12_1002_C007,
    RightOfUseAssets,
    /// P50_0003_D009,
    /// B12_1001_D009,
    /// B12_1002_C009,
    LeaseholdImprovements,
    /// B12_1001_D008,
    /// B12_1002_C008,
    AssetsUnderConstruction,
}
/// Primary Intangibles
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum IntangibleAssetList {
    /// P50_0003_D011,
    /// B13_2002_D000,
    /// B13_2006_C000,
    TrademarkAndPatents,
    /// P50_0003_D012,
    /// B13_2003_D000,
    /// B13_2006_C001,
    ComputerSoftwareLicences,
    /// P50_0003_D010,
    /// B13_2001_D000,
    /// B13_2006_C002,
    OtherIntangibleAssets,
}
/// Depreciation Depletion Amortization
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0003_ {
    /// P50_0003_D000,
    GeneralProjectCosts,
    /// P50_0003_D013,
    GovernmentGrants,
    /// P50_0003_D015,
    NonCapitalProjectExpense,
    Depreciation(PropertyPlantEquipment),
    Amortization(IntangibleAssetList),
    /// P50_0003_D014,
    GainLossMiscellaneousSales,
}
/// Field Trials
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0004_ {
    /// P50_0004_D000,
    ConsumptionOther,
    /// P50_0004_D001,
    Trials,
    /// P50_0004_D002,
    Samples,
}
/// Warranties
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0005_ {
    /// P50_0005_D000,
    Warranties,
}
/// Tax Corporation Tax
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0006_ {
    /// P50_0006_D000,
    CorporationTax,
}
/// Tax Fines And Penalties
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0007_ {
    /// P50_0007_D000,
    EmployeeTax,
    /// P50_0007_D001,
    PropertyTax,
    /// P50_0007_D002,
    TradeLicencesTax,
    /// P50_0007_D003,
    EnviromentalTax,
    /// P50_0007_D004,
    RoadTax,
    /// P50_0007_D005,
    TaxesOther,
    /// P50_0007_D006,
    TaxesOtherAlternateAcct,
    /// P50_0007_D007,
    NonDeductibleFinesPenalties,
    /// P50_0007_D008,
    Penalties,
    /// P50_0007_D009,
    LicensesFeesCertificatesOtherTaxes,
    /// P50_0007_D010,
    CarTax,
    /// P50_0007_D011,
    CapitalGainsTaxonCryptoAssets,
}
/// Claims
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0008_ {
    /// P50_0008_D000,
    Claims,
}
/// Commissions
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0009_ {
    /// P50_0009_D000,
    Manual,
    /// P50_0009_D001,
    IntercompanyExpense,
    /// P50_0009_D002,
    IntercompanyIncome,
    /// P50_0009_D003,
    NetwrkValidationReward,
}
/// Marketing Programs
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0010_ {
    /// P50_0010_D000,
    SamplesConsumptionMarketing,
    /// P50_0010_D001,
    Promos,
    /// P50_0010_D002,
    AdvertisingExpenses,
    /// P50_0010_D003,
    AdvertisingPromosPlantVisit,
    /// P50_0010_D004,
    DesignAndArtwork,
    /// P50_0010_D005,
    AdPromosGifts,
    /// P50_0010_D006,
    AdPromosMeetingsSeminars,
    /// P50_0010_D007,
    AdPromosPrintedMaterial,
    /// P50_0010_D008,
    AdPromosOutdoor,
    /// P50_0010_D009,
    AdPromosPresentationMaterial,
    /// P50_0010_D010,
    DistributorEvents,
    /// P50_0010_D011,
    MarketResearch,
    /// P50_0010_D012,
    FairsAndExhibitions,
    /// P50_0010_D013,
    MediaPrint,
    /// P50_0010_D014,
    MediaRadio,
    /// P50_0010_D015,
    MediaTelevision,
    /// P50_0010_D016,
    MediaOther,
    /// P50_0010_D017,
    Samples,
    /// P50_0010_D018,
    CustomGiftsTaxable,
    /// P50_0010_D019,
    CustomGiftsNonTaxable,
    /// P50_0010_D020,
    TechnicalServiceFees,
    /// P50_0010_D021,
    Literature,
    /// P50_0010_D022,
    BrandMerchandise,
    /// P50_0010_D023,
    TradeShowsExhibitFees,
    /// P50_0010_D024,
    AgencyFees,
    /// P50_0010_D025,
    AudioVisuals,
    /// P50_0010_D026,
    MarketingEventsDaysInTheField,
    /// P50_0010_D027,
    MarketingMiscellaneousExpense,
    /// P50_0010_D028,
    MarketingCostSharing,
}
/// Consulting Fees
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0011_ {
    /// P50_0011_D000,
    ConsultingFees,
}
/// Services
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0012_ {
    /// P50_0012_D000,
    MiscellaneousService,
    /// P50_0012_D001,
    AnalyticalTesting,
    /// P50_0012_D002,
    CommunicationsTarifs,
    /// P50_0012_D003,
    ExperimentalTesting,
    /// P50_0012_D004,
    FinServices,
    /// P50_0012_D005,
    FreightExpenses,
    /// P50_0012_D006,
    HousekeepingLaundry,
    /// P50_0012_D007,
    LanguageTranslationServices,
    /// P50_0012_D008,
    MailDeliveryServices,
    /// P50_0012_D009,
    MiscellaneousFees,
    /// P50_0012_D010,
    Contractors,
    /// P50_0012_D011,
    TechnicalAssistance,
    /// P50_0012_D012,
    LaboratoryExpense,
    /// P50_0012_D013,
    Labour,
    /// P50_0012_D014,
    Printing,
    /// P50_0012_D015,
    Warehouse,
    /// P50_0012_D016,
    PhotographsGraphics,
    /// P50_0012_D017,
    ProjectEngineering,
    /// P50_0012_D018,
    ResearchAssistance,
    /// P50_0012_D019,
    ResearchShoppingExpense,
    /// P50_0012_D020,
    OtherServiceForProduction,
    /// P50_0012_D021,
    SoftwareMaintenanceFees,
    /// P50_0012_D022,
    TaxPreparationFees,
    /// P50_0012_D023,
    TestingServices,
}
/// Travel Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0013_ {
    /// P50_0013_D000,
    PerDiem,
    /// P50_0013_D001,
    TravelGeneral,
    /// P50_0013_D002,
    AirTickets,
    /// P50_0013_D003,
    AirTicketsOutsideCountryOfResidence,
    /// P50_0013_D004,
    Miles,
    /// P50_0013_D005,
    TaxiOtherPersonalTransport,
}
/// Hotels
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0014_ {
    /// P50_0014_D000,
    LodgingHotelCosts,
}
/// Meetings Conferences
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0015_ {
    /// P50_0015_D000,
    MeetingsConferences,
    /// P50_0015_D001,
    OtherMeetings,
    /// P50_0015_D002,
    KeyDistributorMeetings,
    /// P50_0015_D003,
    TravelForeign,
    /// P50_0015_D004,
    TravelNonEmployee,
    /// P50_0015_D005,
    TravelTechMeetingsConferences,
}
/// Restaurant Meals
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0016_ {
    /// P50_0016_D000,
    Entertainment,
    /// P50_0016_D001,
    WithinCountryOfResidence,
    /// P50_0016_D002,
    OutsideCountryOfResidence,
    /// P50_0016_D003,
    EntertainmentNonDeductable,
    /// P50_0016_D004,
    EntertainmentFieldSales,
    /// P50_0016_D005,
    EntertainmentTrainingEducation,
    /// P50_0016_D006,
    EntertainmentBusinessDevelopmentNoMeeting,
    /// P50_0016_D007,
    EntertainmentBusinessDevelopment,
}
/// Other Travel Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0017_ {
    /// P50_0017_D000,
    OtherTravelExpenses,
}
/// Cost Pooling
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0018_ {
    /// P50_0018_D000,
    PublicRelations,
}
/// Car Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0019_ {
    /// P50_0019_D000,
    Other,
    /// P50_0019_D001,
    Parking,
    /// P50_0019_D002,
    CompanyCar,
    /// P50_0019_D003,
    Telephone,
    /// P50_0019_D004,
    Rental,
    /// P50_0019_D005,
    Repairs,
    /// P50_0019_D006,
    CarFinancing,
    /// P50_0019_D007,
    Fuel,
    /// P50_0019_D008,
    LeasedAutos,
}
/// Equipment
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0020_ {
    /// P50_0020_D000,
    Equipment,
}
/// Plant Maintenance
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0021_ {
    /// P50_0021_D000,
    PlantMaintenance,
}
/// Phones Telecom
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0022_ {
    /// P50_0022_D000,
    CellularUsage,
    /// P50_0022_D001,
    CommunicationRemoteAccess,
    /// P50_0022_D002,
    MiscellaneousDataCommunications,
    /// P50_0022_D003,
    TelephoneFacsimiles,
}
/// Rental Leases
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0023_ {
    /// P50_0023_D000,
    Warehouse,
    /// P50_0023_D001,
    NonLeaseRental,
    /// P50_0023_D002,
    BuildingOffice,
    /// P50_0023_D003,
    Land,
    /// P50_0023_D004,
    Equipment,
    /// P50_0023_D005,
    SubLeaseIncome,
    /// P50_0023_D006,
    OtherFacilities,
}
/// Repair Maintenance
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0024_ {
    /// P50_0024_D000,
    OfficeLabFurniture,
    /// P50_0024_D001,
    ServiceContracts,
    /// P50_0024_D002,
    Repairs,
    /// P50_0024_D003,
    RepairsEquipment,
    /// P50_0024_D004,
    ITMaintenance,
    /// P50_0024_D005,
    Maintenance,
    /// P50_0024_D006,
    MaintenanceEquip,
    /// P50_0024_D007,
    MaintOther,
    /// P50_0024_D008,
    MaintenanceOtherMiscellaneousHardware,
    /// P50_0024_D009,
    MaintenanceOtherOfficeCleaning,
    /// P50_0024_D010,
    MaintenanceBuilding,
    /// P50_0024_D011,
    Tools,
    /// P50_0024_D012,
    Services,
    /// P50_0024_D013,
    PurchasedSupplies,
    /// P50_0024_D014,
    PlntMaintEquipmentNonCapital,
    /// P50_0024_D015,
    MiscellaneousMaintenance,
}
/// Supplies
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0025_ {
    /// P50_0025_D000,
    Other,
    /// P50_0025_D001,
    Office,
    /// P50_0025_D002,
    PhotcopyOrDocumentHandling,
    /// P50_0025_D003,
    Lab,
    /// P50_0025_D004,
    Safety,
    /// P50_0025_D005,
    Production,
    /// P50_0025_D006,
    Maintenance,
    /// P50_0025_D007,
    IndustrialChemicals,
    /// P50_0025_D008,
    Cleaning,
}
/// Utilities
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0026_ {
    /// P50_0026_D000,
    ElectricalEquipment,
    /// P50_0026_D001,
    ComputerRelatedEquipment,
    /// P50_0026_D002,
    SoftwareLicenses,
    /// P50_0026_D003,
    Electricity,
    /// P50_0026_D004,
    Water,
    /// P50_0026_D005,
    Gas,
    /// P50_0026_D006,
    Fuel,
    /// P50_0026_D007,
    UtilitiesCombined,
    /// P50_0026_D008,
    CloudStorage,
    /// P50_0026_D009,
    DataCenters,
}
/// Insurance
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0027_ {
    /// P50_0027_D000,
    Other,
    /// P50_0027_D001,
    ExportCredit,
    /// P50_0027_D002,
    SelfInsurance,
    /// P50_0027_D003,
    Motorvehicle,
    /// P50_0027_D004,
    Medical,
    /// P50_0027_D005,
    Life,
    /// P50_0027_D006,
    Travel,
}
/// Corporate Governance
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0028_ {
    /// P50_0028_D000,
    NonExecDirFeesOtherGovernanceCosts,
}
/// Legal Fees
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0029_ {
    /// P50_0029_D000,
    ExternalCorporateSecretarialFees,
    /// P50_0029_D001,
    Litigation,
    /// P50_0029_D002,
    OutsideCounselFees,
    /// P50_0029_D003,
    LicensesPermits,
    /// P50_0029_D004,
    LegalFeesInternational,
    /// P50_0029_D005,
    LegalFeesOther,
    /// P50_0029_D006,
    FCPAGifts,
    /// P50_0029_D007,
    FCPATravel,
    /// P50_0029_D008,
    FCPAMealsEntertainment,
    /// P50_0029_D009,
    BusinessContributions,
    /// P50_0029_D010,
    CharitableContributions,
    /// P50_0029_D011,
    DonationsDisallowed,
    /// P50_0029_D012,
    LocalTax,
    /// P50_0029_D013,
    AuditFeesExternal,
    /// P50_0029_D014,
    AuditorFirmTaxFees,
    /// P50_0029_D015,
    AuditorFirmOtherFees,
    /// P50_0029_D016,
    AuditFees,
    /// P50_0029_D017,
    Patents,
    /// P50_0029_D018,
    ProductRegistrations,
    /// P50_0029_D019,
    TrademarkAndPatents,
    /// P50_0029_D020,
    DesignRights,
}
/// Non Capitalised Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum NonCapitalised {
    /// P50_0030_D015,
    Equipment,
    /// P50_0030_D016,
    Software,
}
/// Recovered Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum RecoveredExpenses {
    /// P50_0030_D018,
    CanteenReceipts,
    /// P50_0030_D019,
    MiscellaneousChargeOut,
    /// P50_0030_D020,
    Insurance,
    /// P50_0030_D021,
    CarContract,
    /// P50_0030_D022,
    Other,
    /// P50_0030_D023,
    Salaries,
    /// P50_0030_D024,
    Wages,
}
/// Administration Cost
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0030_ {
    /// P50_0030_D000,
    NetworkTransactionFees,
    /// P50_0030_D001,
    BankCharges,
    /// P50_0030_D002,
    AdministrationBusinessTax,
    /// P50_0030_D003,
    CorporateMembership,
    /// P50_0030_D004,
    MembershipsSubscriptions,
    /// P50_0030_D005,
    OtherNonDeductible,
    /// P50_0030_D006,
    Postage,
    /// P50_0030_D007,
    Courrier,
    /// P50_0030_D008,
    ManagementAdjustment,
    /// P50_0030_D009,
    CollectionExpense,
    /// P50_0030_D010,
    CreditReports,
    /// P50_0030_D011,
    OtherMiscellaneousExpense,
    /// P50_0030_D012,
    MiscellaneousMonthEndAccruals,
    /// P50_0030_D013,
    Sponsorship,
    /// P50_0030_D014,
    BooksLibrary,
    NonCapitalised(NonCapitalised),
    /// P50_0030_D017,
    MiscellaneousReimbursement,
    RecoveredExpenses(RecoveredExpenses),
    /// P50_0030_D025,
    ProcurementCardPurchases,
    /// P50_0030_D026,
    AdministrationServiceFees,
    /// P50_0030_D027,
    OtherMiscellaneousIncomeExpense,
    //// Bad Debts
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0031_ {
    /// P50_0031_D000,
    BadDebtExpenseReversal,
}
/// Miscellaneous Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0032_ {
    /// P50_0032_D000,
    ShippingCost,
    /// P50_0032_D001,
    OtherCosts,
    /// P50_0032_D002,
    StandardCostIntercompany,
    /// P50_0032_D003,
    FinishedGoodsConsumption,
    /// P50_0032_D004,
    SamplesConsumption,
    /// P50_0032_D005,
    SemiFinishedConsumption,
    /// P50_0032_D006,
    FreightNonCustomer,
    /// P50_0032_D007,
    FreightFromToWarehouse,
    /// P50_0032_D008,
    DutiesPurchases,
    /// P50_0032_D009,
    HandlingWarehouse,
    /// P50_0032_D010,
    Transportation,
    /// P50_0032_D011,
    WasteDisposal,
    /// P50_0032_D013,
    CustomsFees,
    /// P50_0032_D014,
    SponsorshipDonations,
}
/// Financial Guarantee Fees
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0033_ {
    /// P50_0033_D000,
    FinancialGuaranteeFees,
}
/// Royalty Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0034_ {
    /// P50_0034_D000,
    RoyaltyExpenses,
}
/// Extraordinary Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0035_ {
    /// P50_0035_D000,
    ExtraordinaryExpenses,
}
/// Impairmentlossonfinassets
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0036_ {
    /// P50_0036_D000,
    FinancialAssets,
    /// P50_0037_D000,
    ContractAssets,
    Other,
}
/// Provisions
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _0038_ {
    /// P50_0038_D000,
    WarrantyLegalClaimsOther,
    /// P50_0038_D001,
    UnusedLeave,
}
/// Other Miscellaneous Charges
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _1001_ {
    /// P51_1001_D000,
    ITMiscellaneous,
    /// P51_1001_D001,
    SecondaryCosts,
    /// P51_1001_D002,
    ReportingAdjustment,
}
/// Social Security
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum SocialSecurity {
    /// P52_2001_D009,
    EmployerContribution,
    /// P52_2001_D012,
    Provision,
    /// P52_2001_D013,
    Payroll,
    /// P52_2001_D014,
    Accruals13thMonth,
    /// P52_2001_D015,
    AccrualsHolidayPay,
    /// P52_2001_D011,
    Other,
}
/// Salaries
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _2001_ {
    /// P52_2001_D000,
    Salaries,
    /// P52_2001_D001,
    Extra13thMonth,
    /// P52_2001_D002,
    AchievementAwards,
    /// P52_2001_D003,
    RepresentationAllowance,
    /// P52_2001_D004,
    Fees,
    /// P52_2001_D005,
    SalariedOperations,
    /// P52_2001_D006,
    HourlyWages13thMonth,
    /// P52_2001_D007,
    PayrollFees,
    /// P52_2001_D008,
    PayrollLuncheonVouchers,
    SocialSecurity(SocialSecurity),
    /// P52_2001_D010,
    FringesVariable,
    /// P52_2001_D016,
    HousingTax,
    /// P52_2001_D017,
    ApprenticeshipTax,
    /// P52_2001_D018,
    WorkCouncil,
    /// P52_2001_D019,
    SeverencePay,
    /// P52_2001_D020,
    TransportationSubsidy,
    /// P52_2001_D021,
    HoulyWages,
    /// P52_2001_D022,
    HourlyWagesPerDiem,
    /// P52_2001_D023,
    OvertimePayments,
    /// P52_2001_D024,
    SickPay,
    /// P52_2001_D025,
    HolidayPay,
    /// P52_2001_D026,
    VacationPay,
    /// P52_2001_D027,
    OtherEmployeeCompensation,
    /// P52_2001_D028,
    SavingsPlan,
    /// P52_2001_D029,
    FieldIncentivePlan,
    /// P52_2001_D030,
    Pension,
    /// P52_2001_D031,
    InsurancePlanLiveMedical,
    /// P52_2001_D032,
    InsuranceWorkAccident,
    /// P52_2001_D033,
    InsurancePrivateCollective,
    /// P52_2001_D034,
    PensionCostsCompanyPortion,
    /// P52_2001_D035,
    PrivatePatientPlan,
    /// P52_2001_D036,
    EarlyRetirementPlans,
    /// P52_2001_D037,
    EmployeeShareOptionExpense,
}
/// Expat Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _2002_ {
    /// P52_2002_D000,
    SchoolExpenses,
    /// P52_2002_D001,
    HomeLeaveExpatexpenses,
    /// P52_2002_D002,
    ExpatCost,
    /// P52_2002_D003,
    AssigneeAllowancesOther,
    /// P52_2002_D004,
    EmployeLanguageLessons,
    /// P52_2002_D005,
    EmployeeMovingExpense,
    /// P52_2002_D006,
    EmployeeAssistanceProgram,
    /// P52_2002_D007,
    EmployeeHousingExpense,
    /// P52_2002_D008,
    ExpatOtherBenefits,
    /// P52_2002_D009,
    AssigneeTax,
}
/// Incentive Plan
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _2003_ {
    /// P52_2003_D000,
    IncentivePlan,
}
/// Incentive Plan Overhead
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _2004_ {
    /// P52_2004_D000,
    IncentivePlanOverhead,
}
/// Employee Services
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _2005_ {
    /// P52_2005_D000,
    EmployeeServiceAwards,
    /// P52_2005_D001,
    SecurityServices,
    /// P52_2005_D002,
    HealthSafety,
    /// P52_2005_D003,
    Cleaning,
    /// P52_2005_D004,
    CafeteriaService,
    /// P52_2005_D005,
    MedicalBenefits,
    /// P52_2005_D006,
    MedicalServicesAndSupplies,
    /// P52_2005_D007,
    RecreationSportsSocial,
    /// P52_2005_D008,
    Recruiting,
    /// P52_2005_D009,
    EmployeeAgencyFees,
    /// P52_2005_D010,
    TransportOfPersonnel,
    /// P52_2005_D011,
    EducationTraining,
    /// P52_2005_D012,
    EducationTrainingAssociatedTravelCosts,
    /// P52_2005_D013,
    TrainingTax,
    /// P52_2005_D014,
    Tuition,
    /// P52_2005_D015,
    TuitionTaxable,
    /// P52_2005_D016,
    LaundryClothing,
    /// P52_2005_D017,
    MealAllowance,
    /// P52_2005_D018,
    Celebrations,
}
/// Interest Expense
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _3001_ {
    /// P53_3001_D000,
    BankBorrowings,
    /// P53_3001_D001,
    ConvertibleBonds,
    /// P53_3001_D002,
    DividendsOnRedeemablePreferenceShares,
    /// P53_3001_D003,
    LeaseLiabilities,
    /// P53_3001_D004,
    UnwindingOfDiscountOnProvisions,
}
/// Accrued Reveues Receivalbles
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum AccruedRevenuesReceivables {
    /// B10_0010_D000,
    /// B10_0010_D001,
    LoanTo(Parties),
    /// B10_0010_D002,
    Other,
    /// B10_0010_D003,
    FinanceLease,
    /// B10_0010_D004,
    StaffLoans,
    /// B10_0010_D005,
    GovernmentGrant,
}
/// Inventory
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Inventory {
    /// B10_0013_D000,
    RawMaterials,
    /// B10_0013_D001,
    WorkInProgress,
    /// B10_0013_D002,
    FinishedGoods,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CurrentAssets {
    /// B10_0001_D000,
    BankCurrentAccount,
    /// B10_0002_D000,
    BankSavingsAccount,
    /// B10_0003_D000,
    PettyCash,
    /// B10_0004_D000,
    InternalBalance,
    /// B10_0005_D000,
    EscrowDeposit,
    /// B10_0006_D000,
    FixedDeposits,
    /// B10_0007_D000,
    PrepaidExpensesonaccountOperatingExpense,
    /// B10_0008_D000,
    DirectorsLoanAccount,
    /// B10_0009_D000,
    /// B10_0009_D001,
    TradeReceivables(Parties),
    AccruedRevenuesReceivables(AccruedRevenuesReceivables),
    /// B10_0011_C001,
    /// B10_0012_C000,
    ImpairmentLoss(_0036_),
    /// B10_0011_C000,
    AllowanceForDoubtfulDebts,
    Inventory(Inventory),
    /// B10_0014_D000,
    AdvancestosuppliersTradeSupplies,
    /// B10_0015_D000,
    Contractassets,
    /// B10_0016_D000,
    EquitySecuritiesFVTPL,
    /// B10_0017_D000,
    AssetsOfDisposalGroupHeldForSale,
    /// B10_0018_D000,
    LoanNotesFloatingFixedAmortisedCost,
    /// B10_0019_D000,
    /// B10_0019_D001,
    /// B10_0019_D002,
    DerivativeFinInstr(Derivatives),
}
/// CryptoCurrency
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Coins {
    /// B11_4001_D000,
    /// B11_4006_C000,
    UTXOType,
    /// B11_4001_D001,
    /// B11_4006_C001,
    NonceType,
    /// B11_4001_D002,
    /// B11_4006_C002,
    ParachainType,
    /// B11_4001_D003,
    /// B11_4006_C003,
    ThirdPartyCustody,
    /// B11_4001_D004,
    /// B11_4006_C004,
    Other,
}
/// Fungibility
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Fungibility {
    Fungible,
    NonFungible,
}
/// Tokens
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Tokens {
    /// B11_4002_D000,
    /// B11_4003_D000,
    /// B11_4007_C000,
    SmartContract(Fungibility),
    /// B11_4008_C000,
    /// B11_4002_D001,
    /// B11_4003_D001,
    /// B11_4007_C001,
    ThirdParty(Fungibility),
    /// B11_4008_C001,
    /// B11_4004_D000,
    /// B11_4009_C000,
    LiquidityPoolPair,
    /// B11_4005_D000,
    /// B11_4010_C000,
    CryptoLoanCollateral,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CurrentAssetsCrypto {
    Coins(Coins),
    Tokens(Tokens),
    CoinImpairment(Coins),
    TokenImpairment(Tokens),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum FixedAssets {
    PropPlantEquip(PropertyPlantEquipment),
    AccumulatedDepreciation(PropertyPlantEquipment),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum IntagibleAssets {
        Intangibles(IntangibleAssetList),
        /// B13_2004_D000,
        ExternalNetworkCrypto,
        /// B13_2005_D000,
        Goodwill,
        /// B13_2007_C000,
        ImpairmentLoss,
        AccumulatedDepreciationIntangibles(IntangibleAssetList),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OtherReceivables {
    LoansTo(Parties),    
    /// B14_3009_D003,
    FinanceLease,
    /// B14_3009_D004,
    StaffLoans,
    /// B14_3009_D005,
    IndemnificationAssets,
    /// B14_3009_D002,
    Other,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum InvestmentIn {
    /// B14_3001_D000,
    /// B14_3007_C001,
    Subsidiaries,
    /// B14_3002_D000,
    /// B14_3007_C002,
    JointVentures,
    /// B14_3003_D000,
    /// B14_3007_C003,
    Associates,
    /// B14_3004_D000,
    Properties,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum NonCurrentAssets {
    InvestmentIn(InvestmentIn),
    /// B14_3005_D000,
    EquitySecuritiesFVOCI,
    /// B14_3005_D001,
    DebtSecuritiesFVOCI,
    /// B14_3006_D000,
    /// B14_3007_C000,
    DeferredIncomeTax,
    /// B14_3008_D000,
    NonListedDebtInstrFVTPL,
    /// B14_3008_D001,
    MandatorilyMeasuredAtFVTPL,
    /// B14_3008_D002,
    ConvertibleBondsFVTPL,
    OtherReceivables(OtherReceivables),
    FinancialInstruments(Derivatives),
    ImpairmentLossOn(InvestmentIn),
    ImpairmentOfFixedAssets,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Provisions {
    /// B20_0018_C000,
    WarrantyLegalClaimsOthers,
    /// B20_0018_C001,
    UnusedLeave,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CurrentLiabilities {
    /// B20_0001_C000,
    SalesTaxbyJurisdict,
    /// B20_0002_C000,
    FederalStateTaxByJurisdict,
    AccountsPayableTradeCreditors(Parties),
    ShortTermLoansPayable(Parties),
    CurrentPortionOfLongTermDebtNonTrade(Parties),
    /// B20_0004_C000,
    SalariesPayable,
    /// B20_0005_C000,
    WagesPayable,
    /// B20_0006_C000,
    CommissionPayable,
    /// B20_0007_C000,
    FreightPayable,
    /// B20_0008_C000,
    OtherAccruedExpensesPayable,
    /// B20_0009_C000,
    PayrollTaxByJurisdict,
    /// B20_0010_C000,
    InterestPayable,
    /// B20_0011_C000,
    AdvancesFromCustomers,
    /// B20_0012_C000,
    LawsuitsLegalCostsPayable,
    /// B20_0014_C000,
    FinGuarantees,
    /// B20_0015_C000,
    ContractLiabilities,
    /// B20_0016_C000,
    BankoverDrafts,
    /// B20_0017_C000,
    CurrentPortionOfLeaseLiabilities,
    Provisions(Provisions),
    DerivativeFinInstr(Derivatives),
    /// B20_0020_C000,
    DividendPayable,
    /// B20_0021_C000,
    CurrentPortionOfBankBorrowingsPayable,
}
/// Related Or Non Related Parties
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Parties {
    /// B14_3009_D001,
    /// B20_0003_C000,
    /// B20_0003_C001,
    /// B20_0003_C002,
    /// B21_1014_C000,
    RelatedParties,
    /// B14_3009_D000,
    /// B20_0013_C000,
    /// B20_0013_C001,
    /// B20_0013_C002,
    /// B21_1014_C001,
    NonRelatedParties,
}
/// Derivative Financial Intruments
/// Used In Current Liabilities And Non Current Liabilities
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Derivatives {
    /// B14_3010_D000,
    /// B20_0019_C000,
    /// B21_1016_C000,
    InterestRateSwaps,
    /// B14_3010_D001,
    /// B20_0019_C001,
    /// B21_1016_C001,
    CurrencyForwards,
    /// B14_3010_D002,
    /// B20_0019_C002,
    /// B21_1016_C002,
    CommodityForwards,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum NonCurrentLiabilities {
    /// B21_1001_C000,
    WitholdingIncomeTaxJurisdictPayable,
    /// B21_1002_C000,
    DeferredOrUnearnedRevenues,
    /// B21_1003_C000,
    BondsPayable,
    /// B21_1004_C000,
    NotesLoansPayable,
    /// B21_1005_C000,
    WarantyLiab,
    /// B21_1006_C000,
    CorpTaxByJurisdictPayableAtPeriodEnd,
    /// B21_1007_C000,
    InstallmentLoansPayable,
    /// B21_1008_C000,
    PortionBankBorrowingsPayable,
    /// B21_1009_C000,
    MortgageOrPropertyLoansPayable,
    /// B21_1010_C000,
    RedeemablePreferenceShares,
    /// B21_1011_C000,
    PortionLeaseLiab,
    /// B21_1012_C000,
    ContingentConsiderationPayable,
    /// B21_1013_C000,
    DeferredIncomeTax,
    NonCurrentPortionLongTermDebt(Parties),
    /// B21_1015_C000,
    ProvisionsWarrantyLegalClaimsOther,
    FinancialInstruments(Derivatives),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum ShareholdersEquity {
    /// B30_0001_C000,
    PersonalNetWorth,
    /// B30_0002_C000,
    OwnersEquity,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OtherEquity {
    /// B31_1001_C000,
    CorpTaxByJurisdictCalcAfterPL,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum CapitalStock {
    /// B32_2001_C000,
    OrdinaryShares,
    /// B32_2002_C000,
    PreferenceShares,
    /// B32_2003_D000,
    TreasuryShares,
}
/// Fair Value Gains or Losses on Financial Assets at FVOCI
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _3003_ {
    /// B33_3003_C000,
    DebtInstrFVOCIGainLoss,
    /// B33_3003_C001,
    EquityInstrFVOCIGainLoss,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OtherReserves {
    /// B33_3001_C000,
    ShareApplicationMonies,
    /// B33_3002_C000,
    ShareOptionReserve,
    /// Fair Value Gains Or Losses On Financial Assets At FVOCI
    FairValueReserve(_3003_),
    /// B33_3004_C000,
    HedgingReserveCFlowHedges,
    /// B33_3005_C000,
    EquityCompOfConvertibleBonds,
    /// B33_3006_C000,
    AssetRevaluationReserve(PropertyPlantEquipment),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum RetainedEarnings {
    /// B34_4001_D000,
    DividendPaid,
    /// B34_4002_C000,
    RetainedEarnings,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Sales {
    /// P40_0001_C000,
    SalesOfServices,
    /// P40_0002_C000,
    SalesOfGoods,
    /// P40_0003_C000,
    SalesReturnsAndAllowances,
    /// P40_0004_C000,
    SalesDiscounts,
    /// P40_0005_C000,
    FreightBillable,
    /// P40_0006_C000,
    CommissionBillable,
    /// P40_0007_C000,
    NetwrkValidationIncome,
    /// P40_0008_C000,
    NetwrkFeeIncome,
    /// P40_0009_C000,
    MiscellaneousIncome,
}
/// Other Income Interest Income
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum _1002_ {
    /// P41_1002_C000,
    FinAssetsMeasuredAmortisedCost,
    /// P41_1002_C001,
    Investments,
    /// P41_1002_C002,
    TradeReceivables,
    /// P41_1002_C003,
    BankDeposits,
    /// P41_1002_C004,
    LoansToAnassociateSubsidiary,
    /// P41_1002_C005,
    DebtInvestmentsMeasuredFVOCI,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OtherIncome {
    /// P41_1001_C000,
    RoyaltyIncome,
    /// Other Income Interest Income
    IntrestIncome(_1002_),
    /// P41_1002_C005,
    DebtInvestmentsMeasuredFVOCI,
    /// P41_1003_C000,
    DividendIncome,
    /// P41_1004_C000,
    RentalIncome,
    /// P41_1005_C000,
    GrantIncome,
    /// P41_1006_C000,
    FVGainLossFinAssetsAndLiabFVTPL,
    /// P41_1007_C000,
    FVGainLossDerivativeFinInstr,
    /// P41_1008_C000,
    IneffectivenessFVCFlowHedges,
    /// P41_1009_C000,
    FinAssetsFVOCI,
    /// P41_1010_C000,
    FVGainLossInvestmentProperties,
    /// P41_1011_C000,
    FVGainLossContingentConsideration,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OtherOperatingIncome {
    /// P42_1001_C000,
    ShareOfProfitLossOf(InvestmentIn),
    /// P42_1002_C000,
    GainLossDisposal(PropertyPlantEquipment),
    /// P42_1003_C000,
    GainLossDiscontinuedOperations,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OtherComprehensiveIncome {
    /// P43_2001_C000,
    DebtInstr,
    /// P43_2001_C001,
    EquityInstr,
    /// P43_2002_C000,
    FvGainsLossCFlowHedges,
    /// P43_2003_C000,
    ShareOfOCIAssociates,
    /// P43_2004_C000,
    ReclassificationAdjustments,
    /// P43_2005_C000,
    RevaluationGains(PropertyPlantEquipment),
    /// P43_2006_C000,
    /// P43_2006_C001,
    /// P43_2006_C002,
    /// P43_2006_C003,
    /// P43_2006_C004,
    /// P43_2007_C000,
    FVGainLossHoldingCrypto(Coins),
    /// P43_2007_C001,
    /// P43_2008_C000,
    /// P43_2009_C001,
    /// P43_2009_C000,
    /// P43_2010_C000,
    /// P43_2011_C000,
    FVGainLossHoldingTokens(Tokens),
    GainLossDisposalCrypto(Coins),
    GainLossDisposalTokens(Tokens),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OperatingExpenses {
    /// _0001_(Cogs),
    CostOfGoodsSold(Cogs),
    /// _0002_(_0002_),
    ChargesInOut(_0002_),
    /// _0003_(_0003_),
    DepreciationDepletionAmortization(_0003_),
    /// _0004_(_0004_),
    FieldTrials(_0004_),
    /// _0005_(_0005_),
    Warranties(_0005_),
    /// _0006_(_0006_),
    TaxCorporationTax(_0006_),
    /// _0007_(_0007_),
    TaxFinesPenalties(_0007_),
    /// _0008_(_0008_),
    Claims(_0008_),
    /// _0009_(_0009_),
    Commissions(_0009_),
    /// _0010_(_0010_),
    MarketingPrograms(_0010_),
    /// _0011_(_0011_),
    ConsultingFees(_0011_),
    /// _0012_(_0012_),
    Services(_0012_),
    /// _0013_(_0013_),
    TravelExpenses(_0013_),
    /// _0014_(_0014_),
    Hotels(_0014_),
    /// _0015_(_0015_),
    MeetingsConferences(_0015_),
    /// _0016_(_0016_),
    RestaurantMeals(_0016_),
    /// _0017_(_0017_),
    OtherTravelExpenses(_0017_),
    /// _0018_(_0018_),
    CostPooling(_0018_),
    /// _0019_(_0019_),
    CarExpenses(_0019_),
    /// _0020_(_0020_),
    Equipment(_0020_),
    /// _0021_(_0021_),
    PlantMaintenance(_0021_),
    /// _0022_(_0022_),
    PhonesTelecom(_0022_),
    /// _0023_(_0023_),
    RentalLeases(_0023_),
    /// _0024_(_0024_),
    RepairMaintenance(_0024_),
    /// _0025_(_0025_),
    Supplies(_0025_),
    /// _0026_(_0026_),
    Utilities(_0026_),
    /// _0027_(_0027_),
    Insurance(_0027_),
    /// _0028_(_0028_),
    CorporateGovernance(_0028_),
    /// _0029_(_0029_),
    LegalFees(_0029_),
    /// _0030_(_0030_),
    AdministrationCost(_0030_),
    /// _0031_(_0031_),
    BadDebts(_0031_),
    /// _0032_(_0032_),
    MiscellaneousExpenses(_0032_),
    /// _0033_(_0033_),
    FinGuaranteeFees(_0033_),
    /// _0034_(_0034_),
    RoyaltyExpenses(_0034_),
    /// _0035_(_0035_),
    ExtraordinaryExpenses(_0035_),
    /// _0036_(_0036_),
    ImpairmentLoss(_0036_),
    /// _0038_(_0038_),
    Provisions(_0038_),
    /// _0039_(_0039_),
    WriteOff(PropertyPlantEquipment),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum OtherOperatingExpenses {
    /// _1001_(_1001_),
    OtherMiscellaneousCharges(_1001_),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum PersonnelCosts {
    /// _2001_(_2001_),
    Salaries(_2001_),
    /// _2002_(_2002_),
    ExpatExpenses(_2002_),
    /// _2003_(_2003_),
    IncentivePlan(_2003_),
    /// _2004_(_2004_),
    IncentivePlanOverhead(_2004_),
    /// _2005_(_2005_),
    EmployeeServices(_2005_),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum FinanceCosts {
    /// _3001_(_3001_),
    InterestExpense(_3001_),
}
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum ControlAccounts {
    /// C60_0001_000D,
    PurchaseControl,
    /// C60_0002_000D,
    SalesControl,
    /// C60_0003_000D,
    TaxControl,
    /// C60_0004_000D,
    EscrowedFundsControl,
    /// C60_0005_000D,
    BorrowingsControl,
    /// C60_0006_000D,
    DefiBorrowingsControl,
    /// C60_0007_000D,
    LiquidityPoolControl,
}
/// Assets
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum A {
    CurrentAssets(CurrentAssets),
    CurrentAssetsCrypto(CurrentAssetsCrypto),
    FixedAssets(FixedAssets),
    IntagibleAssets(IntagibleAssets),
    NonCurrentAssets(NonCurrentAssets),
}
/// Liabilities
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum L {
    CurrentLiabilities(CurrentLiabilities),
    NonCurrentLiabilities(NonCurrentLiabilities),
}
/// Equity
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum E {
    ShareholdersEquity(ShareholdersEquity),
    OtherEquity(OtherEquity),
    CapitalStock(CapitalStock),
    OtherReserves(OtherReserves),
    RetainedEarnings(RetainedEarnings),
}
/// Income
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum I {
    Sales(Sales),
    OtherIncome(OtherIncome),
    OtherOperatingIncome(OtherOperatingIncome),
    OtherComprehensiveIncome(OtherComprehensiveIncome),
}
/// Expenses
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum X {
    OperatingExpenses(OperatingExpenses),
    OtherOperatingExpenses(OtherOperatingExpenses),
    PersonnelCosts(PersonnelCosts),
    FinanceCosts(FinanceCosts),
}
/// Profit And Loss
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum P {
    Income(I),
    Expenses(X),
}
/// Balance Sheet
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum B {
    Assets(A),
    Liabilities(L),
    Equity(E),
}
/// Ledger
#[allow(non_camel_case_types)]
#[derive(Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Ledger {
    BalanceSheet(B),
    ProfitLoss(P),
    ControlAccounts(ControlAccounts),
}