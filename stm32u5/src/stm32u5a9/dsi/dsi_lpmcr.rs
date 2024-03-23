#[doc = "Register `DSI_LPMCR` reader"]
pub type R = crate::R<DSI_LPMCRrs>;
#[doc = "Register `DSI_LPMCR` writer"]
pub type W = crate::W<DSI_LPMCRrs>;
#[doc = "Field `VLPSIZE` reader - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
pub type VLPSIZE_R = crate::FieldReader;
#[doc = "Field `VLPSIZE` writer - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
pub type VLPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPSIZE` reader - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
pub type LPSIZE_R = crate::FieldReader;
#[doc = "Field `LPSIZE` writer - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
pub type LPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
    #[inline(always)]
    #[must_use]
    pub fn vlpsize(&mut self) -> VLPSIZE_W<DSI_LPMCRrs> {
        VLPSIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
    #[inline(always)]
    #[must_use]
    pub fn lpsize(&mut self) -> LPSIZE_W<DSI_LPMCRrs> {
        LPSIZE_W::new(self, 16)
    }
}
#[doc = "DSI Host low-power mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lpmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LPMCRrs;
impl crate::RegisterSpec for DSI_LPMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpmcr::R`](R) reader structure"]
impl crate::Readable for DSI_LPMCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_lpmcr::W`](W) writer structure"]
impl crate::Writable for DSI_LPMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_LPMCR to value 0"]
impl crate::Resettable for DSI_LPMCRrs {
    const RESET_VALUE: u32 = 0;
}
