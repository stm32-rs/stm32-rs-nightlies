///Register `HRA1` reader
pub type R = crate::R<HRA1rs>;
///Register `HRA1` writer
pub type W = crate::W<HRA1rs>;
///Field `H1` reader - H1
pub type H1_R = crate::FieldReader<u32>;
///Field `H1` writer - H1
pub type H1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - H1
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRA1").field("h1", &self.h1()).finish()
    }
}
impl W {
    ///Bits 0:31 - H1
    #[inline(always)]
    pub fn h1(&mut self) -> H1_W<HRA1rs> {
        H1_W::new(self, 0)
    }
}
/**digest registers

You can [`read`](crate::Reg::read) this register and get [`hra1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hra1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#HASH:HRA1)*/
pub struct HRA1rs;
impl crate::RegisterSpec for HRA1rs {
    type Ux = u32;
}
///`read()` method returns [`hra1::R`](R) reader structure
impl crate::Readable for HRA1rs {}
///`write(|w| ..)` method takes [`hra1::W`](W) writer structure
impl crate::Writable for HRA1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HRA1 to value 0
impl crate::Resettable for HRA1rs {}
