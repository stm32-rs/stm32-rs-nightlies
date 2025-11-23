///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `EOAF` reader - End of acquisition flag
pub type EOAF_R = crate::BitReader;
///Field `EOAF` writer - End of acquisition flag
pub type EOAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCEF` reader - Max count error flag
pub type MCEF_R = crate::BitReader;
///Field `MCEF` writer - Max count error flag
pub type MCEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - End of acquisition flag
    #[inline(always)]
    pub fn eoaf(&self) -> EOAF_R {
        EOAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error flag
    #[inline(always)]
    pub fn mcef(&self) -> MCEF_R {
        MCEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("mcef", &self.mcef())
            .field("eoaf", &self.eoaf())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of acquisition flag
    #[inline(always)]
    pub fn eoaf(&mut self) -> EOAF_W<'_, ISRrs> {
        EOAF_W::new(self, 0)
    }
    ///Bit 1 - Max count error flag
    #[inline(always)]
    pub fn mcef(&mut self) -> MCEF_W<'_, ISRrs> {
        MCEF_W::new(self, 1)
    }
}
/**interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#TSC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
