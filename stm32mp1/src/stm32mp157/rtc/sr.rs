#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `ALRAF` reader - ALRAF"]
pub type ALRAF_R = crate::BitReader;
#[doc = "Field `ALRBF` reader - ALRBF"]
pub type ALRBF_R = crate::BitReader;
#[doc = "Field `WUTF` reader - WUTF"]
pub type WUTF_R = crate::BitReader;
#[doc = "Field `TSF` reader - TSF"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSOVF` reader - TSOVF"]
pub type TSOVF_R = crate::BitReader;
#[doc = "Field `ITSF` reader - ITSF"]
pub type ITSF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ALRAF"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBF"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTF"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TSOVF"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITSF"]
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
