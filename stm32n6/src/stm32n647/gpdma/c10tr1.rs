///Register `C10TR1` reader
pub type R = crate::R<C10TR1rs>;
///Register `C10TR1` writer
pub type W = crate::W<C10TR1rs>;
///Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst in bytes
pub type SDW_LOG2_R = crate::FieldReader;
///Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst in bytes
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SINC` reader - source incrementing burst
pub type SINC_R = crate::BitReader;
///Field `SINC` writer - source incrementing burst
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBL_1` reader - source burst length minus 1, between 0 and 63
pub type SBL_1_R = crate::FieldReader;
///Field `SBL_1` writer - source burst length minus 1, between 0 and 63
pub type SBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PAM` reader - padding/alignment mode
pub type PAM_R = crate::FieldReader;
///Field `PAM` writer - padding/alignment mode
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SBX` reader - source byte exchange within the unaligned half-word of each source word
pub type SBX_R = crate::BitReader;
///Field `SBX` writer - source byte exchange within the unaligned half-word of each source word
pub type SBX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAP` reader - source allocated port
pub type SAP_R = crate::BitReader;
///Field `SAP` writer - source allocated port
pub type SAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSEC` reader - security attribute of the GPDMA transfer from the source
pub type SSEC_R = crate::BitReader;
///Field `SSEC` writer - security attribute of the GPDMA transfer from the source
pub type SSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes
pub type DDW_LOG2_R = crate::FieldReader;
///Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes
pub type DDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINC` reader - destination incrementing burst
pub type DINC_R = crate::BitReader;
///Field `DINC` writer - destination incrementing burst
pub type DINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBL_1` reader - destination burst length minus 1, between 0 and 63
pub type DBL_1_R = crate::FieldReader;
///Field `DBL_1` writer - destination burst length minus 1, between 0 and 63
pub type DBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DBX` reader - destination byte exchange
pub type DBX_R = crate::BitReader;
///Field `DBX` writer - destination byte exchange
pub type DBX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHX` reader - destination half-word exchange
pub type DHX_R = crate::BitReader;
///Field `DHX` writer - destination half-word exchange
pub type DHX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAP` reader - destination allocated port
pub type DAP_R = crate::BitReader;
///Field `DAP` writer - destination allocated port
pub type DAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSEC` reader - security attribute of the GPDMA transfer to the destination
pub type DSEC_R = crate::BitReader;
///Field `DSEC` writer - security attribute of the GPDMA transfer to the destination
pub type DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - source incrementing burst
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63
    #[inline(always)]
    pub fn sbl_1(&self) -> SBL_1_R {
        SBL_1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 11:12 - padding/alignment mode
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word
    #[inline(always)]
    pub fn sbx(&self) -> SBX_R {
        SBX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - source allocated port
    #[inline(always)]
    pub fn sap(&self) -> SAP_R {
        SAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - security attribute of the GPDMA transfer from the source
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - destination incrementing burst
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63
    #[inline(always)]
    pub fn dbl_1(&self) -> DBL_1_R {
        DBL_1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - destination byte exchange
    #[inline(always)]
    pub fn dbx(&self) -> DBX_R {
        DBX_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - destination half-word exchange
    #[inline(always)]
    pub fn dhx(&self) -> DHX_R {
        DHX_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - destination allocated port
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - security attribute of the GPDMA transfer to the destination
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C10TR1")
            .field("sdw_log2", &self.sdw_log2())
            .field("sinc", &self.sinc())
            .field("sbl_1", &self.sbl_1())
            .field("pam", &self.pam())
            .field("sbx", &self.sbx())
            .field("sap", &self.sap())
            .field("ssec", &self.ssec())
            .field("ddw_log2", &self.ddw_log2())
            .field("dinc", &self.dinc())
            .field("dbl_1", &self.dbl_1())
            .field("dbx", &self.dbx())
            .field("dhx", &self.dhx())
            .field("dap", &self.dap())
            .field("dsec", &self.dsec())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes
    #[inline(always)]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<'_, C10TR1rs> {
        SDW_LOG2_W::new(self, 0)
    }
    ///Bit 3 - source incrementing burst
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<'_, C10TR1rs> {
        SINC_W::new(self, 3)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63
    #[inline(always)]
    pub fn sbl_1(&mut self) -> SBL_1_W<'_, C10TR1rs> {
        SBL_1_W::new(self, 4)
    }
    ///Bits 11:12 - padding/alignment mode
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<'_, C10TR1rs> {
        PAM_W::new(self, 11)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word
    #[inline(always)]
    pub fn sbx(&mut self) -> SBX_W<'_, C10TR1rs> {
        SBX_W::new(self, 13)
    }
    ///Bit 14 - source allocated port
    #[inline(always)]
    pub fn sap(&mut self) -> SAP_W<'_, C10TR1rs> {
        SAP_W::new(self, 14)
    }
    ///Bit 15 - security attribute of the GPDMA transfer from the source
    #[inline(always)]
    pub fn ssec(&mut self) -> SSEC_W<'_, C10TR1rs> {
        SSEC_W::new(self, 15)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes
    #[inline(always)]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<'_, C10TR1rs> {
        DDW_LOG2_W::new(self, 16)
    }
    ///Bit 19 - destination incrementing burst
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<'_, C10TR1rs> {
        DINC_W::new(self, 19)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63
    #[inline(always)]
    pub fn dbl_1(&mut self) -> DBL_1_W<'_, C10TR1rs> {
        DBL_1_W::new(self, 20)
    }
    ///Bit 26 - destination byte exchange
    #[inline(always)]
    pub fn dbx(&mut self) -> DBX_W<'_, C10TR1rs> {
        DBX_W::new(self, 26)
    }
    ///Bit 27 - destination half-word exchange
    #[inline(always)]
    pub fn dhx(&mut self) -> DHX_W<'_, C10TR1rs> {
        DHX_W::new(self, 27)
    }
    ///Bit 30 - destination allocated port
    #[inline(always)]
    pub fn dap(&mut self) -> DAP_W<'_, C10TR1rs> {
        DAP_W::new(self, 30)
    }
    ///Bit 31 - security attribute of the GPDMA transfer to the destination
    #[inline(always)]
    pub fn dsec(&mut self) -> DSEC_W<'_, C10TR1rs> {
        DSEC_W::new(self, 31)
    }
}
/**GPDMA channel 10 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c10tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPDMA:C10TR1)*/
pub struct C10TR1rs;
impl crate::RegisterSpec for C10TR1rs {
    type Ux = u32;
}
///`read()` method returns [`c10tr1::R`](R) reader structure
impl crate::Readable for C10TR1rs {}
///`write(|w| ..)` method takes [`c10tr1::W`](W) writer structure
impl crate::Writable for C10TR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C10TR1 to value 0
impl crate::Resettable for C10TR1rs {}
