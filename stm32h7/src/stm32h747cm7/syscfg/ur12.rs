#[doc = "Register `UR12` reader"]
pub type R = crate::R<UR12rs>;
#[doc = "Field `IWDG2M` reader - Independent Watchdog 2 mode"]
pub type IWDG2M_R = crate::BitReader;
#[doc = "Field `SECURE` reader - Secure mode"]
pub type SECURE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Independent Watchdog 2 mode"]
    #[inline(always)]
    pub fn iwdg2m(&self) -> IWDG2M_R {
        IWDG2M_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Secure mode"]
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur12::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR12rs;
impl crate::RegisterSpec for UR12rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur12::R`](R) reader structure"]
impl crate::Readable for UR12rs {}
#[doc = "`reset()` method sets UR12 to value 0"]
impl crate::Resettable for UR12rs {
    const RESET_VALUE: u32 = 0;
}
