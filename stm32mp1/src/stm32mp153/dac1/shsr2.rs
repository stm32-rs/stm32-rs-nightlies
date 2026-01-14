///Register `SHSR2` reader
pub type R = crate::R<SHSR2rs>;
///Register `SHSR2` writer
pub type W = crate::W<SHSR2rs>;
///Field `TSAMPLE2` reader - TSAMPLE2
pub type TSAMPLE2_R = crate::FieldReader<u16>;
///Field `TSAMPLE2` writer - TSAMPLE2
pub type TSAMPLE2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - TSAMPLE2
    #[inline(always)]
    pub fn tsample2(&self) -> TSAMPLE2_R {
        TSAMPLE2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHSR2")
            .field("tsample2", &self.tsample2())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - TSAMPLE2
    #[inline(always)]
    pub fn tsample2(&mut self) -> TSAMPLE2_W<'_, SHSR2rs> {
        TSAMPLE2_W::new(self, 0)
    }
}
/**This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`shsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SHSR2)*/
pub struct SHSR2rs;
impl crate::RegisterSpec for SHSR2rs {
    type Ux = u32;
}
///`read()` method returns [`shsr2::R`](R) reader structure
impl crate::Readable for SHSR2rs {}
///`write(|w| ..)` method takes [`shsr2::W`](W) writer structure
impl crate::Writable for SHSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHSR2 to value 0
impl crate::Resettable for SHSR2rs {}
