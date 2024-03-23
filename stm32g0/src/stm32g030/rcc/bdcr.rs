#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCRrs>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCRrs>;
#[doc = "Field `LSEON` reader - LSE oscillator enable"]
pub type LSEON_R = crate::BitReader;
#[doc = "Field `LSEON` writer - LSE oscillator enable"]
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - LSE oscillator ready"]
pub type LSERDY_R = crate::BitReader;
#[doc = "Field `LSERDY` writer - LSE oscillator ready"]
pub type LSERDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass"]
pub type LSEBYP_R = crate::BitReader;
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass"]
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability"]
pub type LSEDRV_R = crate::FieldReader;
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability"]
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LSECSSON` reader - CSS on LSE enable"]
pub type LSECSSON_R = crate::BitReader;
#[doc = "Field `LSECSSON` writer - CSS on LSE enable"]
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection"]
pub type LSECSSD_R = crate::BitReader;
#[doc = "Field `LSECSSD` writer - CSS on LSE failure Detection"]
pub type LSECSSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RTCSEL_R = crate::FieldReader;
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDRST` reader - RTC domain software reset"]
pub type BDRST_R = crate::BitReader;
#[doc = "Field `BDRST` writer - RTC domain software reset"]
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSCOEN` reader - Low-speed clock output (LSCO) enable"]
pub type LSCOEN_R = crate::BitReader;
#[doc = "Field `LSCOEN` writer - Low-speed clock output (LSCO) enable"]
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSCOSEL` reader - Low-speed clock output selection"]
pub type LSCOSEL_R = crate::BitReader;
#[doc = "Field `LSCOSEL` writer - Low-speed clock output selection"]
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable"]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low-speed clock output selection"]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<BDCRrs> {
        LSEON_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    #[must_use]
    pub fn lserdy(&mut self) -> LSERDY_W<BDCRrs> {
        LSERDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssd(&mut self) -> LSECSSD_W<BDCRrs> {
        LSECSSD_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - RTC domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<BDCRrs> {
        BDRST_W::new(self, 16)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable"]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Low-speed clock output selection"]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
}
#[doc = "RTC domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BDCRrs {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCRrs {
    const RESET_VALUE: u32 = 0;
}
