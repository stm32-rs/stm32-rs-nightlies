///Register `ARR` reader
pub type R = crate::R<ARRrs>;
///Register `ARR` writer
pub type W = crate::W<ARRrs>;
///Field `ARR` reader - Auto reload value
pub type ARR_R = crate::FieldReader<u16>;
///Field `ARR` writer - Auto reload value
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Auto reload value
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARR").field("arr", &self.arr()).finish()
    }
}
impl W {
    ///Bits 0:15 - Auto reload value
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**LPTIM3 autoreload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPTIM3:ARR)*/
pub struct ARRrs;
impl crate::RegisterSpec for ARRrs {
    type Ux = u32;
}
///`read()` method returns [`arr::R`](R) reader structure
impl crate::Readable for ARRrs {}
///`write(|w| ..)` method takes [`arr::W`](W) writer structure
impl crate::Writable for ARRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ARR to value 0x01
impl crate::Resettable for ARRrs {
    const RESET_VALUE: u32 = 0x01;
}
