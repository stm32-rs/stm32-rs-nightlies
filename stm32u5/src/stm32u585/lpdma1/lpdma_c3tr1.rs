///Register `LPDMA_C3TR1` reader
pub type R = crate::R<LPDMA_C3TR1rs>;
///Register `LPDMA_C3TR1` writer
pub type W = crate::W<LPDMA_C3TR1rs>;
/**Field `SDW_LOG2` reader - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.*/
pub type SDW_LOG2_R = crate::FieldReader;
/**Field `SDW_LOG2` writer - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.*/
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SINC` reader - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type SINC_R = crate::BitReader;
///Field `SINC` writer - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAM` reader - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
pub type PAM_R = crate::BitReader;
///Field `PAM` writer - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
pub type PAM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSEC` reader - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
pub type SSEC_R = crate::BitReader;
///Field `SSEC` writer - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
pub type SSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `DDW_LOG2` reader - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.*/
pub type DDW_LOG2_R = crate::FieldReader;
/**Field `DDW_LOG2` writer - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.*/
pub type DDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINC` reader - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type DINC_R = crate::BitReader;
///Field `DINC` writer - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
pub type DINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSEC` reader - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
pub type DSEC_R = crate::BitReader;
///Field `DSEC` writer - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
pub type DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bits 0:1 - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
    versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
    versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.*/
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    /**Bits 16:17 - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
    versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.*/
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 31 - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPDMA_C3TR1")
            .field("sdw_log2", &self.sdw_log2())
            .field("sinc", &self.sinc())
            .field("pam", &self.pam())
            .field("ssec", &self.ssec())
            .field("ddw_log2", &self.ddw_log2())
            .field("dinc", &self.dinc())
            .field("dsec", &self.dsec())
            .finish()
    }
}
impl W {
    /**Bits 0:1 - binary logarithm of the source data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. a source block size must be a multiple of the source data width (LPDMA_CxBR1.BNDT\[2:0\]
    versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address LPDMA_CxSAR\[2:0\]
    versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.*/
    #[inline(always)]
    #[must_use]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<LPDMA_C3TR1rs> {
        SDW_LOG2_W::new(self, 0)
    }
    ///Bit 3 - source incrementing single The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<LPDMA_C3TR1rs> {
        SINC_W::new(self, 3)
    }
    ///Bit 11 - padding/alignment mode If DDW_LOG2\[1:0\]=SDW_LOG2\[1:0\]: if the data width of a single destination transfer is equal to the data width of a single source transfer, this bit is ignored. Else: Case 1: If destination data width > source data width Case 2: If destination data width &lt; source data width
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<LPDMA_C3TR1rs> {
        PAM_W::new(self, 11)
    }
    ///Bit 15 - security attribute of the LPDMA transfer from the source If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer from the source is non-secure.
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<LPDMA_C3TR1rs> {
        SSEC_W::new(self, 15)
    }
    /**Bits 16:17 - binary logarithm of the destination data width of a single in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination single transfer must have an aligned address with its data width (start address LPDMA_CxDAR\[2:0\]
    versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and none transfer is issued.*/
    #[inline(always)]
    #[must_use]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<LPDMA_C3TR1rs> {
        DDW_LOG2_W::new(self, 16)
    }
    ///Bit 19 - destination incrementing single The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer, or is incremented by the offset value corresponding to a contiguous data after a single transfer.
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<LPDMA_C3TR1rs> {
        DINC_W::new(self, 19)
    }
    ///Bit 31 - security attribute of the LPDMA transfer to the destination If LPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored when LPDMA_SECCFGR.SECx = 0. When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the LPDMA transfer to the destination is non-secure.
    #[inline(always)]
    #[must_use]
    pub fn dsec(&mut self) -> DSEC_W<LPDMA_C3TR1rs> {
        DSEC_W::new(self, 31)
    }
}
/**LPDMA channel 3 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPDMA1:LPDMA_C3TR1)*/
pub struct LPDMA_C3TR1rs;
impl crate::RegisterSpec for LPDMA_C3TR1rs {
    type Ux = u32;
}
///`read()` method returns [`lpdma_c3tr1::R`](R) reader structure
impl crate::Readable for LPDMA_C3TR1rs {}
///`write(|w| ..)` method takes [`lpdma_c3tr1::W`](W) writer structure
impl crate::Writable for LPDMA_C3TR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPDMA_C3TR1 to value 0
impl crate::Resettable for LPDMA_C3TR1rs {
    const RESET_VALUE: u32 = 0;
}
