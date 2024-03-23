#[doc = "Register `DIN` writer"]
pub type W = crate::W<DINrs>;
#[doc = "Field `DATAIN` writer - Data input Writing this register pushes the current register content into the FIFO, and the register takes the new value presented on the AHB bus. Reading this register returns zeros."]
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data input Writing this register pushes the current register content into the FIFO, and the register takes the new value presented on the AHB bus. Reading this register returns zeros."]
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DATAIN_W<DINrs> {
        DATAIN_W::new(self, 0)
    }
}
#[doc = "HASH data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINrs;
impl crate::RegisterSpec for DINrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`din::W`](W) writer structure"]
impl crate::Writable for DINrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIN to value 0"]
impl crate::Resettable for DINrs {
    const RESET_VALUE: u32 = 0;
}
