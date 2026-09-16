///Register `BTABLE` reader
pub type R = crate::R<BTABLErs>;
///Register `BTABLE` writer
pub type W = crate::W<BTABLErs>;
///Field `BTABLE` reader - Buffer table
pub type BTABLE_R = crate::FieldReader<u16>;
///Field `BTABLE` writer - Buffer table
pub type BTABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    ///Bits 3:15 - Buffer table
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new((self.bits >> 3) & 0x1fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTABLE")
            .field("btable", &self.btable())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - Buffer table
    #[inline(always)]
    pub fn btable(&mut self) -> BTABLE_W<'_, BTABLErs> {
        BTABLE_W::new(self, 3)
    }
}
/**Buffer table address

You can [`read`](crate::Reg::read) this register and get [`btable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#USB:BTABLE)*/
pub struct BTABLErs;
impl crate::RegisterSpec for BTABLErs {
    type Ux = u16;
}
///`read()` method returns [`btable::R`](R) reader structure
impl crate::Readable for BTABLErs {}
///`write(|w| ..)` method takes [`btable::W`](W) writer structure
impl crate::Writable for BTABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BTABLE to value 0
impl crate::Resettable for BTABLErs {}
