#[doc = "Register `TAMP_BKP7R` reader"]
pub type R = crate::R<TAMP_BKP7Rrs>;
#[doc = "Register `TAMP_BKP7R` writer"]
pub type W = crate::W<TAMP_BKP7Rrs>;
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
pub type BKP_R = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<TAMP_BKP7Rrs> {
        BKP_W::new(self, 0)
    }
}
#[doc = "TAMP backup 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp7r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp7r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_BKP7Rrs;
impl crate::RegisterSpec for TAMP_BKP7Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_bkp7r::R`](R) reader structure"]
impl crate::Readable for TAMP_BKP7Rrs {}
#[doc = "`write(|w| ..)` method takes [`tamp_bkp7r::W`](W) writer structure"]
impl crate::Writable for TAMP_BKP7Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_BKP7R to value 0"]
impl crate::Resettable for TAMP_BKP7Rrs {
    const RESET_VALUE: u32 = 0;
}
