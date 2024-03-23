#[doc = "Register `IOGCSR` reader"]
pub type R = crate::R<IOGCSRrs>;
#[doc = "Register `IOGCSR` writer"]
pub type W = crate::W<IOGCSRrs>;
#[doc = "Field `G1E` reader - Analog I/O group x enable"]
pub type G1E_R = crate::BitReader;
#[doc = "Field `G1E` writer - Analog I/O group x enable"]
pub type G1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G2E` reader - Analog I/O group x enable"]
pub type G2E_R = crate::BitReader;
#[doc = "Field `G2E` writer - Analog I/O group x enable"]
pub type G2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G3E` reader - Analog I/O group x enable"]
pub type G3E_R = crate::BitReader;
#[doc = "Field `G3E` writer - Analog I/O group x enable"]
pub type G3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G4E` reader - Analog I/O group x enable"]
pub type G4E_R = crate::BitReader;
#[doc = "Field `G4E` writer - Analog I/O group x enable"]
pub type G4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G5E` reader - Analog I/O group x enable"]
pub type G5E_R = crate::BitReader;
#[doc = "Field `G5E` writer - Analog I/O group x enable"]
pub type G5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G6E` reader - Analog I/O group x enable"]
pub type G6E_R = crate::BitReader;
#[doc = "Field `G6E` writer - Analog I/O group x enable"]
pub type G6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G7E` reader - Analog I/O group x enable"]
pub type G7E_R = crate::BitReader;
#[doc = "Field `G7E` writer - Analog I/O group x enable"]
pub type G7E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G8E` reader - Analog I/O group x enable"]
pub type G8E_R = crate::BitReader;
#[doc = "Field `G8E` writer - Analog I/O group x enable"]
pub type G8E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G1S` reader - Analog I/O group x status"]
pub type G1S_R = crate::BitReader;
#[doc = "Field `G2S` reader - Analog I/O group x status"]
pub type G2S_R = crate::BitReader;
#[doc = "Field `G3S` reader - Analog I/O group x status"]
pub type G3S_R = crate::BitReader;
#[doc = "Field `G4S` reader - Analog I/O group x status"]
pub type G4S_R = crate::BitReader;
#[doc = "Field `G5S` reader - Analog I/O group x status"]
pub type G5S_R = crate::BitReader;
#[doc = "Field `G6S` reader - Analog I/O group x status"]
pub type G6S_R = crate::BitReader;
#[doc = "Field `G7S` reader - Analog I/O group x status"]
pub type G7S_R = crate::BitReader;
#[doc = "Field `G8S` reader - Analog I/O group x status"]
pub type G8S_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g1e(&self) -> G1E_R {
        G1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g2e(&self) -> G2E_R {
        G2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g3e(&self) -> G3E_R {
        G3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g4e(&self) -> G4E_R {
        G4E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g5e(&self) -> G5E_R {
        G5E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g6e(&self) -> G6E_R {
        G6E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g7e(&self) -> G7E_R {
        G7E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g8e(&self) -> G8E_R {
        G8E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g1s(&self) -> G1S_R {
        G1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g2s(&self) -> G2S_R {
        G2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g3s(&self) -> G3S_R {
        G3S_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g4s(&self) -> G4S_R {
        G4S_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g5s(&self) -> G5S_R {
        G5S_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g6s(&self) -> G6S_R {
        G6S_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g7s(&self) -> G7S_R {
        G7S_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g8s(&self) -> G8S_R {
        G8S_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g1e(&mut self) -> G1E_W<IOGCSRrs> {
        G1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g2e(&mut self) -> G2E_W<IOGCSRrs> {
        G2E_W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g3e(&mut self) -> G3E_W<IOGCSRrs> {
        G3E_W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g4e(&mut self) -> G4E_W<IOGCSRrs> {
        G4E_W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g5e(&mut self) -> G5E_W<IOGCSRrs> {
        G5E_W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g6e(&mut self) -> G6E_W<IOGCSRrs> {
        G6E_W::new(self, 5)
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g7e(&mut self) -> G7E_W<IOGCSRrs> {
        G7E_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    #[must_use]
    pub fn g8e(&mut self) -> G8E_W<IOGCSRrs> {
        G8E_W::new(self, 7)
    }
}
#[doc = "I/O group control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iogcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iogcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOGCSRrs;
impl crate::RegisterSpec for IOGCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iogcsr::R`](R) reader structure"]
impl crate::Readable for IOGCSRrs {}
#[doc = "`write(|w| ..)` method takes [`iogcsr::W`](W) writer structure"]
impl crate::Writable for IOGCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOGCSR to value 0"]
impl crate::Resettable for IOGCSRrs {
    const RESET_VALUE: u32 = 0;
}
