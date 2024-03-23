#[doc = "Register `M5ISR` reader"]
pub type R = crate::R<M5ISRrs>;
#[doc = "Field `SEDC` reader - SEDC"]
pub type SEDC_R = crate::BitReader;
#[doc = "Field `DED` reader - DED"]
pub type DED_R = crate::BitReader;
#[doc = "Field `SRAMBUSY` reader - SRAMBUSY"]
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SEDC"]
    #[inline(always)]
    pub fn sedc(&self) -> SEDC_R {
        SEDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DED"]
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAMBUSY"]
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5ISRrs;
impl crate::RegisterSpec for M5ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5isr::R`](R) reader structure"]
impl crate::Readable for M5ISRrs {}
#[doc = "`reset()` method sets M5ISR to value 0"]
impl crate::Resettable for M5ISRrs {
    const RESET_VALUE: u32 = 0;
}
