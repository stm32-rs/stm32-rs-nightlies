#[doc = "Register `RCC_AXIDIVR` reader"]
pub type R = crate::R<RCC_AXIDIVRrs>;
#[doc = "Register `RCC_AXIDIVR` writer"]
pub type W = crate::W<RCC_AXIDIVRrs>;
#[doc = "Field `AXIDIV` reader - AXIDIV"]
pub type AXIDIV_R = crate::FieldReader;
#[doc = "Field `AXIDIV` writer - AXIDIV"]
pub type AXIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AXIDIVRDY` reader - AXIDIVRDY"]
pub type AXIDIVRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&self) -> AXIDIV_R {
        AXIDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - AXIDIVRDY"]
    #[inline(always)]
    pub fn axidivrdy(&self) -> AXIDIVRDY_R {
        AXIDIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    #[must_use]
    pub fn axidiv(&mut self) -> AXIDIV_W<RCC_AXIDIVRrs> {
        AXIDIV_W::new(self, 0)
    }
}
#[doc = "This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_axidivr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_axidivr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AXIDIVRrs;
impl crate::RegisterSpec for RCC_AXIDIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_axidivr::R`](R) reader structure"]
impl crate::Readable for RCC_AXIDIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_axidivr::W`](W) writer structure"]
impl crate::Writable for RCC_AXIDIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AXIDIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_AXIDIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
