#[doc = "Register `DIEPTXF8` reader"]
pub type R = crate::R<DIEPTXF8rs>;
#[doc = "Register `DIEPTXF8` writer"]
pub type W = crate::W<DIEPTXF8rs>;
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<DIEPTXF8rs> {
        INEPTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<DIEPTXF8rs> {
        INEPTXFD_W::new(self, 16)
    }
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF8rs;
impl crate::RegisterSpec for DIEPTXF8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf8::R`](R) reader structure"]
impl crate::Readable for DIEPTXF8rs {}
#[doc = "`write(|w| ..)` method takes [`dieptxf8::W`](W) writer structure"]
impl crate::Writable for DIEPTXF8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF8 to value 0x0200_1200"]
impl crate::Resettable for DIEPTXF8rs {
    const RESET_VALUE: u32 = 0x0200_1200;
}
