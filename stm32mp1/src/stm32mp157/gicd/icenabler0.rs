///Register `ICENABLER0` reader
pub type R = crate::R<ICENABLER0rs>;
///Register `ICENABLER0` writer
pub type W = crate::W<ICENABLER0rs>;
///Field `ICENABLER0` reader - ICENABLER0
pub type ICENABLER0_R = crate::FieldReader<u32>;
///Field `ICENABLER0` writer - ICENABLER0
pub type ICENABLER0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER0
    #[inline(always)]
    pub fn icenabler0(&self) -> ICENABLER0_R {
        ICENABLER0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER0")
            .field("icenabler0", &self.icenabler0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER0
    #[inline(always)]
    pub fn icenabler0(&mut self) -> ICENABLER0_W<'_, ICENABLER0rs> {
        ICENABLER0_W::new(self, 0)
    }
}
/**For interrupts ID = 0 to ID = 31

You can [`read`](crate::Reg::read) this register and get [`icenabler0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER0)*/
pub struct ICENABLER0rs;
impl crate::RegisterSpec for ICENABLER0rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler0::R`](R) reader structure
impl crate::Readable for ICENABLER0rs {}
///`write(|w| ..)` method takes [`icenabler0::W`](W) writer structure
impl crate::Writable for ICENABLER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER0 to value 0xffff
impl crate::Resettable for ICENABLER0rs {
    const RESET_VALUE: u32 = 0xffff;
}
