///Register `OTG_HS_HAINT` reader
pub type R = crate::R<OTG_HS_HAINTrs>;
///Field `HAINT` reader - Channel interrupts
pub type HAINT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Channel interrupts
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_HAINT")
            .field("haint", &self.haint())
            .finish()
    }
}
/**OTG_HS Host all channels interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_haint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_HOST:OTG_HS_HAINT)*/
pub struct OTG_HS_HAINTrs;
impl crate::RegisterSpec for OTG_HS_HAINTrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_haint::R`](R) reader structure
impl crate::Readable for OTG_HS_HAINTrs {}
///`reset()` method sets OTG_HS_HAINT to value 0
impl crate::Resettable for OTG_HS_HAINTrs {}
