#[doc = "Register `PWR_CR1` reader"]
pub type R = crate::R<PWR_CR1rs>;
#[doc = "Register `PWR_CR1` writer"]
pub type W = crate::W<PWR_CR1rs>;
#[doc = "Field `LPDS` reader - LPDS"]
pub type LPDS_R = crate::BitReader;
#[doc = "Field `LPDS` writer - LPDS"]
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCFG` reader - LPCFG"]
pub type LPCFG_R = crate::BitReader;
#[doc = "Field `LPCFG` writer - LPCFG"]
pub type LPCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDS` reader - LVDS"]
pub type LVDS_R = crate::BitReader;
#[doc = "Field `LVDS` writer - LVDS"]
pub type LVDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDEN` reader - PVDEN"]
pub type PVDEN_R = crate::BitReader;
#[doc = "Field `PVDEN` writer - PVDEN"]
pub type PVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PLS"]
pub type PLS_R = crate::FieldReader;
#[doc = "Field `PLS` writer - PLS"]
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBP` reader - DBP"]
pub type DBP_R = crate::BitReader;
#[doc = "Field `DBP` writer - DBP"]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVDEN` reader - AVDEN"]
pub type AVDEN_R = crate::BitReader;
#[doc = "Field `AVDEN` writer - AVDEN"]
pub type AVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALS` reader - ALS"]
pub type ALS_R = crate::FieldReader;
#[doc = "Field `ALS` writer - ALS"]
pub type ALS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - LPDS"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPCFG"]
    #[inline(always)]
    pub fn lpcfg(&self) -> LPCFG_R {
        LPCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LVDS"]
    #[inline(always)]
    pub fn lvds(&self) -> LVDS_R {
        LVDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PVDEN"]
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - DBP"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - AVDEN"]
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - ALS"]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPDS"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<PWR_CR1rs> {
        LPDS_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPCFG"]
    #[inline(always)]
    #[must_use]
    pub fn lpcfg(&mut self) -> LPCFG_W<PWR_CR1rs> {
        LPCFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - LVDS"]
    #[inline(always)]
    #[must_use]
    pub fn lvds(&mut self) -> LVDS_W<PWR_CR1rs> {
        LVDS_W::new(self, 2)
    }
    #[doc = "Bit 4 - PVDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pvden(&mut self) -> PVDEN_W<PWR_CR1rs> {
        PVDEN_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - PLS"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<PWR_CR1rs> {
        PLS_W::new(self, 5)
    }
    #[doc = "Bit 8 - DBP"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<PWR_CR1rs> {
        DBP_W::new(self, 8)
    }
    #[doc = "Bit 16 - AVDEN"]
    #[inline(always)]
    #[must_use]
    pub fn avden(&mut self) -> AVDEN_W<PWR_CR1rs> {
        AVDEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - ALS"]
    #[inline(always)]
    #[must_use]
    pub fn als(&mut self) -> ALS_W<PWR_CR1rs> {
        ALS_W::new(self, 17)
    }
}
#[doc = "Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_CR1rs;
impl crate::RegisterSpec for PWR_CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr1::R`](R) reader structure"]
impl crate::Readable for PWR_CR1rs {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr1::W`](W) writer structure"]
impl crate::Writable for PWR_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR1 to value 0"]
impl crate::Resettable for PWR_CR1rs {
    const RESET_VALUE: u32 = 0;
}
