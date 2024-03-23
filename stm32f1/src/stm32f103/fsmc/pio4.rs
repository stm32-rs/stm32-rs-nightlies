#[doc = "Register `PIO4` reader"]
pub type R = crate::R<PIO4rs>;
#[doc = "Register `PIO4` writer"]
pub type W = crate::W<PIO4rs>;
#[doc = "Field `IOSETx` reader - IOSETx"]
pub type IOSETX_R = crate::FieldReader;
#[doc = "Field `IOSETx` writer - IOSETx"]
pub type IOSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOWAITx` reader - IOWAITx"]
pub type IOWAITX_R = crate::FieldReader;
#[doc = "Field `IOWAITx` writer - IOWAITx"]
pub type IOWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOHOLDx` reader - IOHOLDx"]
pub type IOHOLDX_R = crate::FieldReader;
#[doc = "Field `IOHOLDx` writer - IOHOLDx"]
pub type IOHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOHIZx` reader - IOHIZx"]
pub type IOHIZX_R = crate::FieldReader;
#[doc = "Field `IOHIZx` writer - IOHIZx"]
pub type IOHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    pub fn iosetx(&self) -> IOSETX_R {
        IOSETX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    pub fn iowaitx(&self) -> IOWAITX_R {
        IOWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    pub fn ioholdx(&self) -> IOHOLDX_R {
        IOHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    pub fn iohizx(&self) -> IOHIZX_R {
        IOHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    #[must_use]
    pub fn iosetx(&mut self) -> IOSETX_W<PIO4rs> {
        IOSETX_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    #[must_use]
    pub fn iowaitx(&mut self) -> IOWAITX_W<PIO4rs> {
        IOWAITX_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    #[must_use]
    pub fn ioholdx(&mut self) -> IOHOLDX_W<PIO4rs> {
        IOHOLDX_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    #[must_use]
    pub fn iohizx(&mut self) -> IOHIZX_W<PIO4rs> {
        IOHIZX_W::new(self, 24)
    }
}
#[doc = "I/O space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIO4rs;
impl crate::RegisterSpec for PIO4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio4::R`](R) reader structure"]
impl crate::Readable for PIO4rs {}
#[doc = "`write(|w| ..)` method takes [`pio4::W`](W) writer structure"]
impl crate::Writable for PIO4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIO4 to value 0xfcfc_fcfc"]
impl crate::Resettable for PIO4rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
