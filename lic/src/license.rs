#![allow(clippy::all)]
#[derive(Debug, Clone, serde::Deserialize, Copy, PartialEq, Eq, Hash, clap :: ValueEnum)]
pub enum License {
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
    #[value(name = "CDDL-1.0")]
    CDDLOneDotZero,
    #[value(name = "SWI-exception")]
    SWIException,
    #[value(name = "GFDL-1.3-invariants-only")]
    GFDLOneDotThreeInvariantsOnly,
    #[value(name = "MS-RL")]
    MSRL,
    #[value(name = "LGPL-2.1-only")]
    LGPLTwoDotOneOnly,
    #[value(name = "HP-1986")]
    HPOneNineEightSix,
    #[value(name = "Python-2.0.1")]
    PythonTwoDotZeroDotOne,
    #[value(name = "RHeCos-1.1")]
    RHeCosOneDotOne,
    #[value(name = "MIT")]
    MIT,
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
    #[value(name = "CNRI-Python-GPL-Compatible")]
    CNRIPythonGPLCompatible,
    #[value(name = "LPD-document")]
    LPDDocument,
    #[value(name = "GPL-CC-1.0")]
    GPLCCOneDotZero,
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
    #[value(name = "RRDtool-FLOSS-exception-2.0")]
    RrDtoolFLOSSExceptionTwoDotZero,
    #[value(name = "Libtool-exception")]
    LibtoolException,
    #[value(name = "PS-or-PDF-font-exception-20170817")]
    PSOrPDFFontExceptionTwoZeroOneSevenZeroEightOneSeven,
    #[value(name = "deprecated_GPL-2.0")]
    DeprecatedGPLTwoDotZero,
    #[value(name = "LGPL-3.0-only")]
    LGPLThreeDotZeroOnly,
    #[value(name = "DigiRule-FOSS-exception")]
    DigiRuleFOSSException,
    #[value(name = "GPL-3.0-linking-exception")]
    GPLThreeDotZeroLinkingException,
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
    #[value(name = "TermReadKey")]
    TermReadKey,
    #[value(name = "UnixCrypt")]
    UnixCrypt,
    #[value(name = "NTP")]
    NTP,
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
    #[value(name = "NCL")]
    NCL,
    #[value(name = "EPICS")]
    EPICS,
    #[value(name = "JSON")]
    JSON,
    #[value(name = "deprecated_GFDL-1.2")]
    DeprecatedGFDLOneDotTwo,
    #[value(name = "Spencer-86")]
    SpencerEightSix,
    #[value(name = "SHL-0.51")]
    SHLZeroDotFiveOne,
    #[value(name = "IJG")]
    IJG,
    #[value(name = "deprecated_GPL-2.0+")]
    DeprecatedGPLTwoDotZeroPlus,
    #[value(name = "erlang-otp-linking-exception")]
    ErlangOtpLinkingException,
    #[value(name = "SAX-PD")]
    SAXPD,
    #[value(name = "PSF-2.0")]
    PSFTwoDotZero,
    #[value(name = "QPL-1.0-INRIA-2004")]
    QPLOneDotZeroINRIATwoZeroZeroFour,
    #[value(name = "deprecated_GPL-2.0-with-classpath-exception")]
    DeprecatedGPLTwoDotZeroWithClasspathException,
    #[value(name = "EUPL-1.2")]
    EUPLOneDotTwo,
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
    #[value(name = "w3m")]
    WThreem,
    #[value(name = "FSL-1.1-MIT")]
    FSLOneDotOneMIT,
    #[value(name = "Texinfo-exception")]
    TexinfoException,
    #[value(name = "GFDL-1.2-no-invariants-or-later")]
    GFDLOneDotTwoNoInvariantsOrLater,
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
    #[value(name = "BSD-Systemics")]
    BSDSystemics,
    #[value(name = "GCC-exception-2.0-note")]
    GCCExceptionTwoDotZeroNote,
    #[value(name = "BSD-Systemics-W3Works")]
    BSDSystemicsWThreeWorks,
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
    #[value(name = "CNRI-Jython")]
    CNRIJython,
    #[value(name = "LiLiQ-P-1.1")]
    LiLiQPOneDotOne,
    #[value(name = "Ferguson-Twofish")]
    FergusonTwofish,
    #[value(name = "HPND-INRIA-IMAG")]
    HPNDINRIAIMAG,
    #[value(name = "Unicode-DFS-2016")]
    UnicodeDFSTwoZeroOneSix,
    #[value(name = "MIT-Modern-Variant")]
    MITModernVariant,
    #[value(name = "deprecated_GPL-3.0+")]
    DeprecatedGPLThreeDotZeroPlus,
    #[value(name = "HPND-Fenneberg-Livingston")]
    HPNDFennebergLivingston,
    #[value(name = "wwl")]
    Wwl,
    #[value(name = "ClArtistic")]
    ClArtistic,
    #[value(name = "HPND-Markus-Kuhn")]
    HPNDMarkusKuhn,
    #[value(name = "softSurfer")]
    SoftSurfer,
    #[value(name = "CrystalStacker")]
    CrystalStacker,
    #[value(name = "NCBI-PD")]
    NCBIPD,
    #[value(name = "GFDL-1.2-invariants-or-later")]
    GFDLOneDotTwoInvariantsOrLater,
    #[value(name = "deprecated_eCos-2.0")]
    DeprecatedECosTwoDotZero,
    #[value(name = "OLDAP-2.5")]
    OLDAPTwoDotFive,
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
    #[value(name = "Plexus")]
    Plexus,
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
    #[value(name = "NCGL-UK-2.0")]
    NCGLUKTwoDotZero,
    #[value(name = "HTMLTIDY")]
    HTMLTIDY,
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
    #[value(name = "deprecated_LGPL-2.1")]
    DeprecatedLGPLTwoDotOne,
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
    #[value(name = "EPL-1.0")]
    EPLOneDotZero,
    #[value(name = "deprecated_LGPL-2.1+")]
    DeprecatedLGPLTwoDotOnePlus,
    #[value(name = "i2p-gpl-java-exception")]
    ITwopGplJavaException,
    #[value(name = "deprecated_GPL-3.0-with-GCC-exception")]
    DeprecatedGPLThreeDotZeroWithGCCException,
    #[value(name = "Jam")]
    Jam,
    #[value(name = "deprecated_AGPL-1.0")]
    DeprecatedAGPLOneDotZero,
    #[value(name = "Qhull")]
    Qhull,
    #[value(name = "OpenSSL-standalone")]
    OpenSslStandalone,
    #[value(name = "PCRE2-exception")]
    PcreTwoException,
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
    #[value(name = "MIT-advertising")]
    MITAdvertising,
    #[value(name = "HPND-export-US-modify")]
    HPNDExportUSModify,
    #[value(name = "swrule")]
    Swrule,
    #[value(name = "SMLNJ")]
    SMLNJ,
    #[value(name = "MPEG-SSG")]
    MPEGSSG,
    #[value(name = "PolyForm-Small-Business-1.0.0")]
    PolyFormSmallBusinessOneDotZeroDotZero,
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
    #[value(name = "BUSL-1.1")]
    BUSLOneDotOne,
    #[value(name = "MakeIndex")]
    MakeIndex,
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
    #[value(name = "OLDAP-2.6")]
    OLDAPTwoDotSix,
    #[value(name = "GNU-compiler-exception")]
    GNUCompilerException,
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
    #[value(name = "OCCT-exception-1.0")]
    OCCTExceptionOneDotZero,
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
    #[value(name = "libpri-OpenH323-exception")]
    LibpriOpenHThreeTwoThreeException,
    #[value(name = "Unicode-TOU")]
    UnicodeTOU,
    #[value(name = "OpenPBS-2.3")]
    OpenPbsTwoDotThree,
    #[value(name = "UCL-1.0")]
    UCLOneDotZero,
    #[value(name = "Zimbra-1.4")]
    ZimbraOneDotFour,
    #[value(name = "TGPPL-1.0")]
    TGPPLOneDotZero,
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
    #[value(name = "CC-BY-SA-3.0")]
    CCBYSAThreeDotZero,
    #[value(name = "CC-BY-NC-SA-3.0")]
    CCBYNCSAThreeDotZero,
    #[value(name = "DSDP")]
    DSDP,
    #[value(name = "McPhee-slideshow")]
    McPheeSlideshow,
    #[value(name = "CC-BY-3.0-NL")]
    CCBYThreeDotZeroNL,
    #[value(name = "OLDAP-1.1")]
    OLDAPOneDotOne,
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
    #[value(name = "Zeeff")]
    Zeeff,
    #[value(name = "CC-BY-NC-SA-4.0")]
    CCBYNCSAFourDotZero,
    #[value(name = "CC-BY-SA-4.0")]
    CCBYSAFourDotZero,
    #[value(name = "GFDL-1.2-or-later")]
    GFDLOneDotTwoOrLater,
    #[value(name = "OFL-1.1")]
    OFLOneDotOne,
    #[value(name = "GPL-3.0-interface-exception")]
    GPLThreeDotZeroInterfaceException,
    #[value(name = "Qt-LGPL-exception-1.1")]
    QtLGPLExceptionOneDotOne,
    #[value(name = "Mackerras-3-Clause")]
    MackerrasThreeClause,
    #[value(name = "EUPL-1.1")]
    EUPLOneDotOne,
    #[value(name = "Sun-PPP")]
    SunPPP,
    #[value(name = "CECILL-B")]
    CECILLB,
    #[value(name = "Linux-OpenIB")]
    LinuxOpenIb,
    #[value(name = "fmt-exception")]
    FmtException,
    #[value(name = "MIT-feh")]
    MITFeh,
    #[value(name = "Ruby-pty")]
    RubyPty,
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
    #[value(name = "HPND-doc")]
    HPNDDoc,
    #[value(name = "URT-RLE")]
    URTRLE,
    #[value(name = "mpich2")]
    MpichTwo,
    #[value(name = "CGAL-linking-exception")]
    CGALLinkingException,
    #[value(name = "MPL-1.1")]
    MPLOneDotOne,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseLicenseError;
impl std::fmt::Display for ParseLicenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Provided string does not match any known license filename"
        )
    }
}
impl std::error::Error for ParseLicenseError {}
impl License {
    #[doc = r#" Returns the original filename of the license (e.g., "Apache-2.0.md")."#]
    pub fn spdx_id(&self) -> &'static str {
        match self {
            Self::LinuxSyscallNote => "Linux-syscall-note",
            Self::DocBookStylesheet => "DocBook-Stylesheet",
            Self::HPNDSellMITDisclaimerXserver => "HPND-sell-MIT-disclaimer-xserver",
            Self::OSLTwoDotOne => "OSL-2.1",
            Self::SWL => "SWL",
            Self::CCBYNCFourDotZero => "CC-BY-NC-4.0",
            Self::NLODTwoDotZero => "NLOD-2.0",
            Self::Unlicense => "Unlicense",
            Self::ODCByOneDotZero => "ODC-By-1.0",
            Self::UPLOneDotZero => "UPL-1.0",
            Self::SNIA => "SNIA",
            Self::GCCExceptionThreeDotOne => "GCC-exception-3.1",
            Self::OLDAPOneDotThree => "OLDAP-1.3",
            Self::Dotseqn => "Dotseqn",
            Self::TMate => "TMate",
            Self::GFDLOneDotOneInvariantsOrLater => "GFDL-1.1-invariants-or-later",
            Self::MmiXware => "MMIXware",
            Self::Crossword => "Crossword",
            Self::OLDAPOneDotFour => "OLDAP-1.4",
            Self::GLWTPL => "GLWTPL",
            Self::Curl => "curl",
            Self::GPLTwoDotZeroOrLater => "GPL-2.0-or-later",
            Self::OFLOneDotOneNoRFN => "OFL-1.1-no-RFN",
            Self::MSLPL => "MS-LPL",
            Self::CCBYOneDotZero => "CC-BY-1.0",
            Self::CDLAPermissiveOneDotZero => "CDLA-Permissive-1.0",
            Self::QPLOneDotZeroINRIATwoZeroZeroFourException => "QPL-1.0-INRIA-2004-exception",
            Self::ClasspathExceptionTwoDotZero => "Classpath-exception-2.0",
            Self::OPLOneDotZero => "OPL-1.0",
            Self::ODbLOneDotZero => "ODbL-1.0",
            Self::IndependentModulesException => "Independent-modules-exception",
            Self::MITFestival => "MIT-Festival",
            Self::CalderaNoPreamble => "Caldera-no-preamble",
            Self::SGIBOneDotOne => "SGI-B-1.1",
            Self::Soundex => "Soundex",
            Self::SANEException => "SANE-exception",
            Self::MirOs => "MirOS",
            Self::Ruby => "Ruby",
            Self::PolyparseException => "polyparse-exception",
            Self::EFLTwoDotZero => "EFL-2.0",
            Self::LLGPL => "LLGPL",
            Self::VsftpdOpensslException => "vsftpd-openssl-exception",
            Self::OFLOneDotZeroNoRFN => "OFL-1.0-no-RFN",
            Self::CCBYNCNDThreeDotZeroDE => "CC-BY-NC-ND-3.0-DE",
            Self::CDDLOneDotZero => "CDDL-1.0",
            Self::SWIException => "SWI-exception",
            Self::GFDLOneDotThreeInvariantsOnly => "GFDL-1.3-invariants-only",
            Self::MSRL => "MS-RL",
            Self::LGPLTwoDotOneOnly => "LGPL-2.1-only",
            Self::HPOneNineEightSix => "HP-1986",
            Self::PythonTwoDotZeroDotOne => "Python-2.0.1",
            Self::RHeCosOneDotOne => "RHeCos-1.1",
            Self::MIT => "MIT",
            Self::RomicException => "romic-exception",
            Self::OSLThreeDotZero => "OSL-3.0",
            Self::CCBYNCThreeDotZero => "CC-BY-NC-3.0",
            Self::GCCExceptionTwoDotZero => "GCC-exception-2.0",
            Self::DeprecatedLGPLThreeDotZeroPlus => "deprecated_LGPL-3.0+",
            Self::Clips => "Clips",
            Self::CNRIPythonGPLCompatible => "CNRI-Python-GPL-Compatible",
            Self::LPDDocument => "LPD-document",
            Self::GPLCCOneDotZero => "GPL-CC-1.0",
            Self::HPNDUC => "HPND-UC",
            Self::ISCVeillard => "ISC-Veillard",
            Self::PDDLOneDotZero => "PDDL-1.0",
            Self::LGPLTwoDotZeroOrLater => "LGPL-2.0-or-later",
            Self::CCBYNCSAThreeDotZeroIGO => "CC-BY-NC-SA-3.0-IGO",
            Self::TORQUEOneDotOne => "TORQUE-1.1",
            Self::FrameworxOneDotZero => "Frameworx-1.0",
            Self::OLDAPTwoDotEight => "OLDAP-2.8",
            Self::DeprecatedGPLThreeDotZero => "deprecated_GPL-3.0",
            Self::QwtExceptionOneDotZero => "Qwt-exception-1.0",
            Self::DocBookDTD => "DocBook-DTD",
            Self::Psfrag => "psfrag",
            Self::IJGShort => "IJG-short",
            Self::Psutils => "psutils",
            Self::DRLOneDotOne => "DRL-1.1",
            Self::CECILLTwoDotZero => "CECILL-2.0",
            Self::WThreeCOneNineNineEightZeroSevenTwoZero => "W3C-19980720",
            Self::OUDAOneDotZero => "O-UDA-1.0",
            Self::CLISPExceptionTwoDotZero => "CLISP-exception-2.0",
            Self::DLDEZEROTwoDotZero => "DL-DE-ZERO-2.0",
            Self::DeprecatedGFDLOneDotThree => "deprecated_GFDL-1.3",
            Self::SgpFour => "SGP4",
            Self::FDKAAC => "FDK-AAC",
            Self::TTWL => "TTWL",
            Self::MartinBirgmeier => "Martin-Birgmeier",
            Self::CCBYNDThreeDotZeroDE => "CC-BY-ND-3.0-DE",
            Self::ECLTwoDotZero => "ECL-2.0",
            Self::ICU => "ICU",
            Self::StunnelException => "stunnel-exception",
            Self::GameProgrammingGems => "Game-Programming-Gems",
            Self::SGIBOneDotZero => "SGI-B-1.0",
            Self::NRL => "NRL",
            Self::Kazlib => "Kazlib",
            Self::PHPThreeDotZeroOne => "PHP-3.01",
            Self::Magaz => "magaz",
            Self::CPALOneDotZero => "CPAL-1.0",
            Self::RrDtoolFLOSSExceptionTwoDotZero => "RRDtool-FLOSS-exception-2.0",
            Self::LibtoolException => "Libtool-exception",
            Self::PSOrPDFFontExceptionTwoZeroOneSevenZeroEightOneSeven => {
                "PS-or-PDF-font-exception-20170817"
            }
            Self::DeprecatedGPLTwoDotZero => "deprecated_GPL-2.0",
            Self::LGPLThreeDotZeroOnly => "LGPL-3.0-only",
            Self::DigiRuleFOSSException => "DigiRule-FOSS-exception",
            Self::GPLThreeDotZeroLinkingException => "GPL-3.0-linking-exception",
            Self::Mailprio => "mailprio",
            Self::LPPLOneDotTwo => "LPPL-1.2",
            Self::CDDLOneDotOne => "CDDL-1.1",
            Self::HPND => "HPND",
            Self::NTPZero => "NTP-0",
            Self::CCBYNDTwoDotFive => "CC-BY-ND-2.5",
            Self::CCBYNCNDTwoDotFive => "CC-BY-NC-ND-2.5",
            Self::FreertosExceptionTwoDotZero => "freertos-exception-2.0",
            Self::DeprecatedGPLTwoDotZeroWithBisonException => {
                "deprecated_GPL-2.0-with-bison-exception"
            }
            Self::SugarCrmOneDotOneDotThree => "SugarCRM-1.1.3",
            Self::SISSL => "SISSL",
            Self::CFITSIO => "CFITSIO",
            Self::TUBerlinTwoDotZero => "TU-Berlin-2.0",
            Self::CMUMachNodoc => "CMU-Mach-nodoc",
            Self::CondorOneDotOne => "Condor-1.1",
            Self::OSLTwoDotZero => "OSL-2.0",
            Self::CCBYNCTwoDotZero => "CC-BY-NC-2.0",
            Self::TermReadKey => "TermReadKey",
            Self::UnixCrypt => "UnixCrypt",
            Self::NTP => "NTP",
            Self::CCBYThreeDotZeroDE => "CC-BY-3.0-DE",
            Self::Kastrup => "Kastrup",
            Self::LGPLTwoDotZeroOnly => "LGPL-2.0-only",
            Self::OLDAPOneDotTwo => "OLDAP-1.2",
            Self::LinuxManPagesOnePara => "Linux-man-pages-1-para",
            Self::DLDEBYTwoDotZero => "DL-DE-BY-2.0",
            Self::CCBYNCThreeDotZeroDE => "CC-BY-NC-3.0-DE",
            Self::FreeImage => "FreeImage",
            Self::CECILLTwoDotOne => "CECILL-2.1",
            Self::DRLOneDotZero => "DRL-1.0",
            Self::HPNDExportUSAcknowledgement => "HPND-export-US-acknowledgement",
            Self::OLDAPTwoDotZeroDotOne => "OLDAP-2.0.1",
            Self::Threeparttable => "threeparttable",
            Self::MITOpenGroup => "MIT-open-group",
            Self::FSFULLR => "FSFULLR",
            Self::NCL => "NCL",
            Self::EPICS => "EPICS",
            Self::JSON => "JSON",
            Self::DeprecatedGFDLOneDotTwo => "deprecated_GFDL-1.2",
            Self::SpencerEightSix => "Spencer-86",
            Self::SHLZeroDotFiveOne => "SHL-0.51",
            Self::IJG => "IJG",
            Self::DeprecatedGPLTwoDotZeroPlus => "deprecated_GPL-2.0+",
            Self::ErlangOtpLinkingException => "erlang-otp-linking-exception",
            Self::SAXPD => "SAX-PD",
            Self::PSFTwoDotZero => "PSF-2.0",
            Self::QPLOneDotZeroINRIATwoZeroZeroFour => "QPL-1.0-INRIA-2004",
            Self::DeprecatedGPLTwoDotZeroWithClasspathException => {
                "deprecated_GPL-2.0-with-classpath-exception"
            }
            Self::EUPLOneDotTwo => "EUPL-1.2",
            Self::GNOMEExamplesException => "GNOME-examples-exception",
            Self::XOneOneDistributeModificationsVariant => "X11-distribute-modifications-variant",
            Self::LPLOneDotZero => "LPL-1.0",
            Self::SpencerNineFour => "Spencer-94",
            Self::CCBYNCSATwoDotFive => "CC-BY-NC-SA-2.5",
            Self::UBootExceptionTwoDotZero => "u-boot-exception-2.0",
            Self::CCBYSATwoDotFive => "CC-BY-SA-2.5",
            Self::Newsletr => "Newsletr",
            Self::Noweb => "Noweb",
            Self::WThreem => "w3m",
            Self::FSLOneDotOneMIT => "FSL-1.1-MIT",
            Self::TexinfoException => "Texinfo-exception",
            Self::GFDLOneDotTwoNoInvariantsOrLater => "GFDL-1.2-no-invariants-or-later",
            Self::EFLOneDotZero => "EFL-1.0",
            Self::WatcomOneDotZero => "Watcom-1.0",
            Self::CCBYThreeDotZeroAU => "CC-BY-3.0-AU",
            Self::ISC => "ISC",
            Self::ZlibAcknowledgement => "zlib-acknowledgement",
            Self::TCPWrappers => "TCP-wrappers",
            Self::UniversalFOSSExceptionOneDotZero => "Universal-FOSS-exception-1.0",
            Self::Hdparm => "hdparm",
            Self::SHLTwoDotZero => "SHL-2.0",
            Self::OLDAPTwoDotTwoDotOne => "OLDAP-2.2.1",
            Self::GFDLOneDotThreeNoInvariantsOnly => "GFDL-1.3-no-invariants-only",
            Self::EGenix => "eGenix",
            Self::FSFAP => "FSFAP",
            Self::SpencerNineNine => "Spencer-99",
            Self::OLDAPTwoDotThree => "OLDAP-2.3",
            Self::GmshException => "Gmsh-exception",
            Self::Dvipdfm => "dvipdfm",
            Self::DeprecatedNetSNMP => "deprecated_Net-SNMP",
            Self::LZMAException => "LZMA-exception",
            Self::Libpng => "Libpng",
            Self::Xnet => "Xnet",
            Self::HPNDMerchantabilityVariant => "HPND-merchantability-variant",
            Self::Intel => "Intel",
            Self::OLDAPTwoDotFour => "OLDAP-2.4",
            Self::WThreeCTwoZeroOneFiveZeroFiveOneThree => "W3C-20150513",
            Self::MITKhronosOld => "MIT-Khronos-old",
            Self::CDLAPermissiveTwoDotZero => "CDLA-Permissive-2.0",
            Self::CCBYTwoDotZero => "CC-BY-2.0",
            Self::MpiPermissive => "mpi-permissive",
            Self::DeprecatedGPLTwoDotZeroWithAutoconfException => {
                "deprecated_GPL-2.0-with-autoconf-exception"
            }
            Self::MIPS => "MIPS",
            Self::Cube => "Cube",
            Self::OSLOneDotOne => "OSL-1.1",
            Self::MPLTwoDotZeroNoCopyleftException => "MPL-2.0-no-copyleft-exception",
            Self::NLODOneDotZero => "NLOD-1.0",
            Self::GFDLOneDotOneOrLater => "GFDL-1.1-or-later",
            Self::WidgetWorkshop => "Widget-Workshop",
            Self::CATOSLOneDotOne => "CATOSL-1.1",
            Self::CryptsetupOpenSslException => "cryptsetup-OpenSSL-exception",
            Self::BSDSystemics => "BSD-Systemics",
            Self::GCCExceptionTwoDotZeroNote => "GCC-exception-2.0-note",
            Self::BSDSystemicsWThreeWorks => "BSD-Systemics-W3Works",
            Self::OCLCTwoDotZero => "OCLC-2.0",
            Self::LALOneDotTwo => "LAL-1.2",
            Self::XdebugOneDotZeroThree => "Xdebug-1.03",
            Self::FSLOneDotOneALvTwo => "FSL-1.1-ALv2",
            Self::HPNDUCExportUS => "HPND-UC-export-US",
            Self::ECLOneDotZero => "ECL-1.0",
            Self::CCBYNCNDThreeDotZeroIGO => "CC-BY-NC-ND-3.0-IGO",
            Self::OGLCanadaTwoDotZero => "OGL-Canada-2.0",
            Self::LGPLLR => "LGPLLR",
            Self::Fair => "Fair",
            Self::SwiftException => "Swift-exception",
            Self::JasPerTwoDotZero => "JasPer-2.0",
            Self::CECILLOneDotZero => "CECILL-1.0",
            Self::GPLThreeDotZeroLinkingSourceException => "GPL-3.0-linking-source-exception",
            Self::BSDSourceCode => "BSD-Source-Code",
            Self::Metamail => "metamail",
            Self::LLVMException => "LLVM-exception",
            Self::VSLOneDotZero => "VSL-1.0",
            Self::LPPLOneDotThreec => "LPPL-1.3c",
            Self::Furuseth => "Furuseth",
            Self::CNRIJython => "CNRI-Jython",
            Self::LiLiQPOneDotOne => "LiLiQ-P-1.1",
            Self::FergusonTwofish => "Ferguson-Twofish",
            Self::HPNDINRIAIMAG => "HPND-INRIA-IMAG",
            Self::UnicodeDFSTwoZeroOneSix => "Unicode-DFS-2016",
            Self::MITModernVariant => "MIT-Modern-Variant",
            Self::DeprecatedGPLThreeDotZeroPlus => "deprecated_GPL-3.0+",
            Self::HPNDFennebergLivingston => "HPND-Fenneberg-Livingston",
            Self::Wwl => "wwl",
            Self::ClArtistic => "ClArtistic",
            Self::HPNDMarkusKuhn => "HPND-Markus-Kuhn",
            Self::SoftSurfer => "softSurfer",
            Self::CrystalStacker => "CrystalStacker",
            Self::NCBIPD => "NCBI-PD",
            Self::GFDLOneDotTwoInvariantsOrLater => "GFDL-1.2-invariants-or-later",
            Self::DeprecatedECosTwoDotZero => "deprecated_eCos-2.0",
            Self::OLDAPTwoDotFive => "OLDAP-2.5",
            Self::GFDLOneDotOneNoInvariantsOrLater => "GFDL-1.1-no-invariants-or-later",
            Self::CCBYFourDotZero => "CC-BY-4.0",
            Self::OLDAPTwoDotTwo => "OLDAP-2.2",
            Self::CNRIPython => "CNRI-Python",
            Self::BSLOneDotZero => "BSL-1.0",
            Self::SMAILGPL => "SMAIL-GPL",
            Self::TUBerlinOneDotZero => "TU-Berlin-1.0",
            Self::VOSTROM => "VOSTROM",
            Self::LibpngTwoDotZero => "libpng-2.0",
            Self::GnuJavamailException => "gnu-javamail-exception",
            Self::CCBYNCOneDotZero => "CC-BY-NC-1.0",
            Self::GPLThreeDotZeroThreeEightNineDsBaseException => "GPL-3.0-389-ds-base-exception",
            Self::OSLOneDotZero => "OSL-1.0",
            Self::UnicodeThreeDotZero => "Unicode-3.0",
            Self::GFDLOneDotThreeOrLater => "GFDL-1.3-or-later",
            Self::InnoSetup => "InnoSetup",
            Self::CALOneDotZero => "CAL-1.0",
            Self::CCBYSAThreeDotZeroIGO => "CC-BY-SA-3.0-IGO",
            Self::Saxpath => "Saxpath",
            Self::OPUBLOneDotZero => "OPUBL-1.0",
            Self::LinuxManPagesCopyleftTwoPara => "Linux-man-pages-copyleft-2-para",
            Self::CCBYThreeDotZeroAT => "CC-BY-3.0-AT",
            Self::GNATException => "GNAT-exception",
            Self::CERNOHLPTwoDotZero => "CERN-OHL-P-2.0",
            Self::XOneOnevncOpensslException => "x11vnc-openssl-exception",
            Self::FSFUL => "FSFUL",
            Self::OPLUKThreeDotZero => "OPL-UK-3.0",
            Self::UMichMerit => "UMich-Merit",
            Self::CornellLosslessJPEG => "Cornell-Lossless-JPEG",
            Self::OGDLTaiwanOneDotZero => "OGDL-Taiwan-1.0",
            Self::Plexus => "Plexus",
            Self::CCBYNCSATwoDotZeroUK => "CC-BY-NC-SA-2.0-UK",
            Self::SGIBTwoDotZero => "SGI-B-2.0",
            Self::CCBYThreeDotZeroIGO => "CC-BY-3.0-IGO",
            Self::HippocraticTwoDotOne => "Hippocratic-2.1",
            Self::SHLTwoDotOne => "SHL-2.1",
            Self::KiCadLibrariesException => "KiCad-libraries-exception",
            Self::CPOLOneDotZeroTwo => "CPOL-1.02",
            Self::DeprecatedGPLOneDotZero => "deprecated_GPL-1.0",
            Self::Cronyx => "Cronyx",
            Self::LatexTwoeTranslatedNotice => "Latex2e-translated-notice",
            Self::KnuthCTAN => "Knuth-CTAN",
            Self::CCBYThreeDotZero => "CC-BY-3.0",
            Self::BzipTwoOneDotZeroDotSix => "bzip2-1.0.6",
            Self::OCCTPL => "OCCT-PL",
            Self::SendmailEightDotTwoThree => "Sendmail-8.23",
            Self::Catharon => "Catharon",
            Self::IPLOneDotZero => "IPL-1.0",
            Self::DeprecatedGPLTwoDotZeroWithFontException => {
                "deprecated_GPL-2.0-with-font-exception"
            }
            Self::Motosoto => "Motosoto",
            Self::CheckCvs => "check-cvs",
            Self::Sendmail => "Sendmail",
            Self::PPL => "PPL",
            Self::PostgreSql => "PostgreSQL",
            Self::CDLOneDotZero => "CDL-1.0",
            Self::GPLOneDotZeroOrLater => "GPL-1.0-or-later",
            Self::CCSAOneDotZero => "CC-SA-1.0",
            Self::IBMPibs => "IBM-pibs",
            Self::CERNOHLOneDotTwo => "CERN-OHL-1.2",
            Self::IntelACPI => "Intel-ACPI",
            Self::DeprecatedLGPLTwoDotZeroPlus => "deprecated_LGPL-2.0+",
            Self::SPLOneDotZero => "SPL-1.0",
            Self::OML => "OML",
            Self::DOC => "DOC",
            Self::MITClick => "MIT-Click",
            Self::MxmlException => "mxml-exception",
            Self::NCGLUKTwoDotZero => "NCGL-UK-2.0",
            Self::HTMLTIDY => "HTMLTIDY",
            Self::LALOneDotThree => "LAL-1.3",
            Self::BSDInfernoNettverk => "BSD-Inferno-Nettverk",
            Self::RPLOneDotFive => "RPL-1.5",
            Self::CcZeroOneDotZero => "CC0-1.0",
            Self::CECILLOneDotOne => "CECILL-1.1",
            Self::GCRDocs => "GCR-docs",
            Self::Glide => "Glide",
            Self::CCBYSATwoDotZeroUK => "CC-BY-SA-2.0-UK",
            Self::Glulxe => "Glulxe",
            Self::MITZero => "MIT-0",
            Self::DeprecatedLGPLTwoDotOne => "deprecated_LGPL-2.1",
            Self::ECosExceptionTwoDotZero => "eCos-exception-2.0",
            Self::CERNOHLWTwoDotZero => "CERN-OHL-W-2.0",
            Self::Minpack => "Minpack",
            Self::HPNDNetrek => "HPND-Netrek",
            Self::DocBookSchema => "DocBook-Schema",
            Self::ManTwohtml => "man2html",
            Self::LGPLTwoDotOneOrLater => "LGPL-2.1-or-later",
            Self::CMUMach => "CMU-Mach",
            Self::OLDAPTwoDotZero => "OLDAP-2.0",
            Self::PHPThreeDotZero => "PHP-3.0",
            Self::HPNDDEC => "HPND-DEC",
            Self::Radvd => "radvd",
            Self::OGTSL => "OGTSL",
            Self::OLDAPTwoDotSeven => "OLDAP-2.7",
            Self::OGLUKTwoDotZero => "OGL-UK-2.0",
            Self::LGPLThreeDotZeroLinkingException => "LGPL-3.0-linking-exception",
            Self::TtypZero => "TTYP0",
            Self::Zed => "Zed",
            Self::PADL => "PADL",
            Self::OFLOneDotOneRFN => "OFL-1.1-RFN",
            Self::OLDAPTwoDotTwoDotTwo => "OLDAP-2.2.2",
            Self::Diffmark => "diffmark",
            Self::WThreeC => "W3C",
            Self::HPNDSellVariantMITDisclaimer => "HPND-sell-variant-MIT-disclaimer",
            Self::OCamlLGPLLinkingException => "OCaml-LGPL-linking-exception",
            Self::PythonTwoDotZero => "Python-2.0",
            Self::MackerrasThreeClauseAcknowledgment => "Mackerras-3-Clause-acknowledgment",
            Self::CCBYNCNDOneDotZero => "CC-BY-NC-ND-1.0",
            Self::CCBYNDOneDotZero => "CC-BY-ND-1.0",
            Self::Mup => "Mup",
            Self::TPDL => "TPDL",
            Self::HIDAPI => "HIDAPI",
            Self::InfoZIP => "Info-ZIP",
            Self::DeprecatedLGPLThreeDotZero => "deprecated_LGPL-3.0",
            Self::HPOneNineEightNine => "HP-1989",
            Self::FSFULLRWD => "FSFULLRWD",
            Self::Leptonica => "Leptonica",
            Self::GraphicsGems => "Graphics-Gems",
            Self::UnicodeDFSTwoZeroOneFive => "Unicode-DFS-2015",
            Self::TrustedQsl => "TrustedQSL",
            Self::NLPL => "NLPL",
            Self::Giftware => "Giftware",
            Self::SSPLOneDotZero => "SSPL-1.0",
            Self::CCBYTwoDotFiveAU => "CC-BY-2.5-AU",
            Self::HPNDSellVariantMITDisclaimerRev => "HPND-sell-variant-MIT-disclaimer-rev",
            Self::DeprecatedGPLOneDotZeroPlus => "deprecated_GPL-1.0+",
            Self::Libtiff => "libtiff",
            Self::IPA => "IPA",
            Self::CCBYSAOneDotZero => "CC-BY-SA-1.0",
            Self::CCBYNCSAOneDotZero => "CC-BY-NC-SA-1.0",
            Self::ErlPlOneDotOne => "ErlPL-1.1",
            Self::MPLTwoDotZero => "MPL-2.0",
            Self::Pkgconf => "pkgconf",
            Self::BSDProtection => "BSD-Protection",
            Self::GFDLOneDotTwoNoInvariantsOnly => "GFDL-1.2-no-invariants-only",
            Self::EPLOneDotZero => "EPL-1.0",
            Self::DeprecatedLGPLTwoDotOnePlus => "deprecated_LGPL-2.1+",
            Self::ITwopGplJavaException => "i2p-gpl-java-exception",
            Self::DeprecatedGPLThreeDotZeroWithGCCException => {
                "deprecated_GPL-3.0-with-GCC-exception"
            }
            Self::Jam => "Jam",
            Self::DeprecatedAGPLOneDotZero => "deprecated_AGPL-1.0",
            Self::Qhull => "Qhull",
            Self::OpenSslStandalone => "OpenSSL-standalone",
            Self::PcreTwoException => "PCRE2-exception",
            Self::NBPLOneDotZero => "NBPL-1.0",
            Self::MulanPslTwoDotZero => "MulanPSL-2.0",
            Self::CCBYThreeDotZeroUS => "CC-BY-3.0-US",
            Self::LucidaBitmapFonts => "Lucida-Bitmap-Fonts",
            Self::CCBYNCSATwoDotZeroFR => "CC-BY-NC-SA-2.0-FR",
            Self::MITAdvertising => "MIT-advertising",
            Self::HPNDExportUSModify => "HPND-export-US-modify",
            Self::Swrule => "swrule",
            Self::SMLNJ => "SMLNJ",
            Self::MPEGSSG => "MPEG-SSG",
            Self::PolyFormSmallBusinessOneDotZeroDotZero => "PolyForm-Small-Business-1.0.0",
            Self::ZPLTwoDotZero => "ZPL-2.0",
            Self::Xpp => "xpp",
            Self::Nokia => "Nokia",
            Self::HPNDKevlinHenney => "HPND-Kevlin-Henney",
            Self::PolyFormNoncommercialOneDotZeroDotZero => "PolyForm-Noncommercial-1.0.0",
            Self::CCBYSATwoDotOneJP => "CC-BY-SA-2.1-JP",
            Self::XkeyboardConfigZinoviev => "xkeyboard-config-Zinoviev",
            Self::NAISTTwoZeroZeroThree => "NAIST-2003",
            Self::HPNDExportUS => "HPND-export-US",
            Self::GStreamerExceptionTwoZeroZeroEight => "GStreamer-exception-2008",
            Self::DeprecatedBSDTwoClauseNetBsd => "deprecated_BSD-2-Clause-NetBSD",
            Self::WxWindowsExceptionThreeDotOne => "WxWindows-exception-3.1",
            Self::LibselinuxOneDotZero => "libselinux-1.0",
            Self::QPLOneDotZero => "QPL-1.0",
            Self::Pixar => "Pixar",
            Self::FawkesRuntimeException => "Fawkes-Runtime-exception",
            Self::CCPDDC => "CC-PDDC",
            Self::Xerox => "Xerox",
            Self::BUSLOneDotOne => "BUSL-1.1",
            Self::MakeIndex => "MakeIndex",
            Self::LGPLThreeDotZeroOrLater => "LGPL-3.0-or-later",
            Self::DeprecatedNunit => "deprecated_Nunit",
            Self::OFFIS => "OFFIS",
            Self::LatexTwoe => "Latex2e",
            Self::SSHOpenSsh => "SSH-OpenSSH",
            Self::Entessa => "Entessa",
            Self::DeprecatedLGPLTwoDotZero => "deprecated_LGPL-2.0",
            Self::ZendTwoDotZero => "Zend-2.0",
            Self::MifException => "mif-exception",
            Self::Xfig => "Xfig",
            Self::Caldera => "Caldera",
            Self::LibutilDavidNugent => "libutil-David-Nugent",
            Self::OLDAPTwoDotSix => "OLDAP-2.6",
            Self::GNUCompilerException => "GNU-compiler-exception",
            Self::SGIOpenGl => "SGI-OpenGL",
            Self::CCPDMOneDotZero => "CC-PDM-1.0",
            Self::MITEnna => "MIT-enna",
            Self::NISTPD => "NIST-PD",
            Self::CCBYTwoDotFive => "CC-BY-2.5",
            Self::HPNDSellRegexpr => "HPND-sell-regexpr",
            Self::LOOP => "LOOP",
            Self::OLDAPTwoDotOne => "OLDAP-2.1",
            Self::SISSLOneDotTwo => "SISSL-1.2",
            Self::OCCTExceptionOneDotZero => "OCCT-exception-1.0",
            Self::SL => "SL",
            Self::FLTKException => "FLTK-exception",
            Self::GFDLOneDotOneOnly => "GFDL-1.1-only",
            Self::CALOneDotZeroCombinedWorkException => "CAL-1.0-Combined-Work-Exception",
            Self::DeprecatedStandardMlNJ => "deprecated_StandardML-NJ",
            Self::ZPLTwoDotOne => "ZPL-2.1",
            Self::ImlibTwo => "Imlib2",
            Self::RPLOneDotOne => "RPL-1.1",
            Self::Gnuplot => "gnuplot",
            Self::DFSLOneDotZero => "D-FSL-1.0",
            Self::OpenSsl => "OpenSSL",
            Self::GPLThreeDotZeroOrLater => "GPL-3.0-or-later",
            Self::OSETPLTwoDotOne => "OSET-PL-2.1",
            Self::LZMASDKNineDotOneOneToNineDotTwoZero => "LZMA-SDK-9.11-to-9.20",
            Self::SAXPDTwoDotZero => "SAX-PD-2.0",
            Self::NASAOneDotThree => "NASA-1.3",
            Self::EuDatagrid => "EUDatagrid",
            Self::CERNOHLOneDotOne => "CERN-OHL-1.1",
            Self::EtalabTwoDotZero => "etalab-2.0",
            Self::DeprecatedGPLThreeDotZeroWithAutoconfException => {
                "deprecated_GPL-3.0-with-autoconf-exception"
            }
            Self::CERNOHLSTwoDotZero => "CERN-OHL-S-2.0",
            Self::Gutmann => "Gutmann",
            Self::OLFLOneDotThree => "OLFL-1.3",
            Self::LinuxManPagesCopyleftVar => "Linux-man-pages-copyleft-var",
            Self::OGLUKThreeDotZero => "OGL-UK-3.0",
            Self::ParitySixDotZeroDotZero => "Parity-6.0.0",
            Self::LPPLOneDotThreea => "LPPL-1.3a",
            Self::Mplus => "mplus",
            Self::COILOneDotZero => "COIL-1.0",
            Self::Gtkbook => "gtkbook",
            Self::XSkat => "XSkat",
            Self::HPNDDocSell => "HPND-doc-sell",
            Self::RSAMD => "RSA-MD",
            Self::CCBYSAThreeDotZeroAT => "CC-BY-SA-3.0-AT",
            Self::NISTPDFallback => "NIST-PD-fallback",
            Self::IMatix => "iMatix",
            Self::GPLThreeDotZeroOnly => "GPL-3.0-only",
            Self::CryptoSwift => "CryptoSwift",
            Self::WTFPL => "WTFPL",
            Self::CveTou => "cve-tou",
            Self::HPNDMITDisclaimer => "HPND-MIT-disclaimer",
            Self::CCBYNDTwoDotZero => "CC-BY-ND-2.0",
            Self::Vim => "Vim",
            Self::NISTSoftware => "NIST-Software",
            Self::LPPLOneDotZero => "LPPL-1.0",
            Self::YPLOneDotOne => "YPL-1.1",
            Self::CCBYNCNDTwoDotZero => "CC-BY-NC-ND-2.0",
            Self::CopyleftNextZeroDotThreeDotOne => "copyleft-next-0.3.1",
            Self::MITCMU => "MIT-CMU",
            Self::RPSLOneDotZero => "RPSL-1.0",
            Self::Dtoa => "dtoa",
            Self::NCSA => "NCSA",
            Self::NPLOneDotOne => "NPL-1.1",
            Self::SCEA => "SCEA",
            Self::SMPPL => "SMPPL",
            Self::LiLiQROneDotOne => "LiLiQ-R-1.1",
            Self::OFLOneDotZeroRFN => "OFL-1.0-RFN",
            Self::NPOSLThreeDotZero => "NPOSL-3.0",
            Self::ImageMagick => "ImageMagick",
            Self::LibpriOpenHThreeTwoThreeException => "libpri-OpenH323-exception",
            Self::UnicodeTOU => "Unicode-TOU",
            Self::OpenPbsTwoDotThree => "OpenPBS-2.3",
            Self::UCLOneDotZero => "UCL-1.0",
            Self::ZimbraOneDotFour => "Zimbra-1.4",
            Self::TGPPLOneDotZero => "TGPPL-1.0",
            Self::Wsuipa => "Wsuipa",
            Self::DeprecatedNokiaQtExceptionOneDotOne => "deprecated_Nokia-Qt-exception-1.1",
            Self::CCBYNCSATwoDotZeroDE => "CC-BY-NC-SA-2.0-DE",
            Self::LinuxManPagesCopyleft => "Linux-man-pages-copyleft",
            Self::Xlock => "xlock",
            Self::LiLiQRplusOneDotOne => "LiLiQ-Rplus-1.1",
            Self::GenericXts => "generic-xts",
            Self::ZimbraOneDotThree => "Zimbra-1.3",
            Self::GPLTwoDotZeroOnly => "GPL-2.0-only",
            Self::OGLUKOneDotZero => "OGL-UK-1.0",
            Self::SsLeayStandalone => "SSLeay-standalone",
            Self::XOneOneSwapped => "X11-swapped",
            Self::OpenJdkAssemblyExceptionOneDotZero => "OpenJDK-assembly-exception-1.0",
            Self::GFDLOneDotThreeNoInvariantsOrLater => "GFDL-1.3-no-invariants-or-later",
            Self::MTLL => "MTLL",
            Self::UbuntuFontOneDotZero => "Ubuntu-font-1.0",
            Self::DocBookXML => "DocBook-XML",
            Self::MITTestregex => "MIT-testregex",
            Self::CCBYNCTwoDotFive => "CC-BY-NC-2.5",
            Self::PythonLdap => "python-ldap",
            Self::GlTwoPs => "GL2PS",
            Self::LPLOneDotZeroTwo => "LPL-1.02",
            Self::MITNFA => "MITNFA",
            Self::Checkmk => "checkmk",
            Self::GFDLOneDotTwoOnly => "GFDL-1.2-only",
            Self::NGPL => "NGPL",
            Self::MulanPslOneDotZero => "MulanPSL-1.0",
            Self::DeprecatedWxWindows => "deprecated_wxWindows",
            Self::OGCOneDotZero => "OGC-1.0",
            Self::Ulem => "ulem",
            Self::HarbourException => "harbour-exception",
            Self::UCAR => "UCAR",
            Self::MSPL => "MS-PL",
            Self::JPLImage => "JPL-image",
            Self::FontExceptionTwoDotZero => "Font-exception-2.0",
            Self::GFDLOneDotThreeInvariantsOrLater => "GFDL-1.3-invariants-or-later",
            Self::Fwlw => "fwlw",
            Self::InnerNetTwoDotZero => "Inner-Net-2.0",
            Self::MPLOneDotZero => "MPL-1.0",
            Self::CommunitySpecOneDotZero => "Community-Spec-1.0",
            Self::CUAOPLOneDotZero => "CUA-OPL-1.0",
            Self::UBDLException => "UBDL-exception",
            Self::GFDLOneDotOneInvariantsOnly => "GFDL-1.1-invariants-only",
            Self::FreeBsdDOC => "FreeBSD-DOC",
            Self::EPLTwoDotZero => "EPL-2.0",
            Self::SendmailOpenSourceOneDotOne => "Sendmail-Open-Source-1.1",
            Self::Eurosym => "Eurosym",
            Self::GPLOneDotZeroOnly => "GPL-1.0-only",
            Self::DeprecatedBSDTwoClauseFreeBsd => "deprecated_BSD-2-Clause-FreeBSD",
            Self::SHLZeroDotFive => "SHL-0.5",
            Self::XOneOne => "X11",
            Self::ThirdEye => "ThirdEye",
            Self::FSFAPNoWarrantyDisclaimer => "FSFAP-no-warranty-disclaimer",
            Self::SimPlTwoDotZero => "SimPL-2.0",
            Self::InterbaseOneDotZero => "Interbase-1.0",
            Self::DeprecatedBzipTwoOneDotZeroDotFive => "deprecated_bzip2-1.0.5",
            Self::Multics => "Multics",
            Self::GStreamerExceptionTwoZeroZeroFive => "GStreamer-exception-2005",
            Self::CCBYNCSATwoDotZero => "CC-BY-NC-SA-2.0",
            Self::GD => "GD",
            Self::CCBYSATwoDotZero => "CC-BY-SA-2.0",
            Self::CECILLC => "CECILL-C",
            Self::ElasticTwoDotZero => "Elastic-2.0",
            Self::MITWu => "MIT-Wu",
            Self::Snprintf => "snprintf",
            Self::EUPLOneDotZero => "EUPL-1.0",
            Self::BSDSourceBeginningFile => "BSD-Source-beginning-file",
            Self::SunPro => "SunPro",
            Self::HPNDIntel => "HPND-Intel",
            Self::LZMASDKNineDotTwoTwo => "LZMA-SDK-9.22",
            Self::OFLOneDotZero => "OFL-1.0",
            Self::DeprecatedGPLTwoDotZeroWithGCCException => {
                "deprecated_GPL-2.0-with-GCC-exception"
            }
            Self::CCBYSAThreeDotZero => "CC-BY-SA-3.0",
            Self::CCBYNCSAThreeDotZero => "CC-BY-NC-SA-3.0",
            Self::DSDP => "DSDP",
            Self::McPheeSlideshow => "McPhee-slideshow",
            Self::CCBYThreeDotZeroNL => "CC-BY-3.0-NL",
            Self::OLDAPOneDotOne => "OLDAP-1.1",
            Self::GFDLOneDotTwoInvariantsOnly => "GFDL-1.2-invariants-only",
            Self::SchemeReport => "SchemeReport",
            Self::HPNDExportTwoUS => "HPND-export2-US",
            Self::Xinetd => "xinetd",
            Self::HaskellReport => "HaskellReport",
            Self::TCL => "TCL",
            Self::NetCdf => "NetCDF",
            Self::Symlinks => "Symlinks",
            Self::OpenvpnOpensslException => "openvpn-openssl-exception",
            Self::HPNDPbmplus => "HPND-Pbmplus",
            Self::OAR => "OAR",
            Self::CopyleftNextZeroDotThreeDotZero => "copyleft-next-0.3.0",
            Self::YPLOneDotZero => "YPL-1.0",
            Self::LPPLOneDotOne => "LPPL-1.1",
            Self::CCBYNDFourDotZero => "CC-BY-ND-4.0",
            Self::XFreeEightSixOneDotOne => "XFree86-1.1",
            Self::CCBYNCNDFourDotZero => "CC-BY-NC-ND-4.0",
            Self::CCBYNCSAThreeDotZeroDE => "CC-BY-NC-SA-3.0-DE",
            Self::TPLOneDotZero => "TPL-1.0",
            Self::Naumen => "Naumen",
            Self::NICTAOneDotZero => "NICTA-1.0",
            Self::NOSL => "NOSL",
            Self::Pnmstitch => "pnmstitch",
            Self::CPLOneDotZero => "CPL-1.0",
            Self::Xzoom => "xzoom",
            Self::IECCodeComponentsEULA => "IEC-Code-Components-EULA",
            Self::DeprecatedAGPLThreeDotZero => "deprecated_AGPL-3.0",
            Self::RSCPL => "RSCPL",
            Self::NPLOneDotZero => "NPL-1.0",
            Self::Sleepycat => "Sleepycat",
            Self::CDLASharingOneDotZero => "CDLA-Sharing-1.0",
            Self::GFDLOneDotThreeOnly => "GFDL-1.3-only",
            Self::Lsof => "lsof",
            Self::ParitySevenDotZeroDotZero => "Parity-7.0.0",
            Self::Zeeff => "Zeeff",
            Self::CCBYNCSAFourDotZero => "CC-BY-NC-SA-4.0",
            Self::CCBYSAFourDotZero => "CC-BY-SA-4.0",
            Self::GFDLOneDotTwoOrLater => "GFDL-1.2-or-later",
            Self::OFLOneDotOne => "OFL-1.1",
            Self::GPLThreeDotZeroInterfaceException => "GPL-3.0-interface-exception",
            Self::QtLGPLExceptionOneDotOne => "Qt-LGPL-exception-1.1",
            Self::MackerrasThreeClause => "Mackerras-3-Clause",
            Self::EUPLOneDotOne => "EUPL-1.1",
            Self::SunPPP => "Sun-PPP",
            Self::CECILLB => "CECILL-B",
            Self::LinuxOpenIb => "Linux-OpenIB",
            Self::FmtException => "fmt-exception",
            Self::MITFeh => "MIT-feh",
            Self::RubyPty => "Ruby-pty",
            Self::HPNDSellVariant => "HPND-sell-variant",
            Self::DECThreeClause => "DEC-3-Clause",
            Self::ZPLOneDotOne => "ZPL-1.1",
            Self::Zlib => "Zlib",
            Self::FTL => "FTL",
            Self::CCBYSAThreeDotZeroDE => "CC-BY-SA-3.0-DE",
            Self::CCBYNCNDThreeDotZero => "CC-BY-NC-ND-3.0",
            Self::TOSL => "TOSL",
            Self::FBM => "FBM",
            Self::CCBYNDThreeDotZero => "CC-BY-ND-3.0",
            Self::CUDAOneDotZero => "C-UDA-1.0",
            Self::QtGPLExceptionOneDotZero => "Qt-GPL-exception-1.0",
            Self::SunPPPTwoZeroZeroZero => "Sun-PPP-2000",
            Self::OpenVision => "OpenVision",
            Self::DeprecatedGFDLOneDotOne => "deprecated_GFDL-1.1",
            Self::SshKeyscan => "ssh-keyscan",
            Self::SSHShort => "SSH-short",
            Self::GSoapOneDotThreeb => "gSOAP-1.3b",
            Self::JPNIC => "JPNIC",
            Self::GFDLOneDotOneNoInvariantsOnly => "GFDL-1.1-no-invariants-only",
            Self::TAPROHLOneDotZero => "TAPR-OHL-1.0",
            Self::Rdisc => "Rdisc",
            Self::HPNDDoc => "HPND-doc",
            Self::URTRLE => "URT-RLE",
            Self::MpichTwo => "mpich2",
            Self::CGALLinkingException => "CGAL-linking-exception",
            Self::MPLOneDotOne => "MPL-1.1",
        }
    }
    #[doc = r" Returns the embedded template content for the license."]
    #[doc = r" The content will be from `.template.txt` if available, otherwise `.txt`."]
    pub fn template_content(&self) -> &'static str {
        match self {
            Self::LinuxSyscallNote => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Linux-syscall-note",
                ".template.txt"
            )),
            Self::DocBookStylesheet => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DocBook-Stylesheet",
                ".template.txt"
            )),
            Self::HPNDSellMITDisclaimerXserver => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-sell-MIT-disclaimer-xserver",
                ".template.txt"
            )),
            Self::OSLTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OSL-2.1",
                ".template.txt"
            )),
            Self::SWL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SWL",
                ".template.txt"
            )),
            Self::CCBYNCFourDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-4.0",
                ".template.txt"
            )),
            Self::NLODTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NLOD-2.0",
                ".template.txt"
            )),
            Self::Unlicense => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Unlicense",
                ".template.txt"
            )),
            Self::ODCByOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ODC-By-1.0",
                ".template.txt"
            )),
            Self::UPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "UPL-1.0",
                ".template.txt"
            )),
            Self::SNIA => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SNIA",
                ".template.txt"
            )),
            Self::GCCExceptionThreeDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GCC-exception-3.1",
                ".template.txt"
            )),
            Self::OLDAPOneDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-1.3",
                ".template.txt"
            )),
            Self::Dotseqn => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Dotseqn",
                ".template.txt"
            )),
            Self::TMate => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TMate",
                ".template.txt"
            )),
            Self::GFDLOneDotOneInvariantsOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.1-invariants-or-later",
                ".template.txt"
            )),
            Self::MmiXware => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MMIXware",
                ".template.txt"
            )),
            Self::Crossword => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Crossword",
                ".template.txt"
            )),
            Self::OLDAPOneDotFour => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-1.4",
                ".template.txt"
            )),
            Self::GLWTPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GLWTPL",
                ".template.txt"
            )),
            Self::Curl => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "curl",
                ".template.txt"
            )),
            Self::GPLTwoDotZeroOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-2.0-or-later",
                ".template.txt"
            )),
            Self::OFLOneDotOneNoRFN => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OFL-1.1-no-RFN",
                ".template.txt"
            )),
            Self::MSLPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MS-LPL",
                ".template.txt"
            )),
            Self::CCBYOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-1.0",
                ".template.txt"
            )),
            Self::CDLAPermissiveOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CDLA-Permissive-1.0",
                ".template.txt"
            )),
            Self::QPLOneDotZeroINRIATwoZeroZeroFourException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "QPL-1.0-INRIA-2004-exception",
                ".template.txt"
            )),
            Self::ClasspathExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Classpath-exception-2.0",
                ".template.txt"
            )),
            Self::OPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OPL-1.0",
                ".template.txt"
            )),
            Self::ODbLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ODbL-1.0",
                ".template.txt"
            )),
            Self::IndependentModulesException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Independent-modules-exception",
                ".template.txt"
            )),
            Self::MITFestival => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-Festival",
                ".template.txt"
            )),
            Self::CalderaNoPreamble => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Caldera-no-preamble",
                ".template.txt"
            )),
            Self::SGIBOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SGI-B-1.1",
                ".template.txt"
            )),
            Self::Soundex => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Soundex",
                ".template.txt"
            )),
            Self::SANEException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SANE-exception",
                ".template.txt"
            )),
            Self::MirOs => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MirOS",
                ".template.txt"
            )),
            Self::Ruby => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Ruby",
                ".template.txt"
            )),
            Self::PolyparseException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "polyparse-exception",
                ".template.txt"
            )),
            Self::EFLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EFL-2.0",
                ".template.txt"
            )),
            Self::LLGPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LLGPL",
                ".template.txt"
            )),
            Self::VsftpdOpensslException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "vsftpd-openssl-exception",
                ".template.txt"
            )),
            Self::OFLOneDotZeroNoRFN => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OFL-1.0-no-RFN",
                ".template.txt"
            )),
            Self::CCBYNCNDThreeDotZeroDE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-ND-3.0-DE",
                ".template.txt"
            )),
            Self::CDDLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CDDL-1.0",
                ".template.txt"
            )),
            Self::SWIException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SWI-exception",
                ".template.txt"
            )),
            Self::GFDLOneDotThreeInvariantsOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.3-invariants-only",
                ".template.txt"
            )),
            Self::MSRL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MS-RL",
                ".template.txt"
            )),
            Self::LGPLTwoDotOneOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPL-2.1-only",
                ".template.txt"
            )),
            Self::HPOneNineEightSix => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HP-1986",
                ".template.txt"
            )),
            Self::PythonTwoDotZeroDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Python-2.0.1",
                ".template.txt"
            )),
            Self::RHeCosOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "RHeCos-1.1",
                ".template.txt"
            )),
            Self::MIT => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT",
                ".template.txt"
            )),
            Self::RomicException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "romic-exception",
                ".template.txt"
            )),
            Self::OSLThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OSL-3.0",
                ".template.txt"
            )),
            Self::CCBYNCThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-3.0",
                ".template.txt"
            )),
            Self::GCCExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GCC-exception-2.0",
                ".template.txt"
            )),
            Self::DeprecatedLGPLThreeDotZeroPlus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_LGPL-3.0+",
                ".template.txt"
            )),
            Self::Clips => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Clips",
                ".template.txt"
            )),
            Self::CNRIPythonGPLCompatible => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CNRI-Python-GPL-Compatible",
                ".template.txt"
            )),
            Self::LPDDocument => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPD-document",
                ".template.txt"
            )),
            Self::GPLCCOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-CC-1.0",
                ".template.txt"
            )),
            Self::HPNDUC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-UC",
                ".template.txt"
            )),
            Self::ISCVeillard => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ISC-Veillard",
                ".template.txt"
            )),
            Self::PDDLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PDDL-1.0",
                ".template.txt"
            )),
            Self::LGPLTwoDotZeroOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPL-2.0-or-later",
                ".template.txt"
            )),
            Self::CCBYNCSAThreeDotZeroIGO => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-3.0-IGO",
                ".template.txt"
            )),
            Self::TORQUEOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TORQUE-1.1",
                ".template.txt"
            )),
            Self::FrameworxOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Frameworx-1.0",
                ".template.txt"
            )),
            Self::OLDAPTwoDotEight => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.8",
                ".template.txt"
            )),
            Self::DeprecatedGPLThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-3.0",
                ".template.txt"
            )),
            Self::QwtExceptionOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Qwt-exception-1.0",
                ".template.txt"
            )),
            Self::DocBookDTD => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DocBook-DTD",
                ".template.txt"
            )),
            Self::Psfrag => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "psfrag",
                ".template.txt"
            )),
            Self::IJGShort => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "IJG-short",
                ".template.txt"
            )),
            Self::Psutils => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "psutils",
                ".template.txt"
            )),
            Self::DRLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DRL-1.1",
                ".template.txt"
            )),
            Self::CECILLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CECILL-2.0",
                ".template.txt"
            )),
            Self::WThreeCOneNineNineEightZeroSevenTwoZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "W3C-19980720",
                ".template.txt"
            )),
            Self::OUDAOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "O-UDA-1.0",
                ".template.txt"
            )),
            Self::CLISPExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CLISP-exception-2.0",
                ".template.txt"
            )),
            Self::DLDEZEROTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DL-DE-ZERO-2.0",
                ".template.txt"
            )),
            Self::DeprecatedGFDLOneDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GFDL-1.3",
                ".template.txt"
            )),
            Self::SgpFour => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SGP4",
                ".template.txt"
            )),
            Self::FDKAAC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FDK-AAC",
                ".template.txt"
            )),
            Self::TTWL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TTWL",
                ".template.txt"
            )),
            Self::MartinBirgmeier => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Martin-Birgmeier",
                ".template.txt"
            )),
            Self::CCBYNDThreeDotZeroDE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-ND-3.0-DE",
                ".template.txt"
            )),
            Self::ECLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ECL-2.0",
                ".template.txt"
            )),
            Self::ICU => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ICU",
                ".template.txt"
            )),
            Self::StunnelException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "stunnel-exception",
                ".template.txt"
            )),
            Self::GameProgrammingGems => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Game-Programming-Gems",
                ".template.txt"
            )),
            Self::SGIBOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SGI-B-1.0",
                ".template.txt"
            )),
            Self::NRL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NRL",
                ".template.txt"
            )),
            Self::Kazlib => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Kazlib",
                ".template.txt"
            )),
            Self::PHPThreeDotZeroOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PHP-3.01",
                ".template.txt"
            )),
            Self::Magaz => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "magaz",
                ".template.txt"
            )),
            Self::CPALOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CPAL-1.0",
                ".template.txt"
            )),
            Self::RrDtoolFLOSSExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "RRDtool-FLOSS-exception-2.0",
                ".template.txt"
            )),
            Self::LibtoolException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Libtool-exception",
                ".template.txt"
            )),
            Self::PSOrPDFFontExceptionTwoZeroOneSevenZeroEightOneSeven => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PS-or-PDF-font-exception-20170817",
                ".template.txt"
            )),
            Self::DeprecatedGPLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-2.0",
                ".template.txt"
            )),
            Self::LGPLThreeDotZeroOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPL-3.0-only",
                ".template.txt"
            )),
            Self::DigiRuleFOSSException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DigiRule-FOSS-exception",
                ".template.txt"
            )),
            Self::GPLThreeDotZeroLinkingException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-3.0-linking-exception",
                ".template.txt"
            )),
            Self::Mailprio => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "mailprio",
                ".template.txt"
            )),
            Self::LPPLOneDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPPL-1.2",
                ".template.txt"
            )),
            Self::CDDLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CDDL-1.1",
                ".template.txt"
            )),
            Self::HPND => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND",
                ".template.txt"
            )),
            Self::NTPZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NTP-0",
                ".template.txt"
            )),
            Self::CCBYNDTwoDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-ND-2.5",
                ".template.txt"
            )),
            Self::CCBYNCNDTwoDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-ND-2.5",
                ".template.txt"
            )),
            Self::FreertosExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "freertos-exception-2.0",
                ".template.txt"
            )),
            Self::DeprecatedGPLTwoDotZeroWithBisonException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-2.0-with-bison-exception",
                ".template.txt"
            )),
            Self::SugarCrmOneDotOneDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SugarCRM-1.1.3",
                ".template.txt"
            )),
            Self::SISSL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SISSL",
                ".template.txt"
            )),
            Self::CFITSIO => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CFITSIO",
                ".template.txt"
            )),
            Self::TUBerlinTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TU-Berlin-2.0",
                ".template.txt"
            )),
            Self::CMUMachNodoc => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CMU-Mach-nodoc",
                ".template.txt"
            )),
            Self::CondorOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Condor-1.1",
                ".template.txt"
            )),
            Self::OSLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OSL-2.0",
                ".template.txt"
            )),
            Self::CCBYNCTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-2.0",
                ".template.txt"
            )),
            Self::TermReadKey => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TermReadKey",
                ".template.txt"
            )),
            Self::UnixCrypt => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "UnixCrypt",
                ".template.txt"
            )),
            Self::NTP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NTP",
                ".template.txt"
            )),
            Self::CCBYThreeDotZeroDE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-3.0-DE",
                ".template.txt"
            )),
            Self::Kastrup => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Kastrup",
                ".template.txt"
            )),
            Self::LGPLTwoDotZeroOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPL-2.0-only",
                ".template.txt"
            )),
            Self::OLDAPOneDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-1.2",
                ".template.txt"
            )),
            Self::LinuxManPagesOnePara => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Linux-man-pages-1-para",
                ".template.txt"
            )),
            Self::DLDEBYTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DL-DE-BY-2.0",
                ".template.txt"
            )),
            Self::CCBYNCThreeDotZeroDE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-3.0-DE",
                ".template.txt"
            )),
            Self::FreeImage => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FreeImage",
                ".template.txt"
            )),
            Self::CECILLTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CECILL-2.1",
                ".template.txt"
            )),
            Self::DRLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DRL-1.0",
                ".template.txt"
            )),
            Self::HPNDExportUSAcknowledgement => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-export-US-acknowledgement",
                ".template.txt"
            )),
            Self::OLDAPTwoDotZeroDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.0.1",
                ".template.txt"
            )),
            Self::Threeparttable => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "threeparttable",
                ".template.txt"
            )),
            Self::MITOpenGroup => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-open-group",
                ".template.txt"
            )),
            Self::FSFULLR => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FSFULLR",
                ".template.txt"
            )),
            Self::NCL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NCL",
                ".template.txt"
            )),
            Self::EPICS => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EPICS",
                ".template.txt"
            )),
            Self::JSON => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "JSON",
                ".template.txt"
            )),
            Self::DeprecatedGFDLOneDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GFDL-1.2",
                ".template.txt"
            )),
            Self::SpencerEightSix => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Spencer-86",
                ".template.txt"
            )),
            Self::SHLZeroDotFiveOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SHL-0.51",
                ".template.txt"
            )),
            Self::IJG => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "IJG",
                ".template.txt"
            )),
            Self::DeprecatedGPLTwoDotZeroPlus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-2.0+",
                ".template.txt"
            )),
            Self::ErlangOtpLinkingException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "erlang-otp-linking-exception",
                ".template.txt"
            )),
            Self::SAXPD => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SAX-PD",
                ".template.txt"
            )),
            Self::PSFTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PSF-2.0",
                ".template.txt"
            )),
            Self::QPLOneDotZeroINRIATwoZeroZeroFour => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "QPL-1.0-INRIA-2004",
                ".template.txt"
            )),
            Self::DeprecatedGPLTwoDotZeroWithClasspathException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-2.0-with-classpath-exception",
                ".template.txt"
            )),
            Self::EUPLOneDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EUPL-1.2",
                ".template.txt"
            )),
            Self::GNOMEExamplesException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GNOME-examples-exception",
                ".template.txt"
            )),
            Self::XOneOneDistributeModificationsVariant => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "X11-distribute-modifications-variant",
                ".template.txt"
            )),
            Self::LPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPL-1.0",
                ".template.txt"
            )),
            Self::SpencerNineFour => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Spencer-94",
                ".template.txt"
            )),
            Self::CCBYNCSATwoDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-2.5",
                ".template.txt"
            )),
            Self::UBootExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "u-boot-exception-2.0",
                ".template.txt"
            )),
            Self::CCBYSATwoDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-2.5",
                ".template.txt"
            )),
            Self::Newsletr => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Newsletr",
                ".template.txt"
            )),
            Self::Noweb => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Noweb",
                ".template.txt"
            )),
            Self::WThreem => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "w3m",
                ".template.txt"
            )),
            Self::FSLOneDotOneMIT => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FSL-1.1-MIT",
                ".template.txt"
            )),
            Self::TexinfoException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Texinfo-exception",
                ".template.txt"
            )),
            Self::GFDLOneDotTwoNoInvariantsOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.2-no-invariants-or-later",
                ".template.txt"
            )),
            Self::EFLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EFL-1.0",
                ".template.txt"
            )),
            Self::WatcomOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Watcom-1.0",
                ".template.txt"
            )),
            Self::CCBYThreeDotZeroAU => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-3.0-AU",
                ".template.txt"
            )),
            Self::ISC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ISC",
                ".template.txt"
            )),
            Self::ZlibAcknowledgement => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "zlib-acknowledgement",
                ".template.txt"
            )),
            Self::TCPWrappers => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TCP-wrappers",
                ".template.txt"
            )),
            Self::UniversalFOSSExceptionOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Universal-FOSS-exception-1.0",
                ".template.txt"
            )),
            Self::Hdparm => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "hdparm",
                ".template.txt"
            )),
            Self::SHLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SHL-2.0",
                ".template.txt"
            )),
            Self::OLDAPTwoDotTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.2.1",
                ".template.txt"
            )),
            Self::GFDLOneDotThreeNoInvariantsOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.3-no-invariants-only",
                ".template.txt"
            )),
            Self::EGenix => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "eGenix",
                ".template.txt"
            )),
            Self::FSFAP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FSFAP",
                ".template.txt"
            )),
            Self::SpencerNineNine => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Spencer-99",
                ".template.txt"
            )),
            Self::OLDAPTwoDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.3",
                ".template.txt"
            )),
            Self::GmshException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Gmsh-exception",
                ".template.txt"
            )),
            Self::Dvipdfm => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "dvipdfm",
                ".template.txt"
            )),
            Self::DeprecatedNetSNMP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_Net-SNMP",
                ".template.txt"
            )),
            Self::LZMAException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LZMA-exception",
                ".template.txt"
            )),
            Self::Libpng => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Libpng",
                ".template.txt"
            )),
            Self::Xnet => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Xnet",
                ".template.txt"
            )),
            Self::HPNDMerchantabilityVariant => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-merchantability-variant",
                ".template.txt"
            )),
            Self::Intel => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Intel",
                ".template.txt"
            )),
            Self::OLDAPTwoDotFour => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.4",
                ".template.txt"
            )),
            Self::WThreeCTwoZeroOneFiveZeroFiveOneThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "W3C-20150513",
                ".template.txt"
            )),
            Self::MITKhronosOld => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-Khronos-old",
                ".template.txt"
            )),
            Self::CDLAPermissiveTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CDLA-Permissive-2.0",
                ".template.txt"
            )),
            Self::CCBYTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-2.0",
                ".template.txt"
            )),
            Self::MpiPermissive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "mpi-permissive",
                ".template.txt"
            )),
            Self::DeprecatedGPLTwoDotZeroWithAutoconfException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-2.0-with-autoconf-exception",
                ".template.txt"
            )),
            Self::MIPS => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIPS",
                ".template.txt"
            )),
            Self::Cube => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Cube",
                ".template.txt"
            )),
            Self::OSLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OSL-1.1",
                ".template.txt"
            )),
            Self::MPLTwoDotZeroNoCopyleftException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MPL-2.0-no-copyleft-exception",
                ".template.txt"
            )),
            Self::NLODOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NLOD-1.0",
                ".template.txt"
            )),
            Self::GFDLOneDotOneOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.1-or-later",
                ".template.txt"
            )),
            Self::WidgetWorkshop => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Widget-Workshop",
                ".template.txt"
            )),
            Self::CATOSLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CATOSL-1.1",
                ".template.txt"
            )),
            Self::CryptsetupOpenSslException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "cryptsetup-OpenSSL-exception",
                ".template.txt"
            )),
            Self::BSDSystemics => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BSD-Systemics",
                ".template.txt"
            )),
            Self::GCCExceptionTwoDotZeroNote => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GCC-exception-2.0-note",
                ".template.txt"
            )),
            Self::BSDSystemicsWThreeWorks => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BSD-Systemics-W3Works",
                ".template.txt"
            )),
            Self::OCLCTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OCLC-2.0",
                ".template.txt"
            )),
            Self::LALOneDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LAL-1.2",
                ".template.txt"
            )),
            Self::XdebugOneDotZeroThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Xdebug-1.03",
                ".template.txt"
            )),
            Self::FSLOneDotOneALvTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FSL-1.1-ALv2",
                ".template.txt"
            )),
            Self::HPNDUCExportUS => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-UC-export-US",
                ".template.txt"
            )),
            Self::ECLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ECL-1.0",
                ".template.txt"
            )),
            Self::CCBYNCNDThreeDotZeroIGO => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-ND-3.0-IGO",
                ".template.txt"
            )),
            Self::OGLCanadaTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OGL-Canada-2.0",
                ".template.txt"
            )),
            Self::LGPLLR => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPLLR",
                ".template.txt"
            )),
            Self::Fair => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Fair",
                ".template.txt"
            )),
            Self::SwiftException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Swift-exception",
                ".template.txt"
            )),
            Self::JasPerTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "JasPer-2.0",
                ".template.txt"
            )),
            Self::CECILLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CECILL-1.0",
                ".template.txt"
            )),
            Self::GPLThreeDotZeroLinkingSourceException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-3.0-linking-source-exception",
                ".template.txt"
            )),
            Self::BSDSourceCode => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BSD-Source-Code",
                ".template.txt"
            )),
            Self::Metamail => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "metamail",
                ".template.txt"
            )),
            Self::LLVMException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LLVM-exception",
                ".template.txt"
            )),
            Self::VSLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "VSL-1.0",
                ".template.txt"
            )),
            Self::LPPLOneDotThreec => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPPL-1.3c",
                ".template.txt"
            )),
            Self::Furuseth => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Furuseth",
                ".template.txt"
            )),
            Self::CNRIJython => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CNRI-Jython",
                ".template.txt"
            )),
            Self::LiLiQPOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LiLiQ-P-1.1",
                ".template.txt"
            )),
            Self::FergusonTwofish => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Ferguson-Twofish",
                ".template.txt"
            )),
            Self::HPNDINRIAIMAG => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-INRIA-IMAG",
                ".template.txt"
            )),
            Self::UnicodeDFSTwoZeroOneSix => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Unicode-DFS-2016",
                ".template.txt"
            )),
            Self::MITModernVariant => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-Modern-Variant",
                ".template.txt"
            )),
            Self::DeprecatedGPLThreeDotZeroPlus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-3.0+",
                ".template.txt"
            )),
            Self::HPNDFennebergLivingston => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-Fenneberg-Livingston",
                ".template.txt"
            )),
            Self::Wwl => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "wwl",
                ".template.txt"
            )),
            Self::ClArtistic => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ClArtistic",
                ".template.txt"
            )),
            Self::HPNDMarkusKuhn => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-Markus-Kuhn",
                ".template.txt"
            )),
            Self::SoftSurfer => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "softSurfer",
                ".template.txt"
            )),
            Self::CrystalStacker => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CrystalStacker",
                ".template.txt"
            )),
            Self::NCBIPD => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NCBI-PD",
                ".template.txt"
            )),
            Self::GFDLOneDotTwoInvariantsOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.2-invariants-or-later",
                ".template.txt"
            )),
            Self::DeprecatedECosTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_eCos-2.0",
                ".template.txt"
            )),
            Self::OLDAPTwoDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.5",
                ".template.txt"
            )),
            Self::GFDLOneDotOneNoInvariantsOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.1-no-invariants-or-later",
                ".template.txt"
            )),
            Self::CCBYFourDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-4.0",
                ".template.txt"
            )),
            Self::OLDAPTwoDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.2",
                ".template.txt"
            )),
            Self::CNRIPython => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CNRI-Python",
                ".template.txt"
            )),
            Self::BSLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BSL-1.0",
                ".template.txt"
            )),
            Self::SMAILGPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SMAIL-GPL",
                ".template.txt"
            )),
            Self::TUBerlinOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TU-Berlin-1.0",
                ".template.txt"
            )),
            Self::VOSTROM => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "VOSTROM",
                ".template.txt"
            )),
            Self::LibpngTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "libpng-2.0",
                ".template.txt"
            )),
            Self::GnuJavamailException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "gnu-javamail-exception",
                ".template.txt"
            )),
            Self::CCBYNCOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-1.0",
                ".template.txt"
            )),
            Self::GPLThreeDotZeroThreeEightNineDsBaseException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-3.0-389-ds-base-exception",
                ".template.txt"
            )),
            Self::OSLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OSL-1.0",
                ".template.txt"
            )),
            Self::UnicodeThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Unicode-3.0",
                ".template.txt"
            )),
            Self::GFDLOneDotThreeOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.3-or-later",
                ".template.txt"
            )),
            Self::InnoSetup => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "InnoSetup",
                ".template.txt"
            )),
            Self::CALOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CAL-1.0",
                ".template.txt"
            )),
            Self::CCBYSAThreeDotZeroIGO => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-3.0-IGO",
                ".template.txt"
            )),
            Self::Saxpath => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Saxpath",
                ".template.txt"
            )),
            Self::OPUBLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OPUBL-1.0",
                ".template.txt"
            )),
            Self::LinuxManPagesCopyleftTwoPara => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Linux-man-pages-copyleft-2-para",
                ".template.txt"
            )),
            Self::CCBYThreeDotZeroAT => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-3.0-AT",
                ".template.txt"
            )),
            Self::GNATException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GNAT-exception",
                ".template.txt"
            )),
            Self::CERNOHLPTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CERN-OHL-P-2.0",
                ".template.txt"
            )),
            Self::XOneOnevncOpensslException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "x11vnc-openssl-exception",
                ".template.txt"
            )),
            Self::FSFUL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FSFUL",
                ".template.txt"
            )),
            Self::OPLUKThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OPL-UK-3.0",
                ".template.txt"
            )),
            Self::UMichMerit => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "UMich-Merit",
                ".template.txt"
            )),
            Self::CornellLosslessJPEG => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Cornell-Lossless-JPEG",
                ".template.txt"
            )),
            Self::OGDLTaiwanOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OGDL-Taiwan-1.0",
                ".template.txt"
            )),
            Self::Plexus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Plexus",
                ".template.txt"
            )),
            Self::CCBYNCSATwoDotZeroUK => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-2.0-UK",
                ".template.txt"
            )),
            Self::SGIBTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SGI-B-2.0",
                ".template.txt"
            )),
            Self::CCBYThreeDotZeroIGO => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-3.0-IGO",
                ".template.txt"
            )),
            Self::HippocraticTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Hippocratic-2.1",
                ".template.txt"
            )),
            Self::SHLTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SHL-2.1",
                ".template.txt"
            )),
            Self::KiCadLibrariesException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "KiCad-libraries-exception",
                ".template.txt"
            )),
            Self::CPOLOneDotZeroTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CPOL-1.02",
                ".template.txt"
            )),
            Self::DeprecatedGPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-1.0",
                ".template.txt"
            )),
            Self::Cronyx => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Cronyx",
                ".template.txt"
            )),
            Self::LatexTwoeTranslatedNotice => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Latex2e-translated-notice",
                ".template.txt"
            )),
            Self::KnuthCTAN => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Knuth-CTAN",
                ".template.txt"
            )),
            Self::CCBYThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-3.0",
                ".template.txt"
            )),
            Self::BzipTwoOneDotZeroDotSix => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "bzip2-1.0.6",
                ".template.txt"
            )),
            Self::OCCTPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OCCT-PL",
                ".template.txt"
            )),
            Self::SendmailEightDotTwoThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Sendmail-8.23",
                ".template.txt"
            )),
            Self::Catharon => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Catharon",
                ".template.txt"
            )),
            Self::IPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "IPL-1.0",
                ".template.txt"
            )),
            Self::DeprecatedGPLTwoDotZeroWithFontException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-2.0-with-font-exception",
                ".template.txt"
            )),
            Self::Motosoto => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Motosoto",
                ".template.txt"
            )),
            Self::CheckCvs => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "check-cvs",
                ".template.txt"
            )),
            Self::Sendmail => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Sendmail",
                ".template.txt"
            )),
            Self::PPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PPL",
                ".template.txt"
            )),
            Self::PostgreSql => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PostgreSQL",
                ".template.txt"
            )),
            Self::CDLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CDL-1.0",
                ".template.txt"
            )),
            Self::GPLOneDotZeroOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-1.0-or-later",
                ".template.txt"
            )),
            Self::CCSAOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-SA-1.0",
                ".template.txt"
            )),
            Self::IBMPibs => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "IBM-pibs",
                ".template.txt"
            )),
            Self::CERNOHLOneDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CERN-OHL-1.2",
                ".template.txt"
            )),
            Self::IntelACPI => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Intel-ACPI",
                ".template.txt"
            )),
            Self::DeprecatedLGPLTwoDotZeroPlus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_LGPL-2.0+",
                ".template.txt"
            )),
            Self::SPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SPL-1.0",
                ".template.txt"
            )),
            Self::OML => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OML",
                ".template.txt"
            )),
            Self::DOC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DOC",
                ".template.txt"
            )),
            Self::MITClick => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-Click",
                ".template.txt"
            )),
            Self::MxmlException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "mxml-exception",
                ".template.txt"
            )),
            Self::NCGLUKTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NCGL-UK-2.0",
                ".template.txt"
            )),
            Self::HTMLTIDY => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HTMLTIDY",
                ".template.txt"
            )),
            Self::LALOneDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LAL-1.3",
                ".template.txt"
            )),
            Self::BSDInfernoNettverk => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BSD-Inferno-Nettverk",
                ".template.txt"
            )),
            Self::RPLOneDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "RPL-1.5",
                ".template.txt"
            )),
            Self::CcZeroOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC0-1.0",
                ".template.txt"
            )),
            Self::CECILLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CECILL-1.1",
                ".template.txt"
            )),
            Self::GCRDocs => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GCR-docs",
                ".template.txt"
            )),
            Self::Glide => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Glide",
                ".template.txt"
            )),
            Self::CCBYSATwoDotZeroUK => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-2.0-UK",
                ".template.txt"
            )),
            Self::Glulxe => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Glulxe",
                ".template.txt"
            )),
            Self::MITZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-0",
                ".template.txt"
            )),
            Self::DeprecatedLGPLTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_LGPL-2.1",
                ".template.txt"
            )),
            Self::ECosExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "eCos-exception-2.0",
                ".template.txt"
            )),
            Self::CERNOHLWTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CERN-OHL-W-2.0",
                ".template.txt"
            )),
            Self::Minpack => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Minpack",
                ".template.txt"
            )),
            Self::HPNDNetrek => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-Netrek",
                ".template.txt"
            )),
            Self::DocBookSchema => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DocBook-Schema",
                ".template.txt"
            )),
            Self::ManTwohtml => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "man2html",
                ".template.txt"
            )),
            Self::LGPLTwoDotOneOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPL-2.1-or-later",
                ".template.txt"
            )),
            Self::CMUMach => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CMU-Mach",
                ".template.txt"
            )),
            Self::OLDAPTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.0",
                ".template.txt"
            )),
            Self::PHPThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PHP-3.0",
                ".template.txt"
            )),
            Self::HPNDDEC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-DEC",
                ".template.txt"
            )),
            Self::Radvd => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "radvd",
                ".template.txt"
            )),
            Self::OGTSL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OGTSL",
                ".template.txt"
            )),
            Self::OLDAPTwoDotSeven => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.7",
                ".template.txt"
            )),
            Self::OGLUKTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OGL-UK-2.0",
                ".template.txt"
            )),
            Self::LGPLThreeDotZeroLinkingException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPL-3.0-linking-exception",
                ".template.txt"
            )),
            Self::TtypZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TTYP0",
                ".template.txt"
            )),
            Self::Zed => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Zed",
                ".template.txt"
            )),
            Self::PADL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PADL",
                ".template.txt"
            )),
            Self::OFLOneDotOneRFN => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OFL-1.1-RFN",
                ".template.txt"
            )),
            Self::OLDAPTwoDotTwoDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.2.2",
                ".template.txt"
            )),
            Self::Diffmark => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "diffmark",
                ".template.txt"
            )),
            Self::WThreeC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "W3C",
                ".template.txt"
            )),
            Self::HPNDSellVariantMITDisclaimer => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-sell-variant-MIT-disclaimer",
                ".template.txt"
            )),
            Self::OCamlLGPLLinkingException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OCaml-LGPL-linking-exception",
                ".template.txt"
            )),
            Self::PythonTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Python-2.0",
                ".template.txt"
            )),
            Self::MackerrasThreeClauseAcknowledgment => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Mackerras-3-Clause-acknowledgment",
                ".template.txt"
            )),
            Self::CCBYNCNDOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-ND-1.0",
                ".template.txt"
            )),
            Self::CCBYNDOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-ND-1.0",
                ".template.txt"
            )),
            Self::Mup => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Mup",
                ".template.txt"
            )),
            Self::TPDL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TPDL",
                ".template.txt"
            )),
            Self::HIDAPI => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HIDAPI",
                ".template.txt"
            )),
            Self::InfoZIP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Info-ZIP",
                ".template.txt"
            )),
            Self::DeprecatedLGPLThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_LGPL-3.0",
                ".template.txt"
            )),
            Self::HPOneNineEightNine => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HP-1989",
                ".template.txt"
            )),
            Self::FSFULLRWD => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FSFULLRWD",
                ".template.txt"
            )),
            Self::Leptonica => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Leptonica",
                ".template.txt"
            )),
            Self::GraphicsGems => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Graphics-Gems",
                ".template.txt"
            )),
            Self::UnicodeDFSTwoZeroOneFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Unicode-DFS-2015",
                ".template.txt"
            )),
            Self::TrustedQsl => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TrustedQSL",
                ".template.txt"
            )),
            Self::NLPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NLPL",
                ".template.txt"
            )),
            Self::Giftware => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Giftware",
                ".template.txt"
            )),
            Self::SSPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SSPL-1.0",
                ".template.txt"
            )),
            Self::CCBYTwoDotFiveAU => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-2.5-AU",
                ".template.txt"
            )),
            Self::HPNDSellVariantMITDisclaimerRev => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-sell-variant-MIT-disclaimer-rev",
                ".template.txt"
            )),
            Self::DeprecatedGPLOneDotZeroPlus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-1.0+",
                ".template.txt"
            )),
            Self::Libtiff => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "libtiff",
                ".template.txt"
            )),
            Self::IPA => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "IPA",
                ".template.txt"
            )),
            Self::CCBYSAOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-1.0",
                ".template.txt"
            )),
            Self::CCBYNCSAOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-1.0",
                ".template.txt"
            )),
            Self::ErlPlOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ErlPL-1.1",
                ".template.txt"
            )),
            Self::MPLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MPL-2.0",
                ".template.txt"
            )),
            Self::Pkgconf => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "pkgconf",
                ".template.txt"
            )),
            Self::BSDProtection => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BSD-Protection",
                ".template.txt"
            )),
            Self::GFDLOneDotTwoNoInvariantsOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.2-no-invariants-only",
                ".template.txt"
            )),
            Self::EPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EPL-1.0",
                ".template.txt"
            )),
            Self::DeprecatedLGPLTwoDotOnePlus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_LGPL-2.1+",
                ".template.txt"
            )),
            Self::ITwopGplJavaException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "i2p-gpl-java-exception",
                ".template.txt"
            )),
            Self::DeprecatedGPLThreeDotZeroWithGCCException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-3.0-with-GCC-exception",
                ".template.txt"
            )),
            Self::Jam => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Jam",
                ".template.txt"
            )),
            Self::DeprecatedAGPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_AGPL-1.0",
                ".template.txt"
            )),
            Self::Qhull => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Qhull",
                ".template.txt"
            )),
            Self::OpenSslStandalone => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OpenSSL-standalone",
                ".template.txt"
            )),
            Self::PcreTwoException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PCRE2-exception",
                ".template.txt"
            )),
            Self::NBPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NBPL-1.0",
                ".template.txt"
            )),
            Self::MulanPslTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MulanPSL-2.0",
                ".template.txt"
            )),
            Self::CCBYThreeDotZeroUS => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-3.0-US",
                ".template.txt"
            )),
            Self::LucidaBitmapFonts => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Lucida-Bitmap-Fonts",
                ".template.txt"
            )),
            Self::CCBYNCSATwoDotZeroFR => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-2.0-FR",
                ".template.txt"
            )),
            Self::MITAdvertising => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-advertising",
                ".template.txt"
            )),
            Self::HPNDExportUSModify => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-export-US-modify",
                ".template.txt"
            )),
            Self::Swrule => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "swrule",
                ".template.txt"
            )),
            Self::SMLNJ => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SMLNJ",
                ".template.txt"
            )),
            Self::MPEGSSG => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MPEG-SSG",
                ".template.txt"
            )),
            Self::PolyFormSmallBusinessOneDotZeroDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PolyForm-Small-Business-1.0.0",
                ".template.txt"
            )),
            Self::ZPLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ZPL-2.0",
                ".template.txt"
            )),
            Self::Xpp => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "xpp",
                ".template.txt"
            )),
            Self::Nokia => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Nokia",
                ".template.txt"
            )),
            Self::HPNDKevlinHenney => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-Kevlin-Henney",
                ".template.txt"
            )),
            Self::PolyFormNoncommercialOneDotZeroDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "PolyForm-Noncommercial-1.0.0",
                ".template.txt"
            )),
            Self::CCBYSATwoDotOneJP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-2.1-JP",
                ".template.txt"
            )),
            Self::XkeyboardConfigZinoviev => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "xkeyboard-config-Zinoviev",
                ".template.txt"
            )),
            Self::NAISTTwoZeroZeroThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NAIST-2003",
                ".template.txt"
            )),
            Self::HPNDExportUS => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-export-US",
                ".template.txt"
            )),
            Self::GStreamerExceptionTwoZeroZeroEight => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GStreamer-exception-2008",
                ".template.txt"
            )),
            Self::DeprecatedBSDTwoClauseNetBsd => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_BSD-2-Clause-NetBSD",
                ".template.txt"
            )),
            Self::WxWindowsExceptionThreeDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "WxWindows-exception-3.1",
                ".template.txt"
            )),
            Self::LibselinuxOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "libselinux-1.0",
                ".template.txt"
            )),
            Self::QPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "QPL-1.0",
                ".template.txt"
            )),
            Self::Pixar => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Pixar",
                ".template.txt"
            )),
            Self::FawkesRuntimeException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Fawkes-Runtime-exception",
                ".template.txt"
            )),
            Self::CCPDDC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-PDDC",
                ".template.txt"
            )),
            Self::Xerox => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Xerox",
                ".template.txt"
            )),
            Self::BUSLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BUSL-1.1",
                ".template.txt"
            )),
            Self::MakeIndex => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MakeIndex",
                ".template.txt"
            )),
            Self::LGPLThreeDotZeroOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LGPL-3.0-or-later",
                ".template.txt"
            )),
            Self::DeprecatedNunit => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_Nunit",
                ".template.txt"
            )),
            Self::OFFIS => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OFFIS",
                ".template.txt"
            )),
            Self::LatexTwoe => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Latex2e",
                ".template.txt"
            )),
            Self::SSHOpenSsh => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SSH-OpenSSH",
                ".template.txt"
            )),
            Self::Entessa => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Entessa",
                ".template.txt"
            )),
            Self::DeprecatedLGPLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_LGPL-2.0",
                ".template.txt"
            )),
            Self::ZendTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Zend-2.0",
                ".template.txt"
            )),
            Self::MifException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "mif-exception",
                ".template.txt"
            )),
            Self::Xfig => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Xfig",
                ".template.txt"
            )),
            Self::Caldera => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Caldera",
                ".template.txt"
            )),
            Self::LibutilDavidNugent => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "libutil-David-Nugent",
                ".template.txt"
            )),
            Self::OLDAPTwoDotSix => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.6",
                ".template.txt"
            )),
            Self::GNUCompilerException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GNU-compiler-exception",
                ".template.txt"
            )),
            Self::SGIOpenGl => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SGI-OpenGL",
                ".template.txt"
            )),
            Self::CCPDMOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-PDM-1.0",
                ".template.txt"
            )),
            Self::MITEnna => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-enna",
                ".template.txt"
            )),
            Self::NISTPD => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NIST-PD",
                ".template.txt"
            )),
            Self::CCBYTwoDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-2.5",
                ".template.txt"
            )),
            Self::HPNDSellRegexpr => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-sell-regexpr",
                ".template.txt"
            )),
            Self::LOOP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LOOP",
                ".template.txt"
            )),
            Self::OLDAPTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-2.1",
                ".template.txt"
            )),
            Self::SISSLOneDotTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SISSL-1.2",
                ".template.txt"
            )),
            Self::OCCTExceptionOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OCCT-exception-1.0",
                ".template.txt"
            )),
            Self::SL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SL",
                ".template.txt"
            )),
            Self::FLTKException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FLTK-exception",
                ".template.txt"
            )),
            Self::GFDLOneDotOneOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.1-only",
                ".template.txt"
            )),
            Self::CALOneDotZeroCombinedWorkException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CAL-1.0-Combined-Work-Exception",
                ".template.txt"
            )),
            Self::DeprecatedStandardMlNJ => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_StandardML-NJ",
                ".template.txt"
            )),
            Self::ZPLTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ZPL-2.1",
                ".template.txt"
            )),
            Self::ImlibTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Imlib2",
                ".template.txt"
            )),
            Self::RPLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "RPL-1.1",
                ".template.txt"
            )),
            Self::Gnuplot => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "gnuplot",
                ".template.txt"
            )),
            Self::DFSLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "D-FSL-1.0",
                ".template.txt"
            )),
            Self::OpenSsl => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OpenSSL",
                ".template.txt"
            )),
            Self::GPLThreeDotZeroOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-3.0-or-later",
                ".template.txt"
            )),
            Self::OSETPLTwoDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OSET-PL-2.1",
                ".template.txt"
            )),
            Self::LZMASDKNineDotOneOneToNineDotTwoZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LZMA-SDK-9.11-to-9.20",
                ".template.txt"
            )),
            Self::SAXPDTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SAX-PD-2.0",
                ".template.txt"
            )),
            Self::NASAOneDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NASA-1.3",
                ".template.txt"
            )),
            Self::EuDatagrid => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EUDatagrid",
                ".template.txt"
            )),
            Self::CERNOHLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CERN-OHL-1.1",
                ".template.txt"
            )),
            Self::EtalabTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "etalab-2.0",
                ".template.txt"
            )),
            Self::DeprecatedGPLThreeDotZeroWithAutoconfException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-3.0-with-autoconf-exception",
                ".template.txt"
            )),
            Self::CERNOHLSTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CERN-OHL-S-2.0",
                ".template.txt"
            )),
            Self::Gutmann => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Gutmann",
                ".template.txt"
            )),
            Self::OLFLOneDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLFL-1.3",
                ".template.txt"
            )),
            Self::LinuxManPagesCopyleftVar => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Linux-man-pages-copyleft-var",
                ".template.txt"
            )),
            Self::OGLUKThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OGL-UK-3.0",
                ".template.txt"
            )),
            Self::ParitySixDotZeroDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Parity-6.0.0",
                ".template.txt"
            )),
            Self::LPPLOneDotThreea => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPPL-1.3a",
                ".template.txt"
            )),
            Self::Mplus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "mplus",
                ".template.txt"
            )),
            Self::COILOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "COIL-1.0",
                ".template.txt"
            )),
            Self::Gtkbook => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "gtkbook",
                ".template.txt"
            )),
            Self::XSkat => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "XSkat",
                ".template.txt"
            )),
            Self::HPNDDocSell => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-doc-sell",
                ".template.txt"
            )),
            Self::RSAMD => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "RSA-MD",
                ".template.txt"
            )),
            Self::CCBYSAThreeDotZeroAT => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-3.0-AT",
                ".template.txt"
            )),
            Self::NISTPDFallback => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NIST-PD-fallback",
                ".template.txt"
            )),
            Self::IMatix => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "iMatix",
                ".template.txt"
            )),
            Self::GPLThreeDotZeroOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-3.0-only",
                ".template.txt"
            )),
            Self::CryptoSwift => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CryptoSwift",
                ".template.txt"
            )),
            Self::WTFPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "WTFPL",
                ".template.txt"
            )),
            Self::CveTou => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "cve-tou",
                ".template.txt"
            )),
            Self::HPNDMITDisclaimer => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-MIT-disclaimer",
                ".template.txt"
            )),
            Self::CCBYNDTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-ND-2.0",
                ".template.txt"
            )),
            Self::Vim => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Vim",
                ".template.txt"
            )),
            Self::NISTSoftware => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NIST-Software",
                ".template.txt"
            )),
            Self::LPPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPPL-1.0",
                ".template.txt"
            )),
            Self::YPLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "YPL-1.1",
                ".template.txt"
            )),
            Self::CCBYNCNDTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-ND-2.0",
                ".template.txt"
            )),
            Self::CopyleftNextZeroDotThreeDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "copyleft-next-0.3.1",
                ".template.txt"
            )),
            Self::MITCMU => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-CMU",
                ".template.txt"
            )),
            Self::RPSLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "RPSL-1.0",
                ".template.txt"
            )),
            Self::Dtoa => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "dtoa",
                ".template.txt"
            )),
            Self::NCSA => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NCSA",
                ".template.txt"
            )),
            Self::NPLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NPL-1.1",
                ".template.txt"
            )),
            Self::SCEA => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SCEA",
                ".template.txt"
            )),
            Self::SMPPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SMPPL",
                ".template.txt"
            )),
            Self::LiLiQROneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LiLiQ-R-1.1",
                ".template.txt"
            )),
            Self::OFLOneDotZeroRFN => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OFL-1.0-RFN",
                ".template.txt"
            )),
            Self::NPOSLThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NPOSL-3.0",
                ".template.txt"
            )),
            Self::ImageMagick => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ImageMagick",
                ".template.txt"
            )),
            Self::LibpriOpenHThreeTwoThreeException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "libpri-OpenH323-exception",
                ".template.txt"
            )),
            Self::UnicodeTOU => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Unicode-TOU",
                ".template.txt"
            )),
            Self::OpenPbsTwoDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OpenPBS-2.3",
                ".template.txt"
            )),
            Self::UCLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "UCL-1.0",
                ".template.txt"
            )),
            Self::ZimbraOneDotFour => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Zimbra-1.4",
                ".template.txt"
            )),
            Self::TGPPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TGPPL-1.0",
                ".template.txt"
            )),
            Self::Wsuipa => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Wsuipa",
                ".template.txt"
            )),
            Self::DeprecatedNokiaQtExceptionOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_Nokia-Qt-exception-1.1",
                ".template.txt"
            )),
            Self::CCBYNCSATwoDotZeroDE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-2.0-DE",
                ".template.txt"
            )),
            Self::LinuxManPagesCopyleft => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Linux-man-pages-copyleft",
                ".template.txt"
            )),
            Self::Xlock => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "xlock",
                ".template.txt"
            )),
            Self::LiLiQRplusOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LiLiQ-Rplus-1.1",
                ".template.txt"
            )),
            Self::GenericXts => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "generic-xts",
                ".template.txt"
            )),
            Self::ZimbraOneDotThree => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Zimbra-1.3",
                ".template.txt"
            )),
            Self::GPLTwoDotZeroOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-2.0-only",
                ".template.txt"
            )),
            Self::OGLUKOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OGL-UK-1.0",
                ".template.txt"
            )),
            Self::SsLeayStandalone => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SSLeay-standalone",
                ".template.txt"
            )),
            Self::XOneOneSwapped => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "X11-swapped",
                ".template.txt"
            )),
            Self::OpenJdkAssemblyExceptionOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OpenJDK-assembly-exception-1.0",
                ".template.txt"
            )),
            Self::GFDLOneDotThreeNoInvariantsOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.3-no-invariants-or-later",
                ".template.txt"
            )),
            Self::MTLL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MTLL",
                ".template.txt"
            )),
            Self::UbuntuFontOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Ubuntu-font-1.0",
                ".template.txt"
            )),
            Self::DocBookXML => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DocBook-XML",
                ".template.txt"
            )),
            Self::MITTestregex => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-testregex",
                ".template.txt"
            )),
            Self::CCBYNCTwoDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-2.5",
                ".template.txt"
            )),
            Self::PythonLdap => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "python-ldap",
                ".template.txt"
            )),
            Self::GlTwoPs => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GL2PS",
                ".template.txt"
            )),
            Self::LPLOneDotZeroTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPL-1.02",
                ".template.txt"
            )),
            Self::MITNFA => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MITNFA",
                ".template.txt"
            )),
            Self::Checkmk => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "checkmk",
                ".template.txt"
            )),
            Self::GFDLOneDotTwoOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.2-only",
                ".template.txt"
            )),
            Self::NGPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NGPL",
                ".template.txt"
            )),
            Self::MulanPslOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MulanPSL-1.0",
                ".template.txt"
            )),
            Self::DeprecatedWxWindows => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_wxWindows",
                ".template.txt"
            )),
            Self::OGCOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OGC-1.0",
                ".template.txt"
            )),
            Self::Ulem => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ulem",
                ".template.txt"
            )),
            Self::HarbourException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "harbour-exception",
                ".template.txt"
            )),
            Self::UCAR => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "UCAR",
                ".template.txt"
            )),
            Self::MSPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MS-PL",
                ".template.txt"
            )),
            Self::JPLImage => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "JPL-image",
                ".template.txt"
            )),
            Self::FontExceptionTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Font-exception-2.0",
                ".template.txt"
            )),
            Self::GFDLOneDotThreeInvariantsOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.3-invariants-or-later",
                ".template.txt"
            )),
            Self::Fwlw => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "fwlw",
                ".template.txt"
            )),
            Self::InnerNetTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Inner-Net-2.0",
                ".template.txt"
            )),
            Self::MPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MPL-1.0",
                ".template.txt"
            )),
            Self::CommunitySpecOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Community-Spec-1.0",
                ".template.txt"
            )),
            Self::CUAOPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CUA-OPL-1.0",
                ".template.txt"
            )),
            Self::UBDLException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "UBDL-exception",
                ".template.txt"
            )),
            Self::GFDLOneDotOneInvariantsOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.1-invariants-only",
                ".template.txt"
            )),
            Self::FreeBsdDOC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FreeBSD-DOC",
                ".template.txt"
            )),
            Self::EPLTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EPL-2.0",
                ".template.txt"
            )),
            Self::SendmailOpenSourceOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Sendmail-Open-Source-1.1",
                ".template.txt"
            )),
            Self::Eurosym => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Eurosym",
                ".template.txt"
            )),
            Self::GPLOneDotZeroOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-1.0-only",
                ".template.txt"
            )),
            Self::DeprecatedBSDTwoClauseFreeBsd => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_BSD-2-Clause-FreeBSD",
                ".template.txt"
            )),
            Self::SHLZeroDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SHL-0.5",
                ".template.txt"
            )),
            Self::XOneOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "X11",
                ".template.txt"
            )),
            Self::ThirdEye => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ThirdEye",
                ".template.txt"
            )),
            Self::FSFAPNoWarrantyDisclaimer => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FSFAP-no-warranty-disclaimer",
                ".template.txt"
            )),
            Self::SimPlTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SimPL-2.0",
                ".template.txt"
            )),
            Self::InterbaseOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Interbase-1.0",
                ".template.txt"
            )),
            Self::DeprecatedBzipTwoOneDotZeroDotFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_bzip2-1.0.5",
                ".template.txt"
            )),
            Self::Multics => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Multics",
                ".template.txt"
            )),
            Self::GStreamerExceptionTwoZeroZeroFive => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GStreamer-exception-2005",
                ".template.txt"
            )),
            Self::CCBYNCSATwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-2.0",
                ".template.txt"
            )),
            Self::GD => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GD",
                ".template.txt"
            )),
            Self::CCBYSATwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-2.0",
                ".template.txt"
            )),
            Self::CECILLC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CECILL-C",
                ".template.txt"
            )),
            Self::ElasticTwoDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Elastic-2.0",
                ".template.txt"
            )),
            Self::MITWu => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-Wu",
                ".template.txt"
            )),
            Self::Snprintf => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "snprintf",
                ".template.txt"
            )),
            Self::EUPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EUPL-1.0",
                ".template.txt"
            )),
            Self::BSDSourceBeginningFile => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "BSD-Source-beginning-file",
                ".template.txt"
            )),
            Self::SunPro => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SunPro",
                ".template.txt"
            )),
            Self::HPNDIntel => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-Intel",
                ".template.txt"
            )),
            Self::LZMASDKNineDotTwoTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LZMA-SDK-9.22",
                ".template.txt"
            )),
            Self::OFLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OFL-1.0",
                ".template.txt"
            )),
            Self::DeprecatedGPLTwoDotZeroWithGCCException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GPL-2.0-with-GCC-exception",
                ".template.txt"
            )),
            Self::CCBYSAThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-3.0",
                ".template.txt"
            )),
            Self::CCBYNCSAThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-3.0",
                ".template.txt"
            )),
            Self::DSDP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DSDP",
                ".template.txt"
            )),
            Self::McPheeSlideshow => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "McPhee-slideshow",
                ".template.txt"
            )),
            Self::CCBYThreeDotZeroNL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-3.0-NL",
                ".template.txt"
            )),
            Self::OLDAPOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OLDAP-1.1",
                ".template.txt"
            )),
            Self::GFDLOneDotTwoInvariantsOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.2-invariants-only",
                ".template.txt"
            )),
            Self::SchemeReport => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SchemeReport",
                ".template.txt"
            )),
            Self::HPNDExportTwoUS => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-export2-US",
                ".template.txt"
            )),
            Self::Xinetd => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "xinetd",
                ".template.txt"
            )),
            Self::HaskellReport => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HaskellReport",
                ".template.txt"
            )),
            Self::TCL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TCL",
                ".template.txt"
            )),
            Self::NetCdf => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NetCDF",
                ".template.txt"
            )),
            Self::Symlinks => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Symlinks",
                ".template.txt"
            )),
            Self::OpenvpnOpensslException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "openvpn-openssl-exception",
                ".template.txt"
            )),
            Self::HPNDPbmplus => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-Pbmplus",
                ".template.txt"
            )),
            Self::OAR => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OAR",
                ".template.txt"
            )),
            Self::CopyleftNextZeroDotThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "copyleft-next-0.3.0",
                ".template.txt"
            )),
            Self::YPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "YPL-1.0",
                ".template.txt"
            )),
            Self::LPPLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "LPPL-1.1",
                ".template.txt"
            )),
            Self::CCBYNDFourDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-ND-4.0",
                ".template.txt"
            )),
            Self::XFreeEightSixOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "XFree86-1.1",
                ".template.txt"
            )),
            Self::CCBYNCNDFourDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-ND-4.0",
                ".template.txt"
            )),
            Self::CCBYNCSAThreeDotZeroDE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-3.0-DE",
                ".template.txt"
            )),
            Self::TPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TPL-1.0",
                ".template.txt"
            )),
            Self::Naumen => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Naumen",
                ".template.txt"
            )),
            Self::NICTAOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NICTA-1.0",
                ".template.txt"
            )),
            Self::NOSL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NOSL",
                ".template.txt"
            )),
            Self::Pnmstitch => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "pnmstitch",
                ".template.txt"
            )),
            Self::CPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CPL-1.0",
                ".template.txt"
            )),
            Self::Xzoom => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "xzoom",
                ".template.txt"
            )),
            Self::IECCodeComponentsEULA => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "IEC-Code-Components-EULA",
                ".template.txt"
            )),
            Self::DeprecatedAGPLThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_AGPL-3.0",
                ".template.txt"
            )),
            Self::RSCPL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "RSCPL",
                ".template.txt"
            )),
            Self::NPLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "NPL-1.0",
                ".template.txt"
            )),
            Self::Sleepycat => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Sleepycat",
                ".template.txt"
            )),
            Self::CDLASharingOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CDLA-Sharing-1.0",
                ".template.txt"
            )),
            Self::GFDLOneDotThreeOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.3-only",
                ".template.txt"
            )),
            Self::Lsof => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "lsof",
                ".template.txt"
            )),
            Self::ParitySevenDotZeroDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Parity-7.0.0",
                ".template.txt"
            )),
            Self::Zeeff => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Zeeff",
                ".template.txt"
            )),
            Self::CCBYNCSAFourDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-SA-4.0",
                ".template.txt"
            )),
            Self::CCBYSAFourDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-4.0",
                ".template.txt"
            )),
            Self::GFDLOneDotTwoOrLater => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.2-or-later",
                ".template.txt"
            )),
            Self::OFLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OFL-1.1",
                ".template.txt"
            )),
            Self::GPLThreeDotZeroInterfaceException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GPL-3.0-interface-exception",
                ".template.txt"
            )),
            Self::QtLGPLExceptionOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Qt-LGPL-exception-1.1",
                ".template.txt"
            )),
            Self::MackerrasThreeClause => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Mackerras-3-Clause",
                ".template.txt"
            )),
            Self::EUPLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "EUPL-1.1",
                ".template.txt"
            )),
            Self::SunPPP => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Sun-PPP",
                ".template.txt"
            )),
            Self::CECILLB => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CECILL-B",
                ".template.txt"
            )),
            Self::LinuxOpenIb => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Linux-OpenIB",
                ".template.txt"
            )),
            Self::FmtException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "fmt-exception",
                ".template.txt"
            )),
            Self::MITFeh => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MIT-feh",
                ".template.txt"
            )),
            Self::RubyPty => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Ruby-pty",
                ".template.txt"
            )),
            Self::HPNDSellVariant => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-sell-variant",
                ".template.txt"
            )),
            Self::DECThreeClause => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "DEC-3-Clause",
                ".template.txt"
            )),
            Self::ZPLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ZPL-1.1",
                ".template.txt"
            )),
            Self::Zlib => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Zlib",
                ".template.txt"
            )),
            Self::FTL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FTL",
                ".template.txt"
            )),
            Self::CCBYSAThreeDotZeroDE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-SA-3.0-DE",
                ".template.txt"
            )),
            Self::CCBYNCNDThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-NC-ND-3.0",
                ".template.txt"
            )),
            Self::TOSL => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TOSL",
                ".template.txt"
            )),
            Self::FBM => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "FBM",
                ".template.txt"
            )),
            Self::CCBYNDThreeDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CC-BY-ND-3.0",
                ".template.txt"
            )),
            Self::CUDAOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "C-UDA-1.0",
                ".template.txt"
            )),
            Self::QtGPLExceptionOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Qt-GPL-exception-1.0",
                ".template.txt"
            )),
            Self::SunPPPTwoZeroZeroZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Sun-PPP-2000",
                ".template.txt"
            )),
            Self::OpenVision => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "OpenVision",
                ".template.txt"
            )),
            Self::DeprecatedGFDLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "deprecated_GFDL-1.1",
                ".template.txt"
            )),
            Self::SshKeyscan => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "ssh-keyscan",
                ".template.txt"
            )),
            Self::SSHShort => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "SSH-short",
                ".template.txt"
            )),
            Self::GSoapOneDotThreeb => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "gSOAP-1.3b",
                ".template.txt"
            )),
            Self::JPNIC => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "JPNIC",
                ".template.txt"
            )),
            Self::GFDLOneDotOneNoInvariantsOnly => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "GFDL-1.1-no-invariants-only",
                ".template.txt"
            )),
            Self::TAPROHLOneDotZero => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "TAPR-OHL-1.0",
                ".template.txt"
            )),
            Self::Rdisc => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "Rdisc",
                ".template.txt"
            )),
            Self::HPNDDoc => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "HPND-doc",
                ".template.txt"
            )),
            Self::URTRLE => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "URT-RLE",
                ".template.txt"
            )),
            Self::MpichTwo => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "mpich2",
                ".template.txt"
            )),
            Self::CGALLinkingException => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "CGAL-linking-exception",
                ".template.txt"
            )),
            Self::MPLOneDotOne => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/licenses/",
                "MPL-1.1",
                ".template.txt"
            )),
        }
    }
}
impl std::fmt::Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spdx_id())
    }
}
impl std::str::FromStr for License {
    type Err = ParseLicenseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Linux-syscall-note" => Ok(Self::LinuxSyscallNote),
            "DocBook-Stylesheet" => Ok(Self::DocBookStylesheet),
            "HPND-sell-MIT-disclaimer-xserver" => Ok(Self::HPNDSellMITDisclaimerXserver),
            "OSL-2.1" => Ok(Self::OSLTwoDotOne),
            "SWL" => Ok(Self::SWL),
            "CC-BY-NC-4.0" => Ok(Self::CCBYNCFourDotZero),
            "NLOD-2.0" => Ok(Self::NLODTwoDotZero),
            "Unlicense" => Ok(Self::Unlicense),
            "ODC-By-1.0" => Ok(Self::ODCByOneDotZero),
            "UPL-1.0" => Ok(Self::UPLOneDotZero),
            "SNIA" => Ok(Self::SNIA),
            "GCC-exception-3.1" => Ok(Self::GCCExceptionThreeDotOne),
            "OLDAP-1.3" => Ok(Self::OLDAPOneDotThree),
            "Dotseqn" => Ok(Self::Dotseqn),
            "TMate" => Ok(Self::TMate),
            "GFDL-1.1-invariants-or-later" => Ok(Self::GFDLOneDotOneInvariantsOrLater),
            "MMIXware" => Ok(Self::MmiXware),
            "Crossword" => Ok(Self::Crossword),
            "OLDAP-1.4" => Ok(Self::OLDAPOneDotFour),
            "GLWTPL" => Ok(Self::GLWTPL),
            "curl" => Ok(Self::Curl),
            "GPL-2.0-or-later" => Ok(Self::GPLTwoDotZeroOrLater),
            "OFL-1.1-no-RFN" => Ok(Self::OFLOneDotOneNoRFN),
            "MS-LPL" => Ok(Self::MSLPL),
            "CC-BY-1.0" => Ok(Self::CCBYOneDotZero),
            "CDLA-Permissive-1.0" => Ok(Self::CDLAPermissiveOneDotZero),
            "QPL-1.0-INRIA-2004-exception" => Ok(Self::QPLOneDotZeroINRIATwoZeroZeroFourException),
            "Classpath-exception-2.0" => Ok(Self::ClasspathExceptionTwoDotZero),
            "OPL-1.0" => Ok(Self::OPLOneDotZero),
            "ODbL-1.0" => Ok(Self::ODbLOneDotZero),
            "Independent-modules-exception" => Ok(Self::IndependentModulesException),
            "MIT-Festival" => Ok(Self::MITFestival),
            "Caldera-no-preamble" => Ok(Self::CalderaNoPreamble),
            "SGI-B-1.1" => Ok(Self::SGIBOneDotOne),
            "Soundex" => Ok(Self::Soundex),
            "SANE-exception" => Ok(Self::SANEException),
            "MirOS" => Ok(Self::MirOs),
            "Ruby" => Ok(Self::Ruby),
            "polyparse-exception" => Ok(Self::PolyparseException),
            "EFL-2.0" => Ok(Self::EFLTwoDotZero),
            "LLGPL" => Ok(Self::LLGPL),
            "vsftpd-openssl-exception" => Ok(Self::VsftpdOpensslException),
            "OFL-1.0-no-RFN" => Ok(Self::OFLOneDotZeroNoRFN),
            "CC-BY-NC-ND-3.0-DE" => Ok(Self::CCBYNCNDThreeDotZeroDE),
            "CDDL-1.0" => Ok(Self::CDDLOneDotZero),
            "SWI-exception" => Ok(Self::SWIException),
            "GFDL-1.3-invariants-only" => Ok(Self::GFDLOneDotThreeInvariantsOnly),
            "MS-RL" => Ok(Self::MSRL),
            "LGPL-2.1-only" => Ok(Self::LGPLTwoDotOneOnly),
            "HP-1986" => Ok(Self::HPOneNineEightSix),
            "Python-2.0.1" => Ok(Self::PythonTwoDotZeroDotOne),
            "RHeCos-1.1" => Ok(Self::RHeCosOneDotOne),
            "MIT" => Ok(Self::MIT),
            "romic-exception" => Ok(Self::RomicException),
            "OSL-3.0" => Ok(Self::OSLThreeDotZero),
            "CC-BY-NC-3.0" => Ok(Self::CCBYNCThreeDotZero),
            "GCC-exception-2.0" => Ok(Self::GCCExceptionTwoDotZero),
            "deprecated_LGPL-3.0+" => Ok(Self::DeprecatedLGPLThreeDotZeroPlus),
            "Clips" => Ok(Self::Clips),
            "CNRI-Python-GPL-Compatible" => Ok(Self::CNRIPythonGPLCompatible),
            "LPD-document" => Ok(Self::LPDDocument),
            "GPL-CC-1.0" => Ok(Self::GPLCCOneDotZero),
            "HPND-UC" => Ok(Self::HPNDUC),
            "ISC-Veillard" => Ok(Self::ISCVeillard),
            "PDDL-1.0" => Ok(Self::PDDLOneDotZero),
            "LGPL-2.0-or-later" => Ok(Self::LGPLTwoDotZeroOrLater),
            "CC-BY-NC-SA-3.0-IGO" => Ok(Self::CCBYNCSAThreeDotZeroIGO),
            "TORQUE-1.1" => Ok(Self::TORQUEOneDotOne),
            "Frameworx-1.0" => Ok(Self::FrameworxOneDotZero),
            "OLDAP-2.8" => Ok(Self::OLDAPTwoDotEight),
            "deprecated_GPL-3.0" => Ok(Self::DeprecatedGPLThreeDotZero),
            "Qwt-exception-1.0" => Ok(Self::QwtExceptionOneDotZero),
            "DocBook-DTD" => Ok(Self::DocBookDTD),
            "psfrag" => Ok(Self::Psfrag),
            "IJG-short" => Ok(Self::IJGShort),
            "psutils" => Ok(Self::Psutils),
            "DRL-1.1" => Ok(Self::DRLOneDotOne),
            "CECILL-2.0" => Ok(Self::CECILLTwoDotZero),
            "W3C-19980720" => Ok(Self::WThreeCOneNineNineEightZeroSevenTwoZero),
            "O-UDA-1.0" => Ok(Self::OUDAOneDotZero),
            "CLISP-exception-2.0" => Ok(Self::CLISPExceptionTwoDotZero),
            "DL-DE-ZERO-2.0" => Ok(Self::DLDEZEROTwoDotZero),
            "deprecated_GFDL-1.3" => Ok(Self::DeprecatedGFDLOneDotThree),
            "SGP4" => Ok(Self::SgpFour),
            "FDK-AAC" => Ok(Self::FDKAAC),
            "TTWL" => Ok(Self::TTWL),
            "Martin-Birgmeier" => Ok(Self::MartinBirgmeier),
            "CC-BY-ND-3.0-DE" => Ok(Self::CCBYNDThreeDotZeroDE),
            "ECL-2.0" => Ok(Self::ECLTwoDotZero),
            "ICU" => Ok(Self::ICU),
            "stunnel-exception" => Ok(Self::StunnelException),
            "Game-Programming-Gems" => Ok(Self::GameProgrammingGems),
            "SGI-B-1.0" => Ok(Self::SGIBOneDotZero),
            "NRL" => Ok(Self::NRL),
            "Kazlib" => Ok(Self::Kazlib),
            "PHP-3.01" => Ok(Self::PHPThreeDotZeroOne),
            "magaz" => Ok(Self::Magaz),
            "CPAL-1.0" => Ok(Self::CPALOneDotZero),
            "RRDtool-FLOSS-exception-2.0" => Ok(Self::RrDtoolFLOSSExceptionTwoDotZero),
            "Libtool-exception" => Ok(Self::LibtoolException),
            "PS-or-PDF-font-exception-20170817" => {
                Ok(Self::PSOrPDFFontExceptionTwoZeroOneSevenZeroEightOneSeven)
            }
            "deprecated_GPL-2.0" => Ok(Self::DeprecatedGPLTwoDotZero),
            "LGPL-3.0-only" => Ok(Self::LGPLThreeDotZeroOnly),
            "DigiRule-FOSS-exception" => Ok(Self::DigiRuleFOSSException),
            "GPL-3.0-linking-exception" => Ok(Self::GPLThreeDotZeroLinkingException),
            "mailprio" => Ok(Self::Mailprio),
            "LPPL-1.2" => Ok(Self::LPPLOneDotTwo),
            "CDDL-1.1" => Ok(Self::CDDLOneDotOne),
            "HPND" => Ok(Self::HPND),
            "NTP-0" => Ok(Self::NTPZero),
            "CC-BY-ND-2.5" => Ok(Self::CCBYNDTwoDotFive),
            "CC-BY-NC-ND-2.5" => Ok(Self::CCBYNCNDTwoDotFive),
            "freertos-exception-2.0" => Ok(Self::FreertosExceptionTwoDotZero),
            "deprecated_GPL-2.0-with-bison-exception" => {
                Ok(Self::DeprecatedGPLTwoDotZeroWithBisonException)
            }
            "SugarCRM-1.1.3" => Ok(Self::SugarCrmOneDotOneDotThree),
            "SISSL" => Ok(Self::SISSL),
            "CFITSIO" => Ok(Self::CFITSIO),
            "TU-Berlin-2.0" => Ok(Self::TUBerlinTwoDotZero),
            "CMU-Mach-nodoc" => Ok(Self::CMUMachNodoc),
            "Condor-1.1" => Ok(Self::CondorOneDotOne),
            "OSL-2.0" => Ok(Self::OSLTwoDotZero),
            "CC-BY-NC-2.0" => Ok(Self::CCBYNCTwoDotZero),
            "TermReadKey" => Ok(Self::TermReadKey),
            "UnixCrypt" => Ok(Self::UnixCrypt),
            "NTP" => Ok(Self::NTP),
            "CC-BY-3.0-DE" => Ok(Self::CCBYThreeDotZeroDE),
            "Kastrup" => Ok(Self::Kastrup),
            "LGPL-2.0-only" => Ok(Self::LGPLTwoDotZeroOnly),
            "OLDAP-1.2" => Ok(Self::OLDAPOneDotTwo),
            "Linux-man-pages-1-para" => Ok(Self::LinuxManPagesOnePara),
            "DL-DE-BY-2.0" => Ok(Self::DLDEBYTwoDotZero),
            "CC-BY-NC-3.0-DE" => Ok(Self::CCBYNCThreeDotZeroDE),
            "FreeImage" => Ok(Self::FreeImage),
            "CECILL-2.1" => Ok(Self::CECILLTwoDotOne),
            "DRL-1.0" => Ok(Self::DRLOneDotZero),
            "HPND-export-US-acknowledgement" => Ok(Self::HPNDExportUSAcknowledgement),
            "OLDAP-2.0.1" => Ok(Self::OLDAPTwoDotZeroDotOne),
            "threeparttable" => Ok(Self::Threeparttable),
            "MIT-open-group" => Ok(Self::MITOpenGroup),
            "FSFULLR" => Ok(Self::FSFULLR),
            "NCL" => Ok(Self::NCL),
            "EPICS" => Ok(Self::EPICS),
            "JSON" => Ok(Self::JSON),
            "deprecated_GFDL-1.2" => Ok(Self::DeprecatedGFDLOneDotTwo),
            "Spencer-86" => Ok(Self::SpencerEightSix),
            "SHL-0.51" => Ok(Self::SHLZeroDotFiveOne),
            "IJG" => Ok(Self::IJG),
            "deprecated_GPL-2.0+" => Ok(Self::DeprecatedGPLTwoDotZeroPlus),
            "erlang-otp-linking-exception" => Ok(Self::ErlangOtpLinkingException),
            "SAX-PD" => Ok(Self::SAXPD),
            "PSF-2.0" => Ok(Self::PSFTwoDotZero),
            "QPL-1.0-INRIA-2004" => Ok(Self::QPLOneDotZeroINRIATwoZeroZeroFour),
            "deprecated_GPL-2.0-with-classpath-exception" => {
                Ok(Self::DeprecatedGPLTwoDotZeroWithClasspathException)
            }
            "EUPL-1.2" => Ok(Self::EUPLOneDotTwo),
            "GNOME-examples-exception" => Ok(Self::GNOMEExamplesException),
            "X11-distribute-modifications-variant" => {
                Ok(Self::XOneOneDistributeModificationsVariant)
            }
            "LPL-1.0" => Ok(Self::LPLOneDotZero),
            "Spencer-94" => Ok(Self::SpencerNineFour),
            "CC-BY-NC-SA-2.5" => Ok(Self::CCBYNCSATwoDotFive),
            "u-boot-exception-2.0" => Ok(Self::UBootExceptionTwoDotZero),
            "CC-BY-SA-2.5" => Ok(Self::CCBYSATwoDotFive),
            "Newsletr" => Ok(Self::Newsletr),
            "Noweb" => Ok(Self::Noweb),
            "w3m" => Ok(Self::WThreem),
            "FSL-1.1-MIT" => Ok(Self::FSLOneDotOneMIT),
            "Texinfo-exception" => Ok(Self::TexinfoException),
            "GFDL-1.2-no-invariants-or-later" => Ok(Self::GFDLOneDotTwoNoInvariantsOrLater),
            "EFL-1.0" => Ok(Self::EFLOneDotZero),
            "Watcom-1.0" => Ok(Self::WatcomOneDotZero),
            "CC-BY-3.0-AU" => Ok(Self::CCBYThreeDotZeroAU),
            "ISC" => Ok(Self::ISC),
            "zlib-acknowledgement" => Ok(Self::ZlibAcknowledgement),
            "TCP-wrappers" => Ok(Self::TCPWrappers),
            "Universal-FOSS-exception-1.0" => Ok(Self::UniversalFOSSExceptionOneDotZero),
            "hdparm" => Ok(Self::Hdparm),
            "SHL-2.0" => Ok(Self::SHLTwoDotZero),
            "OLDAP-2.2.1" => Ok(Self::OLDAPTwoDotTwoDotOne),
            "GFDL-1.3-no-invariants-only" => Ok(Self::GFDLOneDotThreeNoInvariantsOnly),
            "eGenix" => Ok(Self::EGenix),
            "FSFAP" => Ok(Self::FSFAP),
            "Spencer-99" => Ok(Self::SpencerNineNine),
            "OLDAP-2.3" => Ok(Self::OLDAPTwoDotThree),
            "Gmsh-exception" => Ok(Self::GmshException),
            "dvipdfm" => Ok(Self::Dvipdfm),
            "deprecated_Net-SNMP" => Ok(Self::DeprecatedNetSNMP),
            "LZMA-exception" => Ok(Self::LZMAException),
            "Libpng" => Ok(Self::Libpng),
            "Xnet" => Ok(Self::Xnet),
            "HPND-merchantability-variant" => Ok(Self::HPNDMerchantabilityVariant),
            "Intel" => Ok(Self::Intel),
            "OLDAP-2.4" => Ok(Self::OLDAPTwoDotFour),
            "W3C-20150513" => Ok(Self::WThreeCTwoZeroOneFiveZeroFiveOneThree),
            "MIT-Khronos-old" => Ok(Self::MITKhronosOld),
            "CDLA-Permissive-2.0" => Ok(Self::CDLAPermissiveTwoDotZero),
            "CC-BY-2.0" => Ok(Self::CCBYTwoDotZero),
            "mpi-permissive" => Ok(Self::MpiPermissive),
            "deprecated_GPL-2.0-with-autoconf-exception" => {
                Ok(Self::DeprecatedGPLTwoDotZeroWithAutoconfException)
            }
            "MIPS" => Ok(Self::MIPS),
            "Cube" => Ok(Self::Cube),
            "OSL-1.1" => Ok(Self::OSLOneDotOne),
            "MPL-2.0-no-copyleft-exception" => Ok(Self::MPLTwoDotZeroNoCopyleftException),
            "NLOD-1.0" => Ok(Self::NLODOneDotZero),
            "GFDL-1.1-or-later" => Ok(Self::GFDLOneDotOneOrLater),
            "Widget-Workshop" => Ok(Self::WidgetWorkshop),
            "CATOSL-1.1" => Ok(Self::CATOSLOneDotOne),
            "cryptsetup-OpenSSL-exception" => Ok(Self::CryptsetupOpenSslException),
            "BSD-Systemics" => Ok(Self::BSDSystemics),
            "GCC-exception-2.0-note" => Ok(Self::GCCExceptionTwoDotZeroNote),
            "BSD-Systemics-W3Works" => Ok(Self::BSDSystemicsWThreeWorks),
            "OCLC-2.0" => Ok(Self::OCLCTwoDotZero),
            "LAL-1.2" => Ok(Self::LALOneDotTwo),
            "Xdebug-1.03" => Ok(Self::XdebugOneDotZeroThree),
            "FSL-1.1-ALv2" => Ok(Self::FSLOneDotOneALvTwo),
            "HPND-UC-export-US" => Ok(Self::HPNDUCExportUS),
            "ECL-1.0" => Ok(Self::ECLOneDotZero),
            "CC-BY-NC-ND-3.0-IGO" => Ok(Self::CCBYNCNDThreeDotZeroIGO),
            "OGL-Canada-2.0" => Ok(Self::OGLCanadaTwoDotZero),
            "LGPLLR" => Ok(Self::LGPLLR),
            "Fair" => Ok(Self::Fair),
            "Swift-exception" => Ok(Self::SwiftException),
            "JasPer-2.0" => Ok(Self::JasPerTwoDotZero),
            "CECILL-1.0" => Ok(Self::CECILLOneDotZero),
            "GPL-3.0-linking-source-exception" => Ok(Self::GPLThreeDotZeroLinkingSourceException),
            "BSD-Source-Code" => Ok(Self::BSDSourceCode),
            "metamail" => Ok(Self::Metamail),
            "LLVM-exception" => Ok(Self::LLVMException),
            "VSL-1.0" => Ok(Self::VSLOneDotZero),
            "LPPL-1.3c" => Ok(Self::LPPLOneDotThreec),
            "Furuseth" => Ok(Self::Furuseth),
            "CNRI-Jython" => Ok(Self::CNRIJython),
            "LiLiQ-P-1.1" => Ok(Self::LiLiQPOneDotOne),
            "Ferguson-Twofish" => Ok(Self::FergusonTwofish),
            "HPND-INRIA-IMAG" => Ok(Self::HPNDINRIAIMAG),
            "Unicode-DFS-2016" => Ok(Self::UnicodeDFSTwoZeroOneSix),
            "MIT-Modern-Variant" => Ok(Self::MITModernVariant),
            "deprecated_GPL-3.0+" => Ok(Self::DeprecatedGPLThreeDotZeroPlus),
            "HPND-Fenneberg-Livingston" => Ok(Self::HPNDFennebergLivingston),
            "wwl" => Ok(Self::Wwl),
            "ClArtistic" => Ok(Self::ClArtistic),
            "HPND-Markus-Kuhn" => Ok(Self::HPNDMarkusKuhn),
            "softSurfer" => Ok(Self::SoftSurfer),
            "CrystalStacker" => Ok(Self::CrystalStacker),
            "NCBI-PD" => Ok(Self::NCBIPD),
            "GFDL-1.2-invariants-or-later" => Ok(Self::GFDLOneDotTwoInvariantsOrLater),
            "deprecated_eCos-2.0" => Ok(Self::DeprecatedECosTwoDotZero),
            "OLDAP-2.5" => Ok(Self::OLDAPTwoDotFive),
            "GFDL-1.1-no-invariants-or-later" => Ok(Self::GFDLOneDotOneNoInvariantsOrLater),
            "CC-BY-4.0" => Ok(Self::CCBYFourDotZero),
            "OLDAP-2.2" => Ok(Self::OLDAPTwoDotTwo),
            "CNRI-Python" => Ok(Self::CNRIPython),
            "BSL-1.0" => Ok(Self::BSLOneDotZero),
            "SMAIL-GPL" => Ok(Self::SMAILGPL),
            "TU-Berlin-1.0" => Ok(Self::TUBerlinOneDotZero),
            "VOSTROM" => Ok(Self::VOSTROM),
            "libpng-2.0" => Ok(Self::LibpngTwoDotZero),
            "gnu-javamail-exception" => Ok(Self::GnuJavamailException),
            "CC-BY-NC-1.0" => Ok(Self::CCBYNCOneDotZero),
            "GPL-3.0-389-ds-base-exception" => {
                Ok(Self::GPLThreeDotZeroThreeEightNineDsBaseException)
            }
            "OSL-1.0" => Ok(Self::OSLOneDotZero),
            "Unicode-3.0" => Ok(Self::UnicodeThreeDotZero),
            "GFDL-1.3-or-later" => Ok(Self::GFDLOneDotThreeOrLater),
            "InnoSetup" => Ok(Self::InnoSetup),
            "CAL-1.0" => Ok(Self::CALOneDotZero),
            "CC-BY-SA-3.0-IGO" => Ok(Self::CCBYSAThreeDotZeroIGO),
            "Saxpath" => Ok(Self::Saxpath),
            "OPUBL-1.0" => Ok(Self::OPUBLOneDotZero),
            "Linux-man-pages-copyleft-2-para" => Ok(Self::LinuxManPagesCopyleftTwoPara),
            "CC-BY-3.0-AT" => Ok(Self::CCBYThreeDotZeroAT),
            "GNAT-exception" => Ok(Self::GNATException),
            "CERN-OHL-P-2.0" => Ok(Self::CERNOHLPTwoDotZero),
            "x11vnc-openssl-exception" => Ok(Self::XOneOnevncOpensslException),
            "FSFUL" => Ok(Self::FSFUL),
            "OPL-UK-3.0" => Ok(Self::OPLUKThreeDotZero),
            "UMich-Merit" => Ok(Self::UMichMerit),
            "Cornell-Lossless-JPEG" => Ok(Self::CornellLosslessJPEG),
            "OGDL-Taiwan-1.0" => Ok(Self::OGDLTaiwanOneDotZero),
            "Plexus" => Ok(Self::Plexus),
            "CC-BY-NC-SA-2.0-UK" => Ok(Self::CCBYNCSATwoDotZeroUK),
            "SGI-B-2.0" => Ok(Self::SGIBTwoDotZero),
            "CC-BY-3.0-IGO" => Ok(Self::CCBYThreeDotZeroIGO),
            "Hippocratic-2.1" => Ok(Self::HippocraticTwoDotOne),
            "SHL-2.1" => Ok(Self::SHLTwoDotOne),
            "KiCad-libraries-exception" => Ok(Self::KiCadLibrariesException),
            "CPOL-1.02" => Ok(Self::CPOLOneDotZeroTwo),
            "deprecated_GPL-1.0" => Ok(Self::DeprecatedGPLOneDotZero),
            "Cronyx" => Ok(Self::Cronyx),
            "Latex2e-translated-notice" => Ok(Self::LatexTwoeTranslatedNotice),
            "Knuth-CTAN" => Ok(Self::KnuthCTAN),
            "CC-BY-3.0" => Ok(Self::CCBYThreeDotZero),
            "bzip2-1.0.6" => Ok(Self::BzipTwoOneDotZeroDotSix),
            "OCCT-PL" => Ok(Self::OCCTPL),
            "Sendmail-8.23" => Ok(Self::SendmailEightDotTwoThree),
            "Catharon" => Ok(Self::Catharon),
            "IPL-1.0" => Ok(Self::IPLOneDotZero),
            "deprecated_GPL-2.0-with-font-exception" => {
                Ok(Self::DeprecatedGPLTwoDotZeroWithFontException)
            }
            "Motosoto" => Ok(Self::Motosoto),
            "check-cvs" => Ok(Self::CheckCvs),
            "Sendmail" => Ok(Self::Sendmail),
            "PPL" => Ok(Self::PPL),
            "PostgreSQL" => Ok(Self::PostgreSql),
            "CDL-1.0" => Ok(Self::CDLOneDotZero),
            "GPL-1.0-or-later" => Ok(Self::GPLOneDotZeroOrLater),
            "CC-SA-1.0" => Ok(Self::CCSAOneDotZero),
            "IBM-pibs" => Ok(Self::IBMPibs),
            "CERN-OHL-1.2" => Ok(Self::CERNOHLOneDotTwo),
            "Intel-ACPI" => Ok(Self::IntelACPI),
            "deprecated_LGPL-2.0+" => Ok(Self::DeprecatedLGPLTwoDotZeroPlus),
            "SPL-1.0" => Ok(Self::SPLOneDotZero),
            "OML" => Ok(Self::OML),
            "DOC" => Ok(Self::DOC),
            "MIT-Click" => Ok(Self::MITClick),
            "mxml-exception" => Ok(Self::MxmlException),
            "NCGL-UK-2.0" => Ok(Self::NCGLUKTwoDotZero),
            "HTMLTIDY" => Ok(Self::HTMLTIDY),
            "LAL-1.3" => Ok(Self::LALOneDotThree),
            "BSD-Inferno-Nettverk" => Ok(Self::BSDInfernoNettverk),
            "RPL-1.5" => Ok(Self::RPLOneDotFive),
            "CC0-1.0" => Ok(Self::CcZeroOneDotZero),
            "CECILL-1.1" => Ok(Self::CECILLOneDotOne),
            "GCR-docs" => Ok(Self::GCRDocs),
            "Glide" => Ok(Self::Glide),
            "CC-BY-SA-2.0-UK" => Ok(Self::CCBYSATwoDotZeroUK),
            "Glulxe" => Ok(Self::Glulxe),
            "MIT-0" => Ok(Self::MITZero),
            "deprecated_LGPL-2.1" => Ok(Self::DeprecatedLGPLTwoDotOne),
            "eCos-exception-2.0" => Ok(Self::ECosExceptionTwoDotZero),
            "CERN-OHL-W-2.0" => Ok(Self::CERNOHLWTwoDotZero),
            "Minpack" => Ok(Self::Minpack),
            "HPND-Netrek" => Ok(Self::HPNDNetrek),
            "DocBook-Schema" => Ok(Self::DocBookSchema),
            "man2html" => Ok(Self::ManTwohtml),
            "LGPL-2.1-or-later" => Ok(Self::LGPLTwoDotOneOrLater),
            "CMU-Mach" => Ok(Self::CMUMach),
            "OLDAP-2.0" => Ok(Self::OLDAPTwoDotZero),
            "PHP-3.0" => Ok(Self::PHPThreeDotZero),
            "HPND-DEC" => Ok(Self::HPNDDEC),
            "radvd" => Ok(Self::Radvd),
            "OGTSL" => Ok(Self::OGTSL),
            "OLDAP-2.7" => Ok(Self::OLDAPTwoDotSeven),
            "OGL-UK-2.0" => Ok(Self::OGLUKTwoDotZero),
            "LGPL-3.0-linking-exception" => Ok(Self::LGPLThreeDotZeroLinkingException),
            "TTYP0" => Ok(Self::TtypZero),
            "Zed" => Ok(Self::Zed),
            "PADL" => Ok(Self::PADL),
            "OFL-1.1-RFN" => Ok(Self::OFLOneDotOneRFN),
            "OLDAP-2.2.2" => Ok(Self::OLDAPTwoDotTwoDotTwo),
            "diffmark" => Ok(Self::Diffmark),
            "W3C" => Ok(Self::WThreeC),
            "HPND-sell-variant-MIT-disclaimer" => Ok(Self::HPNDSellVariantMITDisclaimer),
            "OCaml-LGPL-linking-exception" => Ok(Self::OCamlLGPLLinkingException),
            "Python-2.0" => Ok(Self::PythonTwoDotZero),
            "Mackerras-3-Clause-acknowledgment" => Ok(Self::MackerrasThreeClauseAcknowledgment),
            "CC-BY-NC-ND-1.0" => Ok(Self::CCBYNCNDOneDotZero),
            "CC-BY-ND-1.0" => Ok(Self::CCBYNDOneDotZero),
            "Mup" => Ok(Self::Mup),
            "TPDL" => Ok(Self::TPDL),
            "HIDAPI" => Ok(Self::HIDAPI),
            "Info-ZIP" => Ok(Self::InfoZIP),
            "deprecated_LGPL-3.0" => Ok(Self::DeprecatedLGPLThreeDotZero),
            "HP-1989" => Ok(Self::HPOneNineEightNine),
            "FSFULLRWD" => Ok(Self::FSFULLRWD),
            "Leptonica" => Ok(Self::Leptonica),
            "Graphics-Gems" => Ok(Self::GraphicsGems),
            "Unicode-DFS-2015" => Ok(Self::UnicodeDFSTwoZeroOneFive),
            "TrustedQSL" => Ok(Self::TrustedQsl),
            "NLPL" => Ok(Self::NLPL),
            "Giftware" => Ok(Self::Giftware),
            "SSPL-1.0" => Ok(Self::SSPLOneDotZero),
            "CC-BY-2.5-AU" => Ok(Self::CCBYTwoDotFiveAU),
            "HPND-sell-variant-MIT-disclaimer-rev" => Ok(Self::HPNDSellVariantMITDisclaimerRev),
            "deprecated_GPL-1.0+" => Ok(Self::DeprecatedGPLOneDotZeroPlus),
            "libtiff" => Ok(Self::Libtiff),
            "IPA" => Ok(Self::IPA),
            "CC-BY-SA-1.0" => Ok(Self::CCBYSAOneDotZero),
            "CC-BY-NC-SA-1.0" => Ok(Self::CCBYNCSAOneDotZero),
            "ErlPL-1.1" => Ok(Self::ErlPlOneDotOne),
            "MPL-2.0" => Ok(Self::MPLTwoDotZero),
            "pkgconf" => Ok(Self::Pkgconf),
            "BSD-Protection" => Ok(Self::BSDProtection),
            "GFDL-1.2-no-invariants-only" => Ok(Self::GFDLOneDotTwoNoInvariantsOnly),
            "EPL-1.0" => Ok(Self::EPLOneDotZero),
            "deprecated_LGPL-2.1+" => Ok(Self::DeprecatedLGPLTwoDotOnePlus),
            "i2p-gpl-java-exception" => Ok(Self::ITwopGplJavaException),
            "deprecated_GPL-3.0-with-GCC-exception" => {
                Ok(Self::DeprecatedGPLThreeDotZeroWithGCCException)
            }
            "Jam" => Ok(Self::Jam),
            "deprecated_AGPL-1.0" => Ok(Self::DeprecatedAGPLOneDotZero),
            "Qhull" => Ok(Self::Qhull),
            "OpenSSL-standalone" => Ok(Self::OpenSslStandalone),
            "PCRE2-exception" => Ok(Self::PcreTwoException),
            "NBPL-1.0" => Ok(Self::NBPLOneDotZero),
            "MulanPSL-2.0" => Ok(Self::MulanPslTwoDotZero),
            "CC-BY-3.0-US" => Ok(Self::CCBYThreeDotZeroUS),
            "Lucida-Bitmap-Fonts" => Ok(Self::LucidaBitmapFonts),
            "CC-BY-NC-SA-2.0-FR" => Ok(Self::CCBYNCSATwoDotZeroFR),
            "MIT-advertising" => Ok(Self::MITAdvertising),
            "HPND-export-US-modify" => Ok(Self::HPNDExportUSModify),
            "swrule" => Ok(Self::Swrule),
            "SMLNJ" => Ok(Self::SMLNJ),
            "MPEG-SSG" => Ok(Self::MPEGSSG),
            "PolyForm-Small-Business-1.0.0" => Ok(Self::PolyFormSmallBusinessOneDotZeroDotZero),
            "ZPL-2.0" => Ok(Self::ZPLTwoDotZero),
            "xpp" => Ok(Self::Xpp),
            "Nokia" => Ok(Self::Nokia),
            "HPND-Kevlin-Henney" => Ok(Self::HPNDKevlinHenney),
            "PolyForm-Noncommercial-1.0.0" => Ok(Self::PolyFormNoncommercialOneDotZeroDotZero),
            "CC-BY-SA-2.1-JP" => Ok(Self::CCBYSATwoDotOneJP),
            "xkeyboard-config-Zinoviev" => Ok(Self::XkeyboardConfigZinoviev),
            "NAIST-2003" => Ok(Self::NAISTTwoZeroZeroThree),
            "HPND-export-US" => Ok(Self::HPNDExportUS),
            "GStreamer-exception-2008" => Ok(Self::GStreamerExceptionTwoZeroZeroEight),
            "deprecated_BSD-2-Clause-NetBSD" => Ok(Self::DeprecatedBSDTwoClauseNetBsd),
            "WxWindows-exception-3.1" => Ok(Self::WxWindowsExceptionThreeDotOne),
            "libselinux-1.0" => Ok(Self::LibselinuxOneDotZero),
            "QPL-1.0" => Ok(Self::QPLOneDotZero),
            "Pixar" => Ok(Self::Pixar),
            "Fawkes-Runtime-exception" => Ok(Self::FawkesRuntimeException),
            "CC-PDDC" => Ok(Self::CCPDDC),
            "Xerox" => Ok(Self::Xerox),
            "BUSL-1.1" => Ok(Self::BUSLOneDotOne),
            "MakeIndex" => Ok(Self::MakeIndex),
            "LGPL-3.0-or-later" => Ok(Self::LGPLThreeDotZeroOrLater),
            "deprecated_Nunit" => Ok(Self::DeprecatedNunit),
            "OFFIS" => Ok(Self::OFFIS),
            "Latex2e" => Ok(Self::LatexTwoe),
            "SSH-OpenSSH" => Ok(Self::SSHOpenSsh),
            "Entessa" => Ok(Self::Entessa),
            "deprecated_LGPL-2.0" => Ok(Self::DeprecatedLGPLTwoDotZero),
            "Zend-2.0" => Ok(Self::ZendTwoDotZero),
            "mif-exception" => Ok(Self::MifException),
            "Xfig" => Ok(Self::Xfig),
            "Caldera" => Ok(Self::Caldera),
            "libutil-David-Nugent" => Ok(Self::LibutilDavidNugent),
            "OLDAP-2.6" => Ok(Self::OLDAPTwoDotSix),
            "GNU-compiler-exception" => Ok(Self::GNUCompilerException),
            "SGI-OpenGL" => Ok(Self::SGIOpenGl),
            "CC-PDM-1.0" => Ok(Self::CCPDMOneDotZero),
            "MIT-enna" => Ok(Self::MITEnna),
            "NIST-PD" => Ok(Self::NISTPD),
            "CC-BY-2.5" => Ok(Self::CCBYTwoDotFive),
            "HPND-sell-regexpr" => Ok(Self::HPNDSellRegexpr),
            "LOOP" => Ok(Self::LOOP),
            "OLDAP-2.1" => Ok(Self::OLDAPTwoDotOne),
            "SISSL-1.2" => Ok(Self::SISSLOneDotTwo),
            "OCCT-exception-1.0" => Ok(Self::OCCTExceptionOneDotZero),
            "SL" => Ok(Self::SL),
            "FLTK-exception" => Ok(Self::FLTKException),
            "GFDL-1.1-only" => Ok(Self::GFDLOneDotOneOnly),
            "CAL-1.0-Combined-Work-Exception" => Ok(Self::CALOneDotZeroCombinedWorkException),
            "deprecated_StandardML-NJ" => Ok(Self::DeprecatedStandardMlNJ),
            "ZPL-2.1" => Ok(Self::ZPLTwoDotOne),
            "Imlib2" => Ok(Self::ImlibTwo),
            "RPL-1.1" => Ok(Self::RPLOneDotOne),
            "gnuplot" => Ok(Self::Gnuplot),
            "D-FSL-1.0" => Ok(Self::DFSLOneDotZero),
            "OpenSSL" => Ok(Self::OpenSsl),
            "GPL-3.0-or-later" => Ok(Self::GPLThreeDotZeroOrLater),
            "OSET-PL-2.1" => Ok(Self::OSETPLTwoDotOne),
            "LZMA-SDK-9.11-to-9.20" => Ok(Self::LZMASDKNineDotOneOneToNineDotTwoZero),
            "SAX-PD-2.0" => Ok(Self::SAXPDTwoDotZero),
            "NASA-1.3" => Ok(Self::NASAOneDotThree),
            "EUDatagrid" => Ok(Self::EuDatagrid),
            "CERN-OHL-1.1" => Ok(Self::CERNOHLOneDotOne),
            "etalab-2.0" => Ok(Self::EtalabTwoDotZero),
            "deprecated_GPL-3.0-with-autoconf-exception" => {
                Ok(Self::DeprecatedGPLThreeDotZeroWithAutoconfException)
            }
            "CERN-OHL-S-2.0" => Ok(Self::CERNOHLSTwoDotZero),
            "Gutmann" => Ok(Self::Gutmann),
            "OLFL-1.3" => Ok(Self::OLFLOneDotThree),
            "Linux-man-pages-copyleft-var" => Ok(Self::LinuxManPagesCopyleftVar),
            "OGL-UK-3.0" => Ok(Self::OGLUKThreeDotZero),
            "Parity-6.0.0" => Ok(Self::ParitySixDotZeroDotZero),
            "LPPL-1.3a" => Ok(Self::LPPLOneDotThreea),
            "mplus" => Ok(Self::Mplus),
            "COIL-1.0" => Ok(Self::COILOneDotZero),
            "gtkbook" => Ok(Self::Gtkbook),
            "XSkat" => Ok(Self::XSkat),
            "HPND-doc-sell" => Ok(Self::HPNDDocSell),
            "RSA-MD" => Ok(Self::RSAMD),
            "CC-BY-SA-3.0-AT" => Ok(Self::CCBYSAThreeDotZeroAT),
            "NIST-PD-fallback" => Ok(Self::NISTPDFallback),
            "iMatix" => Ok(Self::IMatix),
            "GPL-3.0-only" => Ok(Self::GPLThreeDotZeroOnly),
            "CryptoSwift" => Ok(Self::CryptoSwift),
            "WTFPL" => Ok(Self::WTFPL),
            "cve-tou" => Ok(Self::CveTou),
            "HPND-MIT-disclaimer" => Ok(Self::HPNDMITDisclaimer),
            "CC-BY-ND-2.0" => Ok(Self::CCBYNDTwoDotZero),
            "Vim" => Ok(Self::Vim),
            "NIST-Software" => Ok(Self::NISTSoftware),
            "LPPL-1.0" => Ok(Self::LPPLOneDotZero),
            "YPL-1.1" => Ok(Self::YPLOneDotOne),
            "CC-BY-NC-ND-2.0" => Ok(Self::CCBYNCNDTwoDotZero),
            "copyleft-next-0.3.1" => Ok(Self::CopyleftNextZeroDotThreeDotOne),
            "MIT-CMU" => Ok(Self::MITCMU),
            "RPSL-1.0" => Ok(Self::RPSLOneDotZero),
            "dtoa" => Ok(Self::Dtoa),
            "NCSA" => Ok(Self::NCSA),
            "NPL-1.1" => Ok(Self::NPLOneDotOne),
            "SCEA" => Ok(Self::SCEA),
            "SMPPL" => Ok(Self::SMPPL),
            "LiLiQ-R-1.1" => Ok(Self::LiLiQROneDotOne),
            "OFL-1.0-RFN" => Ok(Self::OFLOneDotZeroRFN),
            "NPOSL-3.0" => Ok(Self::NPOSLThreeDotZero),
            "ImageMagick" => Ok(Self::ImageMagick),
            "libpri-OpenH323-exception" => Ok(Self::LibpriOpenHThreeTwoThreeException),
            "Unicode-TOU" => Ok(Self::UnicodeTOU),
            "OpenPBS-2.3" => Ok(Self::OpenPbsTwoDotThree),
            "UCL-1.0" => Ok(Self::UCLOneDotZero),
            "Zimbra-1.4" => Ok(Self::ZimbraOneDotFour),
            "TGPPL-1.0" => Ok(Self::TGPPLOneDotZero),
            "Wsuipa" => Ok(Self::Wsuipa),
            "deprecated_Nokia-Qt-exception-1.1" => Ok(Self::DeprecatedNokiaQtExceptionOneDotOne),
            "CC-BY-NC-SA-2.0-DE" => Ok(Self::CCBYNCSATwoDotZeroDE),
            "Linux-man-pages-copyleft" => Ok(Self::LinuxManPagesCopyleft),
            "xlock" => Ok(Self::Xlock),
            "LiLiQ-Rplus-1.1" => Ok(Self::LiLiQRplusOneDotOne),
            "generic-xts" => Ok(Self::GenericXts),
            "Zimbra-1.3" => Ok(Self::ZimbraOneDotThree),
            "GPL-2.0-only" => Ok(Self::GPLTwoDotZeroOnly),
            "OGL-UK-1.0" => Ok(Self::OGLUKOneDotZero),
            "SSLeay-standalone" => Ok(Self::SsLeayStandalone),
            "X11-swapped" => Ok(Self::XOneOneSwapped),
            "OpenJDK-assembly-exception-1.0" => Ok(Self::OpenJdkAssemblyExceptionOneDotZero),
            "GFDL-1.3-no-invariants-or-later" => Ok(Self::GFDLOneDotThreeNoInvariantsOrLater),
            "MTLL" => Ok(Self::MTLL),
            "Ubuntu-font-1.0" => Ok(Self::UbuntuFontOneDotZero),
            "DocBook-XML" => Ok(Self::DocBookXML),
            "MIT-testregex" => Ok(Self::MITTestregex),
            "CC-BY-NC-2.5" => Ok(Self::CCBYNCTwoDotFive),
            "python-ldap" => Ok(Self::PythonLdap),
            "GL2PS" => Ok(Self::GlTwoPs),
            "LPL-1.02" => Ok(Self::LPLOneDotZeroTwo),
            "MITNFA" => Ok(Self::MITNFA),
            "checkmk" => Ok(Self::Checkmk),
            "GFDL-1.2-only" => Ok(Self::GFDLOneDotTwoOnly),
            "NGPL" => Ok(Self::NGPL),
            "MulanPSL-1.0" => Ok(Self::MulanPslOneDotZero),
            "deprecated_wxWindows" => Ok(Self::DeprecatedWxWindows),
            "OGC-1.0" => Ok(Self::OGCOneDotZero),
            "ulem" => Ok(Self::Ulem),
            "harbour-exception" => Ok(Self::HarbourException),
            "UCAR" => Ok(Self::UCAR),
            "MS-PL" => Ok(Self::MSPL),
            "JPL-image" => Ok(Self::JPLImage),
            "Font-exception-2.0" => Ok(Self::FontExceptionTwoDotZero),
            "GFDL-1.3-invariants-or-later" => Ok(Self::GFDLOneDotThreeInvariantsOrLater),
            "fwlw" => Ok(Self::Fwlw),
            "Inner-Net-2.0" => Ok(Self::InnerNetTwoDotZero),
            "MPL-1.0" => Ok(Self::MPLOneDotZero),
            "Community-Spec-1.0" => Ok(Self::CommunitySpecOneDotZero),
            "CUA-OPL-1.0" => Ok(Self::CUAOPLOneDotZero),
            "UBDL-exception" => Ok(Self::UBDLException),
            "GFDL-1.1-invariants-only" => Ok(Self::GFDLOneDotOneInvariantsOnly),
            "FreeBSD-DOC" => Ok(Self::FreeBsdDOC),
            "EPL-2.0" => Ok(Self::EPLTwoDotZero),
            "Sendmail-Open-Source-1.1" => Ok(Self::SendmailOpenSourceOneDotOne),
            "Eurosym" => Ok(Self::Eurosym),
            "GPL-1.0-only" => Ok(Self::GPLOneDotZeroOnly),
            "deprecated_BSD-2-Clause-FreeBSD" => Ok(Self::DeprecatedBSDTwoClauseFreeBsd),
            "SHL-0.5" => Ok(Self::SHLZeroDotFive),
            "X11" => Ok(Self::XOneOne),
            "ThirdEye" => Ok(Self::ThirdEye),
            "FSFAP-no-warranty-disclaimer" => Ok(Self::FSFAPNoWarrantyDisclaimer),
            "SimPL-2.0" => Ok(Self::SimPlTwoDotZero),
            "Interbase-1.0" => Ok(Self::InterbaseOneDotZero),
            "deprecated_bzip2-1.0.5" => Ok(Self::DeprecatedBzipTwoOneDotZeroDotFive),
            "Multics" => Ok(Self::Multics),
            "GStreamer-exception-2005" => Ok(Self::GStreamerExceptionTwoZeroZeroFive),
            "CC-BY-NC-SA-2.0" => Ok(Self::CCBYNCSATwoDotZero),
            "GD" => Ok(Self::GD),
            "CC-BY-SA-2.0" => Ok(Self::CCBYSATwoDotZero),
            "CECILL-C" => Ok(Self::CECILLC),
            "Elastic-2.0" => Ok(Self::ElasticTwoDotZero),
            "MIT-Wu" => Ok(Self::MITWu),
            "snprintf" => Ok(Self::Snprintf),
            "EUPL-1.0" => Ok(Self::EUPLOneDotZero),
            "BSD-Source-beginning-file" => Ok(Self::BSDSourceBeginningFile),
            "SunPro" => Ok(Self::SunPro),
            "HPND-Intel" => Ok(Self::HPNDIntel),
            "LZMA-SDK-9.22" => Ok(Self::LZMASDKNineDotTwoTwo),
            "OFL-1.0" => Ok(Self::OFLOneDotZero),
            "deprecated_GPL-2.0-with-GCC-exception" => {
                Ok(Self::DeprecatedGPLTwoDotZeroWithGCCException)
            }
            "CC-BY-SA-3.0" => Ok(Self::CCBYSAThreeDotZero),
            "CC-BY-NC-SA-3.0" => Ok(Self::CCBYNCSAThreeDotZero),
            "DSDP" => Ok(Self::DSDP),
            "McPhee-slideshow" => Ok(Self::McPheeSlideshow),
            "CC-BY-3.0-NL" => Ok(Self::CCBYThreeDotZeroNL),
            "OLDAP-1.1" => Ok(Self::OLDAPOneDotOne),
            "GFDL-1.2-invariants-only" => Ok(Self::GFDLOneDotTwoInvariantsOnly),
            "SchemeReport" => Ok(Self::SchemeReport),
            "HPND-export2-US" => Ok(Self::HPNDExportTwoUS),
            "xinetd" => Ok(Self::Xinetd),
            "HaskellReport" => Ok(Self::HaskellReport),
            "TCL" => Ok(Self::TCL),
            "NetCDF" => Ok(Self::NetCdf),
            "Symlinks" => Ok(Self::Symlinks),
            "openvpn-openssl-exception" => Ok(Self::OpenvpnOpensslException),
            "HPND-Pbmplus" => Ok(Self::HPNDPbmplus),
            "OAR" => Ok(Self::OAR),
            "copyleft-next-0.3.0" => Ok(Self::CopyleftNextZeroDotThreeDotZero),
            "YPL-1.0" => Ok(Self::YPLOneDotZero),
            "LPPL-1.1" => Ok(Self::LPPLOneDotOne),
            "CC-BY-ND-4.0" => Ok(Self::CCBYNDFourDotZero),
            "XFree86-1.1" => Ok(Self::XFreeEightSixOneDotOne),
            "CC-BY-NC-ND-4.0" => Ok(Self::CCBYNCNDFourDotZero),
            "CC-BY-NC-SA-3.0-DE" => Ok(Self::CCBYNCSAThreeDotZeroDE),
            "TPL-1.0" => Ok(Self::TPLOneDotZero),
            "Naumen" => Ok(Self::Naumen),
            "NICTA-1.0" => Ok(Self::NICTAOneDotZero),
            "NOSL" => Ok(Self::NOSL),
            "pnmstitch" => Ok(Self::Pnmstitch),
            "CPL-1.0" => Ok(Self::CPLOneDotZero),
            "xzoom" => Ok(Self::Xzoom),
            "IEC-Code-Components-EULA" => Ok(Self::IECCodeComponentsEULA),
            "deprecated_AGPL-3.0" => Ok(Self::DeprecatedAGPLThreeDotZero),
            "RSCPL" => Ok(Self::RSCPL),
            "NPL-1.0" => Ok(Self::NPLOneDotZero),
            "Sleepycat" => Ok(Self::Sleepycat),
            "CDLA-Sharing-1.0" => Ok(Self::CDLASharingOneDotZero),
            "GFDL-1.3-only" => Ok(Self::GFDLOneDotThreeOnly),
            "lsof" => Ok(Self::Lsof),
            "Parity-7.0.0" => Ok(Self::ParitySevenDotZeroDotZero),
            "Zeeff" => Ok(Self::Zeeff),
            "CC-BY-NC-SA-4.0" => Ok(Self::CCBYNCSAFourDotZero),
            "CC-BY-SA-4.0" => Ok(Self::CCBYSAFourDotZero),
            "GFDL-1.2-or-later" => Ok(Self::GFDLOneDotTwoOrLater),
            "OFL-1.1" => Ok(Self::OFLOneDotOne),
            "GPL-3.0-interface-exception" => Ok(Self::GPLThreeDotZeroInterfaceException),
            "Qt-LGPL-exception-1.1" => Ok(Self::QtLGPLExceptionOneDotOne),
            "Mackerras-3-Clause" => Ok(Self::MackerrasThreeClause),
            "EUPL-1.1" => Ok(Self::EUPLOneDotOne),
            "Sun-PPP" => Ok(Self::SunPPP),
            "CECILL-B" => Ok(Self::CECILLB),
            "Linux-OpenIB" => Ok(Self::LinuxOpenIb),
            "fmt-exception" => Ok(Self::FmtException),
            "MIT-feh" => Ok(Self::MITFeh),
            "Ruby-pty" => Ok(Self::RubyPty),
            "HPND-sell-variant" => Ok(Self::HPNDSellVariant),
            "DEC-3-Clause" => Ok(Self::DECThreeClause),
            "ZPL-1.1" => Ok(Self::ZPLOneDotOne),
            "Zlib" => Ok(Self::Zlib),
            "FTL" => Ok(Self::FTL),
            "CC-BY-SA-3.0-DE" => Ok(Self::CCBYSAThreeDotZeroDE),
            "CC-BY-NC-ND-3.0" => Ok(Self::CCBYNCNDThreeDotZero),
            "TOSL" => Ok(Self::TOSL),
            "FBM" => Ok(Self::FBM),
            "CC-BY-ND-3.0" => Ok(Self::CCBYNDThreeDotZero),
            "C-UDA-1.0" => Ok(Self::CUDAOneDotZero),
            "Qt-GPL-exception-1.0" => Ok(Self::QtGPLExceptionOneDotZero),
            "Sun-PPP-2000" => Ok(Self::SunPPPTwoZeroZeroZero),
            "OpenVision" => Ok(Self::OpenVision),
            "deprecated_GFDL-1.1" => Ok(Self::DeprecatedGFDLOneDotOne),
            "ssh-keyscan" => Ok(Self::SshKeyscan),
            "SSH-short" => Ok(Self::SSHShort),
            "gSOAP-1.3b" => Ok(Self::GSoapOneDotThreeb),
            "JPNIC" => Ok(Self::JPNIC),
            "GFDL-1.1-no-invariants-only" => Ok(Self::GFDLOneDotOneNoInvariantsOnly),
            "TAPR-OHL-1.0" => Ok(Self::TAPROHLOneDotZero),
            "Rdisc" => Ok(Self::Rdisc),
            "HPND-doc" => Ok(Self::HPNDDoc),
            "URT-RLE" => Ok(Self::URTRLE),
            "mpich2" => Ok(Self::MpichTwo),
            "CGAL-linking-exception" => Ok(Self::CGALLinkingException),
            "MPL-1.1" => Ok(Self::MPLOneDotOne),
            _ => Err(ParseLicenseError),
        }
    }
}
