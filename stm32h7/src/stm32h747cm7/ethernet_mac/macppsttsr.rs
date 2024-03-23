#[doc = "Register `MACPPSTTSR` reader"]
pub type R = crate::R<MACPPSTTSRrs>;
#[doc = "Register `MACPPSTTSR` writer"]
pub type W = crate::W<MACPPSTTSRrs>;
#[doc = "Field `TSTRH0` reader - PPS Target Time Seconds Register"]
pub type TSTRH0_R = crate::FieldReader<u32>;
#[doc = "Field `TSTRH0` writer - PPS Target Time Seconds Register"]
pub type TSTRH0_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - PPS Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - PPS Target Time Seconds Register"]
    #[inline(always)]
    #[must_use]
    pub fn tstrh0(&mut self) -> TSTRH0_W<MACPPSTTSRrs> {
        TSTRH0_W::new(self, 0)
    }
}
#[doc = "PPS target time seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsttsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsttsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSTTSRrs;
impl crate::RegisterSpec for MACPPSTTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsttsr::R`](R) reader structure"]
impl crate::Readable for MACPPSTTSRrs {}
#[doc = "`write(|w| ..)` method takes [`macppsttsr::W`](W) writer structure"]
impl crate::Writable for MACPPSTTSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPPSTTSR to value 0"]
impl crate::Resettable for MACPPSTTSRrs {
    const RESET_VALUE: u32 = 0;
}
