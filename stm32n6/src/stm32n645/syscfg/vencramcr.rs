///Register `VENCRAMCR` reader
pub type R = crate::R<VENCRAMCRrs>;
///Register `VENCRAMCR` writer
pub type W = crate::W<VENCRAMCRrs>;
///Field `VENCRAM_EN` reader - VENCRAM allocation VENC if active, or to system (if VENC inactive)
pub type VENCRAM_EN_R = crate::BitReader;
///Field `VENCRAM_EN` writer - VENCRAM allocation VENC if active, or to system (if VENC inactive)
pub type VENCRAM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - VENCRAM allocation VENC if active, or to system (if VENC inactive)
    #[inline(always)]
    pub fn vencram_en(&self) -> VENCRAM_EN_R {
        VENCRAM_EN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VENCRAMCR")
            .field("vencram_en", &self.vencram_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - VENCRAM allocation VENC if active, or to system (if VENC inactive)
    #[inline(always)]
    pub fn vencram_en(&mut self) -> VENCRAM_EN_W<'_, VENCRAMCRrs> {
        VENCRAM_EN_W::new(self, 0)
    }
}
/**SYSCFG VENCRAM control register

You can [`read`](crate::Reg::read) this register and get [`vencramcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vencramcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SYSCFG:VENCRAMCR)*/
pub struct VENCRAMCRrs;
impl crate::RegisterSpec for VENCRAMCRrs {
    type Ux = u32;
}
///`read()` method returns [`vencramcr::R`](R) reader structure
impl crate::Readable for VENCRAMCRrs {}
///`write(|w| ..)` method takes [`vencramcr::W`](W) writer structure
impl crate::Writable for VENCRAMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VENCRAMCR to value 0
impl crate::Resettable for VENCRAMCRrs {}
