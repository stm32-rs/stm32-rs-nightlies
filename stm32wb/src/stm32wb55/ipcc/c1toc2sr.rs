#[doc = "Register `C1TOC2SR` reader"]
pub type R = crate::R<C1TOC2SRrs>;
#[doc = "Field `CH1F` reader - processor 1 transmit to process 2 Receive channel 1 status flag"]
pub type CH1F_R = crate::BitReader;
#[doc = "Field `CH2F` reader - processor 1 transmit to process 2 Receive channel 2 status flag"]
pub type CH2F_R = crate::BitReader;
#[doc = "Field `CH3F` reader - processor 1 transmit to process 2 Receive channel 3 status flag"]
pub type CH3F_R = crate::BitReader;
#[doc = "Field `CH4F` reader - processor 1 transmit to process 2 Receive channel 4 status flag"]
pub type CH4F_R = crate::BitReader;
#[doc = "Field `CH5F` reader - processor 1 transmit to process 2 Receive channel 5 status flag"]
pub type CH5F_R = crate::BitReader;
#[doc = "Field `CH6F` reader - processor 1 transmit to process 2 Receive channel 6 status flag"]
pub type CH6F_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - processor 1 transmit to process 2 Receive channel 1 status flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - processor 1 transmit to process 2 Receive channel 2 status flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - processor 1 transmit to process 2 Receive channel 3 status flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - processor 1 transmit to process 2 Receive channel 4 status flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - processor 1 transmit to process 2 Receive channel 5 status flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - processor 1 transmit to process 2 Receive channel 6 status flag"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "CPU1 to CPU2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1toc2sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1TOC2SRrs;
impl crate::RegisterSpec for C1TOC2SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1toc2sr::R`](R) reader structure"]
impl crate::Readable for C1TOC2SRrs {}
#[doc = "`reset()` method sets C1TOC2SR to value 0"]
impl crate::Resettable for C1TOC2SRrs {
    const RESET_VALUE: u32 = 0;
}
