#[doc = "Register `IC1TOC2SR` reader"]
pub type R = crate::R<IC1TOC2SRrs>;
#[doc = "Field `CH1F` reader - CH1F"]
pub type CH1F_R = crate::BitReader;
#[doc = "Field `CH2F` reader - CH2F"]
pub type CH2F_R = crate::BitReader;
#[doc = "Field `CH3F` reader - CH3F"]
pub type CH3F_R = crate::BitReader;
#[doc = "Field `CH4F` reader - CH4F"]
pub type CH4F_R = crate::BitReader;
#[doc = "Field `CH5F` reader - CH5F"]
pub type CH5F_R = crate::BitReader;
#[doc = "Field `CH6F` reader - CH6F"]
pub type CH6F_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH3F"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH4F"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH5F"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH6F"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "IPCC processor 1 to processor 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ic1toc2sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC1TOC2SRrs;
impl crate::RegisterSpec for IC1TOC2SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic1toc2sr::R`](R) reader structure"]
impl crate::Readable for IC1TOC2SRrs {}
#[doc = "`reset()` method sets IC1TOC2SR to value 0"]
impl crate::Resettable for IC1TOC2SRrs {
    const RESET_VALUE: u32 = 0;
}
