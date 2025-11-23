///Register `ISENABLER0` reader
pub type R = crate::R<ISENABLER0rs>;
///Register `ISENABLER0` writer
pub type W = crate::W<ISENABLER0rs>;
///Field `ISENABLER0` reader - ISENABLER0
pub type ISENABLER0_R = crate::FieldReader<u32>;
///Field `ISENABLER0` writer - ISENABLER0
pub type ISENABLER0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER0
    #[inline(always)]
    pub fn isenabler0(&self) -> ISENABLER0_R {
        ISENABLER0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISENABLER0")
            .field("isenabler0", &self.isenabler0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER0
    #[inline(always)]
    pub fn isenabler0(&mut self) -> ISENABLER0_W<'_, ISENABLER0rs> {
        ISENABLER0_W::new(self, 0)
    }
}
/**For interrupts ID = 0 to ID = 31

You can [`read`](crate::Reg::read) this register and get [`isenabler0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISENABLER0)*/
pub struct ISENABLER0rs;
impl crate::RegisterSpec for ISENABLER0rs {
    type Ux = u32;
}
///`read()` method returns [`isenabler0::R`](R) reader structure
impl crate::Readable for ISENABLER0rs {}
///`write(|w| ..)` method takes [`isenabler0::W`](W) writer structure
impl crate::Writable for ISENABLER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISENABLER0 to value 0xffff
impl crate::Resettable for ISENABLER0rs {
    const RESET_VALUE: u32 = 0xffff;
}
