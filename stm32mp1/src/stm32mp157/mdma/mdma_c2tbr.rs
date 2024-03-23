#[doc = "Register `MDMA_C2TBR` reader"]
pub type R = crate::R<MDMA_C2TBRrs>;
#[doc = "Register `MDMA_C2TBR` writer"]
pub type W = crate::W<MDMA_C2TBRrs>;
#[doc = "Field `TSEL` reader - TSEL"]
pub type TSEL_R = crate::FieldReader;
#[doc = "Field `TSEL` writer - TSEL"]
pub type TSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SBUS` reader - SBUS"]
pub type SBUS_R = crate::BitReader;
#[doc = "Field `SBUS` writer - SBUS"]
pub type SBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS` reader - DBUS"]
pub type DBUS_R = crate::BitReader;
#[doc = "Field `DBUS` writer - DBUS"]
pub type DBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - TSEL"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SBUS"]
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DBUS"]
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - TSEL"]
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TSEL_W<MDMA_C2TBRrs> {
        TSEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - SBUS"]
    #[inline(always)]
    #[must_use]
    pub fn sbus(&mut self) -> SBUS_W<MDMA_C2TBRrs> {
        SBUS_W::new(self, 16)
    }
    #[doc = "Bit 17 - DBUS"]
    #[inline(always)]
    #[must_use]
    pub fn dbus(&mut self) -> DBUS_W<MDMA_C2TBRrs> {
        DBUS_W::new(self, 17)
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2tbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2tbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C2TBRrs;
impl crate::RegisterSpec for MDMA_C2TBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c2tbr::R`](R) reader structure"]
impl crate::Readable for MDMA_C2TBRrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c2tbr::W`](W) writer structure"]
impl crate::Writable for MDMA_C2TBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C2TBR to value 0"]
impl crate::Resettable for MDMA_C2TBRrs {
    const RESET_VALUE: u32 = 0;
}
