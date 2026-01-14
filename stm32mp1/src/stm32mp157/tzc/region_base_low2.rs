///Register `REGION_BASE_LOW2` reader
pub type R = crate::R<REGION_BASE_LOW2rs>;
///Register `REGION_BASE_LOW2` writer
pub type W = crate::W<REGION_BASE_LOW2rs>;
///Field `BASE_ADDRESS_LOW` reader - BASE_ADDRESS_LOW
pub type BASE_ADDRESS_LOW_R = crate::FieldReader<u32>;
///Field `BASE_ADDRESS_LOW` writer - BASE_ADDRESS_LOW
pub type BASE_ADDRESS_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 12:31 - BASE_ADDRESS_LOW
    #[inline(always)]
    pub fn base_address_low(&self) -> BASE_ADDRESS_LOW_R {
        BASE_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_BASE_LOW2")
            .field("base_address_low", &self.base_address_low())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - BASE_ADDRESS_LOW
    #[inline(always)]
    pub fn base_address_low(&mut self) -> BASE_ADDRESS_LOW_W<'_, REGION_BASE_LOW2rs> {
        BASE_ADDRESS_LOW_W::new(self, 12)
    }
}
/**Base address low for regions 1 to 8.

You can [`read`](crate::Reg::read) this register and get [`region_base_low2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_base_low2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_BASE_LOW2)*/
pub struct REGION_BASE_LOW2rs;
impl crate::RegisterSpec for REGION_BASE_LOW2rs {
    type Ux = u32;
}
///`read()` method returns [`region_base_low2::R`](R) reader structure
impl crate::Readable for REGION_BASE_LOW2rs {}
///`write(|w| ..)` method takes [`region_base_low2::W`](W) writer structure
impl crate::Writable for REGION_BASE_LOW2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REGION_BASE_LOW2 to value 0
impl crate::Resettable for REGION_BASE_LOW2rs {}
