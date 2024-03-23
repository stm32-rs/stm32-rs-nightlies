#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `MSWU` reader - Master Timer Software update"]
pub type MSWU_R = crate::BitReader;
#[doc = "Field `MSWU` writer - Master Timer Software update"]
pub type MSWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASWU` reader - Timer A Software update"]
pub type TASWU_R = crate::BitReader;
#[doc = "Field `TASWU` writer - Timer A Software update"]
pub type TASWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSWU` reader - Timer B Software Update"]
pub type TBSWU_R = crate::BitReader;
#[doc = "Field `TBSWU` writer - Timer B Software Update"]
pub type TBSWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCSWU` reader - Timer C Software Update"]
pub type TCSWU_R = crate::BitReader;
#[doc = "Field `TCSWU` writer - Timer C Software Update"]
pub type TCSWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDSWU` reader - Timer D Software Update"]
pub type TDSWU_R = crate::BitReader;
#[doc = "Field `TDSWU` writer - Timer D Software Update"]
pub type TDSWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESWU` reader - Timer E Software Update"]
pub type TESWU_R = crate::BitReader;
#[doc = "Field `TESWU` writer - Timer E Software Update"]
pub type TESWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRST` reader - Master Counter software reset"]
pub type MRST_R = crate::BitReader;
#[doc = "Field `MRST` writer - Master Counter software reset"]
pub type MRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARST` reader - Timer A counter software reset"]
pub type TARST_R = crate::BitReader;
#[doc = "Field `TARST` writer - Timer A counter software reset"]
pub type TARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBRST` reader - Timer B counter software reset"]
pub type TBRST_R = crate::BitReader;
#[doc = "Field `TBRST` writer - Timer B counter software reset"]
pub type TBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCRST` reader - Timer C counter software reset"]
pub type TCRST_R = crate::BitReader;
#[doc = "Field `TCRST` writer - Timer C counter software reset"]
pub type TCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRST` reader - Timer D counter software reset"]
pub type TDRST_R = crate::BitReader;
#[doc = "Field `TDRST` writer - Timer D counter software reset"]
pub type TDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERST` reader - Timer E counter software reset"]
pub type TERST_R = crate::BitReader;
#[doc = "Field `TERST` writer - Timer E counter software reset"]
pub type TERST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&self) -> MSWU_R {
        MSWU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&self) -> TASWU_R {
        TASWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&self) -> TBSWU_R {
        TBSWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&self) -> TCSWU_R {
        TCSWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&self) -> TDSWU_R {
        TDSWU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&self) -> TESWU_R {
        TESWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    #[must_use]
    pub fn mswu(&mut self) -> MSWU_W<CR2rs> {
        MSWU_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    #[must_use]
    pub fn taswu(&mut self) -> TASWU_W<CR2rs> {
        TASWU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tbswu(&mut self) -> TBSWU_W<CR2rs> {
        TBSWU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tcswu(&mut self) -> TCSWU_W<CR2rs> {
        TCSWU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tdswu(&mut self) -> TDSWU_W<CR2rs> {
        TDSWU_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn teswu(&mut self) -> TESWU_W<CR2rs> {
        TESWU_W::new(self, 5)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn mrst(&mut self) -> MRST_W<CR2rs> {
        MRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<CR2rs> {
        TARST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<CR2rs> {
        TBRST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<CR2rs> {
        TCRST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<CR2rs> {
        TDRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<CR2rs> {
        TERST_W::new(self, 13)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
