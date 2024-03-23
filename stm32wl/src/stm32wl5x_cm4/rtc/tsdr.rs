#[doc = "Register `TSDR` reader"]
pub type R = crate::R<TSDRrs>;
#[doc = "Field `DU` reader - Date units in BCD format"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `MU` reader - Month units in BCD format"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MT` reader - Month tens in BCD format"]
pub type MT_R = crate::BitReader;
#[doc = "Field `WDU` reader - Week day units"]
pub type WDU_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
}
#[doc = "Timestamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSDRrs;
impl crate::RegisterSpec for TSDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsdr::R`](R) reader structure"]
impl crate::Readable for TSDRrs {}
#[doc = "`reset()` method sets TSDR to value 0"]
impl crate::Resettable for TSDRrs {
    const RESET_VALUE: u32 = 0;
}
