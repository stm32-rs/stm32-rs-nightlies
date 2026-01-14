///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader;
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode
pub type RUN_PD_R = crate::BitReader;
///Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_R = crate::BitReader;
///Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LVEN` reader - LVEN
pub type LVEN_R = crate::BitReader;
///Field `LVEN` writer - LVEN
pub type LVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - LVEN
    #[inline(always)]
    pub fn lven(&self) -> LVEN_R {
        LVEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("run_pd", &self.run_pd())
            .field("sleep_pd", &self.sleep_pd())
            .field("lven", &self.lven())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W<'_, ACRrs> {
        RUN_PD_W::new(self, 13)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<'_, ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
    ///Bit 15 - LVEN
    #[inline(always)]
    pub fn lven(&mut self) -> LVEN_W<'_, ACRrs> {
        LVEN_W::new(self, 15)
    }
}
/**Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACRrs {}
