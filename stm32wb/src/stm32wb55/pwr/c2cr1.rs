#[doc = "Register `C2CR1` reader"]
pub type R = crate::R<C2CR1rs>;
#[doc = "Register `C2CR1` writer"]
pub type W = crate::W<C2CR1rs>;
#[doc = "Field `LPMS` reader - Low-power mode selection for CPU2"]
pub type LPMS_R = crate::FieldReader;
#[doc = "Field `LPMS` writer - Low-power mode selection for CPU2"]
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FPDR` reader - Flash power down mode during LPRun for CPU2"]
pub type FPDR_R = crate::BitReader;
#[doc = "Field `FPDR` writer - Flash power down mode during LPRun for CPU2"]
pub type FPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDS` reader - Flash power down mode during LPSleep for CPU2"]
pub type FPDS_R = crate::BitReader;
#[doc = "Field `FPDS` writer - Flash power down mode during LPSleep for CPU2"]
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEEWKUP` reader - BLE external wakeup signal"]
pub type BLEEWKUP_R = crate::BitReader;
#[doc = "Field `BLEEWKUP` writer - BLE external wakeup signal"]
pub type BLEEWKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_802EWKUP` reader - 802.15.4 external wakeup signal"]
pub type _802EWKUP_R = crate::BitReader;
#[doc = "Field `_802EWKUP` writer - 802.15.4 external wakeup signal"]
pub type _802EWKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Flash power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - BLE external wakeup signal"]
    #[inline(always)]
    pub fn bleewkup(&self) -> BLEEWKUP_R {
        BLEEWKUP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 802.15.4 external wakeup signal"]
    #[inline(always)]
    pub fn _802ewkup(&self) -> _802EWKUP_R {
        _802EWKUP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<C2CR1rs> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bit 4 - Flash power down mode during LPRun for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpdr(&mut self) -> FPDR_W<C2CR1rs> {
        FPDR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flash power down mode during LPSleep for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<C2CR1rs> {
        FPDS_W::new(self, 5)
    }
    #[doc = "Bit 14 - BLE external wakeup signal"]
    #[inline(always)]
    #[must_use]
    pub fn bleewkup(&mut self) -> BLEEWKUP_W<C2CR1rs> {
        BLEEWKUP_W::new(self, 14)
    }
    #[doc = "Bit 15 - 802.15.4 external wakeup signal"]
    #[inline(always)]
    #[must_use]
    pub fn _802ewkup(&mut self) -> _802EWKUP_W<C2CR1rs> {
        _802EWKUP_W::new(self, 15)
    }
}
#[doc = "CPU2 Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2CR1rs;
impl crate::RegisterSpec for C2CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cr1::R`](R) reader structure"]
impl crate::Readable for C2CR1rs {}
#[doc = "`write(|w| ..)` method takes [`c2cr1::W`](W) writer structure"]
impl crate::Writable for C2CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CR1 to value 0"]
impl crate::Resettable for C2CR1rs {
    const RESET_VALUE: u32 = 0;
}
