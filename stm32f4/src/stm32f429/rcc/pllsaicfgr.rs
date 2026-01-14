///Register `PLLSAICFGR` reader
pub type R = crate::R<PLLSAICFGRrs>;
///Register `PLLSAICFGR` writer
pub type W = crate::W<PLLSAICFGRrs>;
///Field `PLLSAIN` reader - PLLSAI division factor for VCO
pub type PLLSAIN_R = crate::FieldReader<u16>;
///Field `PLLSAIN` writer - PLLSAI division factor for VCO
pub type PLLSAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PLLSAIQ` reader - PLLSAI division factor for SAI1 clock
pub type PLLSAIQ_R = crate::FieldReader;
///Field `PLLSAIQ` writer - PLLSAI division factor for SAI1 clock
pub type PLLSAIQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLLSAIR` reader - PLLSAI division factor for LCD clock
pub type PLLSAIR_R = crate::FieldReader;
///Field `PLLSAIR` writer - PLLSAI division factor for LCD clock
pub type PLLSAIR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 6:14 - PLLSAI division factor for VCO
    #[inline(always)]
    pub fn pllsain(&self) -> PLLSAIN_R {
        PLLSAIN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    ///Bits 24:27 - PLLSAI division factor for SAI1 clock
    #[inline(always)]
    pub fn pllsaiq(&self) -> PLLSAIQ_R {
        PLLSAIQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - PLLSAI division factor for LCD clock
    #[inline(always)]
    pub fn pllsair(&self) -> PLLSAIR_R {
        PLLSAIR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSAICFGR")
            .field("pllsair", &self.pllsair())
            .field("pllsaiq", &self.pllsaiq())
            .field("pllsain", &self.pllsain())
            .finish()
    }
}
impl W {
    ///Bits 6:14 - PLLSAI division factor for VCO
    #[inline(always)]
    pub fn pllsain(&mut self) -> PLLSAIN_W<'_, PLLSAICFGRrs> {
        PLLSAIN_W::new(self, 6)
    }
    ///Bits 24:27 - PLLSAI division factor for SAI1 clock
    #[inline(always)]
    pub fn pllsaiq(&mut self) -> PLLSAIQ_W<'_, PLLSAICFGRrs> {
        PLLSAIQ_W::new(self, 24)
    }
    ///Bits 28:30 - PLLSAI division factor for LCD clock
    #[inline(always)]
    pub fn pllsair(&mut self) -> PLLSAIR_W<'_, PLLSAICFGRrs> {
        PLLSAIR_W::new(self, 28)
    }
}
/**RCC PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsaicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsaicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#RCC:PLLSAICFGR)*/
pub struct PLLSAICFGRrs;
impl crate::RegisterSpec for PLLSAICFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllsaicfgr::R`](R) reader structure
impl crate::Readable for PLLSAICFGRrs {}
///`write(|w| ..)` method takes [`pllsaicfgr::W`](W) writer structure
impl crate::Writable for PLLSAICFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLSAICFGR to value 0x2400_3000
impl crate::Resettable for PLLSAICFGRrs {
    const RESET_VALUE: u32 = 0x2400_3000;
}
