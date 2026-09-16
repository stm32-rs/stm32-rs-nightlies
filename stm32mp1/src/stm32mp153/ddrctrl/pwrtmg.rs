///Register `PWRTMG` reader
pub type R = crate::R<PWRTMGrs>;
///Register `PWRTMG` writer
pub type W = crate::W<PWRTMGrs>;
///Field `POWERDOWN_TO_X32` reader - POWERDOWN_TO_X32
pub type POWERDOWN_TO_X32_R = crate::FieldReader;
///Field `POWERDOWN_TO_X32` writer - POWERDOWN_TO_X32
pub type POWERDOWN_TO_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `T_DPD_X4096` reader - T_DPD_X4096
pub type T_DPD_X4096_R = crate::FieldReader;
///Field `T_DPD_X4096` writer - T_DPD_X4096
pub type T_DPD_X4096_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SELFREF_TO_X32` reader - SELFREF_TO_X32
pub type SELFREF_TO_X32_R = crate::FieldReader;
///Field `SELFREF_TO_X32` writer - SELFREF_TO_X32
pub type SELFREF_TO_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:4 - POWERDOWN_TO_X32
    #[inline(always)]
    pub fn powerdown_to_x32(&self) -> POWERDOWN_TO_X32_R {
        POWERDOWN_TO_X32_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:15 - T_DPD_X4096
    #[inline(always)]
    pub fn t_dpd_x4096(&self) -> T_DPD_X4096_R {
        T_DPD_X4096_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - SELFREF_TO_X32
    #[inline(always)]
    pub fn selfref_to_x32(&self) -> SELFREF_TO_X32_R {
        SELFREF_TO_X32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRTMG")
            .field("powerdown_to_x32", &self.powerdown_to_x32())
            .field("t_dpd_x4096", &self.t_dpd_x4096())
            .field("selfref_to_x32", &self.selfref_to_x32())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - POWERDOWN_TO_X32
    #[inline(always)]
    pub fn powerdown_to_x32(&mut self) -> POWERDOWN_TO_X32_W<'_, PWRTMGrs> {
        POWERDOWN_TO_X32_W::new(self, 0)
    }
    ///Bits 8:15 - T_DPD_X4096
    #[inline(always)]
    pub fn t_dpd_x4096(&mut self) -> T_DPD_X4096_W<'_, PWRTMGrs> {
        T_DPD_X4096_W::new(self, 8)
    }
    ///Bits 16:23 - SELFREF_TO_X32
    #[inline(always)]
    pub fn selfref_to_x32(&mut self) -> SELFREF_TO_X32_W<'_, PWRTMGrs> {
        SELFREF_TO_X32_W::new(self, 16)
    }
}
/**DDRCTRL low power timing register

You can [`read`](crate::Reg::read) this register and get [`pwrtmg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrtmg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:PWRTMG)*/
pub struct PWRTMGrs;
impl crate::RegisterSpec for PWRTMGrs {
    type Ux = u32;
}
///`read()` method returns [`pwrtmg::R`](R) reader structure
impl crate::Readable for PWRTMGrs {}
///`write(|w| ..)` method takes [`pwrtmg::W`](W) writer structure
impl crate::Writable for PWRTMGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWRTMG to value 0x0040_2010
impl crate::Resettable for PWRTMGrs {
    const RESET_VALUE: u32 = 0x0040_2010;
}
