///Register `RCC_CECCKSELR` reader
pub type R = crate::R<RCC_CECCKSELRrs>;
///Register `RCC_CECCKSELR` writer
pub type W = crate::W<RCC_CECCKSELRrs>;
///Field `CECSRC` reader - CECSRC
pub type CECSRC_R = crate::FieldReader;
///Field `CECSRC` writer - CECSRC
pub type CECSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - CECSRC
    #[inline(always)]
    pub fn cecsrc(&self) -> CECSRC_R {
        CECSRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CECCKSELR")
            .field("cecsrc", &self.cecsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CECSRC
    #[inline(always)]
    #[must_use]
    pub fn cecsrc(&mut self) -> CECSRC_W<RCC_CECCKSELRrs> {
        CECSRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the CEC-HDMI.

You can [`read`](crate::Reg::read) this register and get [`rcc_cecckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cecckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_CECCKSELR)*/
pub struct RCC_CECCKSELRrs;
impl crate::RegisterSpec for RCC_CECCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cecckselr::R`](R) reader structure
impl crate::Readable for RCC_CECCKSELRrs {}
///`write(|w| ..)` method takes [`rcc_cecckselr::W`](W) writer structure
impl crate::Writable for RCC_CECCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CECCKSELR to value 0
impl crate::Resettable for RCC_CECCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
