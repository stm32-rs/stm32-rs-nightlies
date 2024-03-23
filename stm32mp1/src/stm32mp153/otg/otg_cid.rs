#[doc = "Register `OTG_CID` reader"]
pub type R = crate::R<OTG_CIDrs>;
#[doc = "Register `OTG_CID` writer"]
pub type W = crate::W<OTG_CIDrs>;
#[doc = "Field `PRODUCT_ID` reader - PRODUCT_ID"]
pub type PRODUCT_ID_R = crate::FieldReader<u32>;
#[doc = "Field `PRODUCT_ID` writer - PRODUCT_ID"]
pub type PRODUCT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PRODUCT_ID"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PRODUCT_ID"]
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<OTG_CIDrs> {
        PRODUCT_ID_W::new(self, 0)
    }
}
#[doc = "This is a register containing the Product ID as reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_cid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_cid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_CIDrs;
impl crate::RegisterSpec for OTG_CIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_cid::R`](R) reader structure"]
impl crate::Readable for OTG_CIDrs {}
#[doc = "`write(|w| ..)` method takes [`otg_cid::W`](W) writer structure"]
impl crate::Writable for OTG_CIDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_CID to value 0x4000"]
impl crate::Resettable for OTG_CIDrs {
    const RESET_VALUE: u32 = 0x4000;
}
