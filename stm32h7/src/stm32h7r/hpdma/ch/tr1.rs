///Register `TR1` reader
pub type R = crate::R<TR1rs>;
///Register `TR1` writer
pub type W = crate::W<TR1rs>;
///Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst in bytes if SAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. A source block size must be a multiple of the source data width (HPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address HPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_R = crate::FieldReader;
///Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst in bytes if SAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. A source block size must be a multiple of the source data width (HPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address HPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SINC` reader - source incrementing burst The source address, pointed by HPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_R = crate::BitReader;
///Field `SINC` writer - source incrementing burst The source address, pointed by HPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBL_1` reader - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or an AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If the burst length exceeds 16 on a AHB transfer, or if the burst on a AXI transfer is both with fixed addressing (SINC=0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type SBL_1_R = crate::FieldReader;
///Field `SBL_1` writer - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or an AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If the burst length exceeds 16 on a AHB transfer, or if the burst on a AXI transfer is both with fixed addressing (SINC=0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type SBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PAM` reader - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported.
pub type PAM_R = crate::FieldReader;
///Field `PAM` writer - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported.
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SBX` reader - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word or a double-word, and if source bus is AXI (SAP = 0):
pub type SBX_R = crate::BitReader;
///Field `SBX` writer - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word or a double-word, and if source bus is AXI (SAP = 0):
pub type SBX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAP` reader - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type SAP_R = crate::BitReader;
///Field `SAP` writer - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type SAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes if DAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address HPDMA_CxDAR\[2:0\] and address offset HPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued. Note: When configured in packing mode (PAM\[1\] = 1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (see HPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type DDW_LOG2_R = crate::FieldReader;
///Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes if DAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address HPDMA_CxDAR\[2:0\] and address offset HPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued. Note: When configured in packing mode (PAM\[1\] = 1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (see HPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type DDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINC` reader - destination incrementing burst The destination address, pointed by HPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type DINC_R = crate::BitReader;
///Field `DINC` writer - destination incrementing burst The destination address, pointed by HPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type DINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBL_1` reader - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB/AXI protocol. Note: If the burst length exceeds 16 on an AHB transfer, or if the burst on an AXI transfer is both with fixed addressing (DINC = 0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into bursts of lower length, to be compliant with the AHB or AXI protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type DBL_1_R = crate::FieldReader;
///Field `DBL_1` writer - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB/AXI protocol. Note: If the burst length exceeds 16 on an AHB transfer, or if the burst on an AXI transfer is both with fixed addressing (DINC = 0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into bursts of lower length, to be compliant with the AHB or AXI protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type DBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DBX` reader - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
pub type DBX_R = crate::BitReader;
///Field `DBX` writer - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
pub type DBX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHX` reader - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word or double-word and if destination bus is AXI (DAP = 0):
pub type DHX_R = crate::BitReader;
///Field `DHX` writer - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word or double-word and if destination bus is AXI (DAP = 0):
pub type DHX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DWX` reader - destination word exchange If the destination data size is not a double-word, this bit is ignored. If the destination data size is a double-word and if destination bus is AXI (DAP = 0):
pub type DWX_R = crate::BitReader;
///Field `DWX` writer - destination word exchange If the destination data size is not a double-word, this bit is ignored. If the destination data size is a double-word and if destination bus is AXI (DAP = 0):
pub type DWX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAP` reader - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type DAP_R = crate::BitReader;
///Field `DAP` writer - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type DAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes if SAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. A source block size must be a multiple of the source data width (HPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address HPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - source incrementing burst The source address, pointed by HPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or an AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If the burst length exceeds 16 on a AHB transfer, or if the burst on a AXI transfer is both with fixed addressing (SINC=0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn sbl_1(&self) -> SBL_1_R {
        SBL_1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 11:12 - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported.
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word or a double-word, and if source bus is AXI (SAP = 0):
    #[inline(always)]
    pub fn sbx(&self) -> SBX_R {
        SBX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn sap(&self) -> SAP_R {
        SAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes if DAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address HPDMA_CxDAR\[2:0\] and address offset HPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued. Note: When configured in packing mode (PAM\[1\] = 1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (see HPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - destination incrementing burst The destination address, pointed by HPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB/AXI protocol. Note: If the burst length exceeds 16 on an AHB transfer, or if the burst on an AXI transfer is both with fixed addressing (DINC = 0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into bursts of lower length, to be compliant with the AHB or AXI protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn dbl_1(&self) -> DBL_1_R {
        DBL_1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
    #[inline(always)]
    pub fn dbx(&self) -> DBX_R {
        DBX_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word or double-word and if destination bus is AXI (DAP = 0):
    #[inline(always)]
    pub fn dhx(&self) -> DHX_R {
        DHX_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - destination word exchange If the destination data size is not a double-word, this bit is ignored. If the destination data size is a double-word and if destination bus is AXI (DAP = 0):
    #[inline(always)]
    pub fn dwx(&self) -> DWX_R {
        DWX_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR1")
            .field("sdw_log2", &self.sdw_log2())
            .field("sinc", &self.sinc())
            .field("sbl_1", &self.sbl_1())
            .field("pam", &self.pam())
            .field("sbx", &self.sbx())
            .field("sap", &self.sap())
            .field("ddw_log2", &self.ddw_log2())
            .field("dinc", &self.dinc())
            .field("dbl_1", &self.dbl_1())
            .field("dbx", &self.dbx())
            .field("dhx", &self.dhx())
            .field("dwx", &self.dwx())
            .field("dap", &self.dap())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes if SAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. A source block size must be a multiple of the source data width (HPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address HPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<'_, TR1rs> {
        SDW_LOG2_W::new(self, 0)
    }
    ///Bit 3 - source incrementing burst The source address, pointed by HPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<'_, TR1rs> {
        SINC_W::new(self, 3)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or an AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If the burst length exceeds 16 on a AHB transfer, or if the burst on a AXI transfer is both with fixed addressing (SINC=0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn sbl_1(&mut self) -> SBL_1_W<'_, TR1rs> {
        SBL_1_W::new(self, 4)
    }
    ///Bits 11:12 - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported.
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<'_, TR1rs> {
        PAM_W::new(self, 11)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word or a double-word, and if source bus is AXI (SAP = 0):
    #[inline(always)]
    pub fn sbx(&mut self) -> SBX_W<'_, TR1rs> {
        SBX_W::new(self, 13)
    }
    ///Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn sap(&mut self) -> SAP_W<'_, TR1rs> {
        SAP_W::new(self, 14)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes if DAP = 1, user setting error reported and no transfer issued Note: A burst with a double-word data width must be allocated to the AXI master port, else a user setting error is reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address HPDMA_CxDAR\[2:0\] and address offset HPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued. Note: When configured in packing mode (PAM\[1\] = 1 and destination data width different from source data width), a source block size must be a multiple of the destination data width (see HPDMA_CxBR1.BNDT\[2:0\] vs DDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<'_, TR1rs> {
        DDW_LOG2_W::new(self, 16)
    }
    ///Bit 19 - destination incrementing burst The destination address, pointed by HPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<'_, TR1rs> {
        DINC_W::new(self, 19)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1- or 4-Kbyte address boundary on respectively an AHB or AXI transfer, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB/AXI protocol. Note: If the burst length exceeds 16 on an AHB transfer, or if the burst on an AXI transfer is both with fixed addressing (DINC = 0) and with a burst length which exceeds 16, the HPDMA modifies and shortens the programmed burst into bursts of lower length, to be compliant with the AHB or AXI protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the HPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with HPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn dbl_1(&mut self) -> DBL_1_W<'_, TR1rs> {
        DBL_1_W::new(self, 20)
    }
    ///Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
    #[inline(always)]
    pub fn dbx(&mut self) -> DBX_W<'_, TR1rs> {
        DBX_W::new(self, 26)
    }
    ///Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word or double-word and if destination bus is AXI (DAP = 0):
    #[inline(always)]
    pub fn dhx(&mut self) -> DHX_W<'_, TR1rs> {
        DHX_W::new(self, 27)
    }
    ///Bit 28 - destination word exchange If the destination data size is not a double-word, this bit is ignored. If the destination data size is a double-word and if destination bus is AXI (DAP = 0):
    #[inline(always)]
    pub fn dwx(&mut self) -> DWX_W<'_, TR1rs> {
        DWX_W::new(self, 28)
    }
    ///Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn dap(&mut self) -> DAP_W<'_, TR1rs> {
        DAP_W::new(self, 30)
    }
}
/**HPDMA channel 0 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TR1rs;
impl crate::RegisterSpec for TR1rs {
    type Ux = u32;
}
///`read()` method returns [`tr1::R`](R) reader structure
impl crate::Readable for TR1rs {}
///`write(|w| ..)` method takes [`tr1::W`](W) writer structure
impl crate::Writable for TR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR1 to value 0
impl crate::Resettable for TR1rs {}
