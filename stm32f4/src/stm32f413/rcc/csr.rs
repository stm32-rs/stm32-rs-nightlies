#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Internal low-speed oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    #[doc = "0: LSI oscillator Off"]
    Off = 0,
    #[doc = "1: LSI oscillator On"]
    On = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Off,
            true => LSION::On,
        }
    }
    #[doc = "LSI oscillator Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION::Off
    }
    #[doc = "LSI oscillator On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION::On
    }
}
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Off)
    }
    #[doc = "LSI oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::On)
    }
}
#[doc = "Internal low-speed oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYR {
    #[doc = "0: LSI oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSI oscillator ready"]
    Ready = 1,
}
impl From<LSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader<LSIRDYR>;
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYR {
        match self.bits {
            false => LSIRDYR::NotReady,
            true => LSIRDYR::Ready,
        }
    }
    #[doc = "LSI oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDYR::NotReady
    }
    #[doc = "LSI oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDYR::Ready
    }
}
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVFW {
    #[doc = "1: Clears the reset flag"]
    Clear = 1,
}
impl From<RMVFW> for bool {
    #[inline(always)]
    fn from(variant: RMVFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVFW>;
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RMVFW> {
        match self.bits {
            true => Some(RMVFW::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the reset flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVFW::Clear
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVFW>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the reset flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVFW::Clear)
    }
}
#[doc = "BOR reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTFR {
    #[doc = "0: No reset has occured"]
    NoReset = 0,
    #[doc = "1: A reset has occured"]
    Reset = 1,
}
impl From<BORRSTFR> for bool {
    #[inline(always)]
    fn from(variant: BORRSTFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub type BORRSTF_R = crate::BitReader<BORRSTFR>;
impl BORRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BORRSTFR {
        match self.bits {
            false => BORRSTFR::NoReset,
            true => BORRSTFR::Reset,
        }
    }
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BORRSTFR::NoReset
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BORRSTFR::Reset
    }
}
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG, BORRSTFR>;
impl<'a, REG> BORRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(BORRSTFR::NoReset)
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BORRSTFR::Reset)
    }
}
#[doc = "Field `PADRSTF` reader - PIN reset flag"]
pub use BORRSTF_R as PADRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub use BORRSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub use BORRSTF_R as SFTRSTF_R;
#[doc = "Field `WDGRSTF` reader - Independent watchdog reset flag"]
pub use BORRSTF_R as WDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub use BORRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub use BORRSTF_R as LPWRRSTF_R;
#[doc = "Field `PADRSTF` writer - PIN reset flag"]
pub use BORRSTF_W as PADRSTF_W;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub use BORRSTF_W as PORRSTF_W;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub use BORRSTF_W as SFTRSTF_W;
#[doc = "Field `WDGRSTF` writer - Independent watchdog reset flag"]
pub use BORRSTF_W as WDGRSTF_W;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub use BORRSTF_W as WWDGRSTF_W;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag"]
pub use BORRSTF_W as LPWRRSTF_W;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn wdgrstf(&self) -> WDGRSTF_R {
        WDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 24)
    }
    #[doc = "Bit 25 - BOR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<CSRrs> {
        BORRSTF_W::new(self, 25)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn padrstf(&mut self) -> PADRSTF_W<CSRrs> {
        PADRSTF_W::new(self, 26)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<CSRrs> {
        PORRSTF_W::new(self, 27)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<CSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdgrstf(&mut self) -> WDGRSTF_W<CSRrs> {
        WDGRSTF_W::new(self, 29)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<CSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
#[doc = "clock control &amp; status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x0e00_0000"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0e00_0000;
}
