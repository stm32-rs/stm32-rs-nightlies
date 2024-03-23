#[doc = "Register `CEC_CR` reader"]
pub type R = crate::R<CEC_CRrs>;
#[doc = "Register `CEC_CR` writer"]
pub type W = crate::W<CEC_CRrs>;
#[doc = "Field `CECEN` reader - CEC Enable"]
pub type CECEN_R = crate::BitReader;
#[doc = "Field `CECEN` writer - CEC Enable"]
pub type CECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSOM` reader - Tx Start Of Message"]
pub type TXSOM_R = crate::BitReader;
#[doc = "Field `TXEOM` reader - Tx End Of Message"]
pub type TXEOM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CEC Enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx Start Of Message"]
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx End Of Message"]
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<CEC_CRrs> {
        CECEN_W::new(self, 0)
    }
}
#[doc = "CEC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEC_CRrs;
impl crate::RegisterSpec for CEC_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cec_cr::R`](R) reader structure"]
impl crate::Readable for CEC_CRrs {}
#[doc = "`write(|w| ..)` method takes [`cec_cr::W`](W) writer structure"]
impl crate::Writable for CEC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEC_CR to value 0"]
impl crate::Resettable for CEC_CRrs {
    const RESET_VALUE: u32 = 0;
}
