#[doc = "Register `BSEC_DENABLE` reader"]
pub type R = crate::R<BSEC_DENABLErs>;
#[doc = "Register `BSEC_DENABLE` writer"]
pub type W = crate::W<BSEC_DENABLErs>;
#[doc = "Field `DFTEN` reader - DFTEN"]
pub type DFTEN_R = crate::BitReader;
#[doc = "Field `DFTEN` writer - DFTEN"]
pub type DFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGEN` reader - DBGEN"]
pub type DBGEN_R = crate::BitReader;
#[doc = "Field `DBGEN` writer - DBGEN"]
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIDEN` reader - NIDEN"]
pub type NIDEN_R = crate::BitReader;
#[doc = "Field `NIDEN` writer - NIDEN"]
pub type NIDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICEEN` reader - DEVICEEN"]
pub type DEVICEEN_R = crate::BitReader;
#[doc = "Field `DEVICEEN` writer - DEVICEEN"]
pub type DEVICEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDPEN` reader - HDPEN"]
pub type HDPEN_R = crate::BitReader;
#[doc = "Field `HDPEN` writer - HDPEN"]
pub type HDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDEN` reader - SPIDEN"]
pub type SPIDEN_R = crate::BitReader;
#[doc = "Field `SPIDEN` writer - SPIDEN"]
pub type SPIDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPNIDEN` reader - SPNIDEN"]
pub type SPNIDEN_R = crate::BitReader;
#[doc = "Field `SPNIDEN` writer - SPNIDEN"]
pub type SPNIDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP15SDISABLE` reader - CP15SDISABLE"]
pub type CP15SDISABLE_R = crate::FieldReader;
#[doc = "Field `CP15SDISABLE` writer - CP15SDISABLE"]
pub type CP15SDISABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CFGSDISABLE` reader - CFGSDISABLE"]
pub type CFGSDISABLE_R = crate::BitReader;
#[doc = "Field `CFGSDISABLE` writer - CFGSDISABLE"]
pub type CFGSDISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSWENABLE` reader - DBGSWENABLE"]
pub type DBGSWENABLE_R = crate::BitReader;
#[doc = "Field `DBGSWENABLE` writer - DBGSWENABLE"]
pub type DBGSWENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    pub fn dften(&self) -> DFTEN_R {
        DFTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    pub fn deviceen(&self) -> DEVICEEN_R {
        DEVICEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    pub fn spiden(&self) -> SPIDEN_R {
        SPIDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    pub fn spniden(&self) -> SPNIDEN_R {
        SPNIDEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    pub fn cp15sdisable(&self) -> CP15SDISABLE_R {
        CP15SDISABLE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    pub fn cfgsdisable(&self) -> CFGSDISABLE_R {
        CFGSDISABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    pub fn dbgswenable(&self) -> DBGSWENABLE_R {
        DBGSWENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dften(&mut self) -> DFTEN_W<BSEC_DENABLErs> {
        DFTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<BSEC_DENABLErs> {
        DBGEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    #[must_use]
    pub fn niden(&mut self) -> NIDEN_W<BSEC_DENABLErs> {
        NIDEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    #[must_use]
    pub fn deviceen(&mut self) -> DEVICEEN_W<BSEC_DENABLErs> {
        DEVICEEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hdpen(&mut self) -> HDPEN_W<BSEC_DENABLErs> {
        HDPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    #[must_use]
    pub fn spiden(&mut self) -> SPIDEN_W<BSEC_DENABLErs> {
        SPIDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    #[must_use]
    pub fn spniden(&mut self) -> SPNIDEN_W<BSEC_DENABLErs> {
        SPNIDEN_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    #[must_use]
    pub fn cp15sdisable(&mut self) -> CP15SDISABLE_W<BSEC_DENABLErs> {
        CP15SDISABLE_W::new(self, 7)
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    #[must_use]
    pub fn cfgsdisable(&mut self) -> CFGSDISABLE_W<BSEC_DENABLErs> {
        CFGSDISABLE_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn dbgswenable(&mut self) -> DBGSWENABLE_W<BSEC_DENABLErs> {
        DBGSWENABLE_W::new(self, 10)
    }
}
#[doc = "reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_denable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_denable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_DENABLErs;
impl crate::RegisterSpec for BSEC_DENABLErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_denable::R`](R) reader structure"]
impl crate::Readable for BSEC_DENABLErs {}
#[doc = "`write(|w| ..)` method takes [`bsec_denable::W`](W) writer structure"]
impl crate::Writable for BSEC_DENABLErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_DENABLE to value 0"]
impl crate::Resettable for BSEC_DENABLErs {
    const RESET_VALUE: u32 = 0;
}
