#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ipgr1: IPGR1,
    ipgr2: IPGR2,
    ipgr3: IPGR3,
    _reserved3: [u8; 0x10],
    ipgr8: IPGR8,
    ipc1r1: IPC1R1,
    ipc1r2: IPC1R2,
    ipc1r3: IPC1R3,
    _reserved7: [u8; 0x04],
    ipc2r1: IPC2R1,
    ipc2r2: IPC2R2,
    ipc2r3: IPC2R3,
    _reserved10: [u8; 0x04],
    ipc3r1: IPC3R1,
    ipc3r2: IPC3R2,
    ipc3r3: IPC3R3,
    _reserved13: [u8; 0x04],
    ipc4r1: IPC4R1,
    ipc4r2: IPC4R2,
    ipc4r3: IPC4R3,
    _reserved16: [u8; 0x04],
    ipc5r1: IPC5R1,
    ipc5r2: IPC5R2,
    ipc5r3: IPC5R3,
    _reserved19: [u8; 0x98],
    prcr: PRCR,
    prescr: PRESCR,
    presur: PRESUR,
    _reserved22: [u8; 0xe4],
    prier: PRIER,
    prsr: PRSR,
    prfcr: PRFCR,
    _reserved25: [u8; 0x04],
    cmcr: CMCR,
    cmfrcr: CMFRCR,
    _reserved27: [u8; 0x01e4],
    cmier: CMIER,
    cmsr1: CMSR1,
    cmsr2: CMSR2,
    cmfcr: CMFCR,
    _reserved31: [u8; 0x04],
    p0fscr: P0FSCR,
    _reserved32: [u8; 0xf8],
    p0fctcr: P0FCTCR,
    p0scstr: P0SCSTR,
    p0scszr: P0SCSZR,
    _reserved35: [u8; 0xa4],
    p0dccntr: P0DCCNTR,
    p0dclmtr: P0DCLMTR,
    _reserved37: [u8; 0x08],
    p0ppcr: P0PPCR,
    p0ppm0ar1: P0PPM0AR1,
    p0ppm0ar2: P0PPM0AR2,
    _reserved40: [u8; 0x04],
    p0stm0ar: P0STM0AR,
    _reserved41: [u8; 0x20],
    p0ier: P0IER,
    p0sr: P0SR,
    p0fcr: P0FCR,
    _reserved44: [u8; 0x04],
    p0cfscr: P0CFSCR,
    _reserved45: [u8; 0xf8],
    p0cfctcr: P0CFCTCR,
    p0cscstr: P0CSCSTR,
    p0cscszr: P0CSCSZR,
    _reserved48: [u8; 0xb4],
    p0cppcr: P0CPPCR,
    p0cppm0ar1: P0CPPM0AR1,
    p0cppm0ar2: P0CPPM0AR2,
    _reserved51: [u8; 0x38],
    p1fscr: P1FSCR,
    _reserved52: [u8; 0x18],
    p1srcr: P1SRCR,
    p1bprcr: P1BPRCR,
    p1bprsr: P1BPRSR,
    _reserved55: [u8; 0x04],
    p1decr: P1DECR,
    _reserved56: [u8; 0x0c],
    p1blccr: P1BLCCR,
    p1excr1: P1EXCR1,
    p1excr2: P1EXCR2,
    _reserved59: [u8; 0x04],
    p1st1cr: P1ST1CR,
    p1st2cr: P1ST2CR,
    p1st3cr: P1ST3CR,
    p1ststr: P1STSTR,
    p1stszr: P1STSZR,
    p1st1sr: P1ST1SR,
    p1st2sr: P1ST2SR,
    p1st3sr: P1ST3SR,
    p1dmcr: P1DMCR,
    _reserved68: [u8; 0x0c],
    p1cccr: P1CCCR,
    p1ccrr1: P1CCRR1,
    p1ccrr2: P1CCRR2,
    p1ccgr1: P1CCGR1,
    p1ccgr2: P1CCGR2,
    p1ccbr1: P1CCBR1,
    p1ccbr2: P1CCBR2,
    _reserved75: [u8; 0x04],
    p1ctcr1: P1CTCR1,
    p1ctcr2: P1CTCR2,
    p1ctcr3: P1CTCR3,
    _reserved78: [u8; 0x54],
    p1fctcr: P1FCTCR,
    p1crstr: P1CRSTR,
    p1crszr: P1CRSZR,
    p1dccr: P1DCCR,
    p1dscr: P1DSCR,
    p1dsrtior: P1DSRTIOR,
    p1dsszr: P1DSSZR,
    _reserved85: [u8; 0x04],
    p1cmricr: P1CMRICR,
    p1ri1cr1: P1RI1CR1,
    p1ri1cr2: P1RI1CR2,
    p1ri2cr1: P1RI2CR1,
    p1ri2cr2: P1RI2CR2,
    p1ri3cr1: P1RI3CR1,
    p1ri3cr2: P1RI3CR2,
    p1ri4cr1: P1RI4CR1,
    p1ri4cr2: P1RI4CR2,
    p1ri5cr1: P1RI5CR1,
    p1ri5cr2: P1RI5CR2,
    p1ri6cr1: P1RI6CR1,
    p1ri6cr2: P1RI6CR2,
    p1ri7cr1: P1RI7CR1,
    p1ri7cr2: P1RI7CR2,
    p1ri8cr1: P1RI8CR1,
    p1ri8cr2: P1RI8CR2,
    _reserved102: [u8; 0x0c],
    p1gmcr: P1GMCR,
    _reserved103: [u8; 0x0c],
    p1yuvcr: P1YUVCR,
    p1yuvrr1: P1YUVRR1,
    p1yuvrr2: P1YUVRR2,
    p1yuvgr1: P1YUVGR1,
    p1yuvgr2: P1YUVGR2,
    p1yuvbr1: P1YUVBR1,
    p1yuvbr2: P1YUVBR2,
    _reserved110: [u8; 0x24],
    p1ppcr: P1PPCR,
    p1ppm0ar1: P1PPM0AR1,
    p1ppm0ar2: P1PPM0AR2,
    p1ppm0pr: P1PPM0PR,
    p1stm0ar: P1STM0AR,
    p1ppm1ar1: P1PPM1AR1,
    p1ppm1ar2: P1PPM1AR2,
    p1ppm1pr: P1PPM1PR,
    p1stm1ar: P1STM1AR,
    p1ppm2ar1: P1PPM2AR1,
    p1ppm2ar2: P1PPM2AR2,
    _reserved121: [u8; 0x04],
    p1stm2ar: P1STM2AR,
    p1ier: P1IER,
    p1sr: P1SR,
    p1fcr: P1FCR,
    _reserved125: [u8; 0x04],
    p1cfscr: P1CFSCR,
    _reserved126: [u8; 0x1c],
    p1cbprcr: P1CBPRCR,
    _reserved127: [u8; 0x18],
    p1cblccr: P1CBLCCR,
    p1cexcr1: P1CEXCR1,
    p1cexcr2: P1CEXCR2,
    _reserved130: [u8; 0x04],
    p1cst1cr: P1CST1CR,
    p1cst2cr: P1CST2CR,
    p1cst3cr: P1CST3CR,
    p1cststr: P1CSTSTR,
    p1cstszr: P1CSTSZR,
    _reserved135: [u8; 0x1c],
    p1ccccr: P1CCCCR,
    p1cccrr1: P1CCCRR1,
    p1cccrr2: P1CCCRR2,
    p1cccgr1: P1CCCGR1,
    p1cccgr2: P1CCCGR2,
    p1cccbr1: P1CCCBR1,
    p1cccbr2: P1CCCBR2,
    _reserved142: [u8; 0x04],
    p1cctcr1: P1CCTCR1,
    p1cctcr2: P1CCTCR2,
    p1cctcr3: P1CCTCR3,
    _reserved145: [u8; 0x54],
    p1cfctcr: P1CFCTCR,
    p1ccrstr: P1CCRSTR,
    p1ccrszr: P1CCRSZR,
    p1cdccr: P1CDCCR,
    p1cdscr: P1CDSCR,
    p1cdsrtior: P1CDSRTIOR,
    p1cdsszr: P1CDSSZR,
    _reserved152: [u8; 0x04],
    p1ccmricr: P1CCMRICR,
    p1cri1cr1: P1CRI1CR1,
    p1cri1cr2: P1CRI1CR2,
    p1cri2cr1: P1CRI2CR1,
    p1cri2cr2: P1CRI2CR2,
    p1cri3cr1: P1CRI3CR1,
    p1cri3cr2: P1CRI3CR2,
    p1cri4cr1: P1CRI4CR1,
    p1cri4cr2: P1CRI4CR2,
    p1cri5cr1: P1CRI5CR1,
    p1cri5cr2: P1CRI5CR2,
    p1cri6cr1: P1CRI6CR1,
    p1cri6cr2: P1CRI6CR2,
    p1cri7cr1: P1CRI7CR1,
    p1cri7cr2: P1CRI7CR2,
    p1cri8cr1: P1CRI8CR1,
    p1cri8cr2: P1CRI8CR2,
    _reserved169: [u8; 0x5c],
    p1cppcr: P1CPPCR,
    p1cppm0ar1: P1CPPM0AR1,
    p1cppm0ar2: P1CPPM0AR2,
    p1cppm0pr: P1CPPM0PR,
    _reserved173: [u8; 0x04],
    p1cppm1ar1: P1CPPM1AR1,
    p1cppm1ar2: P1CPPM1AR2,
    p1cppm1pr: P1CPPM1PR,
    _reserved176: [u8; 0x04],
    p1cppm2ar1: P1CPPM2AR1,
    p1cppm2ar2: P1CPPM2AR2,
    _reserved178: [u8; 0x18],
    p2fscr: P2FSCR,
    _reserved179: [u8; 0xf8],
    p2fctcr: P2FCTCR,
    p2crstr: P2CRSTR,
    p2crszr: P2CRSZR,
    p2dccr: P2DCCR,
    p2dscr: P2DSCR,
    p2dsrtior: P2DSRTIOR,
    p2dsszr: P2DSSZR,
    _reserved186: [u8; 0x04],
    p2cmricr: P2CMRICR,
    p2ri1cr1: P2RI1CR1,
    p2ri1cr2: P2RI1CR2,
    p2ri2cr1: P2RI2CR1,
    p2ri2cr2: P2RI2CR2,
    p2ri3cr1: P2RI3CR1,
    p2ri3cr2: P2RI3CR2,
    p2ri4cr1: P2RI4CR1,
    p2ri4cr2: P2RI4CR2,
    p2ri5cr1: P2RI5CR1,
    p2ri5cr2: P2RI5CR2,
    p2ri6cr1: P2RI6CR1,
    p2ri6cr2: P2RI6CR2,
    p2ri7cr1: P2RI7CR1,
    p2ri7cr2: P2RI7CR2,
    p2ri8cr1: P2RI8CR1,
    p2ri8cr2: P2RI8CR2,
    _reserved203: [u8; 0x0c],
    p2gmcr: P2GMCR,
    _reserved204: [u8; 0x4c],
    p2ppcr: P2PPCR,
    p2ppm0ar1: P2PPM0AR1,
    p2ppm0ar2: P2PPM0AR2,
    p2ppm0pr: P2PPM0PR,
    p2stm0ar: P2STM0AR,
    _reserved209: [u8; 0x20],
    p2ier: P2IER,
    p2sr: P2SR,
    p2fcr: P2FCR,
    _reserved212: [u8; 0x04],
    p2cfscr: P2CFSCR,
    _reserved213: [u8; 0xf8],
    p2cfctcr: P2CFCTCR,
    p2ccrstr: P2CCRSTR,
    p2ccrszr: P2CCRSZR,
    p2cdccr: P2CDCCR,
    p2cdscr: P2CDSCR,
    p2cdsrtior: P2CDSRTIOR,
    p2cdsszr: P2CDSSZR,
    _reserved220: [u8; 0x04],
    p2ccmricr: P2CCMRICR,
    p2cri1cr1: P2CRI1CR1,
    p2cri1cr2: P2CRI1CR2,
    p2cri2cr1: P2CRI2CR1,
    p2cri2cr2: P2CRI2CR2,
    p2cri3cr1: P2CRI3CR1,
    p2cri3cr2: P2CRI3CR2,
    p2cri4cr1: P2CRI4CR1,
    p2cri4cr2: P2CRI4CR2,
    p2cri5cr1: P2CRI5CR1,
    p2cri5cr2: P2CRI5CR2,
    p2cri6cr1: P2CRI6CR1,
    p2cri6cr2: P2CRI6CR2,
    p2cri7cr1: P2CRI7CR1,
    p2cri7cr2: P2CRI7CR2,
    p2cri8cr1: P2CRI8CR1,
    p2cri8cr2: P2CRI8CR2,
    _reserved237: [u8; 0x5c],
    p2cppcr: P2CPPCR,
    p2cppm0ar1: P2CPPM0AR1,
    p2cppm0ar2: P2CPPM0AR2,
}
impl RegisterBlock {
    ///0x00 - DCMIPP IPPLUG global register 1
    #[inline(always)]
    pub const fn ipgr1(&self) -> &IPGR1 {
        &self.ipgr1
    }
    ///0x04 - DCMIPP IPPLUG global register 2
    #[inline(always)]
    pub const fn ipgr2(&self) -> &IPGR2 {
        &self.ipgr2
    }
    ///0x08 - DCMIPP IPPLUG global register 3
    #[inline(always)]
    pub const fn ipgr3(&self) -> &IPGR3 {
        &self.ipgr3
    }
    ///0x1c - DCMIPP IPPLUG identification register
    #[inline(always)]
    pub const fn ipgr8(&self) -> &IPGR8 {
        &self.ipgr8
    }
    ///0x20 - DCMIPP IPPLUG Clientx register 1
    #[inline(always)]
    pub const fn ipc1r1(&self) -> &IPC1R1 {
        &self.ipc1r1
    }
    ///0x24 - DCMIPP IPPLUG Clientx register 2
    #[inline(always)]
    pub const fn ipc1r2(&self) -> &IPC1R2 {
        &self.ipc1r2
    }
    ///0x28 - DCMIPP IPPLUG Clientx register 3
    #[inline(always)]
    pub const fn ipc1r3(&self) -> &IPC1R3 {
        &self.ipc1r3
    }
    ///0x30 - DCMIPP IPPLUG Clientx register 1
    #[inline(always)]
    pub const fn ipc2r1(&self) -> &IPC2R1 {
        &self.ipc2r1
    }
    ///0x34 - DCMIPP IPPLUG Clientx register 2
    #[inline(always)]
    pub const fn ipc2r2(&self) -> &IPC2R2 {
        &self.ipc2r2
    }
    ///0x38 - DCMIPP IPPLUG Clientx register 3
    #[inline(always)]
    pub const fn ipc2r3(&self) -> &IPC2R3 {
        &self.ipc2r3
    }
    ///0x40 - DCMIPP IPPLUG Clientx register 1
    #[inline(always)]
    pub const fn ipc3r1(&self) -> &IPC3R1 {
        &self.ipc3r1
    }
    ///0x44 - DCMIPP IPPLUG Clientx register 2
    #[inline(always)]
    pub const fn ipc3r2(&self) -> &IPC3R2 {
        &self.ipc3r2
    }
    ///0x48 - DCMIPP IPPLUG Clientx register 3
    #[inline(always)]
    pub const fn ipc3r3(&self) -> &IPC3R3 {
        &self.ipc3r3
    }
    ///0x50 - DCMIPP IPPLUG Clientx register 1
    #[inline(always)]
    pub const fn ipc4r1(&self) -> &IPC4R1 {
        &self.ipc4r1
    }
    ///0x54 - DCMIPP IPPLUG Clientx register 2
    #[inline(always)]
    pub const fn ipc4r2(&self) -> &IPC4R2 {
        &self.ipc4r2
    }
    ///0x58 - DCMIPP IPPLUG Clientx register 3
    #[inline(always)]
    pub const fn ipc4r3(&self) -> &IPC4R3 {
        &self.ipc4r3
    }
    ///0x60 - DCMIPP IPPLUG Clientx register 1
    #[inline(always)]
    pub const fn ipc5r1(&self) -> &IPC5R1 {
        &self.ipc5r1
    }
    ///0x64 - DCMIPP IPPLUG Clientx register 2
    #[inline(always)]
    pub const fn ipc5r2(&self) -> &IPC5R2 {
        &self.ipc5r2
    }
    ///0x68 - DCMIPP IPPLUG Clientx register 3
    #[inline(always)]
    pub const fn ipc5r3(&self) -> &IPC5R3 {
        &self.ipc5r3
    }
    ///0x104 - DCMIPP parallel interface control register
    #[inline(always)]
    pub const fn prcr(&self) -> &PRCR {
        &self.prcr
    }
    ///0x108 - DCMIPP parallel interface embedded synchronization code register
    #[inline(always)]
    pub const fn prescr(&self) -> &PRESCR {
        &self.prescr
    }
    ///0x10c - DCMIPP parallel interface embedded synchronization unmask register
    #[inline(always)]
    pub const fn presur(&self) -> &PRESUR {
        &self.presur
    }
    ///0x1f4 - DCMIPP parallel interface interrupt enable register
    #[inline(always)]
    pub const fn prier(&self) -> &PRIER {
        &self.prier
    }
    ///0x1f8 - DCMIPP parallel interface status register
    #[inline(always)]
    pub const fn prsr(&self) -> &PRSR {
        &self.prsr
    }
    ///0x1fc - DCMIPP parallel interface interrupt clear register
    #[inline(always)]
    pub const fn prfcr(&self) -> &PRFCR {
        &self.prfcr
    }
    ///0x204 - DCMIPP common configuration register
    #[inline(always)]
    pub const fn cmcr(&self) -> &CMCR {
        &self.cmcr
    }
    ///0x208 - DCMIPP common frame counter register
    #[inline(always)]
    pub const fn cmfrcr(&self) -> &CMFRCR {
        &self.cmfrcr
    }
    ///0x3f0 - DCMIPP common interrupt enable register
    #[inline(always)]
    pub const fn cmier(&self) -> &CMIER {
        &self.cmier
    }
    ///0x3f4 - DCMIPP common status register 1
    #[inline(always)]
    pub const fn cmsr1(&self) -> &CMSR1 {
        &self.cmsr1
    }
    ///0x3f8 - DCMIPP common status register 2
    #[inline(always)]
    pub const fn cmsr2(&self) -> &CMSR2 {
        &self.cmsr2
    }
    ///0x3fc - DCMIPP common interrupt clear register
    #[inline(always)]
    pub const fn cmfcr(&self) -> &CMFCR {
        &self.cmfcr
    }
    ///0x404 - DCMIPP Pipe0 flow selection configuration register
    #[inline(always)]
    pub const fn p0fscr(&self) -> &P0FSCR {
        &self.p0fscr
    }
    ///0x500 - DCMIPP Pipe0 flow control configuration register
    #[inline(always)]
    pub const fn p0fctcr(&self) -> &P0FCTCR {
        &self.p0fctcr
    }
    ///0x504 - DCMIPP Pipe0 stat/crop start register
    #[inline(always)]
    pub const fn p0scstr(&self) -> &P0SCSTR {
        &self.p0scstr
    }
    ///0x508 - DCMIPP Pipe0 stat/crop size register
    #[inline(always)]
    pub const fn p0scszr(&self) -> &P0SCSZR {
        &self.p0scszr
    }
    ///0x5b0 - DCMIPP Pipe0 dump counter register
    #[inline(always)]
    pub const fn p0dccntr(&self) -> &P0DCCNTR {
        &self.p0dccntr
    }
    ///0x5b4 - DCMIPP Pipe0 dump limit register
    #[inline(always)]
    pub const fn p0dclmtr(&self) -> &P0DCLMTR {
        &self.p0dclmtr
    }
    ///0x5c0 - DCMIPP Pipe0 pixel packer configuration register
    #[inline(always)]
    pub const fn p0ppcr(&self) -> &P0PPCR {
        &self.p0ppcr
    }
    ///0x5c4 - DCMIPP Pipe0 pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p0ppm0ar1(&self) -> &P0PPM0AR1 {
        &self.p0ppm0ar1
    }
    ///0x5c8 - DCMIPP Pipe0 pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p0ppm0ar2(&self) -> &P0PPM0AR2 {
        &self.p0ppm0ar2
    }
    ///0x5d0 - DCMIPP Pipe0 status Memory0 address register
    #[inline(always)]
    pub const fn p0stm0ar(&self) -> &P0STM0AR {
        &self.p0stm0ar
    }
    ///0x5f4 - DCMIPP Pipe0 interrupt enable register
    #[inline(always)]
    pub const fn p0ier(&self) -> &P0IER {
        &self.p0ier
    }
    ///0x5f8 - DCMIPP Pipe0 status register
    #[inline(always)]
    pub const fn p0sr(&self) -> &P0SR {
        &self.p0sr
    }
    ///0x5fc - DCMIPP Pipe0 interrupt clear register
    #[inline(always)]
    pub const fn p0fcr(&self) -> &P0FCR {
        &self.p0fcr
    }
    ///0x604 - DCMIPP Pipe0 current flow selection configuration register
    #[inline(always)]
    pub const fn p0cfscr(&self) -> &P0CFSCR {
        &self.p0cfscr
    }
    ///0x700 - DCMIPP Pipe0 current flow control configuration register
    #[inline(always)]
    pub const fn p0cfctcr(&self) -> &P0CFCTCR {
        &self.p0cfctcr
    }
    ///0x704 - DCMIPP Pipe0 current stat/crop start register
    #[inline(always)]
    pub const fn p0cscstr(&self) -> &P0CSCSTR {
        &self.p0cscstr
    }
    ///0x708 - DCMIPP Pipe0 current stat/crop size register
    #[inline(always)]
    pub const fn p0cscszr(&self) -> &P0CSCSZR {
        &self.p0cscszr
    }
    ///0x7c0 - DCMIPP Pipe0 current pixel packer configuration register
    #[inline(always)]
    pub const fn p0cppcr(&self) -> &P0CPPCR {
        &self.p0cppcr
    }
    ///0x7c4 - DCMIPP Pipe0 current pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p0cppm0ar1(&self) -> &P0CPPM0AR1 {
        &self.p0cppm0ar1
    }
    ///0x7c8 - DCMIPP Pipe0 current pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p0cppm0ar2(&self) -> &P0CPPM0AR2 {
        &self.p0cppm0ar2
    }
    ///0x804 - DCMIPP Pipe1 flow selection configuration register
    #[inline(always)]
    pub const fn p1fscr(&self) -> &P1FSCR {
        &self.p1fscr
    }
    ///0x820 - DCMIPP Pipe1 stat removal configuration register
    #[inline(always)]
    pub const fn p1srcr(&self) -> &P1SRCR {
        &self.p1srcr
    }
    ///0x824 - DCMIPP Pipe1 bad pixel removal control register
    #[inline(always)]
    pub const fn p1bprcr(&self) -> &P1BPRCR {
        &self.p1bprcr
    }
    ///0x828 - DCMIPP Pipe1 bad pixel removal status register
    #[inline(always)]
    pub const fn p1bprsr(&self) -> &P1BPRSR {
        &self.p1bprsr
    }
    ///0x830 - DCMIPP Pipe1 decimation register
    #[inline(always)]
    pub const fn p1decr(&self) -> &P1DECR {
        &self.p1decr
    }
    ///0x840 - DCMIPP Pipe1 black level calibration control register
    #[inline(always)]
    pub const fn p1blccr(&self) -> &P1BLCCR {
        &self.p1blccr
    }
    ///0x844 - DCMIPP Pipe1 exposure control register 1
    #[inline(always)]
    pub const fn p1excr1(&self) -> &P1EXCR1 {
        &self.p1excr1
    }
    ///0x848 - DCMIPP Pipe1 exposure control register 2
    #[inline(always)]
    pub const fn p1excr2(&self) -> &P1EXCR2 {
        &self.p1excr2
    }
    ///0x850 - DCMIPP Pipe1 statistics1 control register
    #[inline(always)]
    pub const fn p1st1cr(&self) -> &P1ST1CR {
        &self.p1st1cr
    }
    ///0x854 - DCMIPP Pipe1 statistics 2 control register
    #[inline(always)]
    pub const fn p1st2cr(&self) -> &P1ST2CR {
        &self.p1st2cr
    }
    ///0x858 - DCMIPP Pipe1 statistics 3 control register
    #[inline(always)]
    pub const fn p1st3cr(&self) -> &P1ST3CR {
        &self.p1st3cr
    }
    ///0x85c - DCMIPP Pipe1 statistics window start register
    #[inline(always)]
    pub const fn p1ststr(&self) -> &P1STSTR {
        &self.p1ststr
    }
    ///0x860 - DCMIPP Pipe1 statistics window size register
    #[inline(always)]
    pub const fn p1stszr(&self) -> &P1STSZR {
        &self.p1stszr
    }
    ///0x864 - DCMIPP Pipe1 statistics 1 status register
    #[inline(always)]
    pub const fn p1st1sr(&self) -> &P1ST1SR {
        &self.p1st1sr
    }
    ///0x868 - DCMIPP Pipe1 statistics 2 status register
    #[inline(always)]
    pub const fn p1st2sr(&self) -> &P1ST2SR {
        &self.p1st2sr
    }
    ///0x86c - DCMIPP Pipe1 statistics 3 status register
    #[inline(always)]
    pub const fn p1st3sr(&self) -> &P1ST3SR {
        &self.p1st3sr
    }
    ///0x870 - DCMIPP Pipe1 demosaicing configuration register
    #[inline(always)]
    pub const fn p1dmcr(&self) -> &P1DMCR {
        &self.p1dmcr
    }
    ///0x880 - DCMIPP Pipe1 ColorConv configuration register
    #[inline(always)]
    pub const fn p1cccr(&self) -> &P1CCCR {
        &self.p1cccr
    }
    ///0x884 - DCMIPP Pipe1 ColorConv red coefficient register 1
    #[inline(always)]
    pub const fn p1ccrr1(&self) -> &P1CCRR1 {
        &self.p1ccrr1
    }
    ///0x888 - DCMIPP Pipe1 ColorConv red coefficient register 2
    #[inline(always)]
    pub const fn p1ccrr2(&self) -> &P1CCRR2 {
        &self.p1ccrr2
    }
    ///0x88c - DCMIPP Pipe1 ColorConv green coefficient register 1
    #[inline(always)]
    pub const fn p1ccgr1(&self) -> &P1CCGR1 {
        &self.p1ccgr1
    }
    ///0x890 - DCMIPP Pipe1 ColorConv green coefficient register 2
    #[inline(always)]
    pub const fn p1ccgr2(&self) -> &P1CCGR2 {
        &self.p1ccgr2
    }
    ///0x894 - DCMIPP Pipex ColorConv blue coefficient register 1
    #[inline(always)]
    pub const fn p1ccbr1(&self) -> &P1CCBR1 {
        &self.p1ccbr1
    }
    ///0x898 - DCMIPP Pipe1 ColorConv blue coefficient register 2
    #[inline(always)]
    pub const fn p1ccbr2(&self) -> &P1CCBR2 {
        &self.p1ccbr2
    }
    ///0x8a0 - DCMIPP Pipe1 contrast control register 1
    #[inline(always)]
    pub const fn p1ctcr1(&self) -> &P1CTCR1 {
        &self.p1ctcr1
    }
    ///0x8a4 - DCMIPP Pipe1 contrast control register 2
    #[inline(always)]
    pub const fn p1ctcr2(&self) -> &P1CTCR2 {
        &self.p1ctcr2
    }
    ///0x8a8 - DCMIPP Pipe1 contrast control register 3
    #[inline(always)]
    pub const fn p1ctcr3(&self) -> &P1CTCR3 {
        &self.p1ctcr3
    }
    ///0x900 - DCMIPP Pipex flow control configuration register
    #[inline(always)]
    pub const fn p1fctcr(&self) -> &P1FCTCR {
        &self.p1fctcr
    }
    ///0x904 - DCMIPP Pipex crop window start register
    #[inline(always)]
    pub const fn p1crstr(&self) -> &P1CRSTR {
        &self.p1crstr
    }
    ///0x908 - DCMIPP Pipex crop window size register
    #[inline(always)]
    pub const fn p1crszr(&self) -> &P1CRSZR {
        &self.p1crszr
    }
    ///0x90c - DCMIPP Pipex decimation register
    #[inline(always)]
    pub const fn p1dccr(&self) -> &P1DCCR {
        &self.p1dccr
    }
    ///0x910 - DCMIPP Pipex downsize configuration register
    #[inline(always)]
    pub const fn p1dscr(&self) -> &P1DSCR {
        &self.p1dscr
    }
    ///0x914 - DCMIPP Pipex downsize ratio register
    #[inline(always)]
    pub const fn p1dsrtior(&self) -> &P1DSRTIOR {
        &self.p1dsrtior
    }
    ///0x918 - DCMIPP Pipex downsize destination size register
    #[inline(always)]
    pub const fn p1dsszr(&self) -> &P1DSSZR {
        &self.p1dsszr
    }
    ///0x920 - DCMIPP Pipex common ROI configuration register
    #[inline(always)]
    pub const fn p1cmricr(&self) -> &P1CMRICR {
        &self.p1cmricr
    }
    ///0x924 - DCMIPP Pipe1 ROI1 configuration register 1
    #[inline(always)]
    pub const fn p1ri1cr1(&self) -> &P1RI1CR1 {
        &self.p1ri1cr1
    }
    ///0x928 - DCMIPP Pipe1 ROI1 configuration register 2
    #[inline(always)]
    pub const fn p1ri1cr2(&self) -> &P1RI1CR2 {
        &self.p1ri1cr2
    }
    ///0x92c - DCMIPP Pipe1 ROI2 configuration register 1
    #[inline(always)]
    pub const fn p1ri2cr1(&self) -> &P1RI2CR1 {
        &self.p1ri2cr1
    }
    ///0x930 - DCMIPP Pipe1 ROI2 configuration register 2
    #[inline(always)]
    pub const fn p1ri2cr2(&self) -> &P1RI2CR2 {
        &self.p1ri2cr2
    }
    ///0x934 - DCMIPP Pipe1 ROI3 configuration register 1
    #[inline(always)]
    pub const fn p1ri3cr1(&self) -> &P1RI3CR1 {
        &self.p1ri3cr1
    }
    ///0x938 - DCMIPP Pipe1 ROI3 configuration register 2
    #[inline(always)]
    pub const fn p1ri3cr2(&self) -> &P1RI3CR2 {
        &self.p1ri3cr2
    }
    ///0x93c - DCMIPP Pipe1 ROI4 configuration register 1
    #[inline(always)]
    pub const fn p1ri4cr1(&self) -> &P1RI4CR1 {
        &self.p1ri4cr1
    }
    ///0x940 - DCMIPP Pipe1 ROI4 configuration register 2
    #[inline(always)]
    pub const fn p1ri4cr2(&self) -> &P1RI4CR2 {
        &self.p1ri4cr2
    }
    ///0x944 - DCMIPP Pipe1 ROI5 configuration register 1
    #[inline(always)]
    pub const fn p1ri5cr1(&self) -> &P1RI5CR1 {
        &self.p1ri5cr1
    }
    ///0x948 - DCMIPP Pipe1 ROI5 configuration register 2
    #[inline(always)]
    pub const fn p1ri5cr2(&self) -> &P1RI5CR2 {
        &self.p1ri5cr2
    }
    ///0x94c - DCMIPP Pipe1 ROI6 configuration register 1
    #[inline(always)]
    pub const fn p1ri6cr1(&self) -> &P1RI6CR1 {
        &self.p1ri6cr1
    }
    ///0x950 - DCMIPP Pipe1 ROI6 configuration register 2
    #[inline(always)]
    pub const fn p1ri6cr2(&self) -> &P1RI6CR2 {
        &self.p1ri6cr2
    }
    ///0x954 - DCMIPP Pipe1 ROI7 configuration register 1
    #[inline(always)]
    pub const fn p1ri7cr1(&self) -> &P1RI7CR1 {
        &self.p1ri7cr1
    }
    ///0x958 - DCMIPP Pipe1 ROI7 configuration register 2
    #[inline(always)]
    pub const fn p1ri7cr2(&self) -> &P1RI7CR2 {
        &self.p1ri7cr2
    }
    ///0x95c - DCMIPP Pipe1 ROI8 configuration register 1
    #[inline(always)]
    pub const fn p1ri8cr1(&self) -> &P1RI8CR1 {
        &self.p1ri8cr1
    }
    ///0x960 - DCMIPP Pipe1 ROI8 configuration register 2
    #[inline(always)]
    pub const fn p1ri8cr2(&self) -> &P1RI8CR2 {
        &self.p1ri8cr2
    }
    ///0x970 - DCMIPP Pipex gamma configuration register
    #[inline(always)]
    pub const fn p1gmcr(&self) -> &P1GMCR {
        &self.p1gmcr
    }
    ///0x980 - DCMIPP Pipe1 YUVConv configuration register
    #[inline(always)]
    pub const fn p1yuvcr(&self) -> &P1YUVCR {
        &self.p1yuvcr
    }
    ///0x984 - DCMIPP Pipe1 YUVConv red coefficient register 1
    #[inline(always)]
    pub const fn p1yuvrr1(&self) -> &P1YUVRR1 {
        &self.p1yuvrr1
    }
    ///0x988 - DCMIPP Pipe1 YUVConv red coefficient register 2
    #[inline(always)]
    pub const fn p1yuvrr2(&self) -> &P1YUVRR2 {
        &self.p1yuvrr2
    }
    ///0x98c - DCMIPP Pipe1 YUVConv green coefficient register 1
    #[inline(always)]
    pub const fn p1yuvgr1(&self) -> &P1YUVGR1 {
        &self.p1yuvgr1
    }
    ///0x990 - DCMIPP Pipe1 YUVConv green coefficient register 2
    #[inline(always)]
    pub const fn p1yuvgr2(&self) -> &P1YUVGR2 {
        &self.p1yuvgr2
    }
    ///0x994 - DCMIPP Pipe1 YUVConv blue coefficient register 1
    #[inline(always)]
    pub const fn p1yuvbr1(&self) -> &P1YUVBR1 {
        &self.p1yuvbr1
    }
    ///0x998 - DCMIPP Pipe1 YUV blue coefficient register 2
    #[inline(always)]
    pub const fn p1yuvbr2(&self) -> &P1YUVBR2 {
        &self.p1yuvbr2
    }
    ///0x9c0 - DCMIPP Pipe1 pixel packer configuration register
    #[inline(always)]
    pub const fn p1ppcr(&self) -> &P1PPCR {
        &self.p1ppcr
    }
    ///0x9c4 - DCMIPP Pipe1 pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p1ppm0ar1(&self) -> &P1PPM0AR1 {
        &self.p1ppm0ar1
    }
    ///0x9c8 - DCMIPP Pipe1 pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p1ppm0ar2(&self) -> &P1PPM0AR2 {
        &self.p1ppm0ar2
    }
    ///0x9cc - DCMIPP Pipex pixel packer Memory0 pitch register
    #[inline(always)]
    pub const fn p1ppm0pr(&self) -> &P1PPM0PR {
        &self.p1ppm0pr
    }
    ///0x9d0 - DCMIPP Pipex status Memory0 address register
    #[inline(always)]
    pub const fn p1stm0ar(&self) -> &P1STM0AR {
        &self.p1stm0ar
    }
    ///0x9d4 - DCMIPP Pipex pixel packer Memory1 address register 1
    #[inline(always)]
    pub const fn p1ppm1ar1(&self) -> &P1PPM1AR1 {
        &self.p1ppm1ar1
    }
    ///0x9d8 - DCMIPP Pipex pixel packer Memory1 address register 2
    #[inline(always)]
    pub const fn p1ppm1ar2(&self) -> &P1PPM1AR2 {
        &self.p1ppm1ar2
    }
    ///0x9dc - DCMIPP Pipex pixel packer Memory1 pitch register
    #[inline(always)]
    pub const fn p1ppm1pr(&self) -> &P1PPM1PR {
        &self.p1ppm1pr
    }
    ///0x9e0 - DCMIPP Pipex status Memory1 address register
    #[inline(always)]
    pub const fn p1stm1ar(&self) -> &P1STM1AR {
        &self.p1stm1ar
    }
    ///0x9e4 - DCMIPP Pipex pixel packer memory2 address register 1
    #[inline(always)]
    pub const fn p1ppm2ar1(&self) -> &P1PPM2AR1 {
        &self.p1ppm2ar1
    }
    ///0x9e8 - DCMIPP Pipex pixel packer memory2 address register 2
    #[inline(always)]
    pub const fn p1ppm2ar2(&self) -> &P1PPM2AR2 {
        &self.p1ppm2ar2
    }
    ///0x9f0 - DCMIPP Pipex status Memory2 address register
    #[inline(always)]
    pub const fn p1stm2ar(&self) -> &P1STM2AR {
        &self.p1stm2ar
    }
    ///0x9f4 - DCMIPP Pipe1 interrupt enable register
    #[inline(always)]
    pub const fn p1ier(&self) -> &P1IER {
        &self.p1ier
    }
    ///0x9f8 - DCMIPP Pipe1 status register
    #[inline(always)]
    pub const fn p1sr(&self) -> &P1SR {
        &self.p1sr
    }
    ///0x9fc - DCMIPP Pipe1 interrupt clear register
    #[inline(always)]
    pub const fn p1fcr(&self) -> &P1FCR {
        &self.p1fcr
    }
    ///0xa04 - DCMIPP Pipe1 current flow selection configuration register
    #[inline(always)]
    pub const fn p1cfscr(&self) -> &P1CFSCR {
        &self.p1cfscr
    }
    ///0xa24 - DCMIPP Pipe1 current bad pixel removal register
    #[inline(always)]
    pub const fn p1cbprcr(&self) -> &P1CBPRCR {
        &self.p1cbprcr
    }
    ///0xa40 - DCMIPP Pipe1 current black level calibration control register
    #[inline(always)]
    pub const fn p1cblccr(&self) -> &P1CBLCCR {
        &self.p1cblccr
    }
    ///0xa44 - DCMIPP Pipe1 current exposure control register 1
    #[inline(always)]
    pub const fn p1cexcr1(&self) -> &P1CEXCR1 {
        &self.p1cexcr1
    }
    ///0xa48 - DCMIPP Pipe1 current exposure control register 2
    #[inline(always)]
    pub const fn p1cexcr2(&self) -> &P1CEXCR2 {
        &self.p1cexcr2
    }
    ///0xa50 - DCMIPP Pipe1 current statistics 1 control register
    #[inline(always)]
    pub const fn p1cst1cr(&self) -> &P1CST1CR {
        &self.p1cst1cr
    }
    ///0xa54 - DCMIPP Pipe1 current statistics 2 control register
    #[inline(always)]
    pub const fn p1cst2cr(&self) -> &P1CST2CR {
        &self.p1cst2cr
    }
    ///0xa58 - DCMIPP Pipe1 current statistics 3 control register
    #[inline(always)]
    pub const fn p1cst3cr(&self) -> &P1CST3CR {
        &self.p1cst3cr
    }
    ///0xa5c - DCMIPP Pipe1 current statistics window start register
    #[inline(always)]
    pub const fn p1cststr(&self) -> &P1CSTSTR {
        &self.p1cststr
    }
    ///0xa60 - DCMIPP Pipe1 current statistics window size register
    #[inline(always)]
    pub const fn p1cstszr(&self) -> &P1CSTSZR {
        &self.p1cstszr
    }
    ///0xa80 - DCMIPP Pipe1 current ColorConv configuration register
    #[inline(always)]
    pub const fn p1ccccr(&self) -> &P1CCCCR {
        &self.p1ccccr
    }
    ///0xa84 - DCMIPP Pipe1 current ColorConv red coefficient register 1
    #[inline(always)]
    pub const fn p1cccrr1(&self) -> &P1CCCRR1 {
        &self.p1cccrr1
    }
    ///0xa88 - DCMIPP Pipe1 current ColorConv red coefficient register 2
    #[inline(always)]
    pub const fn p1cccrr2(&self) -> &P1CCCRR2 {
        &self.p1cccrr2
    }
    ///0xa8c - DCMIPP Pipe1 current ColorConv green coefficient register 1
    #[inline(always)]
    pub const fn p1cccgr1(&self) -> &P1CCCGR1 {
        &self.p1cccgr1
    }
    ///0xa90 - DCMIPP Pipe1 current ColorConv green coefficient register 2
    #[inline(always)]
    pub const fn p1cccgr2(&self) -> &P1CCCGR2 {
        &self.p1cccgr2
    }
    ///0xa94 - DCMIPP Pipex current ColorConv blue coefficient register 1
    #[inline(always)]
    pub const fn p1cccbr1(&self) -> &P1CCCBR1 {
        &self.p1cccbr1
    }
    ///0xa98 - DCMIPP Pipe1 current ColorConv blue coefficient register 2
    #[inline(always)]
    pub const fn p1cccbr2(&self) -> &P1CCCBR2 {
        &self.p1cccbr2
    }
    ///0xaa0 - DCMIPP Pipe1 current contrast control register 1
    #[inline(always)]
    pub const fn p1cctcr1(&self) -> &P1CCTCR1 {
        &self.p1cctcr1
    }
    ///0xaa4 - DCMIPP Pipe1 current contrast control register 2
    #[inline(always)]
    pub const fn p1cctcr2(&self) -> &P1CCTCR2 {
        &self.p1cctcr2
    }
    ///0xaa8 - DCMIPP Pipe1 current contrast control register 3
    #[inline(always)]
    pub const fn p1cctcr3(&self) -> &P1CCTCR3 {
        &self.p1cctcr3
    }
    ///0xb00 - DCMIPP Pipex current flow control configuration register
    #[inline(always)]
    pub const fn p1cfctcr(&self) -> &P1CFCTCR {
        &self.p1cfctcr
    }
    ///0xb04 - DCMIPP Pipex current crop window start register
    #[inline(always)]
    pub const fn p1ccrstr(&self) -> &P1CCRSTR {
        &self.p1ccrstr
    }
    ///0xb08 - DCMIPP Pipex current crop window size register
    #[inline(always)]
    pub const fn p1ccrszr(&self) -> &P1CCRSZR {
        &self.p1ccrszr
    }
    ///0xb0c - DCMIPP Pipex current decimation register
    #[inline(always)]
    pub const fn p1cdccr(&self) -> &P1CDCCR {
        &self.p1cdccr
    }
    ///0xb10 - DCMIPP Pipex current downsize configuration register
    #[inline(always)]
    pub const fn p1cdscr(&self) -> &P1CDSCR {
        &self.p1cdscr
    }
    ///0xb14 - DCMIPP Pipex current downsize ratio register
    #[inline(always)]
    pub const fn p1cdsrtior(&self) -> &P1CDSRTIOR {
        &self.p1cdsrtior
    }
    ///0xb18 - DCMIPP Pipex current downsize destination size register
    #[inline(always)]
    pub const fn p1cdsszr(&self) -> &P1CDSSZR {
        &self.p1cdsszr
    }
    ///0xb20 - DCMIPP Pipex current common ROI configuration register
    #[inline(always)]
    pub const fn p1ccmricr(&self) -> &P1CCMRICR {
        &self.p1ccmricr
    }
    ///0xb24 - DCMIPP Pipe1 current ROI1 configuration register 1
    #[inline(always)]
    pub const fn p1cri1cr1(&self) -> &P1CRI1CR1 {
        &self.p1cri1cr1
    }
    ///0xb28 - DCMIPP Pipe1 current ROI1 configuration register 2
    #[inline(always)]
    pub const fn p1cri1cr2(&self) -> &P1CRI1CR2 {
        &self.p1cri1cr2
    }
    ///0xb2c - DCMIPP Pipe1 current ROI2 configuration register 1
    #[inline(always)]
    pub const fn p1cri2cr1(&self) -> &P1CRI2CR1 {
        &self.p1cri2cr1
    }
    ///0xb30 - DCMIPP Pipe1 current ROI2 configuration register 2
    #[inline(always)]
    pub const fn p1cri2cr2(&self) -> &P1CRI2CR2 {
        &self.p1cri2cr2
    }
    ///0xb34 - DCMIPP Pipe1 current ROI3 configuration register 1
    #[inline(always)]
    pub const fn p1cri3cr1(&self) -> &P1CRI3CR1 {
        &self.p1cri3cr1
    }
    ///0xb38 - DCMIPP Pipe1 current ROI3 configuration register 2
    #[inline(always)]
    pub const fn p1cri3cr2(&self) -> &P1CRI3CR2 {
        &self.p1cri3cr2
    }
    ///0xb3c - DCMIPP Pipe1 current ROI4 configuration register 1
    #[inline(always)]
    pub const fn p1cri4cr1(&self) -> &P1CRI4CR1 {
        &self.p1cri4cr1
    }
    ///0xb40 - DCMIPP Pipe1 current ROI4 configuration register 2
    #[inline(always)]
    pub const fn p1cri4cr2(&self) -> &P1CRI4CR2 {
        &self.p1cri4cr2
    }
    ///0xb44 - DCMIPP Pipe1 current ROI5 configuration register 1
    #[inline(always)]
    pub const fn p1cri5cr1(&self) -> &P1CRI5CR1 {
        &self.p1cri5cr1
    }
    ///0xb48 - DCMIPP Pipe1 current ROI5 configuration register 2
    #[inline(always)]
    pub const fn p1cri5cr2(&self) -> &P1CRI5CR2 {
        &self.p1cri5cr2
    }
    ///0xb4c - DCMIPP Pipe1 current ROI6 configuration register 1
    #[inline(always)]
    pub const fn p1cri6cr1(&self) -> &P1CRI6CR1 {
        &self.p1cri6cr1
    }
    ///0xb50 - DCMIPP Pipe1 current ROI6 configuration register 2
    #[inline(always)]
    pub const fn p1cri6cr2(&self) -> &P1CRI6CR2 {
        &self.p1cri6cr2
    }
    ///0xb54 - DCMIPP Pipe1 current ROI7 configuration register 1
    #[inline(always)]
    pub const fn p1cri7cr1(&self) -> &P1CRI7CR1 {
        &self.p1cri7cr1
    }
    ///0xb58 - DCMIPP Pipe1 current ROI7 configuration register 2
    #[inline(always)]
    pub const fn p1cri7cr2(&self) -> &P1CRI7CR2 {
        &self.p1cri7cr2
    }
    ///0xb5c - DCMIPP Pipe1 current ROI8 configuration register 1
    #[inline(always)]
    pub const fn p1cri8cr1(&self) -> &P1CRI8CR1 {
        &self.p1cri8cr1
    }
    ///0xb60 - DCMIPP Pipe1 current ROI8 configuration register 2
    #[inline(always)]
    pub const fn p1cri8cr2(&self) -> &P1CRI8CR2 {
        &self.p1cri8cr2
    }
    ///0xbc0 - DCMIPP Pipe1 current pixel packer configuration register
    #[inline(always)]
    pub const fn p1cppcr(&self) -> &P1CPPCR {
        &self.p1cppcr
    }
    ///0xbc4 - DCMIPP Pipe1 current pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p1cppm0ar1(&self) -> &P1CPPM0AR1 {
        &self.p1cppm0ar1
    }
    ///0xbc8 - DCMIPP Pipe1 current pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p1cppm0ar2(&self) -> &P1CPPM0AR2 {
        &self.p1cppm0ar2
    }
    ///0xbcc - DCMIPP Pipex current pixel packer Memory0 pitch register
    #[inline(always)]
    pub const fn p1cppm0pr(&self) -> &P1CPPM0PR {
        &self.p1cppm0pr
    }
    ///0xbd4 - DCMIPP Pipex current pixel packer Memory1 address register 1
    #[inline(always)]
    pub const fn p1cppm1ar1(&self) -> &P1CPPM1AR1 {
        &self.p1cppm1ar1
    }
    ///0xbd8 - DCMIPP Pipex current pixel packer Memory1 address register 2
    #[inline(always)]
    pub const fn p1cppm1ar2(&self) -> &P1CPPM1AR2 {
        &self.p1cppm1ar2
    }
    ///0xbdc - DCMIPP Pipex current pixel packer Memory1 pitch register
    #[inline(always)]
    pub const fn p1cppm1pr(&self) -> &P1CPPM1PR {
        &self.p1cppm1pr
    }
    ///0xbe4 - DCMIPP Pipex current pixel packer Memory2 address register 1
    #[inline(always)]
    pub const fn p1cppm2ar1(&self) -> &P1CPPM2AR1 {
        &self.p1cppm2ar1
    }
    ///0xbe8 - DCMIPP Pipex current pixel packer Memory2 address register 1
    #[inline(always)]
    pub const fn p1cppm2ar2(&self) -> &P1CPPM2AR2 {
        &self.p1cppm2ar2
    }
    ///0xc04 - DCMIPP Pipe2 flow selection configuration register
    #[inline(always)]
    pub const fn p2fscr(&self) -> &P2FSCR {
        &self.p2fscr
    }
    ///0xd00 - DCMIPP Pipex flow control configuration register
    #[inline(always)]
    pub const fn p2fctcr(&self) -> &P2FCTCR {
        &self.p2fctcr
    }
    ///0xd04 - DCMIPP Pipex crop window start register
    #[inline(always)]
    pub const fn p2crstr(&self) -> &P2CRSTR {
        &self.p2crstr
    }
    ///0xd08 - DCMIPP Pipex crop window size register
    #[inline(always)]
    pub const fn p2crszr(&self) -> &P2CRSZR {
        &self.p2crszr
    }
    ///0xd0c - DCMIPP Pipex decimation register
    #[inline(always)]
    pub const fn p2dccr(&self) -> &P2DCCR {
        &self.p2dccr
    }
    ///0xd10 - DCMIPP Pipex downsize configuration register
    #[inline(always)]
    pub const fn p2dscr(&self) -> &P2DSCR {
        &self.p2dscr
    }
    ///0xd14 - DCMIPP Pipex downsize ratio register
    #[inline(always)]
    pub const fn p2dsrtior(&self) -> &P2DSRTIOR {
        &self.p2dsrtior
    }
    ///0xd18 - DCMIPP Pipex downsize destination size register
    #[inline(always)]
    pub const fn p2dsszr(&self) -> &P2DSSZR {
        &self.p2dsszr
    }
    ///0xd20 - DCMIPP Pipex common ROI configuration register
    #[inline(always)]
    pub const fn p2cmricr(&self) -> &P2CMRICR {
        &self.p2cmricr
    }
    ///0xd24 - DCMIPP Pipe2 ROI1 configuration register 1
    #[inline(always)]
    pub const fn p2ri1cr1(&self) -> &P2RI1CR1 {
        &self.p2ri1cr1
    }
    ///0xd28 - DCMIPP Pipe2 ROI1 configuration register 2
    #[inline(always)]
    pub const fn p2ri1cr2(&self) -> &P2RI1CR2 {
        &self.p2ri1cr2
    }
    ///0xd2c - DCMIPP Pipe2 ROI2 configuration register 1
    #[inline(always)]
    pub const fn p2ri2cr1(&self) -> &P2RI2CR1 {
        &self.p2ri2cr1
    }
    ///0xd30 - DCMIPP Pipe2 ROI2 configuration register 2
    #[inline(always)]
    pub const fn p2ri2cr2(&self) -> &P2RI2CR2 {
        &self.p2ri2cr2
    }
    ///0xd34 - DCMIPP Pipe2 ROI3 configuration register 1
    #[inline(always)]
    pub const fn p2ri3cr1(&self) -> &P2RI3CR1 {
        &self.p2ri3cr1
    }
    ///0xd38 - DCMIPP Pipe2 ROI3 configuration register 2
    #[inline(always)]
    pub const fn p2ri3cr2(&self) -> &P2RI3CR2 {
        &self.p2ri3cr2
    }
    ///0xd3c - DCMIPP Pipe2 ROI4 configuration register 1
    #[inline(always)]
    pub const fn p2ri4cr1(&self) -> &P2RI4CR1 {
        &self.p2ri4cr1
    }
    ///0xd40 - DCMIPP Pipe2 ROI4 configuration register 2
    #[inline(always)]
    pub const fn p2ri4cr2(&self) -> &P2RI4CR2 {
        &self.p2ri4cr2
    }
    ///0xd44 - DCMIPP Pipe2 ROI5 configuration register 1
    #[inline(always)]
    pub const fn p2ri5cr1(&self) -> &P2RI5CR1 {
        &self.p2ri5cr1
    }
    ///0xd48 - DCMIPP Pipe2 ROI5 configuration register 2
    #[inline(always)]
    pub const fn p2ri5cr2(&self) -> &P2RI5CR2 {
        &self.p2ri5cr2
    }
    ///0xd4c - DCMIPP Pipe2 ROI6 configuration register 1
    #[inline(always)]
    pub const fn p2ri6cr1(&self) -> &P2RI6CR1 {
        &self.p2ri6cr1
    }
    ///0xd50 - DCMIPP Pipe2 ROI6 configuration register 2
    #[inline(always)]
    pub const fn p2ri6cr2(&self) -> &P2RI6CR2 {
        &self.p2ri6cr2
    }
    ///0xd54 - DCMIPP Pipe2 ROI7 configuration register 1
    #[inline(always)]
    pub const fn p2ri7cr1(&self) -> &P2RI7CR1 {
        &self.p2ri7cr1
    }
    ///0xd58 - DCMIPP Pipe2 ROI7 configuration register 2
    #[inline(always)]
    pub const fn p2ri7cr2(&self) -> &P2RI7CR2 {
        &self.p2ri7cr2
    }
    ///0xd5c - DCMIPP Pipe2 ROI8 configuration register 1
    #[inline(always)]
    pub const fn p2ri8cr1(&self) -> &P2RI8CR1 {
        &self.p2ri8cr1
    }
    ///0xd60 - DCMIPP Pipe2 ROI8 configuration register 2
    #[inline(always)]
    pub const fn p2ri8cr2(&self) -> &P2RI8CR2 {
        &self.p2ri8cr2
    }
    ///0xd70 - DCMIPP Pipex gamma configuration register
    #[inline(always)]
    pub const fn p2gmcr(&self) -> &P2GMCR {
        &self.p2gmcr
    }
    ///0xdc0 - DCMIPP Pipe2 pixel packer configuration register
    #[inline(always)]
    pub const fn p2ppcr(&self) -> &P2PPCR {
        &self.p2ppcr
    }
    ///0xdc4 - DCMIPP Pipe2 pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p2ppm0ar1(&self) -> &P2PPM0AR1 {
        &self.p2ppm0ar1
    }
    ///0xdc8 - DCMIPP Pipe2 pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p2ppm0ar2(&self) -> &P2PPM0AR2 {
        &self.p2ppm0ar2
    }
    ///0xdcc - DCMIPP Pipex pixel packer Memory0 pitch register
    #[inline(always)]
    pub const fn p2ppm0pr(&self) -> &P2PPM0PR {
        &self.p2ppm0pr
    }
    ///0xdd0 - DCMIPP Pipex status Memory0 address register
    #[inline(always)]
    pub const fn p2stm0ar(&self) -> &P2STM0AR {
        &self.p2stm0ar
    }
    ///0xdf4 - DCMIPP Pipe2 interrupt enable register
    #[inline(always)]
    pub const fn p2ier(&self) -> &P2IER {
        &self.p2ier
    }
    ///0xdf8 - DCMIPP Pipe2 status register
    #[inline(always)]
    pub const fn p2sr(&self) -> &P2SR {
        &self.p2sr
    }
    ///0xdfc - DCMIPP Pipe2 interrupt clear register
    #[inline(always)]
    pub const fn p2fcr(&self) -> &P2FCR {
        &self.p2fcr
    }
    ///0xe04 - DCMIPP Pipe2 current flow selection configuration register
    #[inline(always)]
    pub const fn p2cfscr(&self) -> &P2CFSCR {
        &self.p2cfscr
    }
    ///0xf00 - DCMIPP Pipex current flow control configuration register
    #[inline(always)]
    pub const fn p2cfctcr(&self) -> &P2CFCTCR {
        &self.p2cfctcr
    }
    ///0xf04 - DCMIPP Pipex current crop window start register
    #[inline(always)]
    pub const fn p2ccrstr(&self) -> &P2CCRSTR {
        &self.p2ccrstr
    }
    ///0xf08 - DCMIPP Pipex current crop window size register
    #[inline(always)]
    pub const fn p2ccrszr(&self) -> &P2CCRSZR {
        &self.p2ccrszr
    }
    ///0xf0c - DCMIPP Pipex current decimation register
    #[inline(always)]
    pub const fn p2cdccr(&self) -> &P2CDCCR {
        &self.p2cdccr
    }
    ///0xf10 - DCMIPP Pipex current downsize configuration register
    #[inline(always)]
    pub const fn p2cdscr(&self) -> &P2CDSCR {
        &self.p2cdscr
    }
    ///0xf14 - DCMIPP Pipex current downsize ratio register
    #[inline(always)]
    pub const fn p2cdsrtior(&self) -> &P2CDSRTIOR {
        &self.p2cdsrtior
    }
    ///0xf18 - DCMIPP Pipex current downsize destination size register
    #[inline(always)]
    pub const fn p2cdsszr(&self) -> &P2CDSSZR {
        &self.p2cdsszr
    }
    ///0xf20 - DCMIPP Pipex current common ROI configuration register
    #[inline(always)]
    pub const fn p2ccmricr(&self) -> &P2CCMRICR {
        &self.p2ccmricr
    }
    ///0xf24 - DCMIPP Pipe2 current ROI1 configuration register 1
    #[inline(always)]
    pub const fn p2cri1cr1(&self) -> &P2CRI1CR1 {
        &self.p2cri1cr1
    }
    ///0xf28 - DCMIPP Pipe2 current ROI1 configuration register 2
    #[inline(always)]
    pub const fn p2cri1cr2(&self) -> &P2CRI1CR2 {
        &self.p2cri1cr2
    }
    ///0xf2c - DCMIPP Pipe2 current ROI2 configuration register 1
    #[inline(always)]
    pub const fn p2cri2cr1(&self) -> &P2CRI2CR1 {
        &self.p2cri2cr1
    }
    ///0xf30 - DCMIPP Pipe2 current ROI2 configuration register 2
    #[inline(always)]
    pub const fn p2cri2cr2(&self) -> &P2CRI2CR2 {
        &self.p2cri2cr2
    }
    ///0xf34 - DCMIPP Pipe2 current ROI3 configuration register 1
    #[inline(always)]
    pub const fn p2cri3cr1(&self) -> &P2CRI3CR1 {
        &self.p2cri3cr1
    }
    ///0xf38 - DCMIPP Pipe2 current ROI3 configuration register 2
    #[inline(always)]
    pub const fn p2cri3cr2(&self) -> &P2CRI3CR2 {
        &self.p2cri3cr2
    }
    ///0xf3c - DCMIPP Pipe2 current ROI4 configuration register 1
    #[inline(always)]
    pub const fn p2cri4cr1(&self) -> &P2CRI4CR1 {
        &self.p2cri4cr1
    }
    ///0xf40 - DCMIPP Pipe2 current ROI4 configuration register 2
    #[inline(always)]
    pub const fn p2cri4cr2(&self) -> &P2CRI4CR2 {
        &self.p2cri4cr2
    }
    ///0xf44 - DCMIPP Pipe2 current ROI5 configuration register 1
    #[inline(always)]
    pub const fn p2cri5cr1(&self) -> &P2CRI5CR1 {
        &self.p2cri5cr1
    }
    ///0xf48 - DCMIPP Pipe2 current ROI5 configuration register 2
    #[inline(always)]
    pub const fn p2cri5cr2(&self) -> &P2CRI5CR2 {
        &self.p2cri5cr2
    }
    ///0xf4c - DCMIPP Pipe2 current ROI6 configuration register 1
    #[inline(always)]
    pub const fn p2cri6cr1(&self) -> &P2CRI6CR1 {
        &self.p2cri6cr1
    }
    ///0xf50 - DCMIPP Pipe2 current ROI6 configuration register 2
    #[inline(always)]
    pub const fn p2cri6cr2(&self) -> &P2CRI6CR2 {
        &self.p2cri6cr2
    }
    ///0xf54 - DCMIPP Pipe2 current ROI7 configuration register 1
    #[inline(always)]
    pub const fn p2cri7cr1(&self) -> &P2CRI7CR1 {
        &self.p2cri7cr1
    }
    ///0xf58 - DCMIPP Pipe2 current ROI7 configuration register 2
    #[inline(always)]
    pub const fn p2cri7cr2(&self) -> &P2CRI7CR2 {
        &self.p2cri7cr2
    }
    ///0xf5c - DCMIPP Pipe2 current ROI8 configuration register 1
    #[inline(always)]
    pub const fn p2cri8cr1(&self) -> &P2CRI8CR1 {
        &self.p2cri8cr1
    }
    ///0xf60 - DCMIPP Pipe2 current ROI8 configuration register 2
    #[inline(always)]
    pub const fn p2cri8cr2(&self) -> &P2CRI8CR2 {
        &self.p2cri8cr2
    }
    ///0xfc0 - DCMIPP Pipe2 current pixel packer configuration register
    #[inline(always)]
    pub const fn p2cppcr(&self) -> &P2CPPCR {
        &self.p2cppcr
    }
    ///0xfc4 - DCMIPP Pipe2 current pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p2cppm0ar1(&self) -> &P2CPPM0AR1 {
        &self.p2cppm0ar1
    }
    ///0xfc8 - DCMIPP Pipe2 current pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p2cppm0ar2(&self) -> &P2CPPM0AR2 {
        &self.p2cppm0ar2
    }
}
/**IPGR1 (rw) register accessor: DCMIPP IPPLUG global register 1

You can [`read`](crate::Reg::read) this register and get [`ipgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPGR1)

For information about available fields see [`mod@ipgr1`] module*/
pub type IPGR1 = crate::Reg<ipgr1::IPGR1rs>;
///DCMIPP IPPLUG global register 1
pub mod ipgr1;
/**IPGR2 (rw) register accessor: DCMIPP IPPLUG global register 2

You can [`read`](crate::Reg::read) this register and get [`ipgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPGR2)

For information about available fields see [`mod@ipgr2`] module*/
pub type IPGR2 = crate::Reg<ipgr2::IPGR2rs>;
///DCMIPP IPPLUG global register 2
pub mod ipgr2;
/**IPGR3 (r) register accessor: DCMIPP IPPLUG global register 3

You can [`read`](crate::Reg::read) this register and get [`ipgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPGR3)

For information about available fields see [`mod@ipgr3`] module*/
pub type IPGR3 = crate::Reg<ipgr3::IPGR3rs>;
///DCMIPP IPPLUG global register 3
pub mod ipgr3;
/**IPGR8 (r) register accessor: DCMIPP IPPLUG identification register

You can [`read`](crate::Reg::read) this register and get [`ipgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPGR8)

For information about available fields see [`mod@ipgr8`] module*/
pub type IPGR8 = crate::Reg<ipgr8::IPGR8rs>;
///DCMIPP IPPLUG identification register
pub mod ipgr8;
/**IPC1R1 (rw) register accessor: DCMIPP IPPLUG Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC1R1)

For information about available fields see [`mod@ipc1r1`] module*/
pub type IPC1R1 = crate::Reg<ipc1r1::IPC1R1rs>;
///DCMIPP IPPLUG Clientx register 1
pub mod ipc1r1;
/**IPC1R2 (rw) register accessor: DCMIPP IPPLUG Clientx register 2

You can [`read`](crate::Reg::read) this register and get [`ipc1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC1R2)

For information about available fields see [`mod@ipc1r2`] module*/
pub type IPC1R2 = crate::Reg<ipc1r2::IPC1R2rs>;
///DCMIPP IPPLUG Clientx register 2
pub mod ipc1r2;
/**IPC1R3 (rw) register accessor: DCMIPP IPPLUG Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc1r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC1R3)

For information about available fields see [`mod@ipc1r3`] module*/
pub type IPC1R3 = crate::Reg<ipc1r3::IPC1R3rs>;
///DCMIPP IPPLUG Clientx register 3
pub mod ipc1r3;
/**IPC2R1 (rw) register accessor: DCMIPP IPPLUG Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc2r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc2r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC2R1)

For information about available fields see [`mod@ipc2r1`] module*/
pub type IPC2R1 = crate::Reg<ipc2r1::IPC2R1rs>;
///DCMIPP IPPLUG Clientx register 1
pub mod ipc2r1;
/**IPC2R2 (rw) register accessor: DCMIPP IPPLUG Clientx register 2

You can [`read`](crate::Reg::read) this register and get [`ipc2r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc2r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC2R2)

For information about available fields see [`mod@ipc2r2`] module*/
pub type IPC2R2 = crate::Reg<ipc2r2::IPC2R2rs>;
///DCMIPP IPPLUG Clientx register 2
pub mod ipc2r2;
/**IPC2R3 (rw) register accessor: DCMIPP IPPLUG Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc2r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc2r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC2R3)

For information about available fields see [`mod@ipc2r3`] module*/
pub type IPC2R3 = crate::Reg<ipc2r3::IPC2R3rs>;
///DCMIPP IPPLUG Clientx register 3
pub mod ipc2r3;
/**IPC3R1 (rw) register accessor: DCMIPP IPPLUG Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc3r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc3r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC3R1)

For information about available fields see [`mod@ipc3r1`] module*/
pub type IPC3R1 = crate::Reg<ipc3r1::IPC3R1rs>;
///DCMIPP IPPLUG Clientx register 1
pub mod ipc3r1;
/**IPC3R2 (rw) register accessor: DCMIPP IPPLUG Clientx register 2

You can [`read`](crate::Reg::read) this register and get [`ipc3r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc3r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC3R2)

For information about available fields see [`mod@ipc3r2`] module*/
pub type IPC3R2 = crate::Reg<ipc3r2::IPC3R2rs>;
///DCMIPP IPPLUG Clientx register 2
pub mod ipc3r2;
/**IPC3R3 (rw) register accessor: DCMIPP IPPLUG Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc3r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc3r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC3R3)

For information about available fields see [`mod@ipc3r3`] module*/
pub type IPC3R3 = crate::Reg<ipc3r3::IPC3R3rs>;
///DCMIPP IPPLUG Clientx register 3
pub mod ipc3r3;
/**IPC4R1 (rw) register accessor: DCMIPP IPPLUG Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc4r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc4r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC4R1)

For information about available fields see [`mod@ipc4r1`] module*/
pub type IPC4R1 = crate::Reg<ipc4r1::IPC4R1rs>;
///DCMIPP IPPLUG Clientx register 1
pub mod ipc4r1;
/**IPC4R2 (rw) register accessor: DCMIPP IPPLUG Clientx register 2

You can [`read`](crate::Reg::read) this register and get [`ipc4r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc4r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC4R2)

For information about available fields see [`mod@ipc4r2`] module*/
pub type IPC4R2 = crate::Reg<ipc4r2::IPC4R2rs>;
///DCMIPP IPPLUG Clientx register 2
pub mod ipc4r2;
/**IPC4R3 (rw) register accessor: DCMIPP IPPLUG Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc4r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc4r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC4R3)

For information about available fields see [`mod@ipc4r3`] module*/
pub type IPC4R3 = crate::Reg<ipc4r3::IPC4R3rs>;
///DCMIPP IPPLUG Clientx register 3
pub mod ipc4r3;
/**IPC5R1 (rw) register accessor: DCMIPP IPPLUG Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc5r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc5r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC5R1)

For information about available fields see [`mod@ipc5r1`] module*/
pub type IPC5R1 = crate::Reg<ipc5r1::IPC5R1rs>;
///DCMIPP IPPLUG Clientx register 1
pub mod ipc5r1;
/**IPC5R2 (rw) register accessor: DCMIPP IPPLUG Clientx register 2

You can [`read`](crate::Reg::read) this register and get [`ipc5r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc5r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC5R2)

For information about available fields see [`mod@ipc5r2`] module*/
pub type IPC5R2 = crate::Reg<ipc5r2::IPC5R2rs>;
///DCMIPP IPPLUG Clientx register 2
pub mod ipc5r2;
/**IPC5R3 (rw) register accessor: DCMIPP IPPLUG Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc5r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc5r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPC5R3)

For information about available fields see [`mod@ipc5r3`] module*/
pub type IPC5R3 = crate::Reg<ipc5r3::IPC5R3rs>;
///DCMIPP IPPLUG Clientx register 3
pub mod ipc5r3;
/**PRCR (rw) register accessor: DCMIPP parallel interface control register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:PRCR)

For information about available fields see [`mod@prcr`] module*/
pub type PRCR = crate::Reg<prcr::PRCRrs>;
///DCMIPP parallel interface control register
pub mod prcr;
/**PRESCR (rw) register accessor: DCMIPP parallel interface embedded synchronization code register

You can [`read`](crate::Reg::read) this register and get [`prescr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:PRESCR)

For information about available fields see [`mod@prescr`] module*/
pub type PRESCR = crate::Reg<prescr::PRESCRrs>;
///DCMIPP parallel interface embedded synchronization code register
pub mod prescr;
/**PRESUR (rw) register accessor: DCMIPP parallel interface embedded synchronization unmask register

You can [`read`](crate::Reg::read) this register and get [`presur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:PRESUR)

For information about available fields see [`mod@presur`] module*/
pub type PRESUR = crate::Reg<presur::PRESURrs>;
///DCMIPP parallel interface embedded synchronization unmask register
pub mod presur;
/**PRIER (rw) register accessor: DCMIPP parallel interface interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`prier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:PRIER)

For information about available fields see [`mod@prier`] module*/
pub type PRIER = crate::Reg<prier::PRIERrs>;
///DCMIPP parallel interface interrupt enable register
pub mod prier;
/**PRSR (r) register accessor: DCMIPP parallel interface status register

You can [`read`](crate::Reg::read) this register and get [`prsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:PRSR)

For information about available fields see [`mod@prsr`] module*/
pub type PRSR = crate::Reg<prsr::PRSRrs>;
///DCMIPP parallel interface status register
pub mod prsr;
/**PRFCR (w) register accessor: DCMIPP parallel interface interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prfcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:PRFCR)

For information about available fields see [`mod@prfcr`] module*/
pub type PRFCR = crate::Reg<prfcr::PRFCRrs>;
///DCMIPP parallel interface interrupt clear register
pub mod prfcr;
/**CMCR (rw) register accessor: DCMIPP common configuration register

You can [`read`](crate::Reg::read) this register and get [`cmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMCR)

For information about available fields see [`mod@cmcr`] module*/
pub type CMCR = crate::Reg<cmcr::CMCRrs>;
///DCMIPP common configuration register
pub mod cmcr;
/**CMFRCR (r) register accessor: DCMIPP common frame counter register

You can [`read`](crate::Reg::read) this register and get [`cmfrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMFRCR)

For information about available fields see [`mod@cmfrcr`] module*/
pub type CMFRCR = crate::Reg<cmfrcr::CMFRCRrs>;
///DCMIPP common frame counter register
pub mod cmfrcr;
/**CMIER (rw) register accessor: DCMIPP common interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cmier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMIER)

For information about available fields see [`mod@cmier`] module*/
pub type CMIER = crate::Reg<cmier::CMIERrs>;
///DCMIPP common interrupt enable register
pub mod cmier;
/**CMSR1 (r) register accessor: DCMIPP common status register 1

You can [`read`](crate::Reg::read) this register and get [`cmsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMSR1)

For information about available fields see [`mod@cmsr1`] module*/
pub type CMSR1 = crate::Reg<cmsr1::CMSR1rs>;
///DCMIPP common status register 1
pub mod cmsr1;
/**CMSR2 (r) register accessor: DCMIPP common status register 2

You can [`read`](crate::Reg::read) this register and get [`cmsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMSR2)

For information about available fields see [`mod@cmsr2`] module*/
pub type CMSR2 = crate::Reg<cmsr2::CMSR2rs>;
///DCMIPP common status register 2
pub mod cmsr2;
/**CMFCR (w) register accessor: DCMIPP common interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmfcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMFCR)

For information about available fields see [`mod@cmfcr`] module*/
pub type CMFCR = crate::Reg<cmfcr::CMFCRrs>;
///DCMIPP common interrupt clear register
pub mod cmfcr;
/**P0FSCR (rw) register accessor: DCMIPP Pipe0 flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p0fscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0FSCR)

For information about available fields see [`mod@p0fscr`] module*/
pub type P0FSCR = crate::Reg<p0fscr::P0FSCRrs>;
///DCMIPP Pipe0 flow selection configuration register
pub mod p0fscr;
/**P0FCTCR (rw) register accessor: DCMIPP Pipe0 flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p0fctcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fctcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0FCTCR)

For information about available fields see [`mod@p0fctcr`] module*/
pub type P0FCTCR = crate::Reg<p0fctcr::P0FCTCRrs>;
///DCMIPP Pipe0 flow control configuration register
pub mod p0fctcr;
/**P0SCSTR (rw) register accessor: DCMIPP Pipe0 stat/crop start register

You can [`read`](crate::Reg::read) this register and get [`p0scstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0scstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0SCSTR)

For information about available fields see [`mod@p0scstr`] module*/
pub type P0SCSTR = crate::Reg<p0scstr::P0SCSTRrs>;
///DCMIPP Pipe0 stat/crop start register
pub mod p0scstr;
/**P0SCSZR (rw) register accessor: DCMIPP Pipe0 stat/crop size register

You can [`read`](crate::Reg::read) this register and get [`p0scszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0scszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0SCSZR)

For information about available fields see [`mod@p0scszr`] module*/
pub type P0SCSZR = crate::Reg<p0scszr::P0SCSZRrs>;
///DCMIPP Pipe0 stat/crop size register
pub mod p0scszr;
/**P0DCCNTR (r) register accessor: DCMIPP Pipe0 dump counter register

You can [`read`](crate::Reg::read) this register and get [`p0dccntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0DCCNTR)

For information about available fields see [`mod@p0dccntr`] module*/
pub type P0DCCNTR = crate::Reg<p0dccntr::P0DCCNTRrs>;
///DCMIPP Pipe0 dump counter register
pub mod p0dccntr;
/**P0DCLMTR (rw) register accessor: DCMIPP Pipe0 dump limit register

You can [`read`](crate::Reg::read) this register and get [`p0dclmtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0dclmtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0DCLMTR)

For information about available fields see [`mod@p0dclmtr`] module*/
pub type P0DCLMTR = crate::Reg<p0dclmtr::P0DCLMTRrs>;
///DCMIPP Pipe0 dump limit register
pub mod p0dclmtr;
/**P0PPCR (rw) register accessor: DCMIPP Pipe0 pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p0ppcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0PPCR)

For information about available fields see [`mod@p0ppcr`] module*/
pub type P0PPCR = crate::Reg<p0ppcr::P0PPCRrs>;
///DCMIPP Pipe0 pixel packer configuration register
pub mod p0ppcr;
/**P0PPM0AR1 (rw) register accessor: DCMIPP Pipe0 pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p0ppm0ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppm0ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0PPM0AR1)

For information about available fields see [`mod@p0ppm0ar1`] module*/
pub type P0PPM0AR1 = crate::Reg<p0ppm0ar1::P0PPM0AR1rs>;
///DCMIPP Pipe0 pixel packer Memory0 address register 1
pub mod p0ppm0ar1;
/**P0PPM0AR2 (rw) register accessor: DCMIPP Pipe0 pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p0ppm0ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppm0ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0PPM0AR2)

For information about available fields see [`mod@p0ppm0ar2`] module*/
pub type P0PPM0AR2 = crate::Reg<p0ppm0ar2::P0PPM0AR2rs>;
///DCMIPP Pipe0 pixel packer Memory0 address register 2
pub mod p0ppm0ar2;
/**P0STM0AR (r) register accessor: DCMIPP Pipe0 status Memory0 address register

You can [`read`](crate::Reg::read) this register and get [`p0stm0ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0STM0AR)

For information about available fields see [`mod@p0stm0ar`] module*/
pub type P0STM0AR = crate::Reg<p0stm0ar::P0STM0ARrs>;
///DCMIPP Pipe0 status Memory0 address register
pub mod p0stm0ar;
/**P0IER (rw) register accessor: DCMIPP Pipe0 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`p0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0IER)

For information about available fields see [`mod@p0ier`] module*/
pub type P0IER = crate::Reg<p0ier::P0IERrs>;
///DCMIPP Pipe0 interrupt enable register
pub mod p0ier;
/**P0SR (r) register accessor: DCMIPP Pipe0 status register

You can [`read`](crate::Reg::read) this register and get [`p0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0SR)

For information about available fields see [`mod@p0sr`] module*/
pub type P0SR = crate::Reg<p0sr::P0SRrs>;
///DCMIPP Pipe0 status register
pub mod p0sr;
/**P0FCR (w) register accessor: DCMIPP Pipe0 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0FCR)

For information about available fields see [`mod@p0fcr`] module*/
pub type P0FCR = crate::Reg<p0fcr::P0FCRrs>;
///DCMIPP Pipe0 interrupt clear register
pub mod p0fcr;
/**P0CFSCR (r) register accessor: DCMIPP Pipe0 current flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cfscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CFSCR)

For information about available fields see [`mod@p0cfscr`] module*/
pub type P0CFSCR = crate::Reg<p0cfscr::P0CFSCRrs>;
///DCMIPP Pipe0 current flow selection configuration register
pub mod p0cfscr;
/**P0CFCTCR (r) register accessor: DCMIPP Pipe0 current flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cfctcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CFCTCR)

For information about available fields see [`mod@p0cfctcr`] module*/
pub type P0CFCTCR = crate::Reg<p0cfctcr::P0CFCTCRrs>;
///DCMIPP Pipe0 current flow control configuration register
pub mod p0cfctcr;
/**P0CSCSTR (r) register accessor: DCMIPP Pipe0 current stat/crop start register

You can [`read`](crate::Reg::read) this register and get [`p0cscstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CSCSTR)

For information about available fields see [`mod@p0cscstr`] module*/
pub type P0CSCSTR = crate::Reg<p0cscstr::P0CSCSTRrs>;
///DCMIPP Pipe0 current stat/crop start register
pub mod p0cscstr;
/**P0CSCSZR (r) register accessor: DCMIPP Pipe0 current stat/crop size register

You can [`read`](crate::Reg::read) this register and get [`p0cscszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CSCSZR)

For information about available fields see [`mod@p0cscszr`] module*/
pub type P0CSCSZR = crate::Reg<p0cscszr::P0CSCSZRrs>;
///DCMIPP Pipe0 current stat/crop size register
pub mod p0cscszr;
/**P0CPPCR (r) register accessor: DCMIPP Pipe0 current pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cppcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CPPCR)

For information about available fields see [`mod@p0cppcr`] module*/
pub type P0CPPCR = crate::Reg<p0cppcr::P0CPPCRrs>;
///DCMIPP Pipe0 current pixel packer configuration register
pub mod p0cppcr;
/**P0CPPM0AR1 (r) register accessor: DCMIPP Pipe0 current pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p0cppm0ar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CPPM0AR1)

For information about available fields see [`mod@p0cppm0ar1`] module*/
pub type P0CPPM0AR1 = crate::Reg<p0cppm0ar1::P0CPPM0AR1rs>;
///DCMIPP Pipe0 current pixel packer Memory0 address register 1
pub mod p0cppm0ar1;
/**P0CPPM0AR2 (r) register accessor: DCMIPP Pipe0 current pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p0cppm0ar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CPPM0AR2)

For information about available fields see [`mod@p0cppm0ar2`] module*/
pub type P0CPPM0AR2 = crate::Reg<p0cppm0ar2::P0CPPM0AR2rs>;
///DCMIPP Pipe0 current pixel packer Memory0 address register 2
pub mod p0cppm0ar2;
/**P1FSCR (rw) register accessor: DCMIPP Pipe1 flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p1fscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1fscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1FSCR)

For information about available fields see [`mod@p1fscr`] module*/
pub type P1FSCR = crate::Reg<p1fscr::P1FSCRrs>;
///DCMIPP Pipe1 flow selection configuration register
pub mod p1fscr;
/**P1SRCR (rw) register accessor: DCMIPP Pipe1 stat removal configuration register

You can [`read`](crate::Reg::read) this register and get [`p1srcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1srcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1SRCR)

For information about available fields see [`mod@p1srcr`] module*/
pub type P1SRCR = crate::Reg<p1srcr::P1SRCRrs>;
///DCMIPP Pipe1 stat removal configuration register
pub mod p1srcr;
/**P1BPRCR (rw) register accessor: DCMIPP Pipe1 bad pixel removal control register

You can [`read`](crate::Reg::read) this register and get [`p1bprcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1bprcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1BPRCR)

For information about available fields see [`mod@p1bprcr`] module*/
pub type P1BPRCR = crate::Reg<p1bprcr::P1BPRCRrs>;
///DCMIPP Pipe1 bad pixel removal control register
pub mod p1bprcr;
/**P1BPRSR (r) register accessor: DCMIPP Pipe1 bad pixel removal status register

You can [`read`](crate::Reg::read) this register and get [`p1bprsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1BPRSR)

For information about available fields see [`mod@p1bprsr`] module*/
pub type P1BPRSR = crate::Reg<p1bprsr::P1BPRSRrs>;
///DCMIPP Pipe1 bad pixel removal status register
pub mod p1bprsr;
/**P1DECR (rw) register accessor: DCMIPP Pipe1 decimation register

You can [`read`](crate::Reg::read) this register and get [`p1decr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1decr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1DECR)

For information about available fields see [`mod@p1decr`] module*/
pub type P1DECR = crate::Reg<p1decr::P1DECRrs>;
///DCMIPP Pipe1 decimation register
pub mod p1decr;
/**P1BLCCR (rw) register accessor: DCMIPP Pipe1 black level calibration control register

You can [`read`](crate::Reg::read) this register and get [`p1blccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1blccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1BLCCR)

For information about available fields see [`mod@p1blccr`] module*/
pub type P1BLCCR = crate::Reg<p1blccr::P1BLCCRrs>;
///DCMIPP Pipe1 black level calibration control register
pub mod p1blccr;
/**P1EXCR1 (rw) register accessor: DCMIPP Pipe1 exposure control register 1

You can [`read`](crate::Reg::read) this register and get [`p1excr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1excr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1EXCR1)

For information about available fields see [`mod@p1excr1`] module*/
pub type P1EXCR1 = crate::Reg<p1excr1::P1EXCR1rs>;
///DCMIPP Pipe1 exposure control register 1
pub mod p1excr1;
/**P1EXCR2 (rw) register accessor: DCMIPP Pipe1 exposure control register 2

You can [`read`](crate::Reg::read) this register and get [`p1excr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1excr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1EXCR2)

For information about available fields see [`mod@p1excr2`] module*/
pub type P1EXCR2 = crate::Reg<p1excr2::P1EXCR2rs>;
///DCMIPP Pipe1 exposure control register 2
pub mod p1excr2;
/**P1ST1CR (rw) register accessor: DCMIPP Pipe1 statistics1 control register

You can [`read`](crate::Reg::read) this register and get [`p1st1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1st1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1ST1CR)

For information about available fields see [`mod@p1st1cr`] module*/
pub type P1ST1CR = crate::Reg<p1st1cr::P1ST1CRrs>;
///DCMIPP Pipe1 statistics1 control register
pub mod p1st1cr;
/**P1ST2CR (rw) register accessor: DCMIPP Pipe1 statistics 2 control register

You can [`read`](crate::Reg::read) this register and get [`p1st2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1st2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1ST2CR)

For information about available fields see [`mod@p1st2cr`] module*/
pub type P1ST2CR = crate::Reg<p1st2cr::P1ST2CRrs>;
///DCMIPP Pipe1 statistics 2 control register
pub mod p1st2cr;
/**P1ST3CR (rw) register accessor: DCMIPP Pipe1 statistics 3 control register

You can [`read`](crate::Reg::read) this register and get [`p1st3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1st3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1ST3CR)

For information about available fields see [`mod@p1st3cr`] module*/
pub type P1ST3CR = crate::Reg<p1st3cr::P1ST3CRrs>;
///DCMIPP Pipe1 statistics 3 control register
pub mod p1st3cr;
/**P1STSTR (rw) register accessor: DCMIPP Pipe1 statistics window start register

You can [`read`](crate::Reg::read) this register and get [`p1ststr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ststr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1STSTR)

For information about available fields see [`mod@p1ststr`] module*/
pub type P1STSTR = crate::Reg<p1ststr::P1STSTRrs>;
///DCMIPP Pipe1 statistics window start register
pub mod p1ststr;
/**P1STSZR (rw) register accessor: DCMIPP Pipe1 statistics window size register

You can [`read`](crate::Reg::read) this register and get [`p1stszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1stszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1STSZR)

For information about available fields see [`mod@p1stszr`] module*/
pub type P1STSZR = crate::Reg<p1stszr::P1STSZRrs>;
///DCMIPP Pipe1 statistics window size register
pub mod p1stszr;
/**P1ST1SR (r) register accessor: DCMIPP Pipe1 statistics 1 status register

You can [`read`](crate::Reg::read) this register and get [`p1st1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1ST1SR)

For information about available fields see [`mod@p1st1sr`] module*/
pub type P1ST1SR = crate::Reg<p1st1sr::P1ST1SRrs>;
///DCMIPP Pipe1 statistics 1 status register
pub mod p1st1sr;
/**P1ST2SR (r) register accessor: DCMIPP Pipe1 statistics 2 status register

You can [`read`](crate::Reg::read) this register and get [`p1st2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1ST2SR)

For information about available fields see [`mod@p1st2sr`] module*/
pub type P1ST2SR = crate::Reg<p1st2sr::P1ST2SRrs>;
///DCMIPP Pipe1 statistics 2 status register
pub mod p1st2sr;
/**P1ST3SR (r) register accessor: DCMIPP Pipe1 statistics 3 status register

You can [`read`](crate::Reg::read) this register and get [`p1st3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1ST3SR)

For information about available fields see [`mod@p1st3sr`] module*/
pub type P1ST3SR = crate::Reg<p1st3sr::P1ST3SRrs>;
///DCMIPP Pipe1 statistics 3 status register
pub mod p1st3sr;
/**P1DMCR (rw) register accessor: DCMIPP Pipe1 demosaicing configuration register

You can [`read`](crate::Reg::read) this register and get [`p1dmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1DMCR)

For information about available fields see [`mod@p1dmcr`] module*/
pub type P1DMCR = crate::Reg<p1dmcr::P1DMCRrs>;
///DCMIPP Pipe1 demosaicing configuration register
pub mod p1dmcr;
/**P1CCCR (rw) register accessor: DCMIPP Pipe1 ColorConv configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCR)

For information about available fields see [`mod@p1cccr`] module*/
pub type P1CCCR = crate::Reg<p1cccr::P1CCCRrs>;
///DCMIPP Pipe1 ColorConv configuration register
pub mod p1cccr;
/**P1CCRR1 (rw) register accessor: DCMIPP Pipe1 ColorConv red coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1ccrr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccrr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCRR1)

For information about available fields see [`mod@p1ccrr1`] module*/
pub type P1CCRR1 = crate::Reg<p1ccrr1::P1CCRR1rs>;
///DCMIPP Pipe1 ColorConv red coefficient register 1
pub mod p1ccrr1;
/**P1CCRR2 (rw) register accessor: DCMIPP Pipe1 ColorConv red coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1ccrr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccrr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCRR2)

For information about available fields see [`mod@p1ccrr2`] module*/
pub type P1CCRR2 = crate::Reg<p1ccrr2::P1CCRR2rs>;
///DCMIPP Pipe1 ColorConv red coefficient register 2
pub mod p1ccrr2;
/**P1CCGR1 (rw) register accessor: DCMIPP Pipe1 ColorConv green coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1ccgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCGR1)

For information about available fields see [`mod@p1ccgr1`] module*/
pub type P1CCGR1 = crate::Reg<p1ccgr1::P1CCGR1rs>;
///DCMIPP Pipe1 ColorConv green coefficient register 1
pub mod p1ccgr1;
/**P1CCGR2 (rw) register accessor: DCMIPP Pipe1 ColorConv green coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1ccgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCGR2)

For information about available fields see [`mod@p1ccgr2`] module*/
pub type P1CCGR2 = crate::Reg<p1ccgr2::P1CCGR2rs>;
///DCMIPP Pipe1 ColorConv green coefficient register 2
pub mod p1ccgr2;
/**P1CCBR1 (rw) register accessor: DCMIPP Pipex ColorConv blue coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1ccbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCBR1)

For information about available fields see [`mod@p1ccbr1`] module*/
pub type P1CCBR1 = crate::Reg<p1ccbr1::P1CCBR1rs>;
///DCMIPP Pipex ColorConv blue coefficient register 1
pub mod p1ccbr1;
/**P1CCBR2 (rw) register accessor: DCMIPP Pipe1 ColorConv blue coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1ccbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCBR2)

For information about available fields see [`mod@p1ccbr2`] module*/
pub type P1CCBR2 = crate::Reg<p1ccbr2::P1CCBR2rs>;
///DCMIPP Pipe1 ColorConv blue coefficient register 2
pub mod p1ccbr2;
/**P1CTCR1 (rw) register accessor: DCMIPP Pipe1 contrast control register 1

You can [`read`](crate::Reg::read) this register and get [`p1ctcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ctcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CTCR1)

For information about available fields see [`mod@p1ctcr1`] module*/
pub type P1CTCR1 = crate::Reg<p1ctcr1::P1CTCR1rs>;
///DCMIPP Pipe1 contrast control register 1
pub mod p1ctcr1;
/**P1CTCR2 (rw) register accessor: DCMIPP Pipe1 contrast control register 2

You can [`read`](crate::Reg::read) this register and get [`p1ctcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ctcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CTCR2)

For information about available fields see [`mod@p1ctcr2`] module*/
pub type P1CTCR2 = crate::Reg<p1ctcr2::P1CTCR2rs>;
///DCMIPP Pipe1 contrast control register 2
pub mod p1ctcr2;
/**P1CTCR3 (rw) register accessor: DCMIPP Pipe1 contrast control register 3

You can [`read`](crate::Reg::read) this register and get [`p1ctcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ctcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CTCR3)

For information about available fields see [`mod@p1ctcr3`] module*/
pub type P1CTCR3 = crate::Reg<p1ctcr3::P1CTCR3rs>;
///DCMIPP Pipe1 contrast control register 3
pub mod p1ctcr3;
/**P1FCTCR (rw) register accessor: DCMIPP Pipex flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p1fctcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1fctcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1FCTCR)

For information about available fields see [`mod@p1fctcr`] module*/
pub type P1FCTCR = crate::Reg<p1fctcr::P1FCTCRrs>;
///DCMIPP Pipex flow control configuration register
pub mod p1fctcr;
/**P1CRSTR (rw) register accessor: DCMIPP Pipex crop window start register

You can [`read`](crate::Reg::read) this register and get [`p1crstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1crstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRSTR)

For information about available fields see [`mod@p1crstr`] module*/
pub type P1CRSTR = crate::Reg<p1crstr::P1CRSTRrs>;
///DCMIPP Pipex crop window start register
pub mod p1crstr;
/**P1CRSZR (rw) register accessor: DCMIPP Pipex crop window size register

You can [`read`](crate::Reg::read) this register and get [`p1crszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1crszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRSZR)

For information about available fields see [`mod@p1crszr`] module*/
pub type P1CRSZR = crate::Reg<p1crszr::P1CRSZRrs>;
///DCMIPP Pipex crop window size register
pub mod p1crszr;
/**P1DCCR (rw) register accessor: DCMIPP Pipex decimation register

You can [`read`](crate::Reg::read) this register and get [`p1dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1DCCR)

For information about available fields see [`mod@p1dccr`] module*/
pub type P1DCCR = crate::Reg<p1dccr::P1DCCRrs>;
///DCMIPP Pipex decimation register
pub mod p1dccr;
/**P1DSCR (rw) register accessor: DCMIPP Pipex downsize configuration register

You can [`read`](crate::Reg::read) this register and get [`p1dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1DSCR)

For information about available fields see [`mod@p1dscr`] module*/
pub type P1DSCR = crate::Reg<p1dscr::P1DSCRrs>;
///DCMIPP Pipex downsize configuration register
pub mod p1dscr;
/**P1DSRTIOR (rw) register accessor: DCMIPP Pipex downsize ratio register

You can [`read`](crate::Reg::read) this register and get [`p1dsrtior::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dsrtior::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1DSRTIOR)

For information about available fields see [`mod@p1dsrtior`] module*/
pub type P1DSRTIOR = crate::Reg<p1dsrtior::P1DSRTIORrs>;
///DCMIPP Pipex downsize ratio register
pub mod p1dsrtior;
/**P1DSSZR (rw) register accessor: DCMIPP Pipex downsize destination size register

You can [`read`](crate::Reg::read) this register and get [`p1dsszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dsszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1DSSZR)

For information about available fields see [`mod@p1dsszr`] module*/
pub type P1DSSZR = crate::Reg<p1dsszr::P1DSSZRrs>;
///DCMIPP Pipex downsize destination size register
pub mod p1dsszr;
/**P1CMRICR (rw) register accessor: DCMIPP Pipex common ROI configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cmricr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cmricr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CMRICR)

For information about available fields see [`mod@p1cmricr`] module*/
pub type P1CMRICR = crate::Reg<p1cmricr::P1CMRICRrs>;
///DCMIPP Pipex common ROI configuration register
pub mod p1cmricr;
/**P1RI1CR1 (rw) register accessor: DCMIPP Pipe1 ROI1 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI1CR1)

For information about available fields see [`mod@p1ri1cr1`] module*/
pub type P1RI1CR1 = crate::Reg<p1ri1cr1::P1RI1CR1rs>;
///DCMIPP Pipe1 ROI1 configuration register 1
pub mod p1ri1cr1;
/**P1RI1CR2 (rw) register accessor: DCMIPP Pipe1 ROI1 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri1cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri1cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI1CR2)

For information about available fields see [`mod@p1ri1cr2`] module*/
pub type P1RI1CR2 = crate::Reg<p1ri1cr2::P1RI1CR2rs>;
///DCMIPP Pipe1 ROI1 configuration register 2
pub mod p1ri1cr2;
/**P1RI2CR1 (rw) register accessor: DCMIPP Pipe1 ROI2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI2CR1)

For information about available fields see [`mod@p1ri2cr1`] module*/
pub type P1RI2CR1 = crate::Reg<p1ri2cr1::P1RI2CR1rs>;
///DCMIPP Pipe1 ROI2 configuration register 1
pub mod p1ri2cr1;
/**P1RI2CR2 (rw) register accessor: DCMIPP Pipe1 ROI2 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri2cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri2cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI2CR2)

For information about available fields see [`mod@p1ri2cr2`] module*/
pub type P1RI2CR2 = crate::Reg<p1ri2cr2::P1RI2CR2rs>;
///DCMIPP Pipe1 ROI2 configuration register 2
pub mod p1ri2cr2;
/**P1RI3CR1 (rw) register accessor: DCMIPP Pipe1 ROI3 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri3cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri3cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI3CR1)

For information about available fields see [`mod@p1ri3cr1`] module*/
pub type P1RI3CR1 = crate::Reg<p1ri3cr1::P1RI3CR1rs>;
///DCMIPP Pipe1 ROI3 configuration register 1
pub mod p1ri3cr1;
/**P1RI3CR2 (rw) register accessor: DCMIPP Pipe1 ROI3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri3cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri3cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI3CR2)

For information about available fields see [`mod@p1ri3cr2`] module*/
pub type P1RI3CR2 = crate::Reg<p1ri3cr2::P1RI3CR2rs>;
///DCMIPP Pipe1 ROI3 configuration register 2
pub mod p1ri3cr2;
/**P1RI4CR1 (rw) register accessor: DCMIPP Pipe1 ROI4 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri4cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri4cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI4CR1)

For information about available fields see [`mod@p1ri4cr1`] module*/
pub type P1RI4CR1 = crate::Reg<p1ri4cr1::P1RI4CR1rs>;
///DCMIPP Pipe1 ROI4 configuration register 1
pub mod p1ri4cr1;
/**P1RI4CR2 (rw) register accessor: DCMIPP Pipe1 ROI4 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri4cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri4cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI4CR2)

For information about available fields see [`mod@p1ri4cr2`] module*/
pub type P1RI4CR2 = crate::Reg<p1ri4cr2::P1RI4CR2rs>;
///DCMIPP Pipe1 ROI4 configuration register 2
pub mod p1ri4cr2;
/**P1RI5CR1 (rw) register accessor: DCMIPP Pipe1 ROI5 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri5cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri5cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI5CR1)

For information about available fields see [`mod@p1ri5cr1`] module*/
pub type P1RI5CR1 = crate::Reg<p1ri5cr1::P1RI5CR1rs>;
///DCMIPP Pipe1 ROI5 configuration register 1
pub mod p1ri5cr1;
/**P1RI5CR2 (rw) register accessor: DCMIPP Pipe1 ROI5 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri5cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri5cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI5CR2)

For information about available fields see [`mod@p1ri5cr2`] module*/
pub type P1RI5CR2 = crate::Reg<p1ri5cr2::P1RI5CR2rs>;
///DCMIPP Pipe1 ROI5 configuration register 2
pub mod p1ri5cr2;
/**P1RI6CR1 (rw) register accessor: DCMIPP Pipe1 ROI6 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri6cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri6cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI6CR1)

For information about available fields see [`mod@p1ri6cr1`] module*/
pub type P1RI6CR1 = crate::Reg<p1ri6cr1::P1RI6CR1rs>;
///DCMIPP Pipe1 ROI6 configuration register 1
pub mod p1ri6cr1;
/**P1RI6CR2 (rw) register accessor: DCMIPP Pipe1 ROI6 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri6cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri6cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI6CR2)

For information about available fields see [`mod@p1ri6cr2`] module*/
pub type P1RI6CR2 = crate::Reg<p1ri6cr2::P1RI6CR2rs>;
///DCMIPP Pipe1 ROI6 configuration register 2
pub mod p1ri6cr2;
/**P1RI7CR1 (rw) register accessor: DCMIPP Pipe1 ROI7 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri7cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri7cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI7CR1)

For information about available fields see [`mod@p1ri7cr1`] module*/
pub type P1RI7CR1 = crate::Reg<p1ri7cr1::P1RI7CR1rs>;
///DCMIPP Pipe1 ROI7 configuration register 1
pub mod p1ri7cr1;
/**P1RI7CR2 (rw) register accessor: DCMIPP Pipe1 ROI7 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri7cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri7cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI7CR2)

For information about available fields see [`mod@p1ri7cr2`] module*/
pub type P1RI7CR2 = crate::Reg<p1ri7cr2::P1RI7CR2rs>;
///DCMIPP Pipe1 ROI7 configuration register 2
pub mod p1ri7cr2;
/**P1RI8CR1 (rw) register accessor: DCMIPP Pipe1 ROI8 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri8cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri8cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI8CR1)

For information about available fields see [`mod@p1ri8cr1`] module*/
pub type P1RI8CR1 = crate::Reg<p1ri8cr1::P1RI8CR1rs>;
///DCMIPP Pipe1 ROI8 configuration register 1
pub mod p1ri8cr1;
/**P1RI8CR2 (rw) register accessor: DCMIPP Pipe1 ROI8 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1ri8cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri8cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1RI8CR2)

For information about available fields see [`mod@p1ri8cr2`] module*/
pub type P1RI8CR2 = crate::Reg<p1ri8cr2::P1RI8CR2rs>;
///DCMIPP Pipe1 ROI8 configuration register 2
pub mod p1ri8cr2;
/**P1GMCR (rw) register accessor: DCMIPP Pipex gamma configuration register

You can [`read`](crate::Reg::read) this register and get [`p1gmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1gmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1GMCR)

For information about available fields see [`mod@p1gmcr`] module*/
pub type P1GMCR = crate::Reg<p1gmcr::P1GMCRrs>;
///DCMIPP Pipex gamma configuration register
pub mod p1gmcr;
/**P1YUVCR (rw) register accessor: DCMIPP Pipe1 YUVConv configuration register

You can [`read`](crate::Reg::read) this register and get [`p1yuvcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1YUVCR)

For information about available fields see [`mod@p1yuvcr`] module*/
pub type P1YUVCR = crate::Reg<p1yuvcr::P1YUVCRrs>;
///DCMIPP Pipe1 YUVConv configuration register
pub mod p1yuvcr;
/**P1YUVRR1 (rw) register accessor: DCMIPP Pipe1 YUVConv red coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1yuvrr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvrr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1YUVRR1)

For information about available fields see [`mod@p1yuvrr1`] module*/
pub type P1YUVRR1 = crate::Reg<p1yuvrr1::P1YUVRR1rs>;
///DCMIPP Pipe1 YUVConv red coefficient register 1
pub mod p1yuvrr1;
/**P1YUVRR2 (rw) register accessor: DCMIPP Pipe1 YUVConv red coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1yuvrr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvrr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1YUVRR2)

For information about available fields see [`mod@p1yuvrr2`] module*/
pub type P1YUVRR2 = crate::Reg<p1yuvrr2::P1YUVRR2rs>;
///DCMIPP Pipe1 YUVConv red coefficient register 2
pub mod p1yuvrr2;
/**P1YUVGR1 (rw) register accessor: DCMIPP Pipe1 YUVConv green coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1yuvgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1YUVGR1)

For information about available fields see [`mod@p1yuvgr1`] module*/
pub type P1YUVGR1 = crate::Reg<p1yuvgr1::P1YUVGR1rs>;
///DCMIPP Pipe1 YUVConv green coefficient register 1
pub mod p1yuvgr1;
/**P1YUVGR2 (rw) register accessor: DCMIPP Pipe1 YUVConv green coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1yuvgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1YUVGR2)

For information about available fields see [`mod@p1yuvgr2`] module*/
pub type P1YUVGR2 = crate::Reg<p1yuvgr2::P1YUVGR2rs>;
///DCMIPP Pipe1 YUVConv green coefficient register 2
pub mod p1yuvgr2;
/**P1YUVBR1 (rw) register accessor: DCMIPP Pipe1 YUVConv blue coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1yuvbr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1YUVBR1)

For information about available fields see [`mod@p1yuvbr1`] module*/
pub type P1YUVBR1 = crate::Reg<p1yuvbr1::P1YUVBR1rs>;
///DCMIPP Pipe1 YUVConv blue coefficient register 1
pub mod p1yuvbr1;
/**P1YUVBR2 (rw) register accessor: DCMIPP Pipe1 YUV blue coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1yuvbr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1YUVBR2)

For information about available fields see [`mod@p1yuvbr2`] module*/
pub type P1YUVBR2 = crate::Reg<p1yuvbr2::P1YUVBR2rs>;
///DCMIPP Pipe1 YUV blue coefficient register 2
pub mod p1yuvbr2;
/**P1PPCR (rw) register accessor: DCMIPP Pipe1 pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p1ppcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPCR)

For information about available fields see [`mod@p1ppcr`] module*/
pub type P1PPCR = crate::Reg<p1ppcr::P1PPCRrs>;
///DCMIPP Pipe1 pixel packer configuration register
pub mod p1ppcr;
/**P1PPM0AR1 (rw) register accessor: DCMIPP Pipe1 pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1ppm0ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm0ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM0AR1)

For information about available fields see [`mod@p1ppm0ar1`] module*/
pub type P1PPM0AR1 = crate::Reg<p1ppm0ar1::P1PPM0AR1rs>;
///DCMIPP Pipe1 pixel packer Memory0 address register 1
pub mod p1ppm0ar1;
/**P1PPM0AR2 (rw) register accessor: DCMIPP Pipe1 pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p1ppm0ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm0ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM0AR2)

For information about available fields see [`mod@p1ppm0ar2`] module*/
pub type P1PPM0AR2 = crate::Reg<p1ppm0ar2::P1PPM0AR2rs>;
///DCMIPP Pipe1 pixel packer Memory0 address register 2
pub mod p1ppm0ar2;
/**P1PPM0PR (rw) register accessor: DCMIPP Pipex pixel packer Memory0 pitch register

You can [`read`](crate::Reg::read) this register and get [`p1ppm0pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm0pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM0PR)

For information about available fields see [`mod@p1ppm0pr`] module*/
pub type P1PPM0PR = crate::Reg<p1ppm0pr::P1PPM0PRrs>;
///DCMIPP Pipex pixel packer Memory0 pitch register
pub mod p1ppm0pr;
/**P1STM0AR (r) register accessor: DCMIPP Pipex status Memory0 address register

You can [`read`](crate::Reg::read) this register and get [`p1stm0ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1STM0AR)

For information about available fields see [`mod@p1stm0ar`] module*/
pub type P1STM0AR = crate::Reg<p1stm0ar::P1STM0ARrs>;
///DCMIPP Pipex status Memory0 address register
pub mod p1stm0ar;
/**P1PPM1AR1 (rw) register accessor: DCMIPP Pipex pixel packer Memory1 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1ppm1ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm1ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM1AR1)

For information about available fields see [`mod@p1ppm1ar1`] module*/
pub type P1PPM1AR1 = crate::Reg<p1ppm1ar1::P1PPM1AR1rs>;
///DCMIPP Pipex pixel packer Memory1 address register 1
pub mod p1ppm1ar1;
/**P1PPM1AR2 (rw) register accessor: DCMIPP Pipex pixel packer Memory1 address register 2

You can [`read`](crate::Reg::read) this register and get [`p1ppm1ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm1ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM1AR2)

For information about available fields see [`mod@p1ppm1ar2`] module*/
pub type P1PPM1AR2 = crate::Reg<p1ppm1ar2::P1PPM1AR2rs>;
///DCMIPP Pipex pixel packer Memory1 address register 2
pub mod p1ppm1ar2;
/**P1PPM1PR (rw) register accessor: DCMIPP Pipex pixel packer Memory1 pitch register

You can [`read`](crate::Reg::read) this register and get [`p1ppm1pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm1pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM1PR)

For information about available fields see [`mod@p1ppm1pr`] module*/
pub type P1PPM1PR = crate::Reg<p1ppm1pr::P1PPM1PRrs>;
///DCMIPP Pipex pixel packer Memory1 pitch register
pub mod p1ppm1pr;
/**P1STM1AR (r) register accessor: DCMIPP Pipex status Memory1 address register

You can [`read`](crate::Reg::read) this register and get [`p1stm1ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1STM1AR)

For information about available fields see [`mod@p1stm1ar`] module*/
pub type P1STM1AR = crate::Reg<p1stm1ar::P1STM1ARrs>;
///DCMIPP Pipex status Memory1 address register
pub mod p1stm1ar;
/**P1PPM2AR1 (rw) register accessor: DCMIPP Pipex pixel packer memory2 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1ppm2ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm2ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM2AR1)

For information about available fields see [`mod@p1ppm2ar1`] module*/
pub type P1PPM2AR1 = crate::Reg<p1ppm2ar1::P1PPM2AR1rs>;
///DCMIPP Pipex pixel packer memory2 address register 1
pub mod p1ppm2ar1;
/**P1PPM2AR2 (rw) register accessor: DCMIPP Pipex pixel packer memory2 address register 2

You can [`read`](crate::Reg::read) this register and get [`p1ppm2ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm2ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1PPM2AR2)

For information about available fields see [`mod@p1ppm2ar2`] module*/
pub type P1PPM2AR2 = crate::Reg<p1ppm2ar2::P1PPM2AR2rs>;
///DCMIPP Pipex pixel packer memory2 address register 2
pub mod p1ppm2ar2;
/**P1STM2AR (r) register accessor: DCMIPP Pipex status Memory2 address register

You can [`read`](crate::Reg::read) this register and get [`p1stm2ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1STM2AR)

For information about available fields see [`mod@p1stm2ar`] module*/
pub type P1STM2AR = crate::Reg<p1stm2ar::P1STM2ARrs>;
///DCMIPP Pipex status Memory2 address register
pub mod p1stm2ar;
/**P1IER (rw) register accessor: DCMIPP Pipe1 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`p1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1IER)

For information about available fields see [`mod@p1ier`] module*/
pub type P1IER = crate::Reg<p1ier::P1IERrs>;
///DCMIPP Pipe1 interrupt enable register
pub mod p1ier;
/**P1SR (r) register accessor: DCMIPP Pipe1 status register

You can [`read`](crate::Reg::read) this register and get [`p1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1SR)

For information about available fields see [`mod@p1sr`] module*/
pub type P1SR = crate::Reg<p1sr::P1SRrs>;
///DCMIPP Pipe1 status register
pub mod p1sr;
/**P1FCR (w) register accessor: DCMIPP Pipe1 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1FCR)

For information about available fields see [`mod@p1fcr`] module*/
pub type P1FCR = crate::Reg<p1fcr::P1FCRrs>;
///DCMIPP Pipe1 interrupt clear register
pub mod p1fcr;
/**P1CFSCR (r) register accessor: DCMIPP Pipe1 current flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cfscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CFSCR)

For information about available fields see [`mod@p1cfscr`] module*/
pub type P1CFSCR = crate::Reg<p1cfscr::P1CFSCRrs>;
///DCMIPP Pipe1 current flow selection configuration register
pub mod p1cfscr;
/**P1CBPRCR (r) register accessor: DCMIPP Pipe1 current bad pixel removal register

You can [`read`](crate::Reg::read) this register and get [`p1cbprcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CBPRCR)

For information about available fields see [`mod@p1cbprcr`] module*/
pub type P1CBPRCR = crate::Reg<p1cbprcr::P1CBPRCRrs>;
///DCMIPP Pipe1 current bad pixel removal register
pub mod p1cbprcr;
/**P1CBLCCR (r) register accessor: DCMIPP Pipe1 current black level calibration control register

You can [`read`](crate::Reg::read) this register and get [`p1cblccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CBLCCR)

For information about available fields see [`mod@p1cblccr`] module*/
pub type P1CBLCCR = crate::Reg<p1cblccr::P1CBLCCRrs>;
///DCMIPP Pipe1 current black level calibration control register
pub mod p1cblccr;
/**P1CEXCR1 (r) register accessor: DCMIPP Pipe1 current exposure control register 1

You can [`read`](crate::Reg::read) this register and get [`p1cexcr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CEXCR1)

For information about available fields see [`mod@p1cexcr1`] module*/
pub type P1CEXCR1 = crate::Reg<p1cexcr1::P1CEXCR1rs>;
///DCMIPP Pipe1 current exposure control register 1
pub mod p1cexcr1;
/**P1CEXCR2 (r) register accessor: DCMIPP Pipe1 current exposure control register 2

You can [`read`](crate::Reg::read) this register and get [`p1cexcr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CEXCR2)

For information about available fields see [`mod@p1cexcr2`] module*/
pub type P1CEXCR2 = crate::Reg<p1cexcr2::P1CEXCR2rs>;
///DCMIPP Pipe1 current exposure control register 2
pub mod p1cexcr2;
/**P1CST1CR (r) register accessor: DCMIPP Pipe1 current statistics 1 control register

You can [`read`](crate::Reg::read) this register and get [`p1cst1cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CST1CR)

For information about available fields see [`mod@p1cst1cr`] module*/
pub type P1CST1CR = crate::Reg<p1cst1cr::P1CST1CRrs>;
///DCMIPP Pipe1 current statistics 1 control register
pub mod p1cst1cr;
/**P1CST2CR (r) register accessor: DCMIPP Pipe1 current statistics 2 control register

You can [`read`](crate::Reg::read) this register and get [`p1cst2cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CST2CR)

For information about available fields see [`mod@p1cst2cr`] module*/
pub type P1CST2CR = crate::Reg<p1cst2cr::P1CST2CRrs>;
///DCMIPP Pipe1 current statistics 2 control register
pub mod p1cst2cr;
/**P1CST3CR (r) register accessor: DCMIPP Pipe1 current statistics 3 control register

You can [`read`](crate::Reg::read) this register and get [`p1cst3cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CST3CR)

For information about available fields see [`mod@p1cst3cr`] module*/
pub type P1CST3CR = crate::Reg<p1cst3cr::P1CST3CRrs>;
///DCMIPP Pipe1 current statistics 3 control register
pub mod p1cst3cr;
/**P1CSTSTR (r) register accessor: DCMIPP Pipe1 current statistics window start register

You can [`read`](crate::Reg::read) this register and get [`p1cststr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CSTSTR)

For information about available fields see [`mod@p1cststr`] module*/
pub type P1CSTSTR = crate::Reg<p1cststr::P1CSTSTRrs>;
///DCMIPP Pipe1 current statistics window start register
pub mod p1cststr;
/**P1CSTSZR (r) register accessor: DCMIPP Pipe1 current statistics window size register

You can [`read`](crate::Reg::read) this register and get [`p1cstszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CSTSZR)

For information about available fields see [`mod@p1cstszr`] module*/
pub type P1CSTSZR = crate::Reg<p1cstszr::P1CSTSZRrs>;
///DCMIPP Pipe1 current statistics window size register
pub mod p1cstszr;
/**P1CCCCR (r) register accessor: DCMIPP Pipe1 current ColorConv configuration register

You can [`read`](crate::Reg::read) this register and get [`p1ccccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCCR)

For information about available fields see [`mod@p1ccccr`] module*/
pub type P1CCCCR = crate::Reg<p1ccccr::P1CCCCRrs>;
///DCMIPP Pipe1 current ColorConv configuration register
pub mod p1ccccr;
/**P1CCCRR1 (r) register accessor: DCMIPP Pipe1 current ColorConv red coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1cccrr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCRR1)

For information about available fields see [`mod@p1cccrr1`] module*/
pub type P1CCCRR1 = crate::Reg<p1cccrr1::P1CCCRR1rs>;
///DCMIPP Pipe1 current ColorConv red coefficient register 1
pub mod p1cccrr1;
/**P1CCCRR2 (r) register accessor: DCMIPP Pipe1 current ColorConv red coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1cccrr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCRR2)

For information about available fields see [`mod@p1cccrr2`] module*/
pub type P1CCCRR2 = crate::Reg<p1cccrr2::P1CCCRR2rs>;
///DCMIPP Pipe1 current ColorConv red coefficient register 2
pub mod p1cccrr2;
/**P1CCCGR1 (r) register accessor: DCMIPP Pipe1 current ColorConv green coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1cccgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCGR1)

For information about available fields see [`mod@p1cccgr1`] module*/
pub type P1CCCGR1 = crate::Reg<p1cccgr1::P1CCCGR1rs>;
///DCMIPP Pipe1 current ColorConv green coefficient register 1
pub mod p1cccgr1;
/**P1CCCGR2 (r) register accessor: DCMIPP Pipe1 current ColorConv green coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1cccgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCGR2)

For information about available fields see [`mod@p1cccgr2`] module*/
pub type P1CCCGR2 = crate::Reg<p1cccgr2::P1CCCGR2rs>;
///DCMIPP Pipe1 current ColorConv green coefficient register 2
pub mod p1cccgr2;
/**P1CCCBR1 (r) register accessor: DCMIPP Pipex current ColorConv blue coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1cccbr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCBR1)

For information about available fields see [`mod@p1cccbr1`] module*/
pub type P1CCCBR1 = crate::Reg<p1cccbr1::P1CCCBR1rs>;
///DCMIPP Pipex current ColorConv blue coefficient register 1
pub mod p1cccbr1;
/**P1CCCBR2 (r) register accessor: DCMIPP Pipe1 current ColorConv blue coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1cccbr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCBR2)

For information about available fields see [`mod@p1cccbr2`] module*/
pub type P1CCCBR2 = crate::Reg<p1cccbr2::P1CCCBR2rs>;
///DCMIPP Pipe1 current ColorConv blue coefficient register 2
pub mod p1cccbr2;
/**P1CCTCR1 (r) register accessor: DCMIPP Pipe1 current contrast control register 1

You can [`read`](crate::Reg::read) this register and get [`p1cctcr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCTCR1)

For information about available fields see [`mod@p1cctcr1`] module*/
pub type P1CCTCR1 = crate::Reg<p1cctcr1::P1CCTCR1rs>;
///DCMIPP Pipe1 current contrast control register 1
pub mod p1cctcr1;
/**P1CCTCR2 (r) register accessor: DCMIPP Pipe1 current contrast control register 2

You can [`read`](crate::Reg::read) this register and get [`p1cctcr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCTCR2)

For information about available fields see [`mod@p1cctcr2`] module*/
pub type P1CCTCR2 = crate::Reg<p1cctcr2::P1CCTCR2rs>;
///DCMIPP Pipe1 current contrast control register 2
pub mod p1cctcr2;
/**P1CCTCR3 (r) register accessor: DCMIPP Pipe1 current contrast control register 3

You can [`read`](crate::Reg::read) this register and get [`p1cctcr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCTCR3)

For information about available fields see [`mod@p1cctcr3`] module*/
pub type P1CCTCR3 = crate::Reg<p1cctcr3::P1CCTCR3rs>;
///DCMIPP Pipe1 current contrast control register 3
pub mod p1cctcr3;
/**P1CFCTCR (r) register accessor: DCMIPP Pipex current flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cfctcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CFCTCR)

For information about available fields see [`mod@p1cfctcr`] module*/
pub type P1CFCTCR = crate::Reg<p1cfctcr::P1CFCTCRrs>;
///DCMIPP Pipex current flow control configuration register
pub mod p1cfctcr;
/**P1CCRSTR (r) register accessor: DCMIPP Pipex current crop window start register

You can [`read`](crate::Reg::read) this register and get [`p1ccrstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCRSTR)

For information about available fields see [`mod@p1ccrstr`] module*/
pub type P1CCRSTR = crate::Reg<p1ccrstr::P1CCRSTRrs>;
///DCMIPP Pipex current crop window start register
pub mod p1ccrstr;
/**P1CCRSZR (r) register accessor: DCMIPP Pipex current crop window size register

You can [`read`](crate::Reg::read) this register and get [`p1ccrszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCRSZR)

For information about available fields see [`mod@p1ccrszr`] module*/
pub type P1CCRSZR = crate::Reg<p1ccrszr::P1CCRSZRrs>;
///DCMIPP Pipex current crop window size register
pub mod p1ccrszr;
/**P1CDCCR (rw) register accessor: DCMIPP Pipex current decimation register

You can [`read`](crate::Reg::read) this register and get [`p1cdccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cdccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CDCCR)

For information about available fields see [`mod@p1cdccr`] module*/
pub type P1CDCCR = crate::Reg<p1cdccr::P1CDCCRrs>;
///DCMIPP Pipex current decimation register
pub mod p1cdccr;
/**P1CDSCR (r) register accessor: DCMIPP Pipex current downsize configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cdscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CDSCR)

For information about available fields see [`mod@p1cdscr`] module*/
pub type P1CDSCR = crate::Reg<p1cdscr::P1CDSCRrs>;
///DCMIPP Pipex current downsize configuration register
pub mod p1cdscr;
/**P1CDSRTIOR (r) register accessor: DCMIPP Pipex current downsize ratio register

You can [`read`](crate::Reg::read) this register and get [`p1cdsrtior::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CDSRTIOR)

For information about available fields see [`mod@p1cdsrtior`] module*/
pub type P1CDSRTIOR = crate::Reg<p1cdsrtior::P1CDSRTIORrs>;
///DCMIPP Pipex current downsize ratio register
pub mod p1cdsrtior;
/**P1CDSSZR (r) register accessor: DCMIPP Pipex current downsize destination size register

You can [`read`](crate::Reg::read) this register and get [`p1cdsszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CDSSZR)

For information about available fields see [`mod@p1cdsszr`] module*/
pub type P1CDSSZR = crate::Reg<p1cdsszr::P1CDSSZRrs>;
///DCMIPP Pipex current downsize destination size register
pub mod p1cdsszr;
/**P1CCMRICR (r) register accessor: DCMIPP Pipex current common ROI configuration register

You can [`read`](crate::Reg::read) this register and get [`p1ccmricr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCMRICR)

For information about available fields see [`mod@p1ccmricr`] module*/
pub type P1CCMRICR = crate::Reg<p1ccmricr::P1CCMRICRrs>;
///DCMIPP Pipex current common ROI configuration register
pub mod p1ccmricr;
/**P1CRI1CR1 (r) register accessor: DCMIPP Pipe1 current ROI1 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri1cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI1CR1)

For information about available fields see [`mod@p1cri1cr1`] module*/
pub type P1CRI1CR1 = crate::Reg<p1cri1cr1::P1CRI1CR1rs>;
///DCMIPP Pipe1 current ROI1 configuration register 1
pub mod p1cri1cr1;
/**P1CRI1CR2 (r) register accessor: DCMIPP Pipe1 current ROI1 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri1cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI1CR2)

For information about available fields see [`mod@p1cri1cr2`] module*/
pub type P1CRI1CR2 = crate::Reg<p1cri1cr2::P1CRI1CR2rs>;
///DCMIPP Pipe1 current ROI1 configuration register 2
pub mod p1cri1cr2;
/**P1CRI2CR1 (r) register accessor: DCMIPP Pipe1 current ROI2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri2cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI2CR1)

For information about available fields see [`mod@p1cri2cr1`] module*/
pub type P1CRI2CR1 = crate::Reg<p1cri2cr1::P1CRI2CR1rs>;
///DCMIPP Pipe1 current ROI2 configuration register 1
pub mod p1cri2cr1;
/**P1CRI2CR2 (r) register accessor: DCMIPP Pipe1 current ROI2 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri2cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI2CR2)

For information about available fields see [`mod@p1cri2cr2`] module*/
pub type P1CRI2CR2 = crate::Reg<p1cri2cr2::P1CRI2CR2rs>;
///DCMIPP Pipe1 current ROI2 configuration register 2
pub mod p1cri2cr2;
/**P1CRI3CR1 (r) register accessor: DCMIPP Pipe1 current ROI3 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri3cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI3CR1)

For information about available fields see [`mod@p1cri3cr1`] module*/
pub type P1CRI3CR1 = crate::Reg<p1cri3cr1::P1CRI3CR1rs>;
///DCMIPP Pipe1 current ROI3 configuration register 1
pub mod p1cri3cr1;
/**P1CRI3CR2 (r) register accessor: DCMIPP Pipe1 current ROI3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri3cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI3CR2)

For information about available fields see [`mod@p1cri3cr2`] module*/
pub type P1CRI3CR2 = crate::Reg<p1cri3cr2::P1CRI3CR2rs>;
///DCMIPP Pipe1 current ROI3 configuration register 2
pub mod p1cri3cr2;
/**P1CRI4CR1 (r) register accessor: DCMIPP Pipe1 current ROI4 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri4cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI4CR1)

For information about available fields see [`mod@p1cri4cr1`] module*/
pub type P1CRI4CR1 = crate::Reg<p1cri4cr1::P1CRI4CR1rs>;
///DCMIPP Pipe1 current ROI4 configuration register 1
pub mod p1cri4cr1;
/**P1CRI4CR2 (r) register accessor: DCMIPP Pipe1 current ROI4 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri4cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI4CR2)

For information about available fields see [`mod@p1cri4cr2`] module*/
pub type P1CRI4CR2 = crate::Reg<p1cri4cr2::P1CRI4CR2rs>;
///DCMIPP Pipe1 current ROI4 configuration register 2
pub mod p1cri4cr2;
/**P1CRI5CR1 (r) register accessor: DCMIPP Pipe1 current ROI5 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri5cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI5CR1)

For information about available fields see [`mod@p1cri5cr1`] module*/
pub type P1CRI5CR1 = crate::Reg<p1cri5cr1::P1CRI5CR1rs>;
///DCMIPP Pipe1 current ROI5 configuration register 1
pub mod p1cri5cr1;
/**P1CRI5CR2 (r) register accessor: DCMIPP Pipe1 current ROI5 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri5cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI5CR2)

For information about available fields see [`mod@p1cri5cr2`] module*/
pub type P1CRI5CR2 = crate::Reg<p1cri5cr2::P1CRI5CR2rs>;
///DCMIPP Pipe1 current ROI5 configuration register 2
pub mod p1cri5cr2;
/**P1CRI6CR1 (r) register accessor: DCMIPP Pipe1 current ROI6 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri6cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI6CR1)

For information about available fields see [`mod@p1cri6cr1`] module*/
pub type P1CRI6CR1 = crate::Reg<p1cri6cr1::P1CRI6CR1rs>;
///DCMIPP Pipe1 current ROI6 configuration register 1
pub mod p1cri6cr1;
/**P1CRI6CR2 (r) register accessor: DCMIPP Pipe1 current ROI6 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri6cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI6CR2)

For information about available fields see [`mod@p1cri6cr2`] module*/
pub type P1CRI6CR2 = crate::Reg<p1cri6cr2::P1CRI6CR2rs>;
///DCMIPP Pipe1 current ROI6 configuration register 2
pub mod p1cri6cr2;
/**P1CRI7CR1 (r) register accessor: DCMIPP Pipe1 current ROI7 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri7cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI7CR1)

For information about available fields see [`mod@p1cri7cr1`] module*/
pub type P1CRI7CR1 = crate::Reg<p1cri7cr1::P1CRI7CR1rs>;
///DCMIPP Pipe1 current ROI7 configuration register 1
pub mod p1cri7cr1;
/**P1CRI7CR2 (r) register accessor: DCMIPP Pipe1 current ROI7 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri7cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI7CR2)

For information about available fields see [`mod@p1cri7cr2`] module*/
pub type P1CRI7CR2 = crate::Reg<p1cri7cr2::P1CRI7CR2rs>;
///DCMIPP Pipe1 current ROI7 configuration register 2
pub mod p1cri7cr2;
/**P1CRI8CR1 (r) register accessor: DCMIPP Pipe1 current ROI8 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1cri8cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI8CR1)

For information about available fields see [`mod@p1cri8cr1`] module*/
pub type P1CRI8CR1 = crate::Reg<p1cri8cr1::P1CRI8CR1rs>;
///DCMIPP Pipe1 current ROI8 configuration register 1
pub mod p1cri8cr1;
/**P1CRI8CR2 (r) register accessor: DCMIPP Pipe1 current ROI8 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p1cri8cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CRI8CR2)

For information about available fields see [`mod@p1cri8cr2`] module*/
pub type P1CRI8CR2 = crate::Reg<p1cri8cr2::P1CRI8CR2rs>;
///DCMIPP Pipe1 current ROI8 configuration register 2
pub mod p1cri8cr2;
/**P1CPPCR (r) register accessor: DCMIPP Pipe1 current pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cppcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPCR)

For information about available fields see [`mod@p1cppcr`] module*/
pub type P1CPPCR = crate::Reg<p1cppcr::P1CPPCRrs>;
///DCMIPP Pipe1 current pixel packer configuration register
pub mod p1cppcr;
/**P1CPPM0AR1 (r) register accessor: DCMIPP Pipe1 current pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1cppm0ar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM0AR1)

For information about available fields see [`mod@p1cppm0ar1`] module*/
pub type P1CPPM0AR1 = crate::Reg<p1cppm0ar1::P1CPPM0AR1rs>;
///DCMIPP Pipe1 current pixel packer Memory0 address register 1
pub mod p1cppm0ar1;
/**P1CPPM0AR2 (r) register accessor: DCMIPP Pipe1 current pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p1cppm0ar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM0AR2)

For information about available fields see [`mod@p1cppm0ar2`] module*/
pub type P1CPPM0AR2 = crate::Reg<p1cppm0ar2::P1CPPM0AR2rs>;
///DCMIPP Pipe1 current pixel packer Memory0 address register 2
pub mod p1cppm0ar2;
/**P1CPPM0PR (r) register accessor: DCMIPP Pipex current pixel packer Memory0 pitch register

You can [`read`](crate::Reg::read) this register and get [`p1cppm0pr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM0PR)

For information about available fields see [`mod@p1cppm0pr`] module*/
pub type P1CPPM0PR = crate::Reg<p1cppm0pr::P1CPPM0PRrs>;
///DCMIPP Pipex current pixel packer Memory0 pitch register
pub mod p1cppm0pr;
/**P1CPPM1AR1 (r) register accessor: DCMIPP Pipex current pixel packer Memory1 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1cppm1ar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM1AR1)

For information about available fields see [`mod@p1cppm1ar1`] module*/
pub type P1CPPM1AR1 = crate::Reg<p1cppm1ar1::P1CPPM1AR1rs>;
///DCMIPP Pipex current pixel packer Memory1 address register 1
pub mod p1cppm1ar1;
/**P1CPPM1AR2 (r) register accessor: DCMIPP Pipex current pixel packer Memory1 address register 2

You can [`read`](crate::Reg::read) this register and get [`p1cppm1ar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM1AR2)

For information about available fields see [`mod@p1cppm1ar2`] module*/
pub type P1CPPM1AR2 = crate::Reg<p1cppm1ar2::P1CPPM1AR2rs>;
///DCMIPP Pipex current pixel packer Memory1 address register 2
pub mod p1cppm1ar2;
/**P1CPPM1PR (r) register accessor: DCMIPP Pipex current pixel packer Memory1 pitch register

You can [`read`](crate::Reg::read) this register and get [`p1cppm1pr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM1PR)

For information about available fields see [`mod@p1cppm1pr`] module*/
pub type P1CPPM1PR = crate::Reg<p1cppm1pr::P1CPPM1PRrs>;
///DCMIPP Pipex current pixel packer Memory1 pitch register
pub mod p1cppm1pr;
/**P1CPPM2AR1 (r) register accessor: DCMIPP Pipex current pixel packer Memory2 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1cppm2ar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM2AR1)

For information about available fields see [`mod@p1cppm2ar1`] module*/
pub type P1CPPM2AR1 = crate::Reg<p1cppm2ar1::P1CPPM2AR1rs>;
///DCMIPP Pipex current pixel packer Memory2 address register 1
pub mod p1cppm2ar1;
/**P1CPPM2AR2 (r) register accessor: DCMIPP Pipex current pixel packer Memory2 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1cppm2ar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CPPM2AR2)

For information about available fields see [`mod@p1cppm2ar2`] module*/
pub type P1CPPM2AR2 = crate::Reg<p1cppm2ar2::P1CPPM2AR2rs>;
///DCMIPP Pipex current pixel packer Memory2 address register 1
pub mod p1cppm2ar2;
/**P2FSCR (rw) register accessor: DCMIPP Pipe2 flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p2fscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2fscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2FSCR)

For information about available fields see [`mod@p2fscr`] module*/
pub type P2FSCR = crate::Reg<p2fscr::P2FSCRrs>;
///DCMIPP Pipe2 flow selection configuration register
pub mod p2fscr;
/**P2FCTCR (rw) register accessor: DCMIPP Pipex flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p2fctcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2fctcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2FCTCR)

For information about available fields see [`mod@p2fctcr`] module*/
pub type P2FCTCR = crate::Reg<p2fctcr::P2FCTCRrs>;
///DCMIPP Pipex flow control configuration register
pub mod p2fctcr;
/**P2CRSTR (rw) register accessor: DCMIPP Pipex crop window start register

You can [`read`](crate::Reg::read) this register and get [`p2crstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2crstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRSTR)

For information about available fields see [`mod@p2crstr`] module*/
pub type P2CRSTR = crate::Reg<p2crstr::P2CRSTRrs>;
///DCMIPP Pipex crop window start register
pub mod p2crstr;
/**P2CRSZR (rw) register accessor: DCMIPP Pipex crop window size register

You can [`read`](crate::Reg::read) this register and get [`p2crszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2crszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRSZR)

For information about available fields see [`mod@p2crszr`] module*/
pub type P2CRSZR = crate::Reg<p2crszr::P2CRSZRrs>;
///DCMIPP Pipex crop window size register
pub mod p2crszr;
/**P2DCCR (rw) register accessor: DCMIPP Pipex decimation register

You can [`read`](crate::Reg::read) this register and get [`p2dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2DCCR)

For information about available fields see [`mod@p2dccr`] module*/
pub type P2DCCR = crate::Reg<p2dccr::P2DCCRrs>;
///DCMIPP Pipex decimation register
pub mod p2dccr;
/**P2DSCR (rw) register accessor: DCMIPP Pipex downsize configuration register

You can [`read`](crate::Reg::read) this register and get [`p2dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2DSCR)

For information about available fields see [`mod@p2dscr`] module*/
pub type P2DSCR = crate::Reg<p2dscr::P2DSCRrs>;
///DCMIPP Pipex downsize configuration register
pub mod p2dscr;
/**P2DSRTIOR (rw) register accessor: DCMIPP Pipex downsize ratio register

You can [`read`](crate::Reg::read) this register and get [`p2dsrtior::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dsrtior::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2DSRTIOR)

For information about available fields see [`mod@p2dsrtior`] module*/
pub type P2DSRTIOR = crate::Reg<p2dsrtior::P2DSRTIORrs>;
///DCMIPP Pipex downsize ratio register
pub mod p2dsrtior;
/**P2DSSZR (rw) register accessor: DCMIPP Pipex downsize destination size register

You can [`read`](crate::Reg::read) this register and get [`p2dsszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dsszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2DSSZR)

For information about available fields see [`mod@p2dsszr`] module*/
pub type P2DSSZR = crate::Reg<p2dsszr::P2DSSZRrs>;
///DCMIPP Pipex downsize destination size register
pub mod p2dsszr;
/**P2CMRICR (rw) register accessor: DCMIPP Pipex common ROI configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cmricr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2cmricr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CMRICR)

For information about available fields see [`mod@p2cmricr`] module*/
pub type P2CMRICR = crate::Reg<p2cmricr::P2CMRICRrs>;
///DCMIPP Pipex common ROI configuration register
pub mod p2cmricr;
/**P2RI1CR1 (rw) register accessor: DCMIPP Pipe2 ROI1 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI1CR1)

For information about available fields see [`mod@p2ri1cr1`] module*/
pub type P2RI1CR1 = crate::Reg<p2ri1cr1::P2RI1CR1rs>;
///DCMIPP Pipe2 ROI1 configuration register 1
pub mod p2ri1cr1;
/**P2RI1CR2 (rw) register accessor: DCMIPP Pipe2 ROI1 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri1cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri1cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI1CR2)

For information about available fields see [`mod@p2ri1cr2`] module*/
pub type P2RI1CR2 = crate::Reg<p2ri1cr2::P2RI1CR2rs>;
///DCMIPP Pipe2 ROI1 configuration register 2
pub mod p2ri1cr2;
/**P2RI2CR1 (rw) register accessor: DCMIPP Pipe2 ROI2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI2CR1)

For information about available fields see [`mod@p2ri2cr1`] module*/
pub type P2RI2CR1 = crate::Reg<p2ri2cr1::P2RI2CR1rs>;
///DCMIPP Pipe2 ROI2 configuration register 1
pub mod p2ri2cr1;
/**P2RI2CR2 (rw) register accessor: DCMIPP Pipe2 ROI2 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri2cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri2cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI2CR2)

For information about available fields see [`mod@p2ri2cr2`] module*/
pub type P2RI2CR2 = crate::Reg<p2ri2cr2::P2RI2CR2rs>;
///DCMIPP Pipe2 ROI2 configuration register 2
pub mod p2ri2cr2;
/**P2RI3CR1 (rw) register accessor: DCMIPP Pipe2 ROI3 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri3cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri3cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI3CR1)

For information about available fields see [`mod@p2ri3cr1`] module*/
pub type P2RI3CR1 = crate::Reg<p2ri3cr1::P2RI3CR1rs>;
///DCMIPP Pipe2 ROI3 configuration register 1
pub mod p2ri3cr1;
/**P2RI3CR2 (rw) register accessor: DCMIPP Pipe2 ROI3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri3cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri3cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI3CR2)

For information about available fields see [`mod@p2ri3cr2`] module*/
pub type P2RI3CR2 = crate::Reg<p2ri3cr2::P2RI3CR2rs>;
///DCMIPP Pipe2 ROI3 configuration register 2
pub mod p2ri3cr2;
/**P2RI4CR1 (rw) register accessor: DCMIPP Pipe2 ROI4 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri4cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri4cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI4CR1)

For information about available fields see [`mod@p2ri4cr1`] module*/
pub type P2RI4CR1 = crate::Reg<p2ri4cr1::P2RI4CR1rs>;
///DCMIPP Pipe2 ROI4 configuration register 1
pub mod p2ri4cr1;
/**P2RI4CR2 (rw) register accessor: DCMIPP Pipe2 ROI4 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri4cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri4cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI4CR2)

For information about available fields see [`mod@p2ri4cr2`] module*/
pub type P2RI4CR2 = crate::Reg<p2ri4cr2::P2RI4CR2rs>;
///DCMIPP Pipe2 ROI4 configuration register 2
pub mod p2ri4cr2;
/**P2RI5CR1 (rw) register accessor: DCMIPP Pipe2 ROI5 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri5cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri5cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI5CR1)

For information about available fields see [`mod@p2ri5cr1`] module*/
pub type P2RI5CR1 = crate::Reg<p2ri5cr1::P2RI5CR1rs>;
///DCMIPP Pipe2 ROI5 configuration register 1
pub mod p2ri5cr1;
/**P2RI5CR2 (rw) register accessor: DCMIPP Pipe2 ROI5 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri5cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri5cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI5CR2)

For information about available fields see [`mod@p2ri5cr2`] module*/
pub type P2RI5CR2 = crate::Reg<p2ri5cr2::P2RI5CR2rs>;
///DCMIPP Pipe2 ROI5 configuration register 2
pub mod p2ri5cr2;
/**P2RI6CR1 (rw) register accessor: DCMIPP Pipe2 ROI6 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri6cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri6cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI6CR1)

For information about available fields see [`mod@p2ri6cr1`] module*/
pub type P2RI6CR1 = crate::Reg<p2ri6cr1::P2RI6CR1rs>;
///DCMIPP Pipe2 ROI6 configuration register 1
pub mod p2ri6cr1;
/**P2RI6CR2 (rw) register accessor: DCMIPP Pipe2 ROI6 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri6cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri6cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI6CR2)

For information about available fields see [`mod@p2ri6cr2`] module*/
pub type P2RI6CR2 = crate::Reg<p2ri6cr2::P2RI6CR2rs>;
///DCMIPP Pipe2 ROI6 configuration register 2
pub mod p2ri6cr2;
/**P2RI7CR1 (rw) register accessor: DCMIPP Pipe2 ROI7 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri7cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri7cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI7CR1)

For information about available fields see [`mod@p2ri7cr1`] module*/
pub type P2RI7CR1 = crate::Reg<p2ri7cr1::P2RI7CR1rs>;
///DCMIPP Pipe2 ROI7 configuration register 1
pub mod p2ri7cr1;
/**P2RI7CR2 (rw) register accessor: DCMIPP Pipe2 ROI7 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri7cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri7cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI7CR2)

For information about available fields see [`mod@p2ri7cr2`] module*/
pub type P2RI7CR2 = crate::Reg<p2ri7cr2::P2RI7CR2rs>;
///DCMIPP Pipe2 ROI7 configuration register 2
pub mod p2ri7cr2;
/**P2RI8CR1 (rw) register accessor: DCMIPP Pipe2 ROI8 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2ri8cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri8cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI8CR1)

For information about available fields see [`mod@p2ri8cr1`] module*/
pub type P2RI8CR1 = crate::Reg<p2ri8cr1::P2RI8CR1rs>;
///DCMIPP Pipe2 ROI8 configuration register 1
pub mod p2ri8cr1;
/**P2RI8CR2 (rw) register accessor: DCMIPP Pipe2 ROI8 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2ri8cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ri8cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2RI8CR2)

For information about available fields see [`mod@p2ri8cr2`] module*/
pub type P2RI8CR2 = crate::Reg<p2ri8cr2::P2RI8CR2rs>;
///DCMIPP Pipe2 ROI8 configuration register 2
pub mod p2ri8cr2;
/**P2GMCR (rw) register accessor: DCMIPP Pipex gamma configuration register

You can [`read`](crate::Reg::read) this register and get [`p2gmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2gmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2GMCR)

For information about available fields see [`mod@p2gmcr`] module*/
pub type P2GMCR = crate::Reg<p2gmcr::P2GMCRrs>;
///DCMIPP Pipex gamma configuration register
pub mod p2gmcr;
/**P2PPCR (rw) register accessor: DCMIPP Pipe2 pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p2ppcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ppcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2PPCR)

For information about available fields see [`mod@p2ppcr`] module*/
pub type P2PPCR = crate::Reg<p2ppcr::P2PPCRrs>;
///DCMIPP Pipe2 pixel packer configuration register
pub mod p2ppcr;
/**P2PPM0AR1 (rw) register accessor: DCMIPP Pipe2 pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p2ppm0ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ppm0ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2PPM0AR1)

For information about available fields see [`mod@p2ppm0ar1`] module*/
pub type P2PPM0AR1 = crate::Reg<p2ppm0ar1::P2PPM0AR1rs>;
///DCMIPP Pipe2 pixel packer Memory0 address register 1
pub mod p2ppm0ar1;
/**P2PPM0AR2 (rw) register accessor: DCMIPP Pipe2 pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p2ppm0ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ppm0ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2PPM0AR2)

For information about available fields see [`mod@p2ppm0ar2`] module*/
pub type P2PPM0AR2 = crate::Reg<p2ppm0ar2::P2PPM0AR2rs>;
///DCMIPP Pipe2 pixel packer Memory0 address register 2
pub mod p2ppm0ar2;
/**P2PPM0PR (rw) register accessor: DCMIPP Pipex pixel packer Memory0 pitch register

You can [`read`](crate::Reg::read) this register and get [`p2ppm0pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ppm0pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2PPM0PR)

For information about available fields see [`mod@p2ppm0pr`] module*/
pub type P2PPM0PR = crate::Reg<p2ppm0pr::P2PPM0PRrs>;
///DCMIPP Pipex pixel packer Memory0 pitch register
pub mod p2ppm0pr;
/**P2STM0AR (r) register accessor: DCMIPP Pipex status Memory0 address register

You can [`read`](crate::Reg::read) this register and get [`p2stm0ar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2STM0AR)

For information about available fields see [`mod@p2stm0ar`] module*/
pub type P2STM0AR = crate::Reg<p2stm0ar::P2STM0ARrs>;
///DCMIPP Pipex status Memory0 address register
pub mod p2stm0ar;
/**P2IER (rw) register accessor: DCMIPP Pipe2 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`p2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2IER)

For information about available fields see [`mod@p2ier`] module*/
pub type P2IER = crate::Reg<p2ier::P2IERrs>;
///DCMIPP Pipe2 interrupt enable register
pub mod p2ier;
/**P2SR (r) register accessor: DCMIPP Pipe2 status register

You can [`read`](crate::Reg::read) this register and get [`p2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2SR)

For information about available fields see [`mod@p2sr`] module*/
pub type P2SR = crate::Reg<p2sr::P2SRrs>;
///DCMIPP Pipe2 status register
pub mod p2sr;
/**P2FCR (w) register accessor: DCMIPP Pipe2 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2FCR)

For information about available fields see [`mod@p2fcr`] module*/
pub type P2FCR = crate::Reg<p2fcr::P2FCRrs>;
///DCMIPP Pipe2 interrupt clear register
pub mod p2fcr;
/**P2CFSCR (r) register accessor: DCMIPP Pipe2 current flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cfscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CFSCR)

For information about available fields see [`mod@p2cfscr`] module*/
pub type P2CFSCR = crate::Reg<p2cfscr::P2CFSCRrs>;
///DCMIPP Pipe2 current flow selection configuration register
pub mod p2cfscr;
/**P2CFCTCR (r) register accessor: DCMIPP Pipex current flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cfctcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CFCTCR)

For information about available fields see [`mod@p2cfctcr`] module*/
pub type P2CFCTCR = crate::Reg<p2cfctcr::P2CFCTCRrs>;
///DCMIPP Pipex current flow control configuration register
pub mod p2cfctcr;
/**P2CCRSTR (r) register accessor: DCMIPP Pipex current crop window start register

You can [`read`](crate::Reg::read) this register and get [`p2ccrstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CCRSTR)

For information about available fields see [`mod@p2ccrstr`] module*/
pub type P2CCRSTR = crate::Reg<p2ccrstr::P2CCRSTRrs>;
///DCMIPP Pipex current crop window start register
pub mod p2ccrstr;
/**P2CCRSZR (r) register accessor: DCMIPP Pipex current crop window size register

You can [`read`](crate::Reg::read) this register and get [`p2ccrszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CCRSZR)

For information about available fields see [`mod@p2ccrszr`] module*/
pub type P2CCRSZR = crate::Reg<p2ccrszr::P2CCRSZRrs>;
///DCMIPP Pipex current crop window size register
pub mod p2ccrszr;
/**P2CDCCR (rw) register accessor: DCMIPP Pipex current decimation register

You can [`read`](crate::Reg::read) this register and get [`p2cdccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2cdccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CDCCR)

For information about available fields see [`mod@p2cdccr`] module*/
pub type P2CDCCR = crate::Reg<p2cdccr::P2CDCCRrs>;
///DCMIPP Pipex current decimation register
pub mod p2cdccr;
/**P2CDSCR (r) register accessor: DCMIPP Pipex current downsize configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cdscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CDSCR)

For information about available fields see [`mod@p2cdscr`] module*/
pub type P2CDSCR = crate::Reg<p2cdscr::P2CDSCRrs>;
///DCMIPP Pipex current downsize configuration register
pub mod p2cdscr;
/**P2CDSRTIOR (r) register accessor: DCMIPP Pipex current downsize ratio register

You can [`read`](crate::Reg::read) this register and get [`p2cdsrtior::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CDSRTIOR)

For information about available fields see [`mod@p2cdsrtior`] module*/
pub type P2CDSRTIOR = crate::Reg<p2cdsrtior::P2CDSRTIORrs>;
///DCMIPP Pipex current downsize ratio register
pub mod p2cdsrtior;
/**P2CDSSZR (r) register accessor: DCMIPP Pipex current downsize destination size register

You can [`read`](crate::Reg::read) this register and get [`p2cdsszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CDSSZR)

For information about available fields see [`mod@p2cdsszr`] module*/
pub type P2CDSSZR = crate::Reg<p2cdsszr::P2CDSSZRrs>;
///DCMIPP Pipex current downsize destination size register
pub mod p2cdsszr;
/**P2CCMRICR (r) register accessor: DCMIPP Pipex current common ROI configuration register

You can [`read`](crate::Reg::read) this register and get [`p2ccmricr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CCMRICR)

For information about available fields see [`mod@p2ccmricr`] module*/
pub type P2CCMRICR = crate::Reg<p2ccmricr::P2CCMRICRrs>;
///DCMIPP Pipex current common ROI configuration register
pub mod p2ccmricr;
/**P2CRI1CR1 (r) register accessor: DCMIPP Pipe2 current ROI1 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri1cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI1CR1)

For information about available fields see [`mod@p2cri1cr1`] module*/
pub type P2CRI1CR1 = crate::Reg<p2cri1cr1::P2CRI1CR1rs>;
///DCMIPP Pipe2 current ROI1 configuration register 1
pub mod p2cri1cr1;
/**P2CRI1CR2 (r) register accessor: DCMIPP Pipe2 current ROI1 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri1cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI1CR2)

For information about available fields see [`mod@p2cri1cr2`] module*/
pub type P2CRI1CR2 = crate::Reg<p2cri1cr2::P2CRI1CR2rs>;
///DCMIPP Pipe2 current ROI1 configuration register 2
pub mod p2cri1cr2;
/**P2CRI2CR1 (r) register accessor: DCMIPP Pipe2 current ROI2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri2cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI2CR1)

For information about available fields see [`mod@p2cri2cr1`] module*/
pub type P2CRI2CR1 = crate::Reg<p2cri2cr1::P2CRI2CR1rs>;
///DCMIPP Pipe2 current ROI2 configuration register 1
pub mod p2cri2cr1;
/**P2CRI2CR2 (r) register accessor: DCMIPP Pipe2 current ROI2 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri2cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI2CR2)

For information about available fields see [`mod@p2cri2cr2`] module*/
pub type P2CRI2CR2 = crate::Reg<p2cri2cr2::P2CRI2CR2rs>;
///DCMIPP Pipe2 current ROI2 configuration register 2
pub mod p2cri2cr2;
/**P2CRI3CR1 (r) register accessor: DCMIPP Pipe2 current ROI3 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri3cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI3CR1)

For information about available fields see [`mod@p2cri3cr1`] module*/
pub type P2CRI3CR1 = crate::Reg<p2cri3cr1::P2CRI3CR1rs>;
///DCMIPP Pipe2 current ROI3 configuration register 1
pub mod p2cri3cr1;
/**P2CRI3CR2 (r) register accessor: DCMIPP Pipe2 current ROI3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri3cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI3CR2)

For information about available fields see [`mod@p2cri3cr2`] module*/
pub type P2CRI3CR2 = crate::Reg<p2cri3cr2::P2CRI3CR2rs>;
///DCMIPP Pipe2 current ROI3 configuration register 2
pub mod p2cri3cr2;
/**P2CRI4CR1 (r) register accessor: DCMIPP Pipe2 current ROI4 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri4cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI4CR1)

For information about available fields see [`mod@p2cri4cr1`] module*/
pub type P2CRI4CR1 = crate::Reg<p2cri4cr1::P2CRI4CR1rs>;
///DCMIPP Pipe2 current ROI4 configuration register 1
pub mod p2cri4cr1;
/**P2CRI4CR2 (r) register accessor: DCMIPP Pipe2 current ROI4 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri4cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI4CR2)

For information about available fields see [`mod@p2cri4cr2`] module*/
pub type P2CRI4CR2 = crate::Reg<p2cri4cr2::P2CRI4CR2rs>;
///DCMIPP Pipe2 current ROI4 configuration register 2
pub mod p2cri4cr2;
/**P2CRI5CR1 (r) register accessor: DCMIPP Pipe2 current ROI5 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri5cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI5CR1)

For information about available fields see [`mod@p2cri5cr1`] module*/
pub type P2CRI5CR1 = crate::Reg<p2cri5cr1::P2CRI5CR1rs>;
///DCMIPP Pipe2 current ROI5 configuration register 1
pub mod p2cri5cr1;
/**P2CRI5CR2 (r) register accessor: DCMIPP Pipe2 current ROI5 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri5cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI5CR2)

For information about available fields see [`mod@p2cri5cr2`] module*/
pub type P2CRI5CR2 = crate::Reg<p2cri5cr2::P2CRI5CR2rs>;
///DCMIPP Pipe2 current ROI5 configuration register 2
pub mod p2cri5cr2;
/**P2CRI6CR1 (r) register accessor: DCMIPP Pipe2 current ROI6 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri6cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI6CR1)

For information about available fields see [`mod@p2cri6cr1`] module*/
pub type P2CRI6CR1 = crate::Reg<p2cri6cr1::P2CRI6CR1rs>;
///DCMIPP Pipe2 current ROI6 configuration register 1
pub mod p2cri6cr1;
/**P2CRI6CR2 (r) register accessor: DCMIPP Pipe2 current ROI6 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri6cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI6CR2)

For information about available fields see [`mod@p2cri6cr2`] module*/
pub type P2CRI6CR2 = crate::Reg<p2cri6cr2::P2CRI6CR2rs>;
///DCMIPP Pipe2 current ROI6 configuration register 2
pub mod p2cri6cr2;
/**P2CRI7CR1 (r) register accessor: DCMIPP Pipe2 current ROI7 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri7cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI7CR1)

For information about available fields see [`mod@p2cri7cr1`] module*/
pub type P2CRI7CR1 = crate::Reg<p2cri7cr1::P2CRI7CR1rs>;
///DCMIPP Pipe2 current ROI7 configuration register 1
pub mod p2cri7cr1;
/**P2CRI7CR2 (r) register accessor: DCMIPP Pipe2 current ROI7 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri7cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI7CR2)

For information about available fields see [`mod@p2cri7cr2`] module*/
pub type P2CRI7CR2 = crate::Reg<p2cri7cr2::P2CRI7CR2rs>;
///DCMIPP Pipe2 current ROI7 configuration register 2
pub mod p2cri7cr2;
/**P2CRI8CR1 (r) register accessor: DCMIPP Pipe2 current ROI8 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri8cr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI8CR1)

For information about available fields see [`mod@p2cri8cr1`] module*/
pub type P2CRI8CR1 = crate::Reg<p2cri8cr1::P2CRI8CR1rs>;
///DCMIPP Pipe2 current ROI8 configuration register 1
pub mod p2cri8cr1;
/**P2CRI8CR2 (r) register accessor: DCMIPP Pipe2 current ROI8 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`p2cri8cr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI8CR2)

For information about available fields see [`mod@p2cri8cr2`] module*/
pub type P2CRI8CR2 = crate::Reg<p2cri8cr2::P2CRI8CR2rs>;
///DCMIPP Pipe2 current ROI8 configuration register 2
pub mod p2cri8cr2;
/**P2CPPCR (r) register accessor: DCMIPP Pipe2 current pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cppcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CPPCR)

For information about available fields see [`mod@p2cppcr`] module*/
pub type P2CPPCR = crate::Reg<p2cppcr::P2CPPCRrs>;
///DCMIPP Pipe2 current pixel packer configuration register
pub mod p2cppcr;
/**P2CPPM0AR1 (r) register accessor: DCMIPP Pipe2 current pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p2cppm0ar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CPPM0AR1)

For information about available fields see [`mod@p2cppm0ar1`] module*/
pub type P2CPPM0AR1 = crate::Reg<p2cppm0ar1::P2CPPM0AR1rs>;
///DCMIPP Pipe2 current pixel packer Memory0 address register 1
pub mod p2cppm0ar1;
/**P2CPPM0AR2 (r) register accessor: DCMIPP Pipe2 current pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p2cppm0ar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CPPM0AR2)

For information about available fields see [`mod@p2cppm0ar2`] module*/
pub type P2CPPM0AR2 = crate::Reg<p2cppm0ar2::P2CPPM0AR2rs>;
///DCMIPP Pipe2 current pixel packer Memory0 address register 2
pub mod p2cppm0ar2;
