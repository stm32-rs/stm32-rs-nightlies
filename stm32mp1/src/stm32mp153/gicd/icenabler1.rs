///Register `ICENABLER1` reader
pub type R = crate::R<ICENABLER1rs>;
///Register `ICENABLER1` writer
pub type W = crate::W<ICENABLER1rs>;
///Field `ICENABLER1` reader - ICENABLER1
pub type ICENABLER1_R = crate::FieldReader<u32>;
///Field `ICENABLER1` writer - ICENABLER1
pub type ICENABLER1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER1
    #[inline(always)]
    pub fn icenabler1(&self) -> ICENABLER1_R {
        ICENABLER1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICENABLER1")
            .field("icenabler1", &self.icenabler1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER1
    #[inline(always)]
    pub fn icenabler1(&mut self) -> ICENABLER1_W<'_, ICENABLER1rs> {
        ICENABLER1_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICENABLER1)*/
pub struct ICENABLER1rs;
impl crate::RegisterSpec for ICENABLER1rs {
    type Ux = u32;
}
///`read()` method returns [`icenabler1::R`](R) reader structure
impl crate::Readable for ICENABLER1rs {}
///`write(|w| ..)` method takes [`icenabler1::W`](W) writer structure
impl crate::Writable for ICENABLER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICENABLER1 to value 0
impl crate::Resettable for ICENABLER1rs {}
