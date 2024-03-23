#[doc = "Register `OPFCCR` reader"]
pub type R = crate::R<OPFCCRrs>;
#[doc = "Register `OPFCCR` writer"]
pub type W = crate::W<OPFCCRrs>;
#[doc = "Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless\n\nValue on reset: 0"]
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
#[doc = "Field `CM` reader - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
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
#[doc = "Field `CM` writer - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
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
#[doc = "Swap Bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SB {
    #[doc = "0: Regular byte order"]
    Regular = 0,
    #[doc = "1: Bytes are swapped two by two"]
    SwapBytes = 1,
}
impl From<SB> for bool {
    #[inline(always)]
    fn from(variant: SB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SB` reader - Swap Bytes"]
pub type SB_R = crate::BitReader<SB>;
impl SB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SB {
        match self.bits {
            false => SB::Regular,
            true => SB::SwapBytes,
        }
    }
    #[doc = "Regular byte order"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == SB::Regular
    }
    #[doc = "Bytes are swapped two by two"]
    #[inline(always)]
    pub fn is_swap_bytes(&self) -> bool {
        *self == SB::SwapBytes
    }
}
#[doc = "Field `SB` writer - Swap Bytes"]
pub type SB_W<'a, REG> = crate::BitWriter<'a, REG, SB>;
impl<'a, REG> SB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular byte order"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(SB::Regular)
    }
    #[doc = "Bytes are swapped two by two"]
    #[inline(always)]
    pub fn swap_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SB::SwapBytes)
    }
}
#[doc = "Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI {
    #[doc = "0: Regular alpha"]
    RegularAlpha = 0,
    #[doc = "1: Inverted alpha"]
    InvertedAlpha = 1,
}
impl From<AI> for bool {
    #[inline(always)]
    fn from(variant: AI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` reader - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_R = crate::BitReader<AI>;
impl AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AI {
        match self.bits {
            false => AI::RegularAlpha,
            true => AI::InvertedAlpha,
        }
    }
    #[doc = "Regular alpha"]
    #[inline(always)]
    pub fn is_regular_alpha(&self) -> bool {
        *self == AI::RegularAlpha
    }
    #[doc = "Inverted alpha"]
    #[inline(always)]
    pub fn is_inverted_alpha(&self) -> bool {
        *self == AI::InvertedAlpha
    }
}
#[doc = "Field `AI` writer - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG, AI>;
impl<'a, REG> AI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular alpha"]
    #[inline(always)]
    pub fn regular_alpha(self) -> &'a mut crate::W<REG> {
        self.variant(AI::RegularAlpha)
    }
    #[doc = "Inverted alpha"]
    #[inline(always)]
    pub fn inverted_alpha(self) -> &'a mut crate::W<REG> {
        self.variant(AI::InvertedAlpha)
    }
}
#[doc = "Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBS {
    #[doc = "0: No Red Blue Swap (RGB or ARGB)"]
    Regular = 0,
    #[doc = "1: Red Blue Swap (BGR or ABGR)"]
    Swap = 1,
}
impl From<RBS> for bool {
    #[inline(always)]
    fn from(variant: RBS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBS` reader - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_R = crate::BitReader<RBS>;
impl RBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBS {
        match self.bits {
            false => RBS::Regular,
            true => RBS::Swap,
        }
    }
    #[doc = "No Red Blue Swap (RGB or ARGB)"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == RBS::Regular
    }
    #[doc = "Red Blue Swap (BGR or ABGR)"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == RBS::Swap
    }
}
#[doc = "Field `RBS` writer - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG, RBS>;
impl<'a, REG> RBS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Red Blue Swap (RGB or ARGB)"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(RBS::Regular)
    }
    #[doc = "Red Blue Swap (BGR or ABGR)"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut crate::W<REG> {
        self.variant(RBS::Swap)
    }
}
impl R {
    #[doc = "Bits 0:2 - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Swap Bytes"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<OPFCCRrs> {
        CM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Swap Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SB_W<OPFCCRrs> {
        SB_W::new(self, 8)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<OPFCCRrs> {
        AI_W::new(self, 20)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<OPFCCRrs> {
        RBS_W::new(self, 21)
    }
}
#[doc = "DMA2D output PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opfccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opfccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
