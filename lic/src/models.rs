use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use jiff::civil::Date;
use regex::Regex;
use serde::Deserialize;
use std::path::PathBuf;

/// Represents different types of comment tokens found in languages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentToken {
    /// A single-line comment token (e.g., "//", "#").
    Line(String),
    /// A block comment token pair (e.g., "/*", "*/").
    Block { start: String, end: String },
}

/// Author struct
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Authors(pub Vec<Author>);

#[derive(Debug, Clone, Deserialize, Copy, PartialEq, Eq, Hash, clap :: ValueEnum)]
#[allow(non_camel_case_types)]
pub enum License {
    #[value(name = "AFL-1.2")]
    AFLOneDotTwo,
    #[value(name = "Linux-syscall-note")]
    LinuxSyscallNote,
    #[value(name = "DocBook-Stylesheet")]
    DocBookStylesheet,
    #[value(name = "HPND-sell-MIT-disclaimer-xserver")]
    HPNDSellMITDisclaimerXserver,
    #[value(name = "OSL-2.1")]
    OSLTwoDotOne,
    #[value(name = "SWL")]
    SWL,
    #[value(name = "CC-BY-NC-4.0")]
    CCBYNCFourDotZero,
    #[value(name = "NLOD-2.0")]
    NLODTwoDotZero,
    #[value(name = "Unlicense")]
    Unlicense,
    #[value(name = "ODC-By-1.0")]
    ODCByOneDotZero,
    #[value(name = "UPL-1.0")]
    UPLOneDotZero,
    #[value(name = "SNIA")]
    SNIA,
    #[value(name = "GCC-exception-3.1")]
    GCCExceptionThreeDotOne,
    #[value(name = "OLDAP-1.3")]
    OLDAPOneDotThree,
    #[value(name = "Dotseqn")]
    Dotseqn,
    #[value(name = "TMate")]
    TMate,
    #[value(name = "GFDL-1.1-invariants-or-later")]
    GFDLOneDotOneInvariantsOrLater,
    #[value(name = "MMIXware")]
    MmiXware,
    #[value(name = "Crossword")]
    Crossword,
    #[value(name = "OLDAP-1.4")]
    OLDAPOneDotFour,
    #[value(name = "GLWTPL")]
    GLWTPL,
    #[value(name = "curl")]
    Curl,
    #[value(name = "GPL-2.0-or-later")]
    GPLTwoDotZeroOrLater,
    #[value(name = "OFL-1.1-no-RFN")]
    OFLOneDotOneNoRFN,
    #[value(name = "MS-LPL")]
    MSLPL,
    #[value(name = "CC-BY-1.0")]
    CCBYOneDotZero,
    #[value(name = "CDLA-Permissive-1.0")]
    CDLAPermissiveOneDotZero,
    #[value(name = "QPL-1.0-INRIA-2004-exception")]
    QPLOneDotZeroINRIATwoZeroZeroFourException,
    #[value(name = "Classpath-exception-2.0")]
    ClasspathExceptionTwoDotZero,
    #[value(name = "BSD-3-Clause")]
    BSDThreeClause,
    #[value(name = "OPL-1.0")]
    OPLOneDotZero,
    #[value(name = "ODbL-1.0")]
    ODbLOneDotZero,
    #[value(name = "Independent-modules-exception")]
    IndependentModulesException,
    #[value(name = "MIT-Festival")]
    MITFestival,
    #[value(name = "Caldera-no-preamble")]
    CalderaNoPreamble,
    #[value(name = "SGI-B-1.1")]
    SGIBOneDotOne,
    #[value(name = "Soundex")]
    Soundex,
    #[value(name = "SANE-exception")]
    SANEException,
    #[value(name = "MirOS")]
    MirOs,
    #[value(name = "Ruby")]
    Ruby,
    #[value(name = "polyparse-exception")]
    PolyparseException,
    #[value(name = "BSD-3-Clause-LBNL")]
    BSDThreeClauseLBNL,
    #[value(name = "BSD-2-Clause")]
    BSDTwoClause,
    #[value(name = "EFL-2.0")]
    EFLTwoDotZero,
    #[value(name = "LLGPL")]
    LLGPL,
    #[value(name = "vsftpd-openssl-exception")]
    VsftpdOpensslException,
    #[value(name = "OFL-1.0-no-RFN")]
    OFLOneDotZeroNoRFN,
    #[value(name = "CC-BY-NC-ND-3.0-DE")]
    CCBYNCNDThreeDotZeroDE,
    #[value(name = "Artistic-2.0")]
    ArtisticTwoDotZero,
    #[value(name = "CDDL-1.0")]
    CDDLOneDotZero,
    #[value(name = "SWI-exception")]
    SWIException,
    #[value(name = "GFDL-1.3-invariants-only")]
    GFDLOneDotThreeInvariantsOnly,
    #[value(name = "MS-RL")]
    MSRL,
    #[value(name = "ASWF-Digital-Assets-1.1")]
    ASWFDigitalAssetsOneDotOne,
    #[value(name = "LGPL-2.1-only")]
    LGPLTwoDotOneOnly,
    #[value(name = "HP-1986")]
    HPOneNineEightSix,
    #[value(name = "Python-2.0.1")]
    PythonTwoDotZeroDotOne,
    #[value(name = "Abstyles")]
    Abstyles,
    #[value(name = "RHeCos-1.1")]
    RHeCosOneDotOne,
    #[value(name = "MIT")]
    MIT,
    #[value(name = "BSD-1-Clause")]
    BSDOneClause,
    #[value(name = "romic-exception")]
    RomicException,
    #[value(name = "OSL-3.0")]
    OSLThreeDotZero,
    #[value(name = "CC-BY-NC-3.0")]
    CCBYNCThreeDotZero,
    #[value(name = "GCC-exception-2.0")]
    GCCExceptionTwoDotZero,
    #[value(name = "deprecated_LGPL-3.0+")]
    DeprecatedLGPLThreeDotZeroPlus,
    #[value(name = "Clips")]
    Clips,
    #[value(name = "BSD-2-Clause-Darwin")]
    BSDTwoClauseDarwin,
    #[value(name = "CNRI-Python-GPL-Compatible")]
    CNRIPythonGPLCompatible,
    #[value(name = "LPD-document")]
    LPDDocument,
    #[value(name = "GPL-CC-1.0")]
    GPLCCOneDotZero,
    #[value(name = "BSD-3-Clause-flex")]
    BSDThreeClauseFlex,
    #[value(name = "HPND-UC")]
    HPNDUC,
    #[value(name = "ISC-Veillard")]
    ISCVeillard,
    #[value(name = "PDDL-1.0")]
    PDDLOneDotZero,
    #[value(name = "LGPL-2.0-or-later")]
    LGPLTwoDotZeroOrLater,
    #[value(name = "CC-BY-NC-SA-3.0-IGO")]
    CCBYNCSAThreeDotZeroIGO,
    #[value(name = "TORQUE-1.1")]
    TORQUEOneDotOne,
    #[value(name = "Frameworx-1.0")]
    FrameworxOneDotZero,
    #[value(name = "OLDAP-2.8")]
    OLDAPTwoDotEight,
    #[value(name = "deprecated_GPL-3.0")]
    DeprecatedGPLThreeDotZero,
    #[value(name = "Qwt-exception-1.0")]
    QwtExceptionOneDotZero,
    #[value(name = "DocBook-DTD")]
    DocBookDTD,
    #[value(name = "psfrag")]
    Psfrag,
    #[value(name = "IJG-short")]
    IJGShort,
    #[value(name = "psutils")]
    Psutils,
    #[value(name = "DRL-1.1")]
    DRLOneDotOne,
    #[value(name = "Boehm-GC")]
    BoehmGC,
    #[value(name = "CECILL-2.0")]
    CECILLTwoDotZero,
    #[value(name = "W3C-19980720")]
    WThreeCOneNineNineEightZeroSevenTwoZero,
    #[value(name = "O-UDA-1.0")]
    OUDAOneDotZero,
    #[value(name = "CLISP-exception-2.0")]
    CLISPExceptionTwoDotZero,
    #[value(name = "DL-DE-ZERO-2.0")]
    DLDEZEROTwoDotZero,
    #[value(name = "deprecated_GFDL-1.3")]
    DeprecatedGFDLOneDotThree,
    #[value(name = "SGP4")]
    SgpFour,
    #[value(name = "FDK-AAC")]
    FDKAAC,
    #[value(name = "Apache-1.1")]
    ApacheOneDotOne,
    #[value(name = "TTWL")]
    TTWL,
    #[value(name = "Martin-Birgmeier")]
    MartinBirgmeier,
    #[value(name = "CC-BY-ND-3.0-DE")]
    CCBYNDThreeDotZeroDE,
    #[value(name = "ECL-2.0")]
    ECLTwoDotZero,
    #[value(name = "ICU")]
    ICU,
    #[value(name = "stunnel-exception")]
    StunnelException,
    #[value(name = "Game-Programming-Gems")]
    GameProgrammingGems,
    #[value(name = "BSD-3-Clause-Attribution")]
    BSDThreeClauseAttribution,
    #[value(name = "SGI-B-1.0")]
    SGIBOneDotZero,
    #[value(name = "NRL")]
    NRL,
    #[value(name = "Kazlib")]
    Kazlib,
    #[value(name = "PHP-3.01")]
    PHPThreeDotZeroOne,
    #[value(name = "magaz")]
    Magaz,
    #[value(name = "CPAL-1.0")]
    CPALOneDotZero,
    #[value(name = "App-s2p")]
    AppSTwop,
    #[value(name = "RRDtool-FLOSS-exception-2.0")]
    RrDtoolFLOSSExceptionTwoDotZero,
    #[value(name = "Libtool-exception")]
    LibtoolException,
    #[value(name = "PS-or-PDF-font-exception-20170817")]
    PSOrPDFFontExceptionTwoZeroOneSevenZeroEightOneSeven,
    #[value(name = "Brian-Gladman-2-Clause")]
    BrianGladmanTwoClause,
    #[value(name = "deprecated_GPL-2.0")]
    DeprecatedGPLTwoDotZero,
    #[value(name = "LGPL-3.0-only")]
    LGPLThreeDotZeroOnly,
    #[value(name = "DigiRule-FOSS-exception")]
    DigiRuleFOSSException,
    #[value(name = "GPL-3.0-linking-exception")]
    GPLThreeDotZeroLinkingException,
    #[value(name = "ASWF-Digital-Assets-1.0")]
    ASWFDigitalAssetsOneDotZero,
    #[value(name = "mailprio")]
    Mailprio,
    #[value(name = "LPPL-1.2")]
    LPPLOneDotTwo,
    #[value(name = "CDDL-1.1")]
    CDDLOneDotOne,
    #[value(name = "HPND")]
    HPND,
    #[value(name = "NTP-0")]
    NTPZero,
    #[value(name = "CC-BY-ND-2.5")]
    CCBYNDTwoDotFive,
    #[value(name = "CC-BY-NC-ND-2.5")]
    CCBYNCNDTwoDotFive,
    #[value(name = "freertos-exception-2.0")]
    FreertosExceptionTwoDotZero,
    #[value(name = "Brian-Gladman-3-Clause")]
    BrianGladmanThreeClause,
    #[value(name = "deprecated_GPL-2.0-with-bison-exception")]
    DeprecatedGPLTwoDotZeroWithBisonException,
    #[value(name = "SugarCRM-1.1.3")]
    SugarCrmOneDotOneDotThree,
    #[value(name = "SISSL")]
    SISSL,
    #[value(name = "CFITSIO")]
    CFITSIO,
    #[value(name = "TU-Berlin-2.0")]
    TUBerlinTwoDotZero,
    #[value(name = "CMU-Mach-nodoc")]
    CMUMachNodoc,
    #[value(name = "Condor-1.1")]
    CondorOneDotOne,
    #[value(name = "OSL-2.0")]
    OSLTwoDotZero,
    #[value(name = "CC-BY-NC-2.0")]
    CCBYNCTwoDotZero,
    #[value(name = "AdaCore-doc")]
    AdaCoreDoc,
    #[value(name = "TermReadKey")]
    TermReadKey,
    #[value(name = "UnixCrypt")]
    UnixCrypt,
    #[value(name = "jove")]
    Jove,
    #[value(name = "NTP")]
    NTP,
    #[value(name = "Adobe-Display-PostScript")]
    AdobeDisplayPostScript,
    #[value(name = "CC-BY-3.0-DE")]
    CCBYThreeDotZeroDE,
    #[value(name = "Kastrup")]
    Kastrup,
    #[value(name = "LGPL-2.0-only")]
    LGPLTwoDotZeroOnly,
    #[value(name = "OLDAP-1.2")]
    OLDAPOneDotTwo,
    #[value(name = "Linux-man-pages-1-para")]
    LinuxManPagesOnePara,
    #[value(name = "DL-DE-BY-2.0")]
    DLDEBYTwoDotZero,
    #[value(name = "CC-BY-NC-3.0-DE")]
    CCBYNCThreeDotZeroDE,
    #[value(name = "FreeImage")]
    FreeImage,
    #[value(name = "CECILL-2.1")]
    CECILLTwoDotOne,
    #[value(name = "DRL-1.0")]
    DRLOneDotZero,
    #[value(name = "HPND-export-US-acknowledgement")]
    HPNDExportUSAcknowledgement,
    #[value(name = "OLDAP-2.0.1")]
    OLDAPTwoDotZeroDotOne,
    #[value(name = "threeparttable")]
    Threeparttable,
    #[value(name = "MIT-open-group")]
    MITOpenGroup,
    #[value(name = "FSFULLR")]
    FSFULLR,
    #[value(name = "Autoconf-exception-macro")]
    AutoconfExceptionMacro,
    #[value(name = "NCL")]
    NCL,
    #[value(name = "EPICS")]
    EPICS,
    #[value(name = "Apache-1.0")]
    ApacheOneDotZero,
    #[value(name = "JSON")]
    JSON,
    #[value(name = "deprecated_GFDL-1.2")]
    DeprecatedGFDLOneDotTwo,
    #[value(name = "Spencer-86")]
    SpencerEightSix,
    #[value(name = "BSD-4-Clause")]
    BSDFourClause,
    #[value(name = "SHL-0.51")]
    SHLZeroDotFiveOne,
    #[value(name = "IJG")]
    IJG,
    #[value(name = "APSL-1.2")]
    APSLOneDotTwo,
    #[value(name = "deprecated_GPL-2.0+")]
    DeprecatedGPLTwoDotZeroPlus,
    #[value(name = "erlang-otp-linking-exception")]
    ErlangOtpLinkingException,
    #[value(name = "SAX-PD")]
    SAXPD,
    #[value(name = "PSF-2.0")]
    PSFTwoDotZero,
    #[value(name = "BSD-3-Clause-Modification")]
    BSDThreeClauseModification,
    #[value(name = "QPL-1.0-INRIA-2004")]
    QPLOneDotZeroINRIATwoZeroZeroFour,
    #[value(name = "deprecated_GPL-2.0-with-classpath-exception")]
    DeprecatedGPLTwoDotZeroWithClasspathException,
    #[value(name = "EUPL-1.2")]
    EUPLOneDotTwo,
    #[value(name = "BSD-2-Clause-first-lines")]
    BSDTwoClauseFirstLines,
    #[value(name = "GNOME-examples-exception")]
    GNOMEExamplesException,
    #[value(name = "X11-distribute-modifications-variant")]
    XOneOneDistributeModificationsVariant,
    #[value(name = "LPL-1.0")]
    LPLOneDotZero,
    #[value(name = "Spencer-94")]
    SpencerNineFour,
    #[value(name = "CC-BY-NC-SA-2.5")]
    CCBYNCSATwoDotFive,
    #[value(name = "u-boot-exception-2.0")]
    UBootExceptionTwoDotZero,
    #[value(name = "CC-BY-SA-2.5")]
    CCBYSATwoDotFive,
    #[value(name = "Newsletr")]
    Newsletr,
    #[value(name = "Noweb")]
    Noweb,
    #[value(name = "APL-1.0")]
    APLOneDotZero,
    #[value(name = "w3m")]
    WThreem,
    #[value(name = "FSL-1.1-MIT")]
    FSLOneDotOneMIT,
    #[value(name = "Texinfo-exception")]
    TexinfoException,
    #[value(name = "GFDL-1.2-no-invariants-or-later")]
    GFDLOneDotTwoNoInvariantsOrLater,
    #[value(name = "Artistic-1.0")]
    ArtisticOneDotZero,
    #[value(name = "EFL-1.0")]
    EFLOneDotZero,
    #[value(name = "Watcom-1.0")]
    WatcomOneDotZero,
    #[value(name = "CC-BY-3.0-AU")]
    CCBYThreeDotZeroAU,
    #[value(name = "ISC")]
    ISC,
    #[value(name = "zlib-acknowledgement")]
    ZlibAcknowledgement,
    #[value(name = "TCP-wrappers")]
    TCPWrappers,
    #[value(name = "Universal-FOSS-exception-1.0")]
    UniversalFOSSExceptionOneDotZero,
    #[value(name = "hdparm")]
    Hdparm,
    #[value(name = "SHL-2.0")]
    SHLTwoDotZero,
    #[value(name = "OLDAP-2.2.1")]
    OLDAPTwoDotTwoDotOne,
    #[value(name = "GFDL-1.3-no-invariants-only")]
    GFDLOneDotThreeNoInvariantsOnly,
    #[value(name = "eGenix")]
    EGenix,
    #[value(name = "AGPL-1.0-only")]
    AGPLOneDotZeroOnly,
    #[value(name = "FSFAP")]
    FSFAP,
    #[value(name = "Spencer-99")]
    SpencerNineNine,
    #[value(name = "OLDAP-2.3")]
    OLDAPTwoDotThree,
    #[value(name = "Gmsh-exception")]
    GmshException,
    #[value(name = "dvipdfm")]
    Dvipdfm,
    #[value(name = "deprecated_Net-SNMP")]
    DeprecatedNetSNMP,
    #[value(name = "LZMA-exception")]
    LZMAException,
    #[value(name = "Libpng")]
    Libpng,
    #[value(name = "Xnet")]
    Xnet,
    #[value(name = "HPND-merchantability-variant")]
    HPNDMerchantabilityVariant,
    #[value(name = "Intel")]
    Intel,
    #[value(name = "OLDAP-2.4")]
    OLDAPTwoDotFour,
    #[value(name = "W3C-20150513")]
    WThreeCTwoZeroOneFiveZeroFiveOneThree,
    #[value(name = "MIT-Khronos-old")]
    MITKhronosOld,
    #[value(name = "CDLA-Permissive-2.0")]
    CDLAPermissiveTwoDotZero,
    #[value(name = "CC-BY-2.0")]
    CCBYTwoDotZero,
    #[value(name = "mpi-permissive")]
    MpiPermissive,
    #[value(name = "deprecated_GPL-2.0-with-autoconf-exception")]
    DeprecatedGPLTwoDotZeroWithAutoconfException,
    #[value(name = "MIPS")]
    MIPS,
    #[value(name = "Cube")]
    Cube,
    #[value(name = "OSL-1.1")]
    OSLOneDotOne,
    #[value(name = "MPL-2.0-no-copyleft-exception")]
    MPLTwoDotZeroNoCopyleftException,
    #[value(name = "NLOD-1.0")]
    NLODOneDotZero,
    #[value(name = "GFDL-1.1-or-later")]
    GFDLOneDotOneOrLater,
    #[value(name = "Widget-Workshop")]
    WidgetWorkshop,
    #[value(name = "CATOSL-1.1")]
    CATOSLOneDotOne,
    #[value(name = "cryptsetup-OpenSSL-exception")]
    CryptsetupOpenSslException,
    #[value(name = "BSD-4.3RENO")]
    BSDFourDotThreeReno,
    #[value(name = "BSD-Systemics")]
    BSDSystemics,
    #[value(name = "Boehm-GC-without-fee")]
    BoehmGCWithoutFee,
    #[value(name = "GCC-exception-2.0-note")]
    GCCExceptionTwoDotZeroNote,
    #[value(name = "AGPL-3.0-only")]
    AGPLThreeDotZeroOnly,
    #[value(name = "BSD-Systemics-W3Works")]
    BSDSystemicsWThreeWorks,
    #[value(name = "BSD-3-Clause-Open-MPI")]
    BSDThreeClauseOpenMPI,
    #[value(name = "OCLC-2.0")]
    OCLCTwoDotZero,
    #[value(name = "LAL-1.2")]
    LALOneDotTwo,
    #[value(name = "Xdebug-1.03")]
    XdebugOneDotZeroThree,
    #[value(name = "FSL-1.1-ALv2")]
    FSLOneDotOneALvTwo,
    #[value(name = "HPND-UC-export-US")]
    HPNDUCExportUS,
    #[value(name = "ECL-1.0")]
    ECLOneDotZero,
    #[value(name = "CC-BY-NC-ND-3.0-IGO")]
    CCBYNCNDThreeDotZeroIGO,
    #[value(name = "OGL-Canada-2.0")]
    OGLCanadaTwoDotZero,
    #[value(name = "AGPL-1.0-or-later")]
    AGPLOneDotZeroOrLater,
    #[value(name = "LGPLLR")]
    LGPLLR,
    #[value(name = "Fair")]
    Fair,
    #[value(name = "Swift-exception")]
    SwiftException,
    #[value(name = "JasPer-2.0")]
    JasPerTwoDotZero,
    #[value(name = "CECILL-1.0")]
    CECILLOneDotZero,
    #[value(name = "GPL-3.0-linking-source-exception")]
    GPLThreeDotZeroLinkingSourceException,
    #[value(name = "BSD-Source-Code")]
    BSDSourceCode,
    #[value(name = "metamail")]
    Metamail,
    #[value(name = "LLVM-exception")]
    LLVMException,
    #[value(name = "VSL-1.0")]
    VSLOneDotZero,
    #[value(name = "LPPL-1.3c")]
    LPPLOneDotThreec,
    #[value(name = "Furuseth")]
    Furuseth,
    #[value(name = "BSD-3-Clause-acpica")]
    BSDThreeClauseAcpica,
    #[value(name = "CNRI-Jython")]
    CNRIJython,
    #[value(name = "LiLiQ-P-1.1")]
    LiLiQPOneDotOne,
    #[value(name = "Ferguson-Twofish")]
    FergusonTwofish,
    #[value(name = "HPND-INRIA-IMAG")]
    HPNDINRIAIMAG,
    #[value(name = "Bitstream-Charter")]
    BitstreamCharter,
    #[value(name = "Unicode-DFS-2016")]
    UnicodeDFSTwoZeroOneSix,
    #[value(name = "MIT-Modern-Variant")]
    MITModernVariant,
    #[value(name = "deprecated_GPL-3.0+")]
    DeprecatedGPLThreeDotZeroPlus,
    #[value(name = "Afmparse")]
    Afmparse,
    #[value(name = "HPND-Fenneberg-Livingston")]
    HPNDFennebergLivingston,
    #[value(name = "wwl")]
    Wwl,
    #[value(name = "ClArtistic")]
    ClArtistic,
    #[value(name = "HPND-Markus-Kuhn")]
    HPNDMarkusKuhn,
    #[value(name = "blessing")]
    Blessing,
    #[value(name = "softSurfer")]
    SoftSurfer,
    #[value(name = "Bison-exception-1.24")]
    BisonExceptionOneDotTwoFour,
    #[value(name = "CrystalStacker")]
    CrystalStacker,
    #[value(name = "AML")]
    AML,
    #[value(name = "NCBI-PD")]
    NCBIPD,
    #[value(name = "GFDL-1.2-invariants-or-later")]
    GFDLOneDotTwoInvariantsOrLater,
    #[value(name = "deprecated_eCos-2.0")]
    DeprecatedECosTwoDotZero,
    #[value(name = "OLDAP-2.5")]
    OLDAPTwoDotFive,
    #[value(name = "AMPAS")]
    AMPAS,
    #[value(name = "GFDL-1.1-no-invariants-or-later")]
    GFDLOneDotOneNoInvariantsOrLater,
    #[value(name = "CC-BY-4.0")]
    CCBYFourDotZero,
    #[value(name = "OLDAP-2.2")]
    OLDAPTwoDotTwo,
    #[value(name = "CNRI-Python")]
    CNRIPython,
    #[value(name = "BSL-1.0")]
    BSLOneDotZero,
    #[value(name = "SMAIL-GPL")]
    SMAILGPL,
    #[value(name = "TU-Berlin-1.0")]
    TUBerlinOneDotZero,
    #[value(name = "VOSTROM")]
    VOSTROM,
    #[value(name = "libpng-2.0")]
    LibpngTwoDotZero,
    #[value(name = "gnu-javamail-exception")]
    GnuJavamailException,
    #[value(name = "CC-BY-NC-1.0")]
    CCBYNCOneDotZero,
    #[value(name = "GPL-3.0-389-ds-base-exception")]
    GPLThreeDotZeroThreeEightNineDsBaseException,
    #[value(name = "OSL-1.0")]
    OSLOneDotZero,
    #[value(name = "Unicode-3.0")]
    UnicodeThreeDotZero,
    #[value(name = "GFDL-1.3-or-later")]
    GFDLOneDotThreeOrLater,
    #[value(name = "InnoSetup")]
    InnoSetup,
    #[value(name = "CAL-1.0")]
    CALOneDotZero,
    #[value(name = "CC-BY-SA-3.0-IGO")]
    CCBYSAThreeDotZeroIGO,
    #[value(name = "Saxpath")]
    Saxpath,
    #[value(name = "OPUBL-1.0")]
    OPUBLOneDotZero,
    #[value(name = "Linux-man-pages-copyleft-2-para")]
    LinuxManPagesCopyleftTwoPara,
    #[value(name = "CC-BY-3.0-AT")]
    CCBYThreeDotZeroAT,
    #[value(name = "GNAT-exception")]
    GNATException,
    #[value(name = "CERN-OHL-P-2.0")]
    CERNOHLPTwoDotZero,
    #[value(name = "x11vnc-openssl-exception")]
    XOneOnevncOpensslException,
    #[value(name = "FSFUL")]
    FSFUL,
    #[value(name = "OPL-UK-3.0")]
    OPLUKThreeDotZero,
    #[value(name = "UMich-Merit")]
    UMichMerit,
    #[value(name = "Cornell-Lossless-JPEG")]
    CornellLosslessJPEG,
    #[value(name = "OGDL-Taiwan-1.0")]
    OGDLTaiwanOneDotZero,
    #[value(name = "BSD-3-Clause-HP")]
    BSDThreeClauseHP,
    #[value(name = "Plexus")]
    Plexus,
    #[value(name = "bcrypt-Solar-Designer")]
    BcryptSolarDesigner,
    #[value(name = "CC-BY-NC-SA-2.0-UK")]
    CCBYNCSATwoDotZeroUK,
    #[value(name = "SGI-B-2.0")]
    SGIBTwoDotZero,
    #[value(name = "CC-BY-3.0-IGO")]
    CCBYThreeDotZeroIGO,
    #[value(name = "Hippocratic-2.1")]
    HippocraticTwoDotOne,
    #[value(name = "SHL-2.1")]
    SHLTwoDotOne,
    #[value(name = "KiCad-libraries-exception")]
    KiCadLibrariesException,
    #[value(name = "CPOL-1.02")]
    CPOLOneDotZeroTwo,
    #[value(name = "deprecated_GPL-1.0")]
    DeprecatedGPLOneDotZero,
    #[value(name = "Cronyx")]
    Cronyx,
    #[value(name = "Latex2e-translated-notice")]
    LatexTwoeTranslatedNotice,
    #[value(name = "Knuth-CTAN")]
    KnuthCTAN,
    #[value(name = "CC-BY-3.0")]
    CCBYThreeDotZero,
    #[value(name = "bzip2-1.0.6")]
    BzipTwoOneDotZeroDotSix,
    #[value(name = "OCCT-PL")]
    OCCTPL,
    #[value(name = "Sendmail-8.23")]
    SendmailEightDotTwoThree,
    #[value(name = "Catharon")]
    Catharon,
    #[value(name = "IPL-1.0")]
    IPLOneDotZero,
    #[value(name = "deprecated_GPL-2.0-with-font-exception")]
    DeprecatedGPLTwoDotZeroWithFontException,
    #[value(name = "APAFML")]
    APAFML,
    #[value(name = "Motosoto")]
    Motosoto,
    #[value(name = "check-cvs")]
    CheckCvs,
    #[value(name = "Sendmail")]
    Sendmail,
    #[value(name = "PPL")]
    PPL,
    #[value(name = "PostgreSQL")]
    PostgreSql,
    #[value(name = "CDL-1.0")]
    CDLOneDotZero,
    #[value(name = "GPL-1.0-or-later")]
    GPLOneDotZeroOrLater,
    #[value(name = "CC-SA-1.0")]
    CCSAOneDotZero,
    #[value(name = "IBM-pibs")]
    IBMPibs,
    #[value(name = "CERN-OHL-1.2")]
    CERNOHLOneDotTwo,
    #[value(name = "Intel-ACPI")]
    IntelACPI,
    #[value(name = "Adobe-2006")]
    AdobeTwoZeroZeroSix,
    #[value(name = "deprecated_LGPL-2.0+")]
    DeprecatedLGPLTwoDotZeroPlus,
    #[value(name = "SPL-1.0")]
    SPLOneDotZero,
    #[value(name = "OML")]
    OML,
    #[value(name = "DOC")]
    DOC,
    #[value(name = "MIT-Click")]
    MITClick,
    #[value(name = "mxml-exception")]
    MxmlException,
    #[value(name = "Apache-2.0")]
    ApacheTwoDotZero,
    #[value(name = "NCGL-UK-2.0")]
    NCGLUKTwoDotZero,
    #[value(name = "HTMLTIDY")]
    HTMLTIDY,
    #[value(name = "ANTLR-PD")]
    ANTLRPD,
    #[value(name = "LAL-1.3")]
    LALOneDotThree,
    #[value(name = "BSD-Inferno-Nettverk")]
    BSDInfernoNettverk,
    #[value(name = "RPL-1.5")]
    RPLOneDotFive,
    #[value(name = "CC0-1.0")]
    CcZeroOneDotZero,
    #[value(name = "CECILL-1.1")]
    CECILLOneDotOne,
    #[value(name = "AML-glslang")]
    AMLGlslang,
    #[value(name = "Digia-Qt-LGPL-exception-1.1")]
    DigiaQtLGPLExceptionOneDotOne,
    #[value(name = "GCR-docs")]
    GCRDocs,
    #[value(name = "Glide")]
    Glide,
    #[value(name = "CC-BY-SA-2.0-UK")]
    CCBYSATwoDotZeroUK,
    #[value(name = "Glulxe")]
    Glulxe,
    #[value(name = "MIT-0")]
    MITZero,
    #[value(name = "BSD-4-Clause-UC")]
    BSDFourClauseUC,
    #[value(name = "deprecated_LGPL-2.1")]
    DeprecatedLGPLTwoDotOne,
    #[value(name = "AFL-2.1")]
    AFLTwoDotOne,
    #[value(name = "eCos-exception-2.0")]
    ECosExceptionTwoDotZero,
    #[value(name = "CERN-OHL-W-2.0")]
    CERNOHLWTwoDotZero,
    #[value(name = "Minpack")]
    Minpack,
    #[value(name = "HPND-Netrek")]
    HPNDNetrek,
    #[value(name = "DocBook-Schema")]
    DocBookSchema,
    #[value(name = "man2html")]
    ManTwohtml,
    #[value(name = "LGPL-2.1-or-later")]
    LGPLTwoDotOneOrLater,
    #[value(name = "CMU-Mach")]
    CMUMach,
    #[value(name = "OLDAP-2.0")]
    OLDAPTwoDotZero,
    #[value(name = "Borceux")]
    Borceux,
    #[value(name = "PHP-3.0")]
    PHPThreeDotZero,
    #[value(name = "HPND-DEC")]
    HPNDDEC,
    #[value(name = "radvd")]
    Radvd,
    #[value(name = "OGTSL")]
    OGTSL,
    #[value(name = "OLDAP-2.7")]
    OLDAPTwoDotSeven,
    #[value(name = "OGL-UK-2.0")]
    OGLUKTwoDotZero,
    #[value(name = "LGPL-3.0-linking-exception")]
    LGPLThreeDotZeroLinkingException,
    #[value(name = "TTYP0")]
    TtypZero,
    #[value(name = "Zed")]
    Zed,
    #[value(name = "PADL")]
    PADL,
    #[value(name = "OFL-1.1-RFN")]
    OFLOneDotOneRFN,
    #[value(name = "OLDAP-2.2.2")]
    OLDAPTwoDotTwoDotTwo,
    #[value(name = "diffmark")]
    Diffmark,
    #[value(name = "W3C")]
    WThreeC,
    #[value(name = "HPND-sell-variant-MIT-disclaimer")]
    HPNDSellVariantMITDisclaimer,
    #[value(name = "Artistic-1.0-Perl")]
    ArtisticOneDotZeroPerl,
    #[value(name = "OCaml-LGPL-linking-exception")]
    OCamlLGPLLinkingException,
    #[value(name = "Python-2.0")]
    PythonTwoDotZero,
    #[value(name = "Mackerras-3-Clause-acknowledgment")]
    MackerrasThreeClauseAcknowledgment,
    #[value(name = "CC-BY-NC-ND-1.0")]
    CCBYNCNDOneDotZero,
    #[value(name = "CC-BY-ND-1.0")]
    CCBYNDOneDotZero,
    #[value(name = "Mup")]
    Mup,
    #[value(name = "TPDL")]
    TPDL,
    #[value(name = "HIDAPI")]
    HIDAPI,
    #[value(name = "Info-ZIP")]
    InfoZIP,
    #[value(name = "deprecated_LGPL-3.0")]
    DeprecatedLGPLThreeDotZero,
    #[value(name = "AFL-3.0")]
    AFLThreeDotZero,
    #[value(name = "HP-1989")]
    HPOneNineEightNine,
    #[value(name = "FSFULLRWD")]
    FSFULLRWD,
    #[value(name = "Leptonica")]
    Leptonica,
    #[value(name = "Graphics-Gems")]
    GraphicsGems,
    #[value(name = "Unicode-DFS-2015")]
    UnicodeDFSTwoZeroOneFive,
    #[value(name = "TrustedQSL")]
    TrustedQsl,
    #[value(name = "APSL-2.0")]
    APSLTwoDotZero,
    #[value(name = "NLPL")]
    NLPL,
    #[value(name = "Giftware")]
    Giftware,
    #[value(name = "SSPL-1.0")]
    SSPLOneDotZero,
    #[value(name = "CC-BY-2.5-AU")]
    CCBYTwoDotFiveAU,
    #[value(name = "HPND-sell-variant-MIT-disclaimer-rev")]
    HPNDSellVariantMITDisclaimerRev,
    #[value(name = "deprecated_GPL-1.0+")]
    DeprecatedGPLOneDotZeroPlus,
    #[value(name = "libtiff")]
    Libtiff,
    #[value(name = "IPA")]
    IPA,
    #[value(name = "CC-BY-SA-1.0")]
    CCBYSAOneDotZero,
    #[value(name = "CC-BY-NC-SA-1.0")]
    CCBYNCSAOneDotZero,
    #[value(name = "ErlPL-1.1")]
    ErlPlOneDotOne,
    #[value(name = "MPL-2.0")]
    MPLTwoDotZero,
    #[value(name = "pkgconf")]
    Pkgconf,
    #[value(name = "BSD-Protection")]
    BSDProtection,
    #[value(name = "GFDL-1.2-no-invariants-only")]
    GFDLOneDotTwoNoInvariantsOnly,
    #[value(name = "Adobe-Glyph")]
    AdobeGlyph,
    #[value(name = "EPL-1.0")]
    EPLOneDotZero,
    #[value(name = "deprecated_LGPL-2.1+")]
    DeprecatedLGPLTwoDotOnePlus,
    #[value(name = "i2p-gpl-java-exception")]
    ITwopGplJavaException,
    #[value(name = "deprecated_GPL-3.0-with-GCC-exception")]
    DeprecatedGPLThreeDotZeroWithGCCException,
    #[value(name = "AMD-newlib")]
    AMDNewlib,
    #[value(name = "Jam")]
    Jam,
    #[value(name = "deprecated_AGPL-1.0")]
    DeprecatedAGPLOneDotZero,
    #[value(name = "Baekmuk")]
    Baekmuk,
    #[value(name = "Qhull")]
    Qhull,
    #[value(name = "OpenSSL-standalone")]
    OpenSslStandalone,
    #[value(name = "PCRE2-exception")]
    PcreTwoException,
    #[value(name = "BSD-2-Clause-pkgconf-disclaimer")]
    BSDTwoClausePkgconfDisclaimer,
    #[value(name = "NBPL-1.0")]
    NBPLOneDotZero,
    #[value(name = "MulanPSL-2.0")]
    MulanPslTwoDotZero,
    #[value(name = "CC-BY-3.0-US")]
    CCBYThreeDotZeroUS,
    #[value(name = "Lucida-Bitmap-Fonts")]
    LucidaBitmapFonts,
    #[value(name = "CC-BY-NC-SA-2.0-FR")]
    CCBYNCSATwoDotZeroFR,
    #[value(name = "ANTLR-PD-fallback")]
    ANTLRPDFallback,
    #[value(name = "MIT-advertising")]
    MITAdvertising,
    #[value(name = "HPND-export-US-modify")]
    HPNDExportUSModify,
    #[value(name = "swrule")]
    Swrule,
    #[value(name = "Beerware")]
    Beerware,
    #[value(name = "SMLNJ")]
    SMLNJ,
    #[value(name = "MPEG-SSG")]
    MPEGSSG,
    #[value(name = "PolyForm-Small-Business-1.0.0")]
    PolyFormSmallBusinessOneDotZeroDotZero,
    #[value(name = "AGPL-3.0-or-later")]
    AGPLThreeDotZeroOrLater,
    #[value(name = "BSD-Advertising-Acknowledgement")]
    BSDAdvertisingAcknowledgement,
    #[value(name = "ZPL-2.0")]
    ZPLTwoDotZero,
    #[value(name = "xpp")]
    Xpp,
    #[value(name = "Nokia")]
    Nokia,
    #[value(name = "HPND-Kevlin-Henney")]
    HPNDKevlinHenney,
    #[value(name = "PolyForm-Noncommercial-1.0.0")]
    PolyFormNoncommercialOneDotZeroDotZero,
    #[value(name = "CC-BY-SA-2.1-JP")]
    CCBYSATwoDotOneJP,
    #[value(name = "xkeyboard-config-Zinoviev")]
    XkeyboardConfigZinoviev,
    #[value(name = "NAIST-2003")]
    NAISTTwoZeroZeroThree,
    #[value(name = "HPND-export-US")]
    HPNDExportUS,
    #[value(name = "GStreamer-exception-2008")]
    GStreamerExceptionTwoZeroZeroEight,
    #[value(name = "deprecated_BSD-2-Clause-NetBSD")]
    DeprecatedBSDTwoClauseNetBsd,
    #[value(name = "WxWindows-exception-3.1")]
    WxWindowsExceptionThreeDotOne,
    #[value(name = "libselinux-1.0")]
    LibselinuxOneDotZero,
    #[value(name = "QPL-1.0")]
    QPLOneDotZero,
    #[value(name = "Pixar")]
    Pixar,
    #[value(name = "Fawkes-Runtime-exception")]
    FawkesRuntimeException,
    #[value(name = "CC-PDDC")]
    CCPDDC,
    #[value(name = "Xerox")]
    Xerox,
    #[value(name = "BSD-3-Clause-No-Nuclear-Warranty")]
    BSDThreeClauseNoNuclearWarranty,
    #[value(name = "any-OSI-perl-modules")]
    AnyOSIPerlModules,
    #[value(name = "BUSL-1.1")]
    BUSLOneDotOne,
    #[value(name = "MakeIndex")]
    MakeIndex,
    #[value(name = "NTIA-PD")]
    NTIAPD,
    #[value(name = "LGPL-3.0-or-later")]
    LGPLThreeDotZeroOrLater,
    #[value(name = "deprecated_Nunit")]
    DeprecatedNunit,
    #[value(name = "OFFIS")]
    OFFIS,
    #[value(name = "Latex2e")]
    LatexTwoe,
    #[value(name = "SSH-OpenSSH")]
    SSHOpenSsh,
    #[value(name = "Entessa")]
    Entessa,
    #[value(name = "AFL-2.0")]
    AFLTwoDotZero,
    #[value(name = "deprecated_LGPL-2.0")]
    DeprecatedLGPLTwoDotZero,
    #[value(name = "Zend-2.0")]
    ZendTwoDotZero,
    #[value(name = "mif-exception")]
    MifException,
    #[value(name = "Xfig")]
    Xfig,
    #[value(name = "Caldera")]
    Caldera,
    #[value(name = "libutil-David-Nugent")]
    LibutilDavidNugent,
    #[value(name = "Asterisk-linking-protocols-exception")]
    AsteriskLinkingProtocolsException,
    #[value(name = "OLDAP-2.6")]
    OLDAPTwoDotSix,
    #[value(name = "0BSD")]
    ZeroBsd,
    #[value(name = "GNU-compiler-exception")]
    GNUCompilerException,
    #[value(name = "Barr")]
    Barr,
    #[value(name = "SGI-OpenGL")]
    SGIOpenGl,
    #[value(name = "CC-PDM-1.0")]
    CCPDMOneDotZero,
    #[value(name = "MIT-enna")]
    MITEnna,
    #[value(name = "NIST-PD")]
    NISTPD,
    #[value(name = "CC-BY-2.5")]
    CCBYTwoDotFive,
    #[value(name = "HPND-sell-regexpr")]
    HPNDSellRegexpr,
    #[value(name = "LOOP")]
    LOOP,
    #[value(name = "OLDAP-2.1")]
    OLDAPTwoDotOne,
    #[value(name = "SISSL-1.2")]
    SISSLOneDotTwo,
    #[value(name = "3D-Slicer-1.0")]
    ThreeDSlicerOneDotZero,
    #[value(name = "OCCT-exception-1.0")]
    OCCTExceptionOneDotZero,
    #[value(name = "389-exception")]
    ThreeEightNineException,
    #[value(name = "SL")]
    SL,
    #[value(name = "FLTK-exception")]
    FLTKException,
    #[value(name = "GFDL-1.1-only")]
    GFDLOneDotOneOnly,
    #[value(name = "CAL-1.0-Combined-Work-Exception")]
    CALOneDotZeroCombinedWorkException,
    #[value(name = "deprecated_StandardML-NJ")]
    DeprecatedStandardMlNJ,
    #[value(name = "ADSL")]
    ADSL,
    #[value(name = "BSD-4.3TAHOE")]
    BSDFourDotThreeTahoe,
    #[value(name = "ZPL-2.1")]
    ZPLTwoDotOne,
    #[value(name = "Imlib2")]
    ImlibTwo,
    #[value(name = "RPL-1.1")]
    RPLOneDotOne,
    #[value(name = "gnuplot")]
    Gnuplot,
    #[value(name = "D-FSL-1.0")]
    DFSLOneDotZero,
    #[value(name = "Adobe-Utopia")]
    AdobeUtopia,
    #[value(name = "OpenSSL")]
    OpenSsl,
    #[value(name = "GPL-3.0-or-later")]
    GPLThreeDotZeroOrLater,
    #[value(name = "OSET-PL-2.1")]
    OSETPLTwoDotOne,
    #[value(name = "LZMA-SDK-9.11-to-9.20")]
    LZMASDKNineDotOneOneToNineDotTwoZero,
    #[value(name = "SAX-PD-2.0")]
    SAXPDTwoDotZero,
    #[value(name = "BSD-3-Clause-Clear")]
    BSDThreeClauseClear,
    #[value(name = "NASA-1.3")]
    NASAOneDotThree,
    #[value(name = "EUDatagrid")]
    EuDatagrid,
    #[value(name = "CERN-OHL-1.1")]
    CERNOHLOneDotOne,
    #[value(name = "etalab-2.0")]
    EtalabTwoDotZero,
    #[value(name = "deprecated_GPL-3.0-with-autoconf-exception")]
    DeprecatedGPLThreeDotZeroWithAutoconfException,
    #[value(name = "CERN-OHL-S-2.0")]
    CERNOHLSTwoDotZero,
    #[value(name = "Gutmann")]
    Gutmann,
    #[value(name = "OLFL-1.3")]
    OLFLOneDotThree,
    #[value(name = "Linux-man-pages-copyleft-var")]
    LinuxManPagesCopyleftVar,
    #[value(name = "OGL-UK-3.0")]
    OGLUKThreeDotZero,
    #[value(name = "Parity-6.0.0")]
    ParitySixDotZeroDotZero,
    #[value(name = "LPPL-1.3a")]
    LPPLOneDotThreea,
    #[value(name = "mplus")]
    Mplus,
    #[value(name = "COIL-1.0")]
    COILOneDotZero,
    #[value(name = "gtkbook")]
    Gtkbook,
    #[value(name = "XSkat")]
    XSkat,
    #[value(name = "HPND-doc-sell")]
    HPNDDocSell,
    #[value(name = "RSA-MD")]
    RSAMD,
    #[value(name = "CC-BY-SA-3.0-AT")]
    CCBYSAThreeDotZeroAT,
    #[value(name = "NIST-PD-fallback")]
    NISTPDFallback,
    #[value(name = "iMatix")]
    IMatix,
    #[value(name = "GPL-3.0-only")]
    GPLThreeDotZeroOnly,
    #[value(name = "CryptoSwift")]
    CryptoSwift,
    #[value(name = "WTFPL")]
    WTFPL,
    #[value(name = "BSD-3-Clause-No-Nuclear-License")]
    BSDThreeClauseNoNuclearLicense,
    #[value(name = "cve-tou")]
    CveTou,
    #[value(name = "HPND-MIT-disclaimer")]
    HPNDMITDisclaimer,
    #[value(name = "CC-BY-ND-2.0")]
    CCBYNDTwoDotZero,
    #[value(name = "Vim")]
    Vim,
    #[value(name = "NIST-Software")]
    NISTSoftware,
    #[value(name = "LPPL-1.0")]
    LPPLOneDotZero,
    #[value(name = "YPL-1.1")]
    YPLOneDotOne,
    #[value(name = "CC-BY-NC-ND-2.0")]
    CCBYNCNDTwoDotZero,
    #[value(name = "copyleft-next-0.3.1")]
    CopyleftNextZeroDotThreeDotOne,
    #[value(name = "MIT-CMU")]
    MITCMU,
    #[value(name = "RPSL-1.0")]
    RPSLOneDotZero,
    #[value(name = "BSD-2-Clause-Patent")]
    BSDTwoClausePatent,
    #[value(name = "dtoa")]
    Dtoa,
    #[value(name = "NCSA")]
    NCSA,
    #[value(name = "NPL-1.1")]
    NPLOneDotOne,
    #[value(name = "SCEA")]
    SCEA,
    #[value(name = "SMPPL")]
    SMPPL,
    #[value(name = "LiLiQ-R-1.1")]
    LiLiQROneDotOne,
    #[value(name = "OFL-1.0-RFN")]
    OFLOneDotZeroRFN,
    #[value(name = "NPOSL-3.0")]
    NPOSLThreeDotZero,
    #[value(name = "ImageMagick")]
    ImageMagick,
    #[value(name = "BSD-4-Clause-Shortened")]
    BSDFourClauseShortened,
    #[value(name = "Asterisk-exception")]
    AsteriskException,
    #[value(name = "libpri-OpenH323-exception")]
    LibpriOpenHThreeTwoThreeException,
    #[value(name = "Aladdin")]
    Aladdin,
    #[value(name = "Unicode-TOU")]
    UnicodeTOU,
    #[value(name = "OpenPBS-2.3")]
    OpenPbsTwoDotThree,
    #[value(name = "any-OSI")]
    AnyOSI,
    #[value(name = "UCL-1.0")]
    UCLOneDotZero,
    #[value(name = "Zimbra-1.4")]
    ZimbraOneDotFour,
    #[value(name = "Bootloader-exception")]
    BootloaderException,
    #[value(name = "Bison-exception-2.2")]
    BisonExceptionTwoDotTwo,
    #[value(name = "TGPPL-1.0")]
    TGPPLOneDotZero,
    #[value(name = "BitTorrent-1.1")]
    BitTorrentOneDotOne,
    #[value(name = "Wsuipa")]
    Wsuipa,
    #[value(name = "deprecated_Nokia-Qt-exception-1.1")]
    DeprecatedNokiaQtExceptionOneDotOne,
    #[value(name = "CC-BY-NC-SA-2.0-DE")]
    CCBYNCSATwoDotZeroDE,
    #[value(name = "Linux-man-pages-copyleft")]
    LinuxManPagesCopyleft,
    #[value(name = "xlock")]
    Xlock,
    #[value(name = "LiLiQ-Rplus-1.1")]
    LiLiQRplusOneDotOne,
    #[value(name = "generic-xts")]
    GenericXts,
    #[value(name = "Zimbra-1.3")]
    ZimbraOneDotThree,
    #[value(name = "GPL-2.0-only")]
    GPLTwoDotZeroOnly,
    #[value(name = "OGL-UK-1.0")]
    OGLUKOneDotZero,
    #[value(name = "AFL-1.1")]
    AFLOneDotOne,
    #[value(name = "SSLeay-standalone")]
    SsLeayStandalone,
    #[value(name = "X11-swapped")]
    XOneOneSwapped,
    #[value(name = "OpenJDK-assembly-exception-1.0")]
    OpenJdkAssemblyExceptionOneDotZero,
    #[value(name = "GFDL-1.3-no-invariants-or-later")]
    GFDLOneDotThreeNoInvariantsOrLater,
    #[value(name = "MTLL")]
    MTLL,
    #[value(name = "Ubuntu-font-1.0")]
    UbuntuFontOneDotZero,
    #[value(name = "DocBook-XML")]
    DocBookXML,
    #[value(name = "MIT-testregex")]
    MITTestregex,
    #[value(name = "CC-BY-NC-2.5")]
    CCBYNCTwoDotFive,
    #[value(name = "python-ldap")]
    PythonLdap,
    #[value(name = "GL2PS")]
    GlTwoPs,
    #[value(name = "LPL-1.02")]
    LPLOneDotZeroTwo,
    #[value(name = "MITNFA")]
    MITNFA,
    #[value(name = "checkmk")]
    Checkmk,
    #[value(name = "GFDL-1.2-only")]
    GFDLOneDotTwoOnly,
    #[value(name = "NGPL")]
    NGPL,
    #[value(name = "MulanPSL-1.0")]
    MulanPslOneDotZero,
    #[value(name = "deprecated_wxWindows")]
    DeprecatedWxWindows,
    #[value(name = "OGC-1.0")]
    OGCOneDotZero,
    #[value(name = "ulem")]
    Ulem,
    #[value(name = "Autoconf-exception-3.0")]
    AutoconfExceptionThreeDotZero,
    #[value(name = "harbour-exception")]
    HarbourException,
    #[value(name = "UCAR")]
    UCAR,
    #[value(name = "MS-PL")]
    MSPL,
    #[value(name = "JPL-image")]
    JPLImage,
    #[value(name = "Font-exception-2.0")]
    FontExceptionTwoDotZero,
    #[value(name = "GFDL-1.3-invariants-or-later")]
    GFDLOneDotThreeInvariantsOrLater,
    #[value(name = "fwlw")]
    Fwlw,
    #[value(name = "Inner-Net-2.0")]
    InnerNetTwoDotZero,
    #[value(name = "MPL-1.0")]
    MPLOneDotZero,
    #[value(name = "Community-Spec-1.0")]
    CommunitySpecOneDotZero,
    #[value(name = "CUA-OPL-1.0")]
    CUAOPLOneDotZero,
    #[value(name = "UBDL-exception")]
    UBDLException,
    #[value(name = "GFDL-1.1-invariants-only")]
    GFDLOneDotOneInvariantsOnly,
    #[value(name = "FreeBSD-DOC")]
    FreeBsdDOC,
    #[value(name = "EPL-2.0")]
    EPLTwoDotZero,
    #[value(name = "Sendmail-Open-Source-1.1")]
    SendmailOpenSourceOneDotOne,
    #[value(name = "Eurosym")]
    Eurosym,
    #[value(name = "GPL-1.0-only")]
    GPLOneDotZeroOnly,
    #[value(name = "deprecated_BSD-2-Clause-FreeBSD")]
    DeprecatedBSDTwoClauseFreeBsd,
    #[value(name = "SHL-0.5")]
    SHLZeroDotFive,
    #[value(name = "X11")]
    XOneOne,
    #[value(name = "ThirdEye")]
    ThirdEye,
    #[value(name = "FSFAP-no-warranty-disclaimer")]
    FSFAPNoWarrantyDisclaimer,
    #[value(name = "SimPL-2.0")]
    SimPlTwoDotZero,
    #[value(name = "Interbase-1.0")]
    InterbaseOneDotZero,
    #[value(name = "deprecated_bzip2-1.0.5")]
    DeprecatedBzipTwoOneDotZeroDotFive,
    #[value(name = "Multics")]
    Multics,
    #[value(name = "GStreamer-exception-2005")]
    GStreamerExceptionTwoZeroZeroFive,
    #[value(name = "CC-BY-NC-SA-2.0")]
    CCBYNCSATwoDotZero,
    #[value(name = "GD")]
    GD,
    #[value(name = "CC-BY-SA-2.0")]
    CCBYSATwoDotZero,
    #[value(name = "CECILL-C")]
    CECILLC,
    #[value(name = "Elastic-2.0")]
    ElasticTwoDotZero,
    #[value(name = "MIT-Wu")]
    MITWu,
    #[value(name = "snprintf")]
    Snprintf,
    #[value(name = "EUPL-1.0")]
    EUPLOneDotZero,
    #[value(name = "AMDPLPA")]
    AMDPLPA,
    #[value(name = "APSL-1.0")]
    APSLOneDotZero,
    #[value(name = "BSD-Source-beginning-file")]
    BSDSourceBeginningFile,
    #[value(name = "SunPro")]
    SunPro,
    #[value(name = "HPND-Intel")]
    HPNDIntel,
    #[value(name = "LZMA-SDK-9.22")]
    LZMASDKNineDotTwoTwo,
    #[value(name = "OFL-1.0")]
    OFLOneDotZero,
    #[value(name = "deprecated_GPL-2.0-with-GCC-exception")]
    DeprecatedGPLTwoDotZeroWithGCCException,
    #[value(name = "Autoconf-exception-generic-3.0")]
    AutoconfExceptionGenericThreeDotZero,
    #[value(name = "CC-BY-SA-3.0")]
    CCBYSAThreeDotZero,
    #[value(name = "CC-BY-NC-SA-3.0")]
    CCBYNCSAThreeDotZero,
    #[value(name = "Bahyph")]
    Bahyph,
    #[value(name = "DSDP")]
    DSDP,
    #[value(name = "McPhee-slideshow")]
    McPheeSlideshow,
    #[value(name = "BitTorrent-1.0")]
    BitTorrentOneDotZero,
    #[value(name = "CC-BY-3.0-NL")]
    CCBYThreeDotZeroNL,
    #[value(name = "OLDAP-1.1")]
    OLDAPOneDotOne,
    #[value(name = "Bitstream-Vera")]
    BitstreamVera,
    #[value(name = "GFDL-1.2-invariants-only")]
    GFDLOneDotTwoInvariantsOnly,
    #[value(name = "SchemeReport")]
    SchemeReport,
    #[value(name = "HPND-export2-US")]
    HPNDExportTwoUS,
    #[value(name = "xinetd")]
    Xinetd,
    #[value(name = "HaskellReport")]
    HaskellReport,
    #[value(name = "TCL")]
    TCL,
    #[value(name = "NetCDF")]
    NetCdf,
    #[value(name = "Symlinks")]
    Symlinks,
    #[value(name = "openvpn-openssl-exception")]
    OpenvpnOpensslException,
    #[value(name = "Arphic-1999")]
    ArphicOneNineNineNine,
    #[value(name = "HPND-Pbmplus")]
    HPNDPbmplus,
    #[value(name = "OAR")]
    OAR,
    #[value(name = "copyleft-next-0.3.0")]
    CopyleftNextZeroDotThreeDotZero,
    #[value(name = "YPL-1.0")]
    YPLOneDotZero,
    #[value(name = "LPPL-1.1")]
    LPPLOneDotOne,
    #[value(name = "CC-BY-ND-4.0")]
    CCBYNDFourDotZero,
    #[value(name = "Autoconf-exception-2.0")]
    AutoconfExceptionTwoDotZero,
    #[value(name = "XFree86-1.1")]
    XFreeEightSixOneDotOne,
    #[value(name = "CC-BY-NC-ND-4.0")]
    CCBYNCNDFourDotZero,
    #[value(name = "CC-BY-NC-SA-3.0-DE")]
    CCBYNCSAThreeDotZeroDE,
    #[value(name = "TPL-1.0")]
    TPLOneDotZero,
    #[value(name = "Naumen")]
    Naumen,
    #[value(name = "NICTA-1.0")]
    NICTAOneDotZero,
    #[value(name = "NOSL")]
    NOSL,
    #[value(name = "pnmstitch")]
    Pnmstitch,
    #[value(name = "CPL-1.0")]
    CPLOneDotZero,
    #[value(name = "xzoom")]
    Xzoom,
    #[value(name = "IEC-Code-Components-EULA")]
    IECCodeComponentsEULA,
    #[value(name = "deprecated_AGPL-3.0")]
    DeprecatedAGPLThreeDotZero,
    #[value(name = "RSCPL")]
    RSCPL,
    #[value(name = "NPL-1.0")]
    NPLOneDotZero,
    #[value(name = "BSD-3-Clause-No-Nuclear-License-2014")]
    BSDThreeClauseNoNuclearLicenseTwoZeroOneFour,
    #[value(name = "Sleepycat")]
    Sleepycat,
    #[value(name = "CDLA-Sharing-1.0")]
    CDLASharingOneDotZero,
    #[value(name = "GFDL-1.3-only")]
    GFDLOneDotThreeOnly,
    #[value(name = "lsof")]
    Lsof,
    #[value(name = "Parity-7.0.0")]
    ParitySevenDotZeroDotZero,
    #[value(name = "AAL")]
    AAL,
    #[value(name = "Zeeff")]
    Zeeff,
    #[value(name = "CC-BY-NC-SA-4.0")]
    CCBYNCSAFourDotZero,
    #[value(name = "BlueOak-1.0.0")]
    BlueOakOneDotZeroDotZero,
    #[value(name = "CC-BY-SA-4.0")]
    CCBYSAFourDotZero,
    #[value(name = "GFDL-1.2-or-later")]
    GFDLOneDotTwoOrLater,
    #[value(name = "OFL-1.1")]
    OFLOneDotOne,
    #[value(name = "APSL-1.1")]
    APSLOneDotOne,
    #[value(name = "GPL-3.0-interface-exception")]
    GPLThreeDotZeroInterfaceException,
    #[value(name = "Qt-LGPL-exception-1.1")]
    QtLGPLExceptionOneDotOne,
    #[value(name = "Mackerras-3-Clause")]
    MackerrasThreeClause,
    #[value(name = "EUPL-1.1")]
    EUPLOneDotOne,
    #[value(name = "Autoconf-exception-generic")]
    AutoconfExceptionGeneric,
    #[value(name = "Sun-PPP")]
    SunPPP,
    #[value(name = "CECILL-B")]
    CECILLB,
    #[value(name = "Linux-OpenIB")]
    LinuxOpenIb,
    #[value(name = "BSD-Attribution-HPND-disclaimer")]
    BSDAttributionHPNDDisclaimer,
    #[value(name = "fmt-exception")]
    FmtException,
    #[value(name = "MIT-feh")]
    MITFeh,
    #[value(name = "Ruby-pty")]
    RubyPty,
    #[value(name = "BSD-3-Clause-No-Military-License")]
    BSDThreeClauseNoMilitaryLicense,
    #[value(name = "HPND-sell-variant")]
    HPNDSellVariant,
    #[value(name = "DEC-3-Clause")]
    DECThreeClause,
    #[value(name = "ZPL-1.1")]
    ZPLOneDotOne,
    #[value(name = "Zlib")]
    Zlib,
    #[value(name = "FTL")]
    FTL,
    #[value(name = "CC-BY-SA-3.0-DE")]
    CCBYSAThreeDotZeroDE,
    #[value(name = "CC-BY-NC-ND-3.0")]
    CCBYNCNDThreeDotZero,
    #[value(name = "TOSL")]
    TOSL,
    #[value(name = "FBM")]
    FBM,
    #[value(name = "CC-BY-ND-3.0")]
    CCBYNDThreeDotZero,
    #[value(name = "C-UDA-1.0")]
    CUDAOneDotZero,
    #[value(name = "Qt-GPL-exception-1.0")]
    QtGPLExceptionOneDotZero,
    #[value(name = "Sun-PPP-2000")]
    SunPPPTwoZeroZeroZero,
    #[value(name = "OpenVision")]
    OpenVision,
    #[value(name = "deprecated_GFDL-1.1")]
    DeprecatedGFDLOneDotOne,
    #[value(name = "ssh-keyscan")]
    SshKeyscan,
    #[value(name = "SSH-short")]
    SSHShort,
    #[value(name = "gSOAP-1.3b")]
    GSoapOneDotThreeb,
    #[value(name = "JPNIC")]
    JPNIC,
    #[value(name = "GFDL-1.1-no-invariants-only")]
    GFDLOneDotOneNoInvariantsOnly,
    #[value(name = "TAPR-OHL-1.0")]
    TAPROHLOneDotZero,
    #[value(name = "Rdisc")]
    Rdisc,
    #[value(name = "BSD-2-Clause-Views")]
    BSDTwoClauseViews,
    #[value(name = "Artistic-1.0-cl8")]
    ArtisticOneDotZeroClEight,
    #[value(name = "HPND-doc")]
    HPNDDoc,
    #[value(name = "BSD-3-Clause-Sun")]
    BSDThreeClauseSun,
    #[value(name = "URT-RLE")]
    URTRLE,
    #[value(name = "mpich2")]
    MpichTwo,
    #[value(name = "CGAL-linking-exception")]
    CGALLinkingException,
    #[value(name = "MPL-1.1")]
    MPLOneDotOne,
}

// ▰▰▰ CLI Argument Structs ▰▰▰ //

pub fn parse_year_to_date(s: &str) -> Result<Date, String> {
    println!("{s}");
    // Attempt to parse as a full date; If parsing fails, it's only a problem if it was *meant* to be a full-date.
    match s.parse::<Date>() {
        Ok(date) => return Ok(date),
        Err(e) => {
            // Basic assumption here is that anything longer than 4 chars, the modern length for a year, is an incorrectly formatted date string.
            if s.len() > 4 {
                return Err(e.to_string());
            }
        }
    }

    // Fallback to parsing as year only. This fails if there are any non-numbers in the string
    let year: i16 = s
        .parse()
        .map_err(|e| format!("invalid year `{}`: {} Please use only numerals", s, e))?;

    // Construct to a chill, January 1st of that year.
    Date::new(year, 1, 1).map_err(|e| format!("invalid date: {}", e))
}

pub fn parse_to_author(input: &str) -> Result<Authors, String> {
    // If the whole string is only whitespace, reject it.
    if input.trim().is_empty() {
        return Err("You need to provide at least one author in the format NAME[:EMAIL]".into());
    }

    let authors = input
        .split(',') // Delimter for consecutive authors is a comma
        .map(str::trim) // Whitespace is bad
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            // Split at most once on ':'
            let mut parts = entry.splitn(2, ':');
            let name = parts
                .next()
                .expect("splitn always yields at least one element")
                .trim();

            if name.is_empty() {
                return Err(format!("entry `{}` has empty name", entry));
            }

            // If an email exists, and isn't blank, use it.
            let email = parts
                .next()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);

            Ok(Author {
                name: name.to_string(),
                email,
            })
        })
        .collect::<Result<Vec<Author>, String>>()?; // Fails immeidately with any errors.

    Ok(Authors(authors))
}

// Common arguments related to license information
#[derive(Args, Debug)]
pub struct LicenseArgs {
    /// SPDX identifier of the license to generate (e.g., MIT, Apache-2.0).
    /// Can be omitted if specified in configuration.
    #[arg()]
    pub license: Option<License>,

    /// Author names and emails (In the format NAME:EMAIL; entries seperated by a comma. Email optional).
    #[arg(short, long, value_parser = parse_to_author)]
    pub authors: Option<Authors>,

    /// Date for the license copyright notice (defaults to the current year).
    #[arg(short, long, value_parser = parse_year_to_date)]
    pub date: Option<Date>,

    /// Enable support for multiple licenses in the same project (Default is replace)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub multiple: Option<bool>,
}

// Common arguments for file processing
#[derive(Args, Debug)]
pub struct FileProcessingArgs {
    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,

    /// Regex pattern for files/directories to exclude. Applied during directory traversal.
    #[arg(short, long)]
    pub exclude: Option<Regex>,

    /// Do not respect the git_ignore file (If present in directory) and other pattern defaults
    #[arg(short = 'A', long, action = clap::ArgAction::SetTrue)]
    pub all: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a license file
    Gen(GenArgs),

    /// Remove license headers in source files
    Unapply(UnapplyArgs),

    /// Apply license headers to source files
    Apply(ApplyArgs),

    /// Initialize a default configuration file
    Init(InitArgs),
}

#[derive(Args, Debug)]
pub struct GenArgs {
    #[command(flatten)]
    pub license_args: LicenseArgs,

    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,
}

#[derive(Args, Debug)]
pub struct ApplyArgs {
    #[command(flatten)]
    pub license_args: LicenseArgs,

    #[command(flatten)]
    pub file_args: FileProcessingArgs,

    /// Run without modification. See what would be changed.
    #[arg(short = 'D', long, action = clap::ArgAction::SetTrue)]
    pub dry_run: Option<bool>,

    /// When applying headers, which kind of comment token the user *wants*
    /// Completely possible line or block doesn't exist, in which case it falls back to the other.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub prefer_block: Option<bool>,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Optional path where the configuration should be initialized.
    /// Defaults to the current directory.
    #[arg()]
    pub target: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct UnapplyArgs {
    #[command(flatten)]
    pub file_args: FileProcessingArgs,

    /// Run without modification. See what would be changed.
    #[arg(short = 'D', long)]
    pub dry_run: Option<bool>,
}

// The main Cli struct
#[derive(Parser, Debug)]
#[command(author, version, about, styles = clap::builder::styling::Styles::styled() // Define styles inline or omit for build
    .header(clap::builder::styling::AnsiColor::Green.on_default().bold())
    .usage(clap::builder::styling::AnsiColor::Green.on_default().bold())
    .literal(clap::builder::styling::AnsiColor::Blue.on_default().bold())
    .placeholder(clap::builder::styling::AnsiColor::Cyan.on_default()),
    long_about = None, color = clap::ColorChoice::Auto)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
    #[arg(long, short)]
    pub config: Option<PathBuf>,
}
