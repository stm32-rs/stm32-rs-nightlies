#[doc = "Register `ECCNMIR` reader"]
pub type R = crate::R<ECCNMIRrs>;
#[doc = "Register `ECCNMIR` writer"]
pub type W = crate::W<ECCNMIRrs>;
#[doc = "Field `ECCNMI_MASK_EN` reader - NMI behavior setup when a double ECC error occurs on flitf data part"]
pub type ECCNMI_MASK_EN_R = crate::BitReader;
#[doc = "Field `ECCNMI_MASK_EN` writer - NMI behavior setup when a double ECC error occurs on flitf data part"]
pub type ECCNMI_MASK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part"]
    #[inline(always)]
    pub fn eccnmi_mask_en(&self) -> ECCNMI_MASK_EN_R {
        ECCNMI_MASK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part"]
    #[inline(always)]
    #[must_use]
    pub fn eccnmi_mask_en(&mut self) -> ECCNMI_MASK_EN_W<ECCNMIRrs> {
        ECCNMI_MASK_EN_W::new(self, 0)
    }
}
#[doc = "SBS flift ECC NMI mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccnmir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccnmir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCNMIRrs;
impl crate::RegisterSpec for ECCNMIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccnmir::R`](R) reader structure"]
impl crate::Readable for ECCNMIRrs {}
#[doc = "`write(|w| ..)` method takes [`eccnmir::W`](W) writer structure"]
impl crate::Writable for ECCNMIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCNMIR to value 0"]
impl crate::Resettable for ECCNMIRrs {
    const RESET_VALUE: u32 = 0;
}
