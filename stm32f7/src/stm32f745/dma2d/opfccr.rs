#[doc = "Register `OPFCCR` reader"]
pub type R = crate::R<OPFCCRrs>;
#[doc = "Register `OPFCCR` writer"]
pub type W = crate::W<OPFCCRrs>;
#[doc = "Color mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM {
    #[doc = "0: ARGB8888"]
    Argb8888 = 0,
    #[doc = "1: RGB888"]
    Rgb888 = 1,
    #[doc = "2: RGB565"]
    Rgb565 = 2,
    #[doc = "3: ARGB1555"]
    Argb1555 = 3,
    #[doc = "4: ARGB4444"]
    Argb4444 = 4,
}
impl From<CM> for u8 {
    #[inline(always)]
    fn from(variant: CM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CM {
    type Ux = u8;
}
#[doc = "Field `CM` reader - Color mode"]
pub type CM_R = crate::FieldReader<CM>;
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CM> {
        match self.bits {
            0 => Some(CM::Argb8888),
            1 => Some(CM::Rgb888),
            2 => Some(CM::Rgb565),
            3 => Some(CM::Argb1555),
            4 => Some(CM::Argb4444),
            _ => None,
        }
    }
    #[doc = "ARGB8888"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM::Argb8888
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM::Rgb888
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM::Rgb565
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM::Argb1555
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM::Argb4444
    }
}
#[doc = "Field `CM` writer - Color mode"]
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CM>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb8888)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb888)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb565)
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb1555)
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb4444)
    }
}
impl R {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<OPFCCRrs> {
        CM_W::new(self, 0)
    }
}
#[doc = "output PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opfccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opfccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPFCCRrs;
impl crate::RegisterSpec for OPFCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opfccr::R`](R) reader structure"]
impl crate::Readable for OPFCCRrs {}
#[doc = "`write(|w| ..)` method takes [`opfccr::W`](W) writer structure"]
impl crate::Writable for OPFCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPFCCR to value 0"]
impl crate::Resettable for OPFCCRrs {
    const RESET_VALUE: u32 = 0;
}
