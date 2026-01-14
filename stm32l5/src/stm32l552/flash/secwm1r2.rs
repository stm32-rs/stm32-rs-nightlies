///Register `SECWM1R2` reader
pub type R = crate::R<SECWM1R2rs>;
///Register `SECWM1R2` writer
pub type W = crate::W<SECWM1R2rs>;
///Field `PCROP1_PSTRT` reader - PCROP1_PSTRT
pub type PCROP1_PSTRT_R = crate::FieldReader;
///Field `PCROP1_PSTRT` writer - PCROP1_PSTRT
pub type PCROP1_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PCROP1EN` reader - PCROP1EN
pub type PCROP1EN_R = crate::BitReader;
///Field `PCROP1EN` writer - PCROP1EN
pub type PCROP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDP1_PEND` reader - HDP1_PEND
pub type HDP1_PEND_R = crate::FieldReader;
///Field `HDP1_PEND` writer - HDP1_PEND
pub type HDP1_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HDP1EN` reader - HDP1EN
pub type HDP1EN_R = crate::BitReader;
///Field `HDP1EN` writer - HDP1EN
pub type HDP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - PCROP1_PSTRT
    #[inline(always)]
    pub fn pcrop1_pstrt(&self) -> PCROP1_PSTRT_R {
        PCROP1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 15 - PCROP1EN
    #[inline(always)]
    pub fn pcrop1en(&self) -> PCROP1EN_R {
        PCROP1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:22 - HDP1_PEND
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - HDP1EN
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM1R2")
            .field("pcrop1_pstrt", &self.pcrop1_pstrt())
            .field("pcrop1en", &self.pcrop1en())
            .field("hdp1_pend", &self.hdp1_pend())
            .field("hdp1en", &self.hdp1en())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - PCROP1_PSTRT
    #[inline(always)]
    pub fn pcrop1_pstrt(&mut self) -> PCROP1_PSTRT_W<'_, SECWM1R2rs> {
        PCROP1_PSTRT_W::new(self, 0)
    }
    ///Bit 15 - PCROP1EN
    #[inline(always)]
    pub fn pcrop1en(&mut self) -> PCROP1EN_W<'_, SECWM1R2rs> {
        PCROP1EN_W::new(self, 15)
    }
    ///Bits 16:22 - HDP1_PEND
    #[inline(always)]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W<'_, SECWM1R2rs> {
        HDP1_PEND_W::new(self, 16)
    }
    ///Bit 31 - HDP1EN
    #[inline(always)]
    pub fn hdp1en(&mut self) -> HDP1EN_W<'_, SECWM1R2rs> {
        HDP1EN_W::new(self, 31)
    }
}
/**Flash secure watermak1 register 2

You can [`read`](crate::Reg::read) this register and get [`secwm1r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECWM1R2)*/
pub struct SECWM1R2rs;
impl crate::RegisterSpec for SECWM1R2rs {
    type Ux = u32;
}
///`read()` method returns [`secwm1r2::R`](R) reader structure
impl crate::Readable for SECWM1R2rs {}
///`write(|w| ..)` method takes [`secwm1r2::W`](W) writer structure
impl crate::Writable for SECWM1R2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWM1R2 to value 0x0f00_0f00
impl crate::Resettable for SECWM1R2rs {
    const RESET_VALUE: u32 = 0x0f00_0f00;
}
