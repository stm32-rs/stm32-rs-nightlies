///Register `HRA2` reader
pub type R = crate::R<HRA2rs>;
///Register `HRA2` writer
pub type W = crate::W<HRA2rs>;
///Field `H1` reader - H2
pub type H1_R = crate::FieldReader<u32>;
///Field `H1` writer - H2
pub type H1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - H2
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRA2").field("h1", &self.h1()).finish()
    }
}
impl W {
    ///Bits 0:31 - H2
    #[inline(always)]
    pub fn h1(&mut self) -> H1_W<HRA2rs> {
        H1_W::new(self, 0)
    }
}
/**digest registers

You can [`read`](crate::Reg::read) this register and get [`hra2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hra2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#HASH:HRA2)*/
pub struct HRA2rs;
impl crate::RegisterSpec for HRA2rs {
    type Ux = u32;
}
///`read()` method returns [`hra2::R`](R) reader structure
impl crate::Readable for HRA2rs {}
///`write(|w| ..)` method takes [`hra2::W`](W) writer structure
impl crate::Writable for HRA2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HRA2 to value 0
impl crate::Resettable for HRA2rs {
    const RESET_VALUE: u32 = 0;
}
