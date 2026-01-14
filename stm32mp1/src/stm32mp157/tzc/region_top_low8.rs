///Register `REGION_TOP_LOW8` reader
pub type R = crate::R<REGION_TOP_LOW8rs>;
///Register `REGION_TOP_LOW8` writer
pub type W = crate::W<REGION_TOP_LOW8rs>;
///Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW
pub type TOP_ADDRESS_LOW_R = crate::FieldReader<u32>;
///Field `TOP_ADDRESS_LOW` writer - TOP_ADDRESS_LOW
pub type TOP_ADDRESS_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 12:31 - TOP_ADDRESS_LOW
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_TOP_LOW8")
            .field("top_address_low", &self.top_address_low())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - TOP_ADDRESS_LOW
    #[inline(always)]
    pub fn top_address_low(&mut self) -> TOP_ADDRESS_LOW_W<'_, REGION_TOP_LOW8rs> {
        TOP_ADDRESS_LOW_W::new(self, 12)
    }
}
/**Top address bits \[31:12\] for region x.

You can [`read`](crate::Reg::read) this register and get [`region_top_low8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_top_low8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_TOP_LOW8)*/
pub struct REGION_TOP_LOW8rs;
impl crate::RegisterSpec for REGION_TOP_LOW8rs {
    type Ux = u32;
}
///`read()` method returns [`region_top_low8::R`](R) reader structure
impl crate::Readable for REGION_TOP_LOW8rs {}
///`write(|w| ..)` method takes [`region_top_low8::W`](W) writer structure
impl crate::Writable for REGION_TOP_LOW8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REGION_TOP_LOW8 to value 0x0fff
impl crate::Resettable for REGION_TOP_LOW8rs {
    const RESET_VALUE: u32 = 0x0fff;
}
