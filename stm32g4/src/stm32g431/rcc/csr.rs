#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "LSI oscillator enable\n\nValue on reset: 0"]
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
#[doc = "Field `LSION` reader - LSI oscillator enable"]
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
#[doc = "Field `LSION` writer - LSI oscillator enable"]
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
#[doc = "LSI oscillator ready\n\nValue on reset: 0"]
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
#[doc = "Field `LSIRDY` reader - LSI oscillator ready"]
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
#[doc = "Option byte loader reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTFR {
    #[doc = "0: No reset has occured"]
    NoReset = 0,
    #[doc = "1: A reset has occured"]
    Reset = 1,
}
impl From<OBLRSTFR> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OBLRSTF_R = crate::BitReader<OBLRSTFR>;
impl OBLRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OBLRSTFR {
        match self.bits {
            false => OBLRSTFR::NoReset,
            true => OBLRSTFR::Reset,
        }
    }
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTFR::NoReset
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTFR::Reset
    }
}
#[doc = "Field `PINRSTF` reader - Pad reset flag"]
pub use OBLRSTF_R as PINRSTF_R;
#[doc = "Field `BORRSTF` reader - BOR flag"]
pub use OBLRSTF_R as BORRSTF_R;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub use OBLRSTF_R as SFTRSTF_R;
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub use OBLRSTF_R as IWDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub use OBLRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRSTF` reader - Low-power reset flag"]
pub use OBLRSTF_R as LPWRSTF_R;
impl R {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pad reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 23)
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
