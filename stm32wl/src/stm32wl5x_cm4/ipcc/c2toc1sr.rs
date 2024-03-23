#[doc = "Register `C2TOC1SR` reader"]
pub type R = crate::R<C2TOC1SRrs>;
#[doc = "CH1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1F {
    #[doc = "0: Channel free, data can be written by the sending processor. Generates a channel TX free interrupt to the current processor, when unmasked"]
    Free = 0,
    #[doc = "1: Channel occupied, data can be read by the receiving processor. Generates a channel RX occupied interrupt to the other processor, when unmasked"]
    Occupied = 1,
}
impl From<CH1F> for bool {
    #[inline(always)]
    fn from(variant: CH1F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1F` reader - CH1F"]
pub type CH1F_R = crate::BitReader<CH1F>;
impl CH1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1F {
        match self.bits {
            false => CH1F::Free,
            true => CH1F::Occupied,
        }
    }
    #[doc = "Channel free, data can be written by the sending processor. Generates a channel TX free interrupt to the current processor, when unmasked"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == CH1F::Free
    }
    #[doc = "Channel occupied, data can be read by the receiving processor. Generates a channel RX occupied interrupt to the other processor, when unmasked"]
    #[inline(always)]
    pub fn is_occupied(&self) -> bool {
        *self == CH1F::Occupied
    }
}
#[doc = "Field `CH2F` reader - CH2F"]
pub use CH1F_R as CH2F_R;
#[doc = "Field `CH3F` reader - CH3F"]
pub use CH1F_R as CH3F_R;
#[doc = "Field `CH4F` reader - CH4F"]
pub use CH1F_R as CH4F_R;
#[doc = "Field `CH5F` reader - CH5F"]
pub use CH1F_R as CH5F_R;
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
#[doc = "IPCC processor 2 to processor 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2toc1sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2TOC1SRrs;
impl crate::RegisterSpec for C2TOC1SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2toc1sr::R`](R) reader structure"]
impl crate::Readable for C2TOC1SRrs {}
#[doc = "`reset()` method sets C2TOC1SR to value 0"]
impl crate::Resettable for C2TOC1SRrs {
    const RESET_VALUE: u32 = 0;
}
