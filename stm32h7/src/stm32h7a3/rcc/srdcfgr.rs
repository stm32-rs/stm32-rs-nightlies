///Register `SRDCFGR` reader
pub type R = crate::R<SRDCFGRrs>;
///Register `SRDCFGR` writer
pub type W = crate::W<SRDCFGRrs>;
///Field `SRDPPRE` reader - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)
pub type SRDPPRE_R = crate::FieldReader;
///Field `SRDPPRE` writer - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)
pub type SRDPPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)
    #[inline(always)]
    pub fn srdppre(&self) -> SRDPPRE_R {
        SRDPPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRDCFGR")
            .field("srdppre", &self.srdppre())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)
    #[inline(always)]
    pub fn srdppre(&mut self) -> SRDPPRE_W<'_, SRDCFGRrs> {
        SRDPPRE_W::new(self, 4)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`srdcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srdcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#RCC:SRDCFGR)*/
pub struct SRDCFGRrs;
impl crate::RegisterSpec for SRDCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`srdcfgr::R`](R) reader structure
impl crate::Readable for SRDCFGRrs {}
///`write(|w| ..)` method takes [`srdcfgr::W`](W) writer structure
impl crate::Writable for SRDCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRDCFGR to value 0
impl crate::Resettable for SRDCFGRrs {}
