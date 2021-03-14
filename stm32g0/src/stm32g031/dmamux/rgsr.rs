#[doc = "Reader of register RGSR"]
pub type R = crate::R<u32, super::RGSR>;
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x0f) as u8)
    }
}
