#[doc = "Register `L1PFCR` reader"]
pub type R = crate::R<L1PFCRrs>;
#[doc = "Register `L1PFCR` writer"]
pub type W = crate::W<L1PFCRrs>;
#[doc = "Pixel Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF {
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
    #[doc = "5: L8 (8-bit luminance)"]
    L8 = 5,
    #[doc = "6: AL44 (4-bit alpha, 4-bit luminance)"]
    Al44 = 6,
    #[doc = "7: AL88 (8-bit alpha, 8-bit luminance)"]
    Al88 = 7,
}
impl From<PF> for u8 {
    #[inline(always)]
    fn from(variant: PF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PF {
    type Ux = u8;
}
#[doc = "Field `PF` reader - Pixel Format"]
pub type PF_R = crate::FieldReader<PF>;
impl PF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PF {
        match self.bits {
            0 => PF::Argb8888,
            1 => PF::Rgb888,
            2 => PF::Rgb565,
            3 => PF::Argb1555,
            4 => PF::Argb4444,
            5 => PF::L8,
            6 => PF::Al44,
            7 => PF::Al88,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB8888"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == PF::Argb8888
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == PF::Rgb888
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == PF::Rgb565
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == PF::Argb1555
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == PF::Argb4444
    }
    #[doc = "L8 (8-bit luminance)"]
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == PF::L8
    }
    #[doc = "AL44 (4-bit alpha, 4-bit luminance)"]
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == PF::Al44
    }
    #[doc = "AL88 (8-bit alpha, 8-bit luminance)"]
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == PF::Al88
    }
}
#[doc = "Field `PF` writer - Pixel Format"]
pub type PF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PF>;
impl<'a, REG> PF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Argb8888)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Rgb888)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Rgb565)
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Argb1555)
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Argb4444)
    }
    #[doc = "L8 (8-bit luminance)"]
    #[inline(always)]
    pub fn l8(self) -> &'a mut crate::W<REG> {
        self.variant(PF::L8)
    }
    #[doc = "AL44 (4-bit alpha, 4-bit luminance)"]
    #[inline(always)]
    pub fn al44(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Al44)
    }
    #[doc = "AL88 (8-bit alpha, 8-bit luminance)"]
    #[inline(always)]
    pub fn al88(self) -> &'a mut crate::W<REG> {
        self.variant(PF::Al88)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<L1PFCRrs> {
        PF_W::new(self, 0)
    }
}
#[doc = "LTDC Layer Pixel Format Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1pfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1pfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1PFCRrs;
impl crate::RegisterSpec for L1PFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1pfcr::R`](R) reader structure"]
impl crate::Readable for L1PFCRrs {}
#[doc = "`write(|w| ..)` method takes [`l1pfcr::W`](W) writer structure"]
impl crate::Writable for L1PFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1PFCR to value 0"]
impl crate::Resettable for L1PFCRrs {
    const RESET_VALUE: u32 = 0;
}
