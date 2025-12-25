///Register `EVR` reader
pub type R = crate::R<EVRrs>;
///Field `CFEF` reader - C-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the C-FIFO is empty when controller, and that the I3C_CR register contains no control word (i.e. none IBI/CR/HJ request) when target. This flag is de-asserted by hardware to indicate that the C-FIFO is not empty when controller, and that the I3C_CR register contains one control word (i.e. a pending IBI/CR/HJ request) when target. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer.
pub type CFEF_R = crate::BitReader;
///Field `TXFEF` reader - TX-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the TX-FIFO is empty. This flag is de-asserted by hardware to indicate that the TX-FIFO is not empty. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer.
pub type TXFEF_R = crate::BitReader;
///Field `CFNFF` reader - C-FIFO not full flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a control word is to be written to the C-FIFO. This flag is de-asserted by hardware to indicate that a control word is not to be written to the C-FIFO. Note: The software must wait for CFNFF=1 (by polling or via the enabled interrupt) before writing to C-FIFO (i.e. writing to I3C_CR).
pub type CFNFF_R = crate::BitReader;
///Field `SFNEF` reader - S-FIFO not empty flag (when the I3C is acting as controller) When the I3C is acting as controller, if the S-FIFO is enabled (i.e. I3C_CFGR.RMODE=1), this flag is asserted by hardware to indicate that a status word is to be read from the S-FIFO. This flag is de-asserted by hardware to indicate that a status word is not to be read from the S-FIFO.
pub type SFNEF_R = crate::BitReader;
///Field `TXFNFF` reader - TX-FIFO not full flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte/word is to be written to the TX-FIFO. This flag is de-asserted by hardware to indicate that a data byte/word is not to be written to the TX-FIFO. Note: The software must wait for TXFNFF=1 (by polling or via the enabled interrupt) before writing to TX-FIFO (i.e. writing to I3C_TDR or I3C_TDWR depending on I3C_CFGR.TXTHRES). Note: When the I3C is acting as target, if the software intends to use the TXFNFF flag for writing into I3C_TDR/I3C_TDWR, it must have configured and set the TX-FIFO preload (i.e. write I3C_TGTTDR.PRELOAD).
pub type TXFNFF_R = crate::BitReader;
///Field `RXFNEF` reader - RX-FIFO not empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte is to be read from the RX-FIFO. This flag is de-asserted by hardware to indicate that a data byte is not to be read from the RX-FIFO. Note: The software must wait for RXFNEF=1 (by polling or via the enabled interrupt) before reading from RX-FIFO (i.e. writing to I3C_RDR or I3C_RDWR depending on I3C_CFGR.RXTHRES).
pub type RXFNEF_R = crate::BitReader;
///Field `TXLASTF` reader - last written data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.TXTHRES) of a message is to be written to the TX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is written.
pub type TXLASTF_R = crate::BitReader;
///Field `RXLASTF` reader - last read data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.RXTHRES) of a message is to be read from the RX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is read.
pub type RXLASTF_R = crate::BitReader;
///Field `FCF` reader - frame complete flag (whatever the I3C is acting as controller/target) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a frame has been (normally) completed on the I3C bus, i.e when a stop is issued. When the I3C is acting as target, this flag is asserted by hardware to indicate that a message addressed to/by this target has been (normally) completed on the I3C bus, i.e when a next stop or repeated start is then issued by the controller. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CFCF bit.
pub type FCF_R = crate::BitReader;
///Field `RXTGTENDF` reader - target-initiated read end flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that the target has prematurely ended a read transfer. Then, software should read I3C_SR to get more information on the prematurely read transfer. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRXTGTENDF bit.
pub type RXTGTENDF_R = crate::BitReader;
///Field `ERRF` reader - flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that an error occurred.Then, software should read I3C_SER to get the error type. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CERRF bit.
pub type ERRF_R = crate::BitReader;
///Field `IBIF` reader - IBI flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an IBI request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIF bit.
pub type IBIF_R = crate::BitReader;
///Field `IBIENDF` reader - IBI end flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a IBI transfer has been received and completed (IBI acknowledged and IBI data bytes read by controller if any). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIENDF bit.
pub type IBIENDF_R = crate::BitReader;
///Field `CRF` reader - controller-role request flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a controller-role request has been acknowledged and completed (by hardware). The software should then issue a GETACCCR CCC (get accept controller role) for the controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRF bit.
pub type CRF_R = crate::BitReader;
///Field `CRUPDF` reader - controller-role update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that it has now gained the controller role after the completed controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRUPDF bit.
pub type CRUPDF_R = crate::BitReader;
///Field `HJF` reader - hot-join flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an hot join request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CHJF bit.
pub type HJF_R = crate::BitReader;
///Field `WKPF` reader - wakeup/missed start flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a start has been detected (i.e. a SDA falling edge followed by a SCL falling edge) but on the next SCL falling edge, the I3C kernel clock is (still) gated. Thus an I3C bus transaction may have been lost by the target. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CWKPF bit.
pub type WKPF_R = crate::BitReader;
///Field `GETF` reader - get flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that any direct CCC of get type (GET*** CCC) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGETF bit.
pub type GETF_R = crate::BitReader;
///Field `STAF` reader - get status flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct GETSTATUS CCC (get status) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CSTAF bit.
pub type STAF_R = crate::BitReader;
///Field `DAUPDF` reader - dynamic address update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a dynamic address update has been received via any of the broadcast ENTDAA, RSTDAA and direct SETNEWDA CCC. Then, software should read I3C_DEVR0.DA\[6:0\] to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDAUPDF bit.
pub type DAUPDF_R = crate::BitReader;
///Field `MWLUPDF` reader - maximum write length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMWL CCC (set max write length) has been received. Then, software should read I3C_MAXWLR.MWL\[15:0\] to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMWLUPDF bit.
pub type MWLUPDF_R = crate::BitReader;
///Field `MRLUPDF` reader - maximum read length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMRL CCC (set max read length) has been received. Then, software should read I3C_MAXRLR.MRL\[15:0\] to get the maximum read length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMRLUPDF bit.
pub type MRLUPDF_R = crate::BitReader;
///Field `RSTF` reader - reset pattern flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a reset pattern has been detected (i.e. 14 SDA transitions while SCL is low, followed by repeated start, then stop). Then, software should read I3C_DEVR0.RSTACT\[1:0\] and I3C_DEVR0.RSTVAL, to know what reset level is required. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\[1:0\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRSTF bit.
pub type RSTF_R = crate::BitReader;
///Field `ASUPDF` reader - activity state update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENTASx CCC (with x=0...3) has been received. Then, software should read I3C_DEVR0.AS\[1:0\]. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CASUPDF bit.
pub type ASUPDF_R = crate::BitReader;
///Field `INTUPDF` reader - interrupt/controller-role/hot-join update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENEC/DISEC CCC (enable/disable target events) has been received, where a target event is either an interrupt/IBI request, a controller-role request, or an hot-join request. Then, software should read respectively I3C_DEVR0.IBIEN, I3C_DEVR0.CREN or I3C_DEVR0.HJEN. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CINTUPDF bit.
pub type INTUPDF_R = crate::BitReader;
///Field `DEFF` reader - DEFTGTS flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFTGTS CCC (define list of targets) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDEFF bit.
pub type DEFF_R = crate::BitReader;
///Field `GRPF` reader - group addressing flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFGRPA CCC (define list of group addresses) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGRPF bit.
pub type GRPF_R = crate::BitReader;
impl R {
    ///Bit 0 - C-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the C-FIFO is empty when controller, and that the I3C_CR register contains no control word (i.e. none IBI/CR/HJ request) when target. This flag is de-asserted by hardware to indicate that the C-FIFO is not empty when controller, and that the I3C_CR register contains one control word (i.e. a pending IBI/CR/HJ request) when target. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer.
    #[inline(always)]
    pub fn cfef(&self) -> CFEF_R {
        CFEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the TX-FIFO is empty. This flag is de-asserted by hardware to indicate that the TX-FIFO is not empty. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. I3C_CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer.
    #[inline(always)]
    pub fn txfef(&self) -> TXFEF_R {
        TXFEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - C-FIFO not full flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a control word is to be written to the C-FIFO. This flag is de-asserted by hardware to indicate that a control word is not to be written to the C-FIFO. Note: The software must wait for CFNFF=1 (by polling or via the enabled interrupt) before writing to C-FIFO (i.e. writing to I3C_CR).
    #[inline(always)]
    pub fn cfnff(&self) -> CFNFF_R {
        CFNFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - S-FIFO not empty flag (when the I3C is acting as controller) When the I3C is acting as controller, if the S-FIFO is enabled (i.e. I3C_CFGR.RMODE=1), this flag is asserted by hardware to indicate that a status word is to be read from the S-FIFO. This flag is de-asserted by hardware to indicate that a status word is not to be read from the S-FIFO.
    #[inline(always)]
    pub fn sfnef(&self) -> SFNEF_R {
        SFNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX-FIFO not full flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte/word is to be written to the TX-FIFO. This flag is de-asserted by hardware to indicate that a data byte/word is not to be written to the TX-FIFO. Note: The software must wait for TXFNFF=1 (by polling or via the enabled interrupt) before writing to TX-FIFO (i.e. writing to I3C_TDR or I3C_TDWR depending on I3C_CFGR.TXTHRES). Note: When the I3C is acting as target, if the software intends to use the TXFNFF flag for writing into I3C_TDR/I3C_TDWR, it must have configured and set the TX-FIFO preload (i.e. write I3C_TGTTDR.PRELOAD).
    #[inline(always)]
    pub fn txfnff(&self) -> TXFNFF_R {
        TXFNFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RX-FIFO not empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte is to be read from the RX-FIFO. This flag is de-asserted by hardware to indicate that a data byte is not to be read from the RX-FIFO. Note: The software must wait for RXFNEF=1 (by polling or via the enabled interrupt) before reading from RX-FIFO (i.e. writing to I3C_RDR or I3C_RDWR depending on I3C_CFGR.RXTHRES).
    #[inline(always)]
    pub fn rxfnef(&self) -> RXFNEF_R {
        RXFNEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - last written data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.TXTHRES) of a message is to be written to the TX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is written.
    #[inline(always)]
    pub fn txlastf(&self) -> TXLASTF_R {
        TXLASTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - last read data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on I3C_CFGR.RXTHRES) of a message is to be read from the RX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is read.
    #[inline(always)]
    pub fn rxlastf(&self) -> RXLASTF_R {
        RXLASTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - frame complete flag (whatever the I3C is acting as controller/target) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a frame has been (normally) completed on the I3C bus, i.e when a stop is issued. When the I3C is acting as target, this flag is asserted by hardware to indicate that a message addressed to/by this target has been (normally) completed on the I3C bus, i.e when a next stop or repeated start is then issued by the controller. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CFCF bit.
    #[inline(always)]
    pub fn fcf(&self) -> FCF_R {
        FCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - target-initiated read end flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that the target has prematurely ended a read transfer. Then, software should read I3C_SR to get more information on the prematurely read transfer. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRXTGTENDF bit.
    #[inline(always)]
    pub fn rxtgtendf(&self) -> RXTGTENDF_R {
        RXTGTENDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that an error occurred.Then, software should read I3C_SER to get the error type. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CERRF bit.
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - IBI flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an IBI request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIF bit.
    #[inline(always)]
    pub fn ibif(&self) -> IBIF_R {
        IBIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IBI end flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a IBI transfer has been received and completed (IBI acknowledged and IBI data bytes read by controller if any). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CIBIENDF bit.
    #[inline(always)]
    pub fn ibiendf(&self) -> IBIENDF_R {
        IBIENDF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - controller-role request flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a controller-role request has been acknowledged and completed (by hardware). The software should then issue a GETACCCR CCC (get accept controller role) for the controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRF bit.
    #[inline(always)]
    pub fn crf(&self) -> CRF_R {
        CRF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - controller-role update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that it has now gained the controller role after the completed controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CCRUPDF bit.
    #[inline(always)]
    pub fn crupdf(&self) -> CRUPDF_R {
        CRUPDF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - hot-join flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an hot join request has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CHJF bit.
    #[inline(always)]
    pub fn hjf(&self) -> HJF_R {
        HJF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - wakeup/missed start flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a start has been detected (i.e. a SDA falling edge followed by a SCL falling edge) but on the next SCL falling edge, the I3C kernel clock is (still) gated. Thus an I3C bus transaction may have been lost by the target. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CWKPF bit.
    #[inline(always)]
    pub fn wkpf(&self) -> WKPF_R {
        WKPF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - get flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that any direct CCC of get type (GET*** CCC) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGETF bit.
    #[inline(always)]
    pub fn getf(&self) -> GETF_R {
        GETF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - get status flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct GETSTATUS CCC (get status) has been received. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CSTAF bit.
    #[inline(always)]
    pub fn staf(&self) -> STAF_R {
        STAF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - dynamic address update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a dynamic address update has been received via any of the broadcast ENTDAA, RSTDAA and direct SETNEWDA CCC. Then, software should read I3C_DEVR0.DA\[6:0\] to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDAUPDF bit.
    #[inline(always)]
    pub fn daupdf(&self) -> DAUPDF_R {
        DAUPDF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - maximum write length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMWL CCC (set max write length) has been received. Then, software should read I3C_MAXWLR.MWL\[15:0\] to get the maximum write length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMWLUPDF bit.
    #[inline(always)]
    pub fn mwlupdf(&self) -> MWLUPDF_R {
        MWLUPDF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - maximum read length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMRL CCC (set max read length) has been received. Then, software should read I3C_MAXRLR.MRL\[15:0\] to get the maximum read length value. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CMRLUPDF bit.
    #[inline(always)]
    pub fn mrlupdf(&self) -> MRLUPDF_R {
        MRLUPDF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - reset pattern flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a reset pattern has been detected (i.e. 14 SDA transitions while SCL is low, followed by repeated start, then stop). Then, software should read I3C_DEVR0.RSTACT\[1:0\] and I3C_DEVR0.RSTVAL, to know what reset level is required. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\[1:0\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding I3C_CEVR.CRSTF bit.
    #[inline(always)]
    pub fn rstf(&self) -> RSTF_R {
        RSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - activity state update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENTASx CCC (with x=0...3) has been received. Then, software should read I3C_DEVR0.AS\[1:0\]. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CASUPDF bit.
    #[inline(always)]
    pub fn asupdf(&self) -> ASUPDF_R {
        ASUPDF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - interrupt/controller-role/hot-join update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENEC/DISEC CCC (enable/disable target events) has been received, where a target event is either an interrupt/IBI request, a controller-role request, or an hot-join request. Then, software should read respectively I3C_DEVR0.IBIEN, I3C_DEVR0.CREN or I3C_DEVR0.HJEN. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CINTUPDF bit.
    #[inline(always)]
    pub fn intupdf(&self) -> INTUPDF_R {
        INTUPDF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DEFTGTS flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFTGTS CCC (define list of targets) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CDEFF bit.
    #[inline(always)]
    pub fn deff(&self) -> DEFF_R {
        DEFF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - group addressing flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFGRPA CCC (define list of group addresses) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding I3C_CEVR.CGRPF bit.
    #[inline(always)]
    pub fn grpf(&self) -> GRPF_R {
        GRPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVR")
            .field("cfef", &self.cfef())
            .field("txfef", &self.txfef())
            .field("cfnff", &self.cfnff())
            .field("sfnef", &self.sfnef())
            .field("txfnff", &self.txfnff())
            .field("rxfnef", &self.rxfnef())
            .field("txlastf", &self.txlastf())
            .field("rxlastf", &self.rxlastf())
            .field("fcf", &self.fcf())
            .field("rxtgtendf", &self.rxtgtendf())
            .field("errf", &self.errf())
            .field("ibif", &self.ibif())
            .field("ibiendf", &self.ibiendf())
            .field("crf", &self.crf())
            .field("crupdf", &self.crupdf())
            .field("hjf", &self.hjf())
            .field("wkpf", &self.wkpf())
            .field("getf", &self.getf())
            .field("staf", &self.staf())
            .field("daupdf", &self.daupdf())
            .field("mwlupdf", &self.mwlupdf())
            .field("mrlupdf", &self.mrlupdf())
            .field("rstf", &self.rstf())
            .field("asupdf", &self.asupdf())
            .field("intupdf", &self.intupdf())
            .field("deff", &self.deff())
            .field("grpf", &self.grpf())
            .finish()
    }
}
/**I3C event register

You can [`read`](crate::Reg::read) this register and get [`evr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:EVR)*/
pub struct EVRrs;
impl crate::RegisterSpec for EVRrs {
    type Ux = u32;
}
///`read()` method returns [`evr::R`](R) reader structure
impl crate::Readable for EVRrs {}
///`reset()` method sets EVR to value 0x03
impl crate::Resettable for EVRrs {
    const RESET_VALUE: u32 = 0x03;
}
