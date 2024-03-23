#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UIF_R = crate::BitReader;
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIF(1-1)` reader - Capture/compare %s interrupt flag"]
pub type CCIF_R = crate::BitReader;
#[doc = "Field `CCIF(1-1)` writer - Capture/compare %s interrupt flag"]
pub type CCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type COMIF_R = crate::BitReader;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BIF_R = crate::BitReader;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCOF(1-1)` reader - Capture/Compare %s overcapture flag"]
pub type CCOF_R = crate::BitReader;
#[doc = "Field `CCOF(1-1)` writer - Capture/Compare %s overcapture flag"]
pub type CCOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Capture/compare (1-1) interrupt flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IF` field"]
    #[inline(always)]
    pub fn ccif(&self, n: u8) -> CCIF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIF_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/compare (1-1) interrupt flag"]
    #[inline(always)]
    pub fn ccif_iter(&self) -> impl Iterator<Item = CCIF_R> + '_ {
        (0..1).map(move |n| CCIF_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Capture/Compare (1-1) overcapture flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1OF` field"]
    #[inline(always)]
    pub fn ccof(&self, n: u8) -> CCOF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCOF_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) overcapture flag"]
    #[inline(always)]
    pub fn ccof_iter(&self) -> impl Iterator<Item = CCOF_R> + '_ {
        (0..1).map(move |n| CCOF_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Capture/compare (1-1) interrupt flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IF` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccif(&mut self, n: u8) -> CCIF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIF_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 1)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<SRrs> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<SRrs> {
        BIF_W::new(self, 7)
    }
    #[doc = "Capture/Compare (1-1) overcapture flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1OF` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccof(&mut self, n: u8) -> CCOF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCOF_W::new(self, n * 0 + 9)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 9)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
