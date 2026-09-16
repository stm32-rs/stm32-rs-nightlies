///Register `CECCKSELR` reader
pub type R = crate::R<CECCKSELRrs>;
///Register `CECCKSELR` writer
pub type W = crate::W<CECCKSELRrs>;
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
        f.debug_struct("CECCKSELR")
            .field("cecsrc", &self.cecsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CECSRC
    #[inline(always)]
    pub fn cecsrc(&mut self) -> CECSRC_W<'_, CECCKSELRrs> {
        CECSRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the CEC-HDMI.

You can [`read`](crate::Reg::read) this register and get [`cecckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:CECCKSELR)*/
pub struct CECCKSELRrs;
impl crate::RegisterSpec for CECCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`cecckselr::R`](R) reader structure
impl crate::Readable for CECCKSELRrs {}
///`write(|w| ..)` method takes [`cecckselr::W`](W) writer structure
impl crate::Writable for CECCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECCKSELR to value 0
impl crate::Resettable for CECCKSELRrs {}
