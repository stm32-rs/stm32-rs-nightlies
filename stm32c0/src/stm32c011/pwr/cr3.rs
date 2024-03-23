#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `EWUP1` reader - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register."]
pub type EWUP1_R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register."]
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register."]
pub type EWUP2_R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register."]
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP3` reader - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register."]
pub type EWUP3_R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register."]
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP4` reader - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type EWUP4_R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP6` reader - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register."]
pub type EWUP6_R = crate::BitReader;
#[doc = "Field `EWUP6` writer - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register."]
pub type EWUP6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
pub type APC_R = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIWUL` reader - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
pub type EIWUL_R = crate::BitReader;
#[doc = "Field `EIWUL` writer - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<CR3rs> {
        EWUP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<CR3rs> {
        EWUP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<CR3rs> {
        EWUP3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<CR3rs> {
        EWUP4_W::new(self, 3)
    }
    #[doc = "Bit 5 - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup6(&mut self) -> EWUP6_W<CR3rs> {
        EWUP6_W::new(self, 5)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<CR3rs> {
        APC_W::new(self, 10)
    }
    #[doc = "Bit 15 - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EIWUL_W<CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
#[doc = "PWR control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0x8000"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
