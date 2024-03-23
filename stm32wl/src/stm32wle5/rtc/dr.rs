#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DRrs>;
#[doc = "Field `DU` reader - Date units in BCD format"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units in BCD format"]
pub type DU_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `MU` reader - Month units in BCD format"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MU` writer - Month units in BCD format"]
pub type MU_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `MT` reader - Month tens in BCD format"]
pub type MT_R = crate::BitReader;
#[doc = "Field `MT` writer - Month tens in BCD format"]
pub type MT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDU` reader - Week day units"]
pub type WDU_R = crate::FieldReader;
#[doc = "Field `WDU` writer - Week day units"]
pub type WDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `YU` reader - Year units in BCD format"]
pub type YU_R = crate::FieldReader;
#[doc = "Field `YU` writer - Year units in BCD format"]
pub type YU_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `YT` reader - Year tens in BCD format"]
pub type YT_R = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens in BCD format"]
pub type YT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<DRrs> {
        DU_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<DRrs> {
        DT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<DRrs> {
        MU_W::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<DRrs> {
        MT_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    #[must_use]
    pub fn wdu(&mut self) -> WDU_W<DRrs> {
        WDU_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yu(&mut self) -> YU_W<DRrs> {
        YU_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yt(&mut self) -> YT_W<DRrs> {
        YT_W::new(self, 20)
    }
}
#[doc = "Date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0x2101"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0x2101;
}
