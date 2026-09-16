///Register `VENCRAMCR` reader
pub type R = crate::R<VENCRAMCRrs>;
///Register `VENCRAMCR` writer
pub type W = crate::W<VENCRAMCRrs>;
///Field `SRAMER` reader - SRAM erase
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VENCRAMCR")
            .field("sramer", &self.sramer())
            .finish()
    }
}
impl W {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, VENCRAMCRrs> {
        SRAMER_W::new(self, 8)
    }
}
/**RAMCFG VENCRAM control register

You can [`read`](crate::Reg::read) this register and get [`vencramcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vencramcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RAMCFG:VENCRAMCR)*/
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
