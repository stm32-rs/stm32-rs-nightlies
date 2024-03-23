#[doc = "Register `BSLOTR` reader"]
pub type R = crate::R<BSLOTRrs>;
#[doc = "Register `BSLOTR` writer"]
pub type W = crate::W<BSLOTRrs>;
#[doc = "Field `FBOFF` reader - First bit offset"]
pub type FBOFF_R = crate::FieldReader;
#[doc = "Field `FBOFF` writer - First bit offset"]
pub type FBOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SLOTSZ` reader - Slot size"]
pub type SLOTSZ_R = crate::FieldReader;
#[doc = "Field `SLOTSZ` writer - Slot size"]
pub type SLOTSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBSLOT` reader - Number of slots in an audio frame"]
pub type NBSLOT_R = crate::FieldReader;
#[doc = "Field `NBSLOT` writer - Number of slots in an audio frame"]
pub type NBSLOT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLOTEN` reader - Slot enable"]
pub type SLOTEN_R = crate::FieldReader<u16>;
#[doc = "Field `SLOTEN` writer - Slot enable"]
pub type SLOTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    #[must_use]
    pub fn fboff(&mut self) -> FBOFF_W<BSLOTRrs> {
        FBOFF_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    #[must_use]
    pub fn slotsz(&mut self) -> SLOTSZ_W<BSLOTRrs> {
        SLOTSZ_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    #[must_use]
    pub fn nbslot(&mut self) -> NBSLOT_W<BSLOTRrs> {
        NBSLOT_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    #[must_use]
    pub fn sloten(&mut self) -> SLOTEN_W<BSLOTRrs> {
        SLOTEN_W::new(self, 16)
    }
}
#[doc = "B Slot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bslotr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bslotr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSLOTRrs;
impl crate::RegisterSpec for BSLOTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bslotr::R`](R) reader structure"]
impl crate::Readable for BSLOTRrs {}
#[doc = "`write(|w| ..)` method takes [`bslotr::W`](W) writer structure"]
impl crate::Writable for BSLOTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSLOTR to value 0"]
impl crate::Resettable for BSLOTRrs {
    const RESET_VALUE: u32 = 0;
}
