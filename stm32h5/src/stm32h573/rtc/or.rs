///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `OUT2_RMP` reader - RTC_OUT2 mapping
pub type OUT2_RMP_R = crate::BitReader;
///Field `OUT2_RMP` writer - RTC_OUT2 mapping
pub type OUT2_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RTC_OUT2 mapping
    #[inline(always)]
    pub fn out2_rmp(&self) -> OUT2_RMP_R {
        OUT2_RMP_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("out2_rmp", &self.out2_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - RTC_OUT2 mapping
    #[inline(always)]
    pub fn out2_rmp(&mut self) -> OUT2_RMP_W<'_, ORrs> {
        OUT2_RMP_W::new(self, 0)
    }
}
/**RTC option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RTC:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
