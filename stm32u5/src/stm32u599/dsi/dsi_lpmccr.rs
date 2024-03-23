#[doc = "Register `DSI_LPMCCR` reader"]
pub type R = crate::R<DSI_LPMCCRrs>;
#[doc = "Field `VLPSIZE` reader - VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode."]
pub type VLPSIZE_R = crate::FieldReader;
#[doc = "Field `LPSIZE` reader - Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode."]
pub type LPSIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode."]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode."]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpmccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LPMCCRrs;
impl crate::RegisterSpec for DSI_LPMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpmccr::R`](R) reader structure"]
impl crate::Readable for DSI_LPMCCRrs {}
#[doc = "`reset()` method sets DSI_LPMCCR to value 0"]
impl crate::Resettable for DSI_LPMCCRrs {
    const RESET_VALUE: u32 = 0;
}
