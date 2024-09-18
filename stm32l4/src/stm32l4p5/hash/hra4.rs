///Register `HRA4` reader
pub type R = crate::R<HRA4rs>;
///Register `HRA4` writer
pub type W = crate::W<HRA4rs>;
///Field `H1` reader - H4
pub type H1_R = crate::FieldReader<u32>;
///Field `H1` writer - H4
pub type H1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - H4
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRA4").field("h1", &self.h1()).finish()
    }
}
impl W {
    ///Bits 0:31 - H4
    #[inline(always)]
    #[must_use]
    pub fn h1(&mut self) -> H1_W<HRA4rs> {
        H1_W::new(self, 0)
    }
}
/**digest registers

You can [`read`](crate::Reg::read) this register and get [`hra4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hra4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#HASH:HRA4)*/
pub struct HRA4rs;
impl crate::RegisterSpec for HRA4rs {
    type Ux = u32;
}
///`read()` method returns [`hra4::R`](R) reader structure
impl crate::Readable for HRA4rs {}
///`write(|w| ..)` method takes [`hra4::W`](W) writer structure
impl crate::Writable for HRA4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HRA4 to value 0
impl crate::Resettable for HRA4rs {
    const RESET_VALUE: u32 = 0;
}
