#[doc = "Register `DSI_LCCCR` reader"]
pub type R = crate::R<DSI_LCCCRrs>;
#[doc = "Field `COLC` reader - Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit"]
pub type COLC_R = crate::FieldReader;
#[doc = "Field `LPE` reader - Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations."]
pub type LPE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations."]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host LTDC current color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LCCCRrs;
impl crate::RegisterSpec for DSI_LCCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lcccr::R`](R) reader structure"]
impl crate::Readable for DSI_LCCCRrs {}
#[doc = "`reset()` method sets DSI_LCCCR to value 0"]
impl crate::Resettable for DSI_LCCCRrs {
    const RESET_VALUE: u32 = 0;
}
