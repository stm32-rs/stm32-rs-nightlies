#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `EWUP1` reader - Enable WKUP pin 1"]
pub type EWUP1_R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable WKUP pin 1"]
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable WKUP pin 2"]
pub type EWUP2_R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable WKUP pin 2"]
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP4` reader - Enable WKUP pin 4"]
pub type EWUP4_R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable WKUP pin 4"]
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP5` reader - Enable WKUP pin 5"]
pub type EWUP5_R = crate::BitReader;
#[doc = "Field `EWUP5` writer - Enable WKUP pin 5"]
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP6` reader - Enable WKUP pin 6"]
pub type EWUP6_R = crate::BitReader;
#[doc = "Field `EWUP6` writer - Enable WKUP pin 6"]
pub type EWUP6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP7` reader - Enable WKUP pin 7"]
pub type EWUP7_R = crate::BitReader;
#[doc = "Field `EWUP7` writer - Enable WKUP pin 7"]
pub type EWUP7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable WKUP pin 4"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable WKUP pin 5"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable WKUP pin 6"]
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable WKUP pin 7"]
    #[inline(always)]
    pub fn ewup7(&self) -> EWUP7_R {
        EWUP7_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<CSRrs> {
        EWUP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<CSRrs> {
        EWUP2_W::new(self, 9)
    }
    #[doc = "Bit 11 - Enable WKUP pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<CSRrs> {
        EWUP4_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable WKUP pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<CSRrs> {
        EWUP5_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable WKUP pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn ewup6(&mut self) -> EWUP6_W<CSRrs> {
        EWUP6_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable WKUP pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn ewup7(&mut self) -> EWUP7_W<CSRrs> {
        EWUP7_W::new(self, 14)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
