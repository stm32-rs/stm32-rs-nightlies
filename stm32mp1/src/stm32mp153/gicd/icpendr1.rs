///Register `ICPENDR1` reader
pub type R = crate::R<ICPENDR1rs>;
///Register `ICPENDR1` writer
pub type W = crate::W<ICPENDR1rs>;
///Field `ICPENDR1` reader - ICPENDR1
pub type ICPENDR1_R = crate::FieldReader<u32>;
///Field `ICPENDR1` writer - ICPENDR1
pub type ICPENDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR1
    #[inline(always)]
    pub fn icpendr1(&self) -> ICPENDR1_R {
        ICPENDR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICPENDR1")
            .field("icpendr1", &self.icpendr1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR1
    #[inline(always)]
    pub fn icpendr1(&mut self) -> ICPENDR1_W<'_, ICPENDR1rs> {
        ICPENDR1_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICPENDR1)*/
pub struct ICPENDR1rs;
impl crate::RegisterSpec for ICPENDR1rs {
    type Ux = u32;
}
///`read()` method returns [`icpendr1::R`](R) reader structure
impl crate::Readable for ICPENDR1rs {}
///`write(|w| ..)` method takes [`icpendr1::W`](W) writer structure
impl crate::Writable for ICPENDR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICPENDR1 to value 0
impl crate::Resettable for ICPENDR1rs {}
