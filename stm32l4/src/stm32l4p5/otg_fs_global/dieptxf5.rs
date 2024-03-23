#[doc = "Register `DIEPTXF5` reader"]
pub type R = crate::R<DIEPTXF5rs>;
#[doc = "Register `DIEPTXF5` writer"]
pub type W = crate::W<DIEPTXF5rs>;
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFO6 transmit RAM start address"]
pub type INEPTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFO6 transmit RAM start address"]
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO6 transmit RAM start address"]
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
    #[doc = "Bits 0:15 - IN endpoint FIFO6 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<DIEPTXF5rs> {
        INEPTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<DIEPTXF5rs> {
        INEPTXFD_W::new(self, 16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF5rs;
impl crate::RegisterSpec for DIEPTXF5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf5::R`](R) reader structure"]
impl crate::Readable for DIEPTXF5rs {}
#[doc = "`write(|w| ..)` method takes [`dieptxf5::W`](W) writer structure"]
impl crate::Writable for DIEPTXF5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF5 to value 0"]
impl crate::Resettable for DIEPTXF5rs {
    const RESET_VALUE: u32 = 0;
}
