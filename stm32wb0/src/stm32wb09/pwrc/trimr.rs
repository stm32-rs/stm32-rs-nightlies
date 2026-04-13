///Register `TRIMR` reader
pub type R = crate::R<TRIMRrs>;
///Register `TRIMR` writer
pub type W = crate::W<TRIMRrs>;
///Field `RFD_REG_TRIM` reader - RF LDO trimming.
pub type RFD_REG_TRIM_R = crate::FieldReader;
///Field `RFD_REG_TRIM` writer - RF LDO trimming.
pub type RFD_REG_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIM_MR` reader - Main regulator voltage trimming.
pub type TRIM_MR_R = crate::FieldReader;
///Field `TRIM_MR` writer - Main regulator voltage trimming.
pub type TRIM_MR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SMPS_TRIM` reader - SMPS output voltage trimming.
pub type SMPS_TRIM_R = crate::FieldReader;
///Field `SMPS_TRIM` writer - SMPS output voltage trimming.
pub type SMPS_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - RF LDO trimming.
    #[inline(always)]
    pub fn rfd_reg_trim(&self) -> RFD_REG_TRIM_R {
        RFD_REG_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Main regulator voltage trimming.
    #[inline(always)]
    pub fn trim_mr(&self) -> TRIM_MR_R {
        TRIM_MR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - SMPS output voltage trimming.
    #[inline(always)]
    pub fn smps_trim(&self) -> SMPS_TRIM_R {
        SMPS_TRIM_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIMR")
            .field("rfd_reg_trim", &self.rfd_reg_trim())
            .field("trim_mr", &self.trim_mr())
            .field("smps_trim", &self.smps_trim())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - RF LDO trimming.
    #[inline(always)]
    pub fn rfd_reg_trim(&mut self) -> RFD_REG_TRIM_W<'_, TRIMRrs> {
        RFD_REG_TRIM_W::new(self, 0)
    }
    ///Bits 4:7 - Main regulator voltage trimming.
    #[inline(always)]
    pub fn trim_mr(&mut self) -> TRIM_MR_W<'_, TRIMRrs> {
        TRIM_MR_W::new(self, 4)
    }
    ///Bits 8:10 - SMPS output voltage trimming.
    #[inline(always)]
    pub fn smps_trim(&mut self) -> SMPS_TRIM_W<'_, TRIMRrs> {
        SMPS_TRIM_W::new(self, 8)
    }
}
/**This register provides the trimming values applied by hardware.

You can [`read`](crate::Reg::read) this register and get [`trimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:TRIMR)*/
pub struct TRIMRrs;
impl crate::RegisterSpec for TRIMRrs {
    type Ux = u32;
}
///`read()` method returns [`trimr::R`](R) reader structure
impl crate::Readable for TRIMRrs {}
///`write(|w| ..)` method takes [`trimr::W`](W) writer structure
impl crate::Writable for TRIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRIMR to value 0
impl crate::Resettable for TRIMRrs {}
