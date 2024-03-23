#[doc = "Register `TBR` reader"]
pub type R = crate::R<TBRrs>;
#[doc = "Register `TBR` writer"]
pub type W = crate::W<TBRrs>;
#[doc = "Field `TSEL` reader - Trigger selection"]
pub type TSEL_R = crate::FieldReader;
#[doc = "Field `TSEL` writer - Trigger selection"]
pub type TSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SBUS` reader - Source BUS select This bit is protected and can be written only if EN is 0."]
pub type SBUS_R = crate::BitReader;
#[doc = "Field `SBUS` writer - Source BUS select This bit is protected and can be written only if EN is 0."]
pub type SBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS` reader - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub type DBUS_R = crate::BitReader;
#[doc = "Field `DBUS` writer - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub type DBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TSEL_W<TBRrs> {
        TSEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sbus(&mut self) -> SBUS_W<TBRrs> {
        SBUS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn dbus(&mut self) -> DBUS_W<TBRrs> {
        DBUS_W::new(self, 17)
    }
}
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBRrs;
impl crate::RegisterSpec for TBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbr::R`](R) reader structure"]
impl crate::Readable for TBRrs {}
#[doc = "`write(|w| ..)` method takes [`tbr::W`](W) writer structure"]
impl crate::Writable for TBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBR to value 0"]
impl crate::Resettable for TBRrs {
    const RESET_VALUE: u32 = 0;
}
