#[doc = "Register `PTPTSSR` reader"]
pub type R = crate::R<PTPTSSRrs>;
#[doc = "Field `TSSO` reader - TSSO"]
pub type TSSO_R = crate::BitReader;
#[doc = "Field `TSTTR` reader - TSSO"]
pub type TSTTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TSSO"]
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSSO"]
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Ethernet PTP time stamp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSSRrs;
impl crate::RegisterSpec for PTPTSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptssr::R`](R) reader structure"]
impl crate::Readable for PTPTSSRrs {}
#[doc = "`reset()` method sets PTPTSSR to value 0"]
impl crate::Resettable for PTPTSSRrs {
    const RESET_VALUE: u32 = 0;
}
