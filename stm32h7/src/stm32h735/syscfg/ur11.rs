#[doc = "Register `UR11` reader"]
pub type R = crate::R<UR11rs>;
#[doc = "Field `IWDG1M` reader - Independent Watchdog 1 mode"]
pub type IWDG1M_R = crate::BitReader;
impl R {
    #[doc = "Bit 16 - Independent Watchdog 1 mode"]
    #[inline(always)]
    pub fn iwdg1m(&self) -> IWDG1M_R {
        IWDG1M_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR11rs;
impl crate::RegisterSpec for UR11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur11::R`](R) reader structure"]
impl crate::Readable for UR11rs {}
#[doc = "`reset()` method sets UR11 to value 0"]
impl crate::Resettable for UR11rs {
    const RESET_VALUE: u32 = 0;
}
