///Register `LPOTR` reader
pub type R = crate::R<LPOTRrs>;
///Register `LPOTR` writer
pub type W = crate::W<LPOTRrs>;
///Field `AO1_OPT_OFFSET_TRIM_LP` reader - OPAMP1, 10-bit offset trim value for low power mode
pub type AO1_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16>;
///Field `AO1_OPT_OFFSET_TRIM_LP` writer - OPAMP1, 10-bit offset trim value for low power mode
pub type AO1_OPT_OFFSET_TRIM_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `AO2_OPT_OFFSET_TRIM_LP` reader - OPAMP2, 10-bit offset trim value for low power mode
pub type AO2_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16>;
///Field `AO2_OPT_OFFSET_TRIM_LP` writer - OPAMP2, 10-bit offset trim value for low power mode
pub type AO2_OPT_OFFSET_TRIM_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `AO3_OPT_OFFSET_TRIM_LP` reader - OPAMP3, 10-bit offset trim value for low power mode
pub type AO3_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16>;
///Field `AO3_OPT_OFFSET_TRIM_LP` writer - OPAMP3, 10-bit offset trim value for low power mode
pub type AO3_OPT_OFFSET_TRIM_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao1_opt_offset_trim_lp(&self) -> AO1_OPT_OFFSET_TRIM_LP_R {
        AO1_OPT_OFFSET_TRIM_LP_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao2_opt_offset_trim_lp(&self) -> AO2_OPT_OFFSET_TRIM_LP_R {
        AO2_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao3_opt_offset_trim_lp(&self) -> AO3_OPT_OFFSET_TRIM_LP_R {
        AO3_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPOTR")
            .field("ao3_opt_offset_trim_lp", &self.ao3_opt_offset_trim_lp())
            .field("ao2_opt_offset_trim_lp", &self.ao2_opt_offset_trim_lp())
            .field("ao1_opt_offset_trim_lp", &self.ao1_opt_offset_trim_lp())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao1_opt_offset_trim_lp(&mut self) -> AO1_OPT_OFFSET_TRIM_LP_W<'_, LPOTRrs> {
        AO1_OPT_OFFSET_TRIM_LP_W::new(self, 0)
    }
    ///Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao2_opt_offset_trim_lp(&mut self) -> AO2_OPT_OFFSET_TRIM_LP_W<'_, LPOTRrs> {
        AO2_OPT_OFFSET_TRIM_LP_W::new(self, 10)
    }
    ///Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao3_opt_offset_trim_lp(&mut self) -> AO3_OPT_OFFSET_TRIM_LP_W<'_, LPOTRrs> {
        AO3_OPT_OFFSET_TRIM_LP_W::new(self, 20)
    }
}
/**OPAMP offset trimming register for low power mode

You can [`read`](crate::Reg::read) this register and get [`lpotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#OPAMP:LPOTR)*/
pub struct LPOTRrs;
impl crate::RegisterSpec for LPOTRrs {
    type Ux = u32;
}
///`read()` method returns [`lpotr::R`](R) reader structure
impl crate::Readable for LPOTRrs {}
///`write(|w| ..)` method takes [`lpotr::W`](W) writer structure
impl crate::Writable for LPOTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPOTR to value 0
impl crate::Resettable for LPOTRrs {}
