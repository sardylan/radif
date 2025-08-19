/*
 * radif
 * Copyright (C) 2025 - Luca Cireddu (IS0GVH) <sardylan@gmail.com>
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::data::AdifData;
use crate::error::AdifError;
use radif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum DxccEntityCode {
    #[adif("0")]
    None, // None (the contacted station is known to not be within a DXCC entity)
    #[adif("1")]
    Canada, // CANADA
    #[adif("2")]
    AbuAilIslandDeleted, // ABU AIL IS.
    #[adif("3")]
    Afghanistan, // AFGHANISTAN
    #[adif("4")]
    AgalegaAndStBrandonIsland, // AGALEGA & ST. BRANDON IS.
    #[adif("5")]
    AlandIsland, // ALAND IS.
    #[adif("6")]
    Alaska, // ALASKA
    #[adif("7")]
    Albania, // ALBANIA
    #[adif("8")]
    AldabraDeleted, // ALDABRA
    #[adif("9")]
    AmericanSamoa, // AMERICAN SAMOA
    #[adif("10")]
    AmsterdamAndStPaulIsland, // AMSTERDAM & ST. PAUL IS.
    #[adif("11")]
    AndamanAndNicobarIsland, // ANDAMAN & NICOBAR IS.
    #[adif("12")]
    Anguilla, // ANGUILLA
    #[adif("13")]
    Antarctica, // ANTARCTICA
    #[adif("14")]
    Armenia, // ARMENIA
    #[adif("15")]
    AsiaticRussia, // ASIATIC RUSSIA
    #[adif("16")]
    NewZealandSubantarcticIslands, // NEW ZEALAND SUBANTARCTIC ISLANDS
    #[adif("17")]
    AvesIsland, // AVES I.
    #[adif("18")]
    Azerbaijan, // AZERBAIJAN
    #[adif("19")]
    BajoNuevoDeleted, // BAJO NUEVO
    #[adif("20")]
    BakerAndHowlandIsland, // BAKER & HOWLAND IS.
    #[adif("21")]
    BalearicIsland, // BALEARIC IS.
    #[adif("22")]
    Palau, // PALAU
    #[adif("23")]
    BlenheimReefDeleted, // BLENHEIM REEF
    #[adif("24")]
    Bouvet, // BOUVET
    #[adif("25")]
    BritishNorthBorneoDeleted, // BRITISH NORTH BORNEO
    #[adif("26")]
    BritishSomalilandDeleted, // BRITISH SOMALILAND
    #[adif("27")]
    Belarus, // BELARUS
    #[adif("28")]
    CanalZoneDeleted, // CANAL ZONE
    #[adif("29")]
    CanaryIsland, // CANARY IS.
    #[adif("30")]
    CelebeAndMoluccaIslandDeleted, // CELEBE & MOLUCCA IS.
    #[adif("31")]
    CKiribatiBritishPhoenixIsland, // C. KIRIBATI (BRITISH PHOENIX IS.)
    #[adif("32")]
    CeutaAndMelilla, // CEUTA & MELILLA
    #[adif("33")]
    ChagosIsland, // CHAGOS IS.
    #[adif("34")]
    ChathamIsland, // CHATHAM IS.
    #[adif("35")]
    ChristmasIsland, // CHRISTMAS I.
    #[adif("36")]
    ClippertonIsland, // CLIPPERTON I.
    #[adif("37")]
    CocosIsland, // COCOS I.
    #[adif("38")]
    CocosKeelingIsland, // COCOS (KEELING) IS.
    #[adif("39")]
    ComorosDeleted, // COMOROS
    #[adif("40")]
    Crete, // CRETE
    #[adif("41")]
    CrozetIsland, // CROZET I.
    #[adif("42")]
    DamaoDiuDeleted, // DAMAO, DIU
    #[adif("43")]
    DesecheoIsland, // DESECHEO I.
    #[adif("44")]
    DesrochesDeleted, // DESROCHES
    #[adif("45")]
    Dodecanese, // DODECANESE
    #[adif("46")]
    EastMalaysia, // EAST MALAYSIA
    #[adif("47")]
    EasterIsland, // EASTER I.
    #[adif("48")]
    EKiribatiLineIsland, // E. KIRIBATI (LINE IS.)
    #[adif("49")]
    EquatorialGuinea, // EQUATORIAL GUINEA
    #[adif("50")]
    Mexico, // MEXICO
    #[adif("51")]
    Eritrea, // ERITREA
    #[adif("52")]
    Estonia, // ESTONIA
    #[adif("53")]
    Ethiopia, // ETHIOPIA
    #[adif("54")]
    EuropeanRussia, // EUROPEAN RUSSIA
    #[adif("55")]
    FarquharDeleted, // FARQUHAR
    #[adif("56")]
    FernandoDeNoronha, // FERNANDO DE NORONHA
    #[adif("57")]
    FrenchEquatorialAfricaDeleted, // FRENCH EQUATORIAL AFRICA
    #[adif("58")]
    FrenchIndoChinaDeleted, // FRENCH INDO-CHINA
    #[adif("59")]
    FrenchWestAfricaDeleted, // FRENCH WEST AFRICA
    #[adif("60")]
    Bahamas, // BAHAMAS
    #[adif("61")]
    FranzJosefLand, // FRANZ JOSEF LAND
    #[adif("62")]
    Barbados, // BARBADOS
    #[adif("63")]
    FrenchGuiana, // FRENCH GUIANA
    #[adif("64")]
    Bermuda, // BERMUDA
    #[adif("65")]
    BritishVirginIsland, // BRITISH VIRGIN IS.
    #[adif("66")]
    Belize, // BELIZE
    #[adif("67")]
    FrenchIndiaDeleted, // FRENCH INDIA
    #[adif("68")]
    KuwaitSaudiArabiaNeutralZoneDeleted, // KUWAIT/SAUDI ARABIA NEUTRAL ZONE
    #[adif("69")]
    CaymanIsland, // CAYMAN IS.
    #[adif("70")]
    Cuba, // CUBA
    #[adif("71")]
    GalapagosIsland, // GALAPAGOS IS.
    #[adif("72")]
    DominicanRepublic, // DOMINICAN REPUBLIC
    #[adif("74")]
    ElSalvador, // EL SALVADOR
    #[adif("75")]
    Georgia, // GEORGIA
    #[adif("76")]
    Guatemala, // GUATEMALA
    #[adif("77")]
    Grenada, // GRENADA
    #[adif("78")]
    Haiti, // HAITI
    #[adif("79")]
    Guadeloupe, // GUADELOUPE
    #[adif("80")]
    Honduras, // HONDURAS
    #[adif("81")]
    GermanyDeleted, // GERMANY
    #[adif("82")]
    Jamaica, // JAMAICA
    #[adif("84")]
    Martinique, // MARTINIQUE
    #[adif("85")]
    BonaireCuracaoDeleted, // BONAIRE, CURACAO
    #[adif("86")]
    Nicaragua, // NICARAGUA
    #[adif("88")]
    Panama, // PANAMA
    #[adif("89")]
    TurksAndCaicosIsland, // TURKS & CAICOS IS.
    #[adif("90")]
    TrinidadAndTobago, // TRINIDAD & TOBAGO
    #[adif("91")]
    Aruba, // ARUBA
    #[adif("93")]
    GeyserReefDeleted, // GEYSER REEF
    #[adif("94")]
    AntiguaAndBarbuda, // ANTIGUA & BARBUDA
    #[adif("95")]
    Dominica, // DOMINICA
    #[adif("96")]
    Montserrat, // MONTSERRAT
    #[adif("97")]
    StLucia, // ST. LUCIA
    #[adif("98")]
    StVincent, // ST. VINCENT
    #[adif("99")]
    GloriosoIsland, // GLORIOSO IS.
    #[adif("100")]
    Argentina, // ARGENTINA
    #[adif("101")]
    GoaDeleted, // GOA
    #[adif("102")]
    GoldCoastTogolandDeleted, // GOLD COAST, TOGOLAND
    #[adif("103")]
    Guam, // GUAM
    #[adif("104")]
    Bolivia, // BOLIVIA
    #[adif("105")]
    GuantanamoBay, // GUANTANAMO BAY
    #[adif("106")]
    Guernsey, // GUERNSEY
    #[adif("107")]
    Guinea, // GUINEA
    #[adif("108")]
    Brazil, // BRAZIL
    #[adif("109")]
    GuineaBissau, // GUINEA-BISSAU
    #[adif("110")]
    Hawaii, // HAWAII
    #[adif("111")]
    HeardIsland, // HEARD I.
    #[adif("112")]
    Chile, // CHILE
    #[adif("113")]
    IfniDeleted, // IFNI
    #[adif("114")]
    IsleOfMan, // ISLE OF MAN
    #[adif("115")]
    ItalianSomalilandDeleted, // ITALIAN SOMALILAND
    #[adif("116")]
    Colombia, // COLOMBIA
    #[adif("117")]
    ItuHq, // ITU HQ
    #[adif("118")]
    JanMayen, // JAN MAYEN
    #[adif("119")]
    JavaDeleted, // JAVA
    #[adif("120")]
    Ecuador, // ECUADOR
    #[adif("122")]
    Jersey, // JERSEY
    #[adif("123")]
    JohnstonIsland, // JOHNSTON I.
    #[adif("124")]
    JuanDeNovaEuropa, // JUAN DE NOVA, EUROPA
    #[adif("125")]
    JuanFernandezIsland, // JUAN FERNANDEZ IS.
    #[adif("126")]
    Kaliningrad, // KALININGRAD
    #[adif("127")]
    KamaranIslandDeleted, // KAMARAN IS.
    #[adif("128")]
    KareloFinnishRepublicDeleted, // KARELO-FINNISH REPUBLIC
    #[adif("129")]
    Guyana, // GUYANA
    #[adif("130")]
    Kazakhstan, // KAZAKHSTAN
    #[adif("131")]
    KerguelenIsland, // KERGUELEN IS.
    #[adif("132")]
    Paraguay, // PARAGUAY
    #[adif("133")]
    KermadecIsland, // KERMADEC IS.
    #[adif("134")]
    KingmanReefDeleted, // KINGMAN REEF
    #[adif("135")]
    Kyrgyzstan, // KYRGYZSTAN
    #[adif("136")]
    Peru, // PERU
    #[adif("137")]
    RepublicOfKorea, // REPUBLIC OF KOREA
    #[adif("138")]
    KureIsland, // KURE I.
    #[adif("139")]
    KuriaMuriaIslandDeleted, // KURIA MURIA I.
    #[adif("140")]
    Suriname, // SURINAME
    #[adif("141")]
    FalklandIsland, // FALKLAND IS.
    #[adif("142")]
    LakshadweepIsland, // LAKSHADWEEP IS.
    #[adif("143")]
    Laos, // LAOS
    #[adif("144")]
    Uruguay, // URUGUAY
    #[adif("145")]
    Latvia, // LATVIA
    #[adif("146")]
    Lithuania, // LITHUANIA
    #[adif("147")]
    LordHoweIsland, // LORD HOWE I.
    #[adif("148")]
    Venezuela, // VENEZUELA
    #[adif("149")]
    Azores, // AZORES
    #[adif("150")]
    Australia, // AUSTRALIA
    #[adif("151")]
    MalyjVysotskijIslandDeleted, // MALYJ VYSOTSKIJ I.
    #[adif("152")]
    Macao, // MACAO
    #[adif("153")]
    MacquarieIsland, // MACQUARIE I.
    #[adif("154")]
    YemenArabRepublicDeleted, // YEMEN ARAB REPUBLIC
    #[adif("155")]
    MalayaDeleted, // MALAYA
    #[adif("157")]
    Nauru, // NAURU
    #[adif("158")]
    Vanuatu, // VANUATU
    #[adif("159")]
    Maldives, // MALDIVES
    #[adif("160")]
    Tonga, // TONGA
    #[adif("161")]
    MalpeloIsland, // MALPELO I.
    #[adif("162")]
    NewCaledonia, // NEW CALEDONIA
    #[adif("163")]
    PapuaNewGuinea, // PAPUA NEW GUINEA
    #[adif("164")]
    ManchuriaDeleted, // MANCHURIA
    #[adif("165")]
    Mauritius, // MAURITIUS
    #[adif("166")]
    MarianaIsland, // MARIANA IS.
    #[adif("167")]
    MarketReef, // MARKET REEF
    #[adif("168")]
    MarshallIsland, // MARSHALL IS.
    #[adif("169")]
    Mayotte, // MAYOTTE
    #[adif("170")]
    NewZealand, // NEW ZEALAND
    #[adif("171")]
    MellishReef, // MELLISH REEF
    #[adif("172")]
    PitcairnIsland, // PITCAIRN I.
    #[adif("173")]
    Micronesia, // MICRONESIA
    #[adif("174")]
    MidwayIsland, // MIDWAY I.
    #[adif("175")]
    FrenchPolynesia, // FRENCH POLYNESIA
    #[adif("176")]
    Fiji, // FIJI
    #[adif("177")]
    MinamiTorishima, // MINAMI TORISHIMA
    #[adif("178")]
    MinervaReefDeleted, // MINERVA REEF
    #[adif("179")]
    Moldova, // MOLDOVA
    #[adif("180")]
    MountAthos, // MOUNT ATHOS
    #[adif("181")]
    Mozambique, // MOZAMBIQUE
    #[adif("182")]
    NavassaIsland, // NAVASSA I.
    #[adif("183")]
    NetherlandsBorneoDeleted, // NETHERLANDS BORNEO
    #[adif("184")]
    NetherlandsNewGuineaDeleted, // NETHERLANDS NEW GUINEA
    #[adif("185")]
    SolomonIsland, // SOLOMON IS.
    #[adif("186")]
    NewfoundlandLabradorDeleted, // NEWFOUNDLAND, LABRADOR
    #[adif("187")]
    Niger, // NIGER
    #[adif("188")]
    Niue, // NIUE
    #[adif("189")]
    NorfolkIsland, // NORFOLK I.
    #[adif("190")]
    Samoa, // SAMOA
    #[adif("191")]
    NorthCookIsland, // NORTH COOK IS.
    #[adif("192")]
    Ogasawara, // OGASAWARA
    #[adif("193")]
    OkinawaRyukyuIslandDeleted, // OKINAWA (RYUKYU IS.)
    #[adif("194")]
    OkinoToriShimaDeleted, // OKINO TORI-SHIMA
    #[adif("195")]
    AnnobonIsland, // ANNOBON I.
    #[adif("196")]
    PalestineDeleted, // PALESTINE
    #[adif("197")]
    PalmyraAndJarvisIsland, // PALMYRA & JARVIS IS.
    #[adif("198")]
    PapuaTerritoryDeleted, // PAPUA TERRITORY
    #[adif("199")]
    PeterIsland, // PETER 1 I.
    #[adif("200")]
    PortugueseTimorDeleted, // PORTUGUESE TIMOR
    #[adif("201")]
    PrinceEdwardAndMarionIsland, // PRINCE EDWARD & MARION IS.
    #[adif("202")]
    PuertoRico, // PUERTO RICO
    #[adif("203")]
    Andorra, // ANDORRA
    #[adif("204")]
    Revillagigedo, // REVILLAGIGEDO
    #[adif("205")]
    AscensionIsland, // ASCENSION I.
    #[adif("206")]
    Austria, // AUSTRIA
    #[adif("207")]
    RodriguezIsland, // RODRIGUEZ I.
    #[adif("208")]
    RuandaUrundiDeleted, // RUANDA-URUNDI
    #[adif("209")]
    Belgium, // BELGIUM
    #[adif("210")]
    SaarDeleted, // SAAR
    #[adif("211")]
    SableIsland, // SABLE I.
    #[adif("212")]
    Bulgaria, // BULGARIA
    #[adif("213")]
    SaintMartin, // SAINT MARTIN
    #[adif("214")]
    Corsica, // CORSICA
    #[adif("215")]
    Cyprus, // CYPRUS
    #[adif("216")]
    SanAndresAndProvidencia, // SAN ANDRES & PROVIDENCIA
    #[adif("217")]
    SanFelixAndSanAmbrosio, // SAN FELIX & SAN AMBROSIO
    #[adif("218")]
    CzechoslovakiaDeleted, // CZECHOSLOVAKIA
    #[adif("219")]
    SaoTomeAndPrincipe, // SAO TOME & PRINCIPE
    #[adif("220")]
    SarawakDeleted, // SARAWAK
    #[adif("221")]
    Denmark, // DENMARK
    #[adif("222")]
    FaroeIsland, // FAROE IS.
    #[adif("223")]
    England, // ENGLAND
    #[adif("224")]
    Finland, // FINLAND
    #[adif("225")]
    Sardinia, // SARDINIA
    #[adif("226")]
    SaudiArabiaIraqNeutralZoneDeleted, // SAUDI ARABIA/IRAQ NEUTRAL ZONE
    #[adif("227")]
    France, // FRANCE
    #[adif("228")]
    SerranaBankAndRoncadorCayDeleted, // SERRANA BANK & RONCADOR CAY
    #[adif("229")]
    GermanDemocraticRepublicDeleted, // GERMAN DEMOCRATIC REPUBLIC
    #[adif("230")]
    FederalRepublicOfGermany, // FEDERAL REPUBLIC OF GERMANY
    #[adif("231")]
    SikkimDeleted, // SIKKIM
    #[adif("232")]
    Somalia, // SOMALIA
    #[adif("233")]
    Gibraltar, // GIBRALTAR
    #[adif("234")]
    SouthCookIsland, // SOUTH COOK IS.
    #[adif("235")]
    SouthGeorgiaIsland, // SOUTH GEORGIA I.
    #[adif("236")]
    Greece, // GREECE
    #[adif("237")]
    Greenland, // GREENLAND
    #[adif("238")]
    SouthOrkneyIsland, // SOUTH ORKNEY IS.
    #[adif("239")]
    Hungary, // HUNGARY
    #[adif("240")]
    SouthSandwichIsland, // SOUTH SANDWICH IS.
    #[adif("241")]
    SouthShetlandIsland, // SOUTH SHETLAND IS.
    #[adif("242")]
    Iceland, // ICELAND
    #[adif("243")]
    PeopleSDemocraticRepOfYemenDeleted, // PEOPLE'S DEMOCRATIC REP. OF YEMEN
    #[adif("244")]
    SouthernSudanDeleted, // SOUTHERN SUDAN
    #[adif("245")]
    Ireland, // IRELAND
    #[adif("246")]
    SovereignMilitaryOrderOfMalta, // SOVEREIGN MILITARY ORDER OF MALTA
    #[adif("247")]
    SpratlyIsland, // SPRATLY IS.
    #[adif("248")]
    Italy, // ITALY
    #[adif("249")]
    StKittsAndNevis, // ST. KITTS & NEVIS
    #[adif("250")]
    StHelena, // ST. HELENA
    #[adif("251")]
    Liechtenstein, // LIECHTENSTEIN
    #[adif("252")]
    StPaulIsland, // ST. PAUL I.
    #[adif("253")]
    StPeterAndStPaulRocks, // ST. PETER & ST. PAUL ROCKS
    #[adif("254")]
    Luxembourg, // LUXEMBOURG
    #[adif("255")]
    StMaartenSabaStEustatiusDeleted, // ST. MAARTEN, SABA, ST. EUSTATIUS
    #[adif("256")]
    MadeiraIsland, // MADEIRA IS.
    #[adif("257")]
    Malta, // MALTA
    #[adif("258")]
    SumatraDeleted, // SUMATRA
    #[adif("259")]
    Svalbard, // SVALBARD
    #[adif("260")]
    Monaco, // MONACO
    #[adif("261")]
    SwanIslandDeleted, // SWAN IS.
    #[adif("262")]
    Tajikistan, // TAJIKISTAN
    #[adif("263")]
    Netherlands, // NETHERLANDS
    #[adif("264")]
    TangierDeleted, // TANGIER
    #[adif("265")]
    NorthernIreland, // NORTHERN IRELAND
    #[adif("266")]
    Norway, // NORWAY
    #[adif("267")]
    TerritoryOfNewGuineaDeleted, // TERRITORY OF NEW GUINEA
    #[adif("268")]
    TibetDeleted, // TIBET
    #[adif("269")]
    Poland, // POLAND
    #[adif("270")]
    TokelauIsland, // TOKELAU IS.
    #[adif("271")]
    TriesteDeleted, // TRIESTE
    #[adif("272")]
    Portugal, // PORTUGAL
    #[adif("273")]
    TrindadeAndMartimVazIsland, // TRINDADE & MARTIM VAZ IS.
    #[adif("274")]
    TristanDaCunhaAndGoughIsland, // TRISTAN DA CUNHA & GOUGH I.
    #[adif("275")]
    Romania, // ROMANIA
    #[adif("276")]
    TromelinIsland, // TROMELIN I.
    #[adif("277")]
    StPierreAndMiquelon, // ST. PIERRE & MIQUELON
    #[adif("278")]
    SanMarino, // SAN MARINO
    #[adif("279")]
    Scotland, // SCOTLAND
    #[adif("280")]
    Turkmenistan, // TURKMENISTAN
    #[adif("281")]
    Spain, // SPAIN
    #[adif("282")]
    Tuvalu, // TUVALU
    #[adif("283")]
    UkSovereignBaseAreasOnCyprus, // UK SOVEREIGN BASE AREAS ON CYPRUS
    #[adif("284")]
    Sweden, // SWEDEN
    #[adif("285")]
    VirginIsland, // VIRGIN IS.
    #[adif("286")]
    Uganda, // UGANDA
    #[adif("287")]
    Switzerland, // SWITZERLAND
    #[adif("288")]
    Ukraine, // UKRAINE
    #[adif("289")]
    UnitedNationsHq, // UNITED NATIONS HQ
    #[adif("291")]
    UnitedStatesOfAmerica, // UNITED STATES OF AMERICA
    #[adif("292")]
    Uzbekistan, // UZBEKISTAN
    #[adif("293")]
    VietNam, // VIET NAM
    #[adif("294")]
    Wales, // WALES
    #[adif("295")]
    Vatican, // VATICAN
    #[adif("296")]
    Serbia, // SERBIA
    #[adif("297")]
    WakeIsland, // WAKE I.
    #[adif("298")]
    WallisAndFutunaIsland, // WALLIS & FUTUNA IS.
    #[adif("299")]
    WestMalaysia, // WEST MALAYSIA
    #[adif("301")]
    WKiribatiGilbertIsland, // W. KIRIBATI (GILBERT IS. )
    #[adif("302")]
    WesternSahara, // WESTERN SAHARA
    #[adif("303")]
    WillisIsland, // WILLIS I.
    #[adif("304")]
    Bahrain, // BAHRAIN
    #[adif("305")]
    Bangladesh, // BANGLADESH
    #[adif("306")]
    Bhutan, // BHUTAN
    #[adif("307")]
    ZanzibarDeleted, // ZANZIBAR
    #[adif("308")]
    CostaRica, // COSTA RICA
    #[adif("309")]
    Myanmar, // MYANMAR
    #[adif("312")]
    Cambodia, // CAMBODIA
    #[adif("315")]
    SriLanka, // SRI LANKA
    #[adif("318")]
    China, // CHINA
    #[adif("321")]
    HongKong, // HONG KONG
    #[adif("324")]
    India, // INDIA
    #[adif("327")]
    Indonesia, // INDONESIA
    #[adif("330")]
    Iran, // IRAN
    #[adif("333")]
    Iraq, // IRAQ
    #[adif("336")]
    Israel, // ISRAEL
    #[adif("339")]
    Japan, // JAPAN
    #[adif("342")]
    Jordan, // JORDAN
    #[adif("344")]
    DemocraticPeopleSRepOfKorea, // DEMOCRATIC PEOPLE'S REP. OF KOREA
    #[adif("345")]
    BruneiDarussalam, // BRUNEI DARUSSALAM
    #[adif("348")]
    Kuwait, // KUWAIT
    #[adif("354")]
    Lebanon, // LEBANON
    #[adif("363")]
    Mongolia, // MONGOLIA
    #[adif("369")]
    Nepal, // NEPAL
    #[adif("370")]
    Oman, // OMAN
    #[adif("372")]
    Pakistan, // PAKISTAN
    #[adif("375")]
    Philippines, // PHILIPPINES
    #[adif("376")]
    Qatar, // QATAR
    #[adif("378")]
    SaudiArabia, // SAUDI ARABIA
    #[adif("379")]
    Seychelles, // SEYCHELLES
    #[adif("381")]
    Singapore, // SINGAPORE
    #[adif("382")]
    Djibouti, // DJIBOUTI
    #[adif("384")]
    Syria, // SYRIA
    #[adif("386")]
    Taiwan, // TAIWAN
    #[adif("387")]
    Thailand, // THAILAND
    #[adif("390")]
    Turkey, // TURKEY
    #[adif("391")]
    UnitedArabEmirates, // UNITED ARAB EMIRATES
    #[adif("400")]
    Algeria, // ALGERIA
    #[adif("401")]
    Angola, // ANGOLA
    #[adif("402")]
    Botswana, // BOTSWANA
    #[adif("404")]
    Burundi, // BURUNDI
    #[adif("406")]
    Cameroon, // CAMEROON
    #[adif("408")]
    CentralAfrica, // CENTRAL AFRICA
    #[adif("409")]
    CapeVerde, // CAPE VERDE
    #[adif("410")]
    Chad, // CHAD
    #[adif("411")]
    Comoros, // COMOROS
    #[adif("412")]
    RepublicOfTheCongo, // REPUBLIC OF THE CONGO
    #[adif("414")]
    DemocraticRepublicOfTheCongo, // DEMOCRATIC REPUBLIC OF THE CONGO
    #[adif("416")]
    Benin, // BENIN
    #[adif("420")]
    Gabon, // GABON
    #[adif("422")]
    TheGambia, // THE GAMBIA
    #[adif("424")]
    Ghana, // GHANA
    #[adif("428")]
    CoteDIvoire, // COTE D'IVOIRE
    #[adif("430")]
    Kenya, // KENYA
    #[adif("432")]
    Lesotho, // LESOTHO
    #[adif("434")]
    Liberia, // LIBERIA
    #[adif("436")]
    Libya, // LIBYA
    #[adif("438")]
    Madagascar, // MADAGASCAR
    #[adif("440")]
    Malawi, // MALAWI
    #[adif("442")]
    Mali, // MALI
    #[adif("444")]
    Mauritania, // MAURITANIA
    #[adif("446")]
    Morocco, // MOROCCO
    #[adif("450")]
    Nigeria, // NIGERIA
    #[adif("452")]
    Zimbabwe, // ZIMBABWE
    #[adif("453")]
    ReunionIsland, // REUNION I.
    #[adif("454")]
    Rwanda, // RWANDA
    #[adif("456")]
    Senegal, // SENEGAL
    #[adif("458")]
    SierraLeone, // SIERRA LEONE
    #[adif("460")]
    RotumaIsland, // ROTUMA I.
    #[adif("462")]
    SouthAfrica, // SOUTH AFRICA
    #[adif("464")]
    Namibia, // NAMIBIA
    #[adif("466")]
    Sudan, // SUDAN
    #[adif("468")]
    Swaziland, // SWAZILAND
    #[adif("470")]
    Tanzania, // TANZANIA
    #[adif("474")]
    Tunisia, // TUNISIA
    #[adif("478")]
    Egypt, // EGYPT
    #[adif("480")]
    BurkinaFaso, // BURKINA FASO
    #[adif("482")]
    Zambia, // ZAMBIA
    #[adif("483")]
    Togo, // TOGO
    #[adif("488")]
    WalvisBayDeleted, // WALVIS BAY
    #[adif("489")]
    ConwayReef, // CONWAY REEF
    #[adif("490")]
    BanabaIslandOceanIsland, // BANABA I. (OCEAN I.)
    #[adif("492")]
    Yemen, // YEMEN
    #[adif("493")]
    PenguinIslandDeleted, // PENGUIN IS.
    #[adif("497")]
    Croatia, // CROATIA
    #[adif("499")]
    Slovenia, // SLOVENIA
    #[adif("501")]
    BosniaHerzegovina, // BOSNIA-HERZEGOVINA
    #[adif("502")]
    Macedonia, // MACEDONIA
    #[adif("503")]
    CzechRepublic, // CZECH REPUBLIC
    #[adif("504")]
    SlovakRepublic, // SLOVAK REPUBLIC
    #[adif("505")]
    PratasIsland, // PRATAS I.
    #[adif("506")]
    ScarboroughReef, // SCARBOROUGH REEF
    #[adif("507")]
    TemotuProvince, // TEMOTU PROVINCE
    #[adif("508")]
    AustralIsland, // AUSTRAL I.
    #[adif("509")]
    MarquesasIsland, // MARQUESAS IS.
    #[adif("510")]
    Palestine, // PALESTINE
    #[adif("511")]
    TimorLeste, // TIMOR-LESTE
    #[adif("512")]
    ChesterfieldIsland, // CHESTERFIELD IS.
    #[adif("513")]
    DucieIsland, // DUCIE I.
    #[adif("514")]
    Montenegro, // MONTENEGRO
    #[adif("515")]
    SwainsIsland, // SWAINS I.
    #[adif("516")]
    SaintBarthelemy, // SAINT BARTHELEMY
    #[adif("517")]
    Curacao, // CURACAO
    #[adif("518")]
    StMaarten, // ST MAARTEN
    #[adif("519")]
    SabaAndStEustatius, // SABA & ST. EUSTATIUS
    #[adif("520")]
    Bonaire, // BONAIRE
    #[adif("521")]
    SouthSudanRepublicOf, // SOUTH SUDAN (REPUBLIC OF)
    #[adif("522")]
    RepublicOfKosovo, // REPUBLIC OF KOSOVO
}
