///Register `DCR3` reader
pub type R = crate::R<DCR3rs>;
///Register `DCR3` writer
pub type W = crate::W<DCR3rs>;
///Field `CSBOUND` reader - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes
pub type CSBOUND_R = crate::FieldReader;
///Field `CSBOUND` writer - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes
pub type CSBOUND_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 16:20 - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes
    #[inline(always)]
    pub fn csbound(&self) -> CSBOUND_R {
        CSBOUND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR3")
            .field("csbound", &self.csbound())
            .finish()
    }
}
impl W {
    ///Bits 16:20 - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes
    #[inline(always)]
    pub fn csbound(&mut self) -> CSBOUND_W<'_, DCR3rs> {
        CSBOUND_W::new(self, 16)
    }
}
/**OCTOSPI device configuration register 3

You can [`read`](crate::Reg::read) this register and get [`dcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#OCTOSPI:DCR3)*/
pub struct DCR3rs;
impl crate::RegisterSpec for DCR3rs {
    type Ux = u32;
}
///`read()` method returns [`dcr3::R`](R) reader structure
impl crate::Readable for DCR3rs {}
///`write(|w| ..)` method takes [`dcr3::W`](W) writer structure
impl crate::Writable for DCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR3 to value 0
impl crate::Resettable for DCR3rs {}
