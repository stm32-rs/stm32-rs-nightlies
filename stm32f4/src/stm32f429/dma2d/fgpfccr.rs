#[doc = "Register `FGPFCCR` reader"]
pub type R = crate::R<FGPFCCRrs>;
#[doc = "Register `FGPFCCR` writer"]
pub type W = crate::W<FGPFCCRrs>;
#[doc = "Field `CM` reader - Color mode"]
pub type CM_R = crate::FieldReader;
#[doc = "Field `CM` writer - Color mode"]
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCM` reader - CLUT color mode"]
pub type CCM_R = crate::BitReader;
#[doc = "Field `CCM` writer - CLUT color mode"]
pub type CCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - CLUT size"]
pub type CS_R = crate::FieldReader;
#[doc = "Field `CS` writer - CLUT size"]
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AM` reader - Alpha mode"]
pub type AM_R = crate::FieldReader;
#[doc = "Field `AM` writer - Alpha mode"]
pub type AM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALPHA` reader - Alpha value"]
pub type ALPHA_R = crate::FieldReader;
#[doc = "Field `ALPHA` writer - Alpha value"]
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CLUT color mode"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<FGPFCCRrs> {
        CM_W::new(self, 0)
    }
    #[doc = "Bit 4 - CLUT color mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<FGPFCCRrs> {
        CCM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<FGPFCCRrs> {
        START_W::new(self, 5)
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<FGPFCCRrs> {
        CS_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<FGPFCCRrs> {
        AM_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<FGPFCCRrs> {
        ALPHA_W::new(self, 24)
    }
}
#[doc = "foreground PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgpfccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgpfccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FGPFCCRrs;
impl crate::RegisterSpec for FGPFCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fgpfccr::R`](R) reader structure"]
impl crate::Readable for FGPFCCRrs {}
#[doc = "`write(|w| ..)` method takes [`fgpfccr::W`](W) writer structure"]
impl crate::Writable for FGPFCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FGPFCCR to value 0"]
impl crate::Resettable for FGPFCCRrs {
    const RESET_VALUE: u32 = 0;
}
