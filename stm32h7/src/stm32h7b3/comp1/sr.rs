#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `C1VAL` reader - COMP channel 1 output status bit"]
pub type C1VAL_R = crate::BitReader;
#[doc = "Field `C2VAL` reader - COMP channel 2 output status bit"]
pub type C2VAL_R = crate::BitReader;
#[doc = "Field `C1IF` reader - COMP channel 1 Interrupt Flag"]
pub type C1IF_R = crate::BitReader;
#[doc = "Field `C2IF` reader - COMP channel 2 Interrupt Flag"]
pub type C2IF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - COMP channel 1 output status bit"]
    #[inline(always)]
    pub fn c1val(&self) -> C1VAL_R {
        C1VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMP channel 2 output status bit"]
    #[inline(always)]
    pub fn c2val(&self) -> C2VAL_R {
        C2VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2IF_R {
        C2IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Comparator status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
