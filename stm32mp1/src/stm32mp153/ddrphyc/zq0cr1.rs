///Register `ZQ0CR1` reader
pub type R = crate::R<ZQ0CR1rs>;
///Register `ZQ0CR1` writer
pub type W = crate::W<ZQ0CR1rs>;
///Field `ZPROG` reader - ZPROG
pub type ZPROG_R = crate::FieldReader;
///Field `ZPROG` writer - ZPROG
pub type ZPROG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - ZPROG
    #[inline(always)]
    pub fn zprog(&self) -> ZPROG_R {
        ZPROG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZQ0CR1")
            .field("zprog", &self.zprog())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ZPROG
    #[inline(always)]
    pub fn zprog(&mut self) -> ZPROG_W<'_, ZQ0CR1rs> {
        ZPROG_W::new(self, 0)
    }
}
/**DDRPHYC ZQ0CR1 register

You can [`read`](crate::Reg::read) this register and get [`zq0cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zq0cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:ZQ0CR1)*/
pub struct ZQ0CR1rs;
impl crate::RegisterSpec for ZQ0CR1rs {
    type Ux = u8;
}
///`read()` method returns [`zq0cr1::R`](R) reader structure
impl crate::Readable for ZQ0CR1rs {}
///`write(|w| ..)` method takes [`zq0cr1::W`](W) writer structure
impl crate::Writable for ZQ0CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ZQ0CR1 to value 0x7b
impl crate::Resettable for ZQ0CR1rs {
    const RESET_VALUE: u8 = 0x7b;
}
