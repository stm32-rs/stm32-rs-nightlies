#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_R = crate::BitReader;
#[doc = "Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_R = crate::BitReader;
#[doc = "Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVEN` reader - LVEN"]
pub type LVEN_R = crate::BitReader;
#[doc = "Field `LVEN` writer - LVEN"]
pub type LVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LVEN"]
    #[inline(always)]
    pub fn lven(&self) -> LVEN_R {
        LVEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<ACRrs> {
        RUN_PD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
    #[doc = "Bit 15 - LVEN"]
    #[inline(always)]
    #[must_use]
    pub fn lven(&mut self) -> LVEN_W<ACRrs> {
        LVEN_W::new(self, 15)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0;
}
