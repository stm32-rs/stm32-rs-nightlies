#[doc = "Register `OTG_DIEPTXF3` reader"]
pub type R = crate::R<OTG_DIEPTXF3rs>;
#[doc = "Register `OTG_DIEPTXF3` writer"]
pub type W = crate::W<OTG_DIEPTXF3rs>;
#[doc = "Field `INEPTXSA` reader - INEPTXSA"]
pub type INEPTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - INEPTXSA"]
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFD` reader - INEPTXFD"]
pub type INEPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - INEPTXFD"]
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - INEPTXSA"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - INEPTXFD"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INEPTXSA"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<OTG_DIEPTXF3rs> {
        INEPTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - INEPTXFD"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<OTG_DIEPTXF3rs> {
        INEPTXFD_W::new(self, 16)
    }
}
#[doc = "OTG device IN endpoint transmit FIFO 3 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DIEPTXF3rs;
impl crate::RegisterSpec for OTG_DIEPTXF3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_dieptxf3::R`](R) reader structure"]
impl crate::Readable for OTG_DIEPTXF3rs {}
#[doc = "`write(|w| ..)` method takes [`otg_dieptxf3::W`](W) writer structure"]
impl crate::Writable for OTG_DIEPTXF3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DIEPTXF3 to value 0x0200_0400"]
impl crate::Resettable for OTG_DIEPTXF3rs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
